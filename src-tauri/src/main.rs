mod application;
mod desktop_api;
mod domain;
mod infrastructure;
mod navigation_policy;

use application::{
    import::ImportService,
    parity::{ParityService, RandomParityIds},
    planner::{PlannerService, RandomPlannerIds},
    workspace::{RandomUuidGenerator, WorkspaceService},
};
use infrastructure::{
    linux::{
        instance::{self, InstanceRole, SecondaryInstance, ACTIVATE_MAIN_V1},
        secrets::bootstrap_vault,
        xdg::{DesktopPaths, XdgEnvironment},
    },
    sqlite::{
        import::SqliteImportRepository, parity::SqliteParityRepository,
        repository::SqlitePlannerRepository, workspace::SqliteWorkspaceCatalog,
    },
};
use tauri::{
    webview::{NewWindowResponse, WebviewWindowBuilder},
    Manager, WebviewUrl,
};

fn run() -> Result<(), String> {
    let prepared = DesktopPaths::resolve(&XdgEnvironment::current(), "default")
        .map_err(|err| format!("unable to resolve XDG profile paths: {err:?}"))?
        .prepare()
        .map_err(|err| format!("unable to prepare XDG profile paths: {err:?}"))?;

    let mut primary = match instance::acquire(&prepared)
        .map_err(|err| format!("unable to acquire profile instance control: {err:?}"))?
    {
        InstanceRole::Primary(primary) => primary,
        InstanceRole::Secondary(SecondaryInstance::Routed) => {
            eprintln!("p2pKanban is already running; activation routed to the primary instance");
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
            desktop_api::desktop_api_unsynced_parity_count
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
                    let mut buf = [0_u8; 64];
                    while let Ok(read) = receiver.recv(&mut buf) {
                        if &buf[..read] != ACTIVATE_MAIN_V1 {
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
