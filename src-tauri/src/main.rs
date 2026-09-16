mod application;
mod desktop_api;
mod domain;
mod infrastructure;
mod navigation_policy;

use application::{
    import::ImportService,
    integration::{IntegrationAdapter, IntegrationService},
    parity::{ParityService, RandomParityIds},
    planner::{PlannerService, RandomPlannerIds},
    workspace::{RandomUuidGenerator, WorkspaceService},
};
use infrastructure::{
    linux::{
        instance::{self, InstanceRole, SecondaryInstance, ACTIVATE_MAIN_V1, MAX_ACTIVATION_PAYLOAD_BYTES},
        integration::LinuxIntegrationAdapter,
        secrets::bootstrap_vault,
        xdg::{DesktopPaths, XdgEnvironment},
    },
    sqlite::{
        import::SqliteImportRepository, parity::SqliteParityRepository,
        repository::SqlitePlannerRepository, workspace::SqliteWorkspaceCatalog,
    },
};
use crate::domain::integration::{
    decode_deep_link_activation, encode_deep_link_activation, parse_deep_link, DeepLinkIntent,
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
    let vault_service = bootstrap_vault(&prepared.paths.profile, "default");
    let import_repository = SqliteImportRepository::open(&prepared.paths.profile)
        .map_err(|err| format!("unable to open durable import repository: {err:?}"))?;
    let import_service = ImportService::new(Box::new(import_repository));
    let parity_repository = SqliteParityRepository::open(&prepared.paths.profile)
        .map_err(|err| format!("unable to open durable parity repository: {err:?}"))?;
    let parity_service = ParityService::new(
        Box::new(parity_repository),
        Box::new(RandomParityIds),
    );
    let integration_service = IntegrationService::new(Box::new(LinuxIntegrationAdapter::new(
        prepared.activation_socket().is_some(),
    )));
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
            desktop_api::desktop_api_take_deep_link_intents
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
