mod application;
mod desktop_api;
mod domain;
mod infrastructure;
mod navigation_policy;

use std::{io::Write as _, net::Ipv4Addr, sync::Arc, time::Duration};

use application::{
    import::ImportService,
    integration::{IntegrationAdapter, IntegrationService},
    lan_bridge::{LanBridgePayloadHandler, LanBridgeRuntime, LanBridgeService},
    parity::{ParityService, RandomParityIds},
    planner::{PlannerService, RandomPlannerIds},
    workspace::{RandomUuidGenerator, WorkspaceService},
};
use infrastructure::{
    linux::{
        instance::{self, InstanceRole, SecondaryInstance, ACTIVATE_MAIN_V1, MAX_ACTIVATION_PAYLOAD_BYTES},
        integration::LinuxIntegrationAdapter,
        lan_bridge::LinuxLanBridgeRuntime,
        secrets::bootstrap_vault,
        xdg::{DesktopPaths, XdgEnvironment},
    },
    sqlite::{
        import::SqliteImportRepository, parity::SqliteParityRepository,
        repository::SqlitePlannerRepository, workspace::SqliteWorkspaceCatalog,
    },
};
use crate::domain::{
    integration::{decode_deep_link_activation, encode_deep_link_activation, parse_deep_link, DeepLinkIntent},
    lan_bridge::{seal_lan_bridge_payload, LanBridgeLifecycle, LanBridgeStartRequest, LAN_BRIDGE_NONCE_BYTES, LAN_BRIDGE_TOKEN_BYTES},
};
use tauri::{
    webview::{NewWindowResponse, WebviewWindowBuilder},
    Manager, WebviewUrl,
};


fn startup_deep_link() -> Result<Option<DeepLinkIntent>, String> {
    let mut found = None;
    for argument in std::env::args().skip(1) {
        if argument == "--integration-capabilities-json" {
            continue;
        }
        if !argument.starts_with("p2pkanban:") {
            continue;
        }
        if found.is_some() {
            return Err("only one p2pkanban deep link may be supplied per launch".to_owned());
        }
        found = Some(
            parse_deep_link(&argument)
                .map_err(|error| format!("invalid p2pkanban deep link: {error:?}"))?,
        );
    }
    Ok(found)
}

fn integration_probe_requested() -> bool {
    std::env::args().skip(1).any(|argument| argument == "--integration-capabilities-json")
}


fn lan_bridge_host_probe_requested() -> bool {
    std::env::args().skip(1).any(|argument| argument == "--lan-bridge-host-probe")
}

fn run_lan_bridge_host_probe() -> Result<(), String> {
    if std::env::var("P2PKANBAN_UTS_LAN_BRIDGE_PROBE").ok().as_deref() != Some("1") {
        return Err("LAN bridge host probe is reserved for UserTestSpace verification".to_owned());
    }
    let runtime = LinuxLanBridgeRuntime::host_probe();
    let request = LanBridgeStartRequest::validated(&Ipv4Addr::LOCALHOST.to_string(), 30)
        .map_err(|error| format!("invalid host-probe request: {error:?}"))?;
    let mut token = [0_u8; LAN_BRIDGE_TOKEN_BYTES];
    let mut nonce = [0_u8; LAN_BRIDGE_NONCE_BYTES];
    getrandom::fill(&mut token).map_err(|_| "host-probe randomness unavailable".to_owned())?;
    getrandom::fill(&mut nonce).map_err(|_| "host-probe randomness unavailable".to_owned())?;
    let payload = br#"{"kind":"a14-host-probe"}"#;
    let envelope = seal_lan_bridge_payload(payload, &token, &nonce)
        .map_err(|error| format!("unable to seal host-probe payload: {error:?}"))?;
    let handler: LanBridgePayloadHandler = Arc::new(|plaintext| {
        if plaintext == br#"{"kind":"a14-host-probe"}"# {
            Ok(r#"{"status":"accepted","probe":"a14"}"#.to_owned())
        } else {
            Err("unexpected-host-probe-payload".to_owned())
        }
    });
    let handle = runtime
        .start(request, token, handler)
        .map_err(|error| format!("unable to start host-probe bridge: {error:?}"))?;
    let initial = handle.status();
    let endpoint = initial.endpoint.clone().ok_or_else(|| "host-probe endpoint missing".to_owned())?;
    println!(
        "{}",
        serde_json::json!({
            "protocol": "p2p-kanban-lan-bridge/1",
            "endpoint": endpoint,
            "envelope": String::from_utf8(envelope).map_err(|_| "host-probe envelope was not UTF-8 JSON".to_owned())?,
            "expiresAtUnix": initial.expires_at_unix,
        })
    );
    std::io::stdout().flush().map_err(|error| format!("unable to flush host-probe descriptor: {error}"))?;

    for _ in 0..200 {
        std::thread::sleep(Duration::from_millis(50));
        let status = handle.status();
        match status.lifecycle {
            LanBridgeLifecycle::Completed => return Ok(()),
            LanBridgeLifecycle::Failed | LanBridgeLifecycle::Expired | LanBridgeLifecycle::Stopped => {
                return Err(format!("host-probe bridge terminated before acceptance: {:?} {:?}", status.lifecycle, status.last_result));
            }
            LanBridgeLifecycle::Listening => {}
        }
    }
    let _ = handle.stop();
    Err("host-probe bridge was not consumed within 10 seconds".to_owned())
}

fn print_integration_probe(prepared: &infrastructure::linux::xdg::PreparedDesktopPaths) {
    let adapter = LinuxIntegrationAdapter::new(prepared.activation_socket().is_some());
    let capabilities = adapter.detect();
    println!(
        "{}",
        serde_json::json!({
            "sessionType": capabilities.session.as_str(),
            "desktop": capabilities.desktop.unwrap_or_else(|| "unknown".to_owned()),
            "sessionBus": capabilities.session_bus.as_str(),
            "notifications": capabilities.notifications.as_str(),
            "statusNotifier": capabilities.status_notifier.as_str(),
            "portal": capabilities.portal.as_str(),
            "runtimeActivation": capabilities.runtime_activation.as_str(),
            "trayLifecycle": "disabled",
            "systemdUserService": "disabled",
        })
    );
}

fn run() -> Result<(), String> {
    let startup_deep_link = startup_deep_link()?;
    let prepared = DesktopPaths::resolve(&XdgEnvironment::current(), "default")
        .map_err(|err| format!("unable to resolve XDG profile paths: {err:?}"))?
        .prepare()
        .map_err(|err| format!("unable to prepare XDG profile paths: {err:?}"))?;

    if integration_probe_requested() {
        print_integration_probe(&prepared);
        return Ok(());
    }
    if lan_bridge_host_probe_requested() {
        return run_lan_bridge_host_probe();
    }

    let mut primary = match instance::acquire(&prepared)
        .map_err(|err| format!("unable to acquire profile instance control: {err:?}"))?
    {
        InstanceRole::Primary(primary) => primary,
        InstanceRole::Secondary(SecondaryInstance::Routed) => {
            if let Some(intent) = &startup_deep_link {
                let payload = encode_deep_link_activation(intent);
                if !instance::route_payload(&prepared, &payload) {
                    return Err("primary instance was activated but validated deep-link routing failed".to_owned());
                }
                eprintln!("p2pKanban is already running; validated deep link routed to the primary instance");
            } else {
                eprintln!("p2pKanban is already running; activation routed to the primary instance");
            }
            return Ok(());
        }
        InstanceRole::Secondary(SecondaryInstance::RoutingUnavailable) => {
            eprintln!(
                "p2pKanban is already running; writer ownership is protected but XDG runtime activation routing is unavailable"
            );
            return Ok(());
        }
    };

    let workspace_repository = SqliteWorkspaceCatalog::open(&prepared.paths.profile)
        .map_err(|err| format!("unable to open durable workspace catalog: {err:?}"))?;
    let workspace_service = WorkspaceService::new(
        Box::new(workspace_repository),
        Box::new(RandomUuidGenerator),
    );
    let planner_repository = SqlitePlannerRepository::open(&prepared.paths.profile)
        .map_err(|err| format!("unable to open durable planner repository: {err:?}"))?;
    let planner_service = PlannerService::new(
        Box::new(planner_repository),
        Box::new(RandomPlannerIds),
    );
    let vault_service = Arc::new(bootstrap_vault(&prepared.paths.profile, "default"));
    let import_repository = SqliteImportRepository::open(&prepared.paths.profile)
        .map_err(|err| format!("unable to open durable import repository: {err:?}"))?;
    let import_service = Arc::new(ImportService::new(Box::new(import_repository)));
    let parity_repository = SqliteParityRepository::open(&prepared.paths.profile)
        .map_err(|err| format!("unable to open durable parity repository: {err:?}"))?;
    let parity_service = ParityService::new(
        Box::new(parity_repository),
        Box::new(RandomParityIds),
    );
    let integration_service = IntegrationService::new(Box::new(LinuxIntegrationAdapter::new(
        prepared.activation_socket().is_some(),
    )));
    let lan_bridge_service = LanBridgeService::new(
        Box::new(LinuxLanBridgeRuntime::production()),
        Arc::clone(&import_service),
        Arc::clone(&vault_service),
    );
    if let Some(intent) = startup_deep_link {
        integration_service.enqueue_deep_link(intent);
    }

    let activation_receiver = primary.take_activation_receiver();
    let diagnostics = prepared.diagnostics();

    tauri::Builder::default()
        .manage(primary)
        .manage(diagnostics)
        .manage(application::ApplicationServices::desktop())
        .manage(workspace_service)
        .manage(planner_service)
        .manage(vault_service)
        .manage(import_service)
        .manage(parity_service)
        .manage(integration_service)
        .manage(lan_bridge_service)
        .invoke_handler(tauri::generate_handler![
            desktop_api::desktop_api_health,
            desktop_api::desktop_api_profile_diagnostics,
            desktop_api::desktop_api_vault_status,
            desktop_api::desktop_api_list_workspaces,
            desktop_api::desktop_api_create_workspace,
            desktop_api::desktop_api_list_boards,
            desktop_api::desktop_api_create_board,
            desktop_api::desktop_api_open_board,
            desktop_api::desktop_api_list_columns,
            desktop_api::desktop_api_create_column,
            desktop_api::desktop_api_list_cards,
            desktop_api::desktop_api_create_card,
            desktop_api::desktop_api_move_card,
            desktop_api::desktop_api_swap_card_order,
            desktop_api::desktop_api_set_card_archived,
            desktop_api::desktop_api_delete_card,
            desktop_api::desktop_api_list_checklists,
            desktop_api::desktop_api_create_checklist,
            desktop_api::desktop_api_delete_checklist,
            desktop_api::desktop_api_list_checklist_items,
            desktop_api::desktop_api_create_checklist_item,
            desktop_api::desktop_api_set_checklist_item_done,
            desktop_api::desktop_api_delete_checklist_item,
            desktop_api::desktop_api_pending_change_count,
            desktop_api::desktop_api_list_labels,
            desktop_api::desktop_api_create_label,
            desktop_api::desktop_api_delete_label,
            desktop_api::desktop_api_list_card_label_ids,
            desktop_api::desktop_api_set_card_label,
            desktop_api::desktop_api_list_comments,
            desktop_api::desktop_api_create_comment,
            desktop_api::desktop_api_delete_comment,
            desktop_api::desktop_api_get_appearance,
            desktop_api::desktop_api_set_appearance,
            desktop_api::desktop_api_list_activity,
            desktop_api::desktop_api_unsynced_parity_count,
            desktop_api::desktop_api_integration_capabilities,
            desktop_api::desktop_api_take_deep_link_intents,
            desktop_api::desktop_api_lan_bridge_addresses,
            desktop_api::desktop_api_lan_bridge_status,
            desktop_api::desktop_api_start_lan_bridge,
            desktop_api::desktop_api_stop_lan_bridge
        ])
        .setup(move |app| {
            WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                .title("p2pKanban")
                .inner_size(1180.0, 760.0)
                .min_inner_size(820.0, 560.0)
                .devtools(false)
                .on_navigation(navigation_policy::allows_top_level_navigation)
                .on_new_window(|_, _| NewWindowResponse::Deny)
                .on_download(|_, _| false)
                .build()?;

            if let Some(receiver) = activation_receiver {
                let handle = app.handle().clone();
                std::thread::spawn(move || {
                    let mut buf = [0_u8; MAX_ACTIVATION_PAYLOAD_BYTES];
                    while let Ok(read) = receiver.recv(&mut buf) {
                        let payload = &buf[..read];
                        let accepted = if payload == ACTIVATE_MAIN_V1 {
                            true
                        } else if let Ok(intent) = decode_deep_link_activation(payload) {
                            eprintln!(
                                "p2pKanban integration: accepted validated deep-link target={}",
                                intent.target.kind()
                            );
                            handle.state::<IntegrationService>().enqueue_deep_link(intent);
                            true
                        } else {
                            false
                        };
                        if !accepted {
                            continue;
                        }
                        if let Some(window) = handle.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                });
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .map_err(|err| format!("failed to run p2pKanban Arch-native shell: {err}"))
}

fn main() {
    if let Err(error) = run() {
        eprintln!("p2pKanban startup failed: {error}");
        std::process::exit(1);
    }
}
