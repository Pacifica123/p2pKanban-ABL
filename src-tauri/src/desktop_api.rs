use std::collections::BTreeMap;

use tauri::State;

use crate::{
    application::{
        planner::{
            CardView, ChecklistItemView, ChecklistView, ColumnView, PlannerService, PlannerServiceError,
        },
        system::HealthView,
        vault::{VaultMode, VaultService, VaultState, VaultStatus},
        workspace::{BoardView, WorkspaceService, WorkspaceServiceError, WorkspaceView},
        ApplicationServices,
    },
    infrastructure::linux::xdg::ProfileDiagnostics,
};

fn health_to_wire(view: HealthView) -> BTreeMap<&'static str, String> {
    BTreeMap::from([
        ("status", view.status.to_string()),
        ("service", view.service.to_string()),
        ("version", view.version.to_string()),
        ("env", view.environment.to_string()),
    ])
}

fn profile_diagnostics_to_wire(view: &ProfileDiagnostics) -> BTreeMap<&'static str, String> {
    BTreeMap::from([
        ("dataRoot", view.data_root.display().to_string()),
        ("configRoot", view.config_root.display().to_string()),
        ("stateRoot", view.state_root.display().to_string()),
        ("cacheRoot", view.cache_root.display().to_string()),
        ("profileDatabase", view.profile_database.display().to_string()),
        (
            "runtimeActivation",
            if view.runtime_activation_available { "available" } else { "unavailable" }.to_owned(),
        ),
    ])
}

fn workspace_to_wire(view: WorkspaceView) -> BTreeMap<&'static str, String> {
    BTreeMap::from([
        ("id", view.id),
        ("title", view.title),
        ("accessEpoch", view.access_epoch.to_string()),
    ])
}

fn board_to_wire(view: BoardView) -> BTreeMap<&'static str, String> {
    BTreeMap::from([
        ("id", view.id),
        ("workspaceId", view.workspace_id),
        ("title", view.title),
    ])
}


fn column_to_wire(view: ColumnView) -> BTreeMap<&'static str, String> {
    BTreeMap::from([
        ("id", view.id),
        ("boardId", view.board_id),
        ("title", view.title),
        ("position", view.position.to_string()),
    ])
}

fn card_to_wire(view: CardView) -> BTreeMap<&'static str, String> {
    BTreeMap::from([
        ("id", view.id),
        ("workspaceId", view.workspace_id),
        ("boardId", view.board_id),
        ("columnId", view.column_id),
        ("title", view.title),
        ("position", view.position.to_string()),
        ("archived", view.archived.to_string()),
    ])
}

fn checklist_to_wire(view: ChecklistView) -> BTreeMap<&'static str, String> {
    BTreeMap::from([
        ("id", view.id),
        ("cardId", view.card_id),
        ("title", view.title),
        ("position", view.position.to_string()),
    ])
}

fn checklist_item_to_wire(view: ChecklistItemView) -> BTreeMap<&'static str, String> {
    BTreeMap::from([
        ("id", view.id),
        ("checklistId", view.checklist_id),
        ("title", view.title),
        ("position", view.position.to_string()),
        ("isDone", view.is_done.to_string()),
    ])
}

fn planner_error_code(error: PlannerServiceError) -> String {
    match error {
        PlannerServiceError::EmptyTitle => "TITLE_REQUIRED",
        PlannerServiceError::TitleTooLong => "TITLE_TOO_LONG",
        PlannerServiceError::InvalidId => "INVALID_ID",
        PlannerServiceError::OrderExhausted => "ORDER_EXHAUSTED",
        PlannerServiceError::RepositoryPoisoned => "PLANNER_SERVICE_UNAVAILABLE",
        PlannerServiceError::Repository(inner) => match inner {
            crate::application::repository::RepositoryError::WorkspaceNotFound => "WORKSPACE_NOT_FOUND",
            crate::application::repository::RepositoryError::StaleAccessEpoch { .. } => "STALE_ACCESS_EPOCH",
            crate::application::repository::RepositoryError::BoardNotFound => "BOARD_NOT_FOUND",
            crate::application::repository::RepositoryError::ColumnNotFound => "COLUMN_NOT_FOUND",
            crate::application::repository::RepositoryError::CardNotFound => "CARD_NOT_FOUND",
            crate::application::repository::RepositoryError::ScopeMismatch => "SCOPE_MISMATCH",
            crate::application::repository::RepositoryError::DuplicateCard => "CARD_ALREADY_EXISTS",
            crate::application::repository::RepositoryError::DuplicateColumn => "COLUMN_ALREADY_EXISTS",
            crate::application::repository::RepositoryError::DuplicateChecklist => "CHECKLIST_ALREADY_EXISTS",
            crate::application::repository::RepositoryError::DuplicateChecklistItem => "CHECKLIST_ITEM_ALREADY_EXISTS",
            crate::application::repository::RepositoryError::ChecklistNotFound => "CHECKLIST_NOT_FOUND",
            crate::application::repository::RepositoryError::ChecklistItemNotFound => "CHECKLIST_ITEM_NOT_FOUND",
            crate::application::repository::RepositoryError::DuplicateReorderItem => "DUPLICATE_REORDER_ITEM",
            crate::application::repository::RepositoryError::Tombstoned => "ENTITY_TOMBSTONED",
            crate::application::repository::RepositoryError::StorageFailure => "PLANNER_STORAGE_FAILURE",
        },
    }
    .to_owned()
}

fn vault_status_to_wire(status: VaultStatus) -> BTreeMap<&'static str, String> {
    let mode = match status.mode {
        VaultMode::SessionOnly => "session-only",
        VaultMode::SecretService => "secret-service",
        VaultMode::Passphrase => "passphrase",
    };
    let state = match status.state {
        VaultState::Ready => "ready",
        VaultState::ProviderUnavailable => "provider-unavailable",
        VaultState::ProviderLocked => "provider-locked",
        VaultState::ProviderCorrupt => "provider-corrupt",
        VaultState::PassphraseRequired => "passphrase-required",
    };
    BTreeMap::from([
        ("mode", mode.to_owned()),
        ("state", state.to_owned()),
        ("durable", status.durable.to_string()),
        ("passphraseFallbackAvailable", status.passphrase_fallback_available.to_string()),
    ])
}

fn workspace_error_code(error: WorkspaceServiceError) -> String {
    match error {
        WorkspaceServiceError::EmptyTitle => "TITLE_REQUIRED",
        WorkspaceServiceError::TitleTooLong => "TITLE_TOO_LONG",
        WorkspaceServiceError::InvalidId => "INVALID_ID",
        WorkspaceServiceError::RepositoryPoisoned => "WORKSPACE_SERVICE_UNAVAILABLE",
        WorkspaceServiceError::Repository(inner) => match inner {
            crate::application::workspace::WorkspaceRepositoryError::WorkspaceNotFound => "WORKSPACE_NOT_FOUND",
            crate::application::workspace::WorkspaceRepositoryError::BoardNotFound => "BOARD_NOT_FOUND",
            crate::application::workspace::WorkspaceRepositoryError::DuplicateWorkspace => "WORKSPACE_ALREADY_EXISTS",
            crate::application::workspace::WorkspaceRepositoryError::DuplicateBoard => "BOARD_ALREADY_EXISTS",
            crate::application::workspace::WorkspaceRepositoryError::StorageFailure => "WORKSPACE_STORAGE_FAILURE",
        },
    }
    .to_owned()
}

#[tauri::command]
pub fn desktop_api_health(app: State<'_, ApplicationServices>) -> BTreeMap<&'static str, String> {
    health_to_wire(app.system().health())
}

#[tauri::command]
pub fn desktop_api_profile_diagnostics(
    diagnostics: State<'_, ProfileDiagnostics>,
) -> BTreeMap<&'static str, String> {
    profile_diagnostics_to_wire(&diagnostics)
}

#[tauri::command]
pub fn desktop_api_vault_status(
    vault: State<'_, VaultService>,
) -> BTreeMap<&'static str, String> {
    vault_status_to_wire(vault.status())
}

#[tauri::command]
pub fn desktop_api_list_workspaces(
    workspaces: State<'_, WorkspaceService>,
) -> Result<Vec<BTreeMap<&'static str, String>>, String> {
    workspaces
        .list_workspaces()
        .map(|items| items.into_iter().map(workspace_to_wire).collect())
        .map_err(workspace_error_code)
}

#[tauri::command]
pub fn desktop_api_create_workspace(
    title: String,
    workspaces: State<'_, WorkspaceService>,
) -> Result<BTreeMap<&'static str, String>, String> {
    workspaces
        .create_workspace(&title)
        .map(workspace_to_wire)
        .map_err(workspace_error_code)
}

#[allow(non_snake_case)]
#[tauri::command]
pub fn desktop_api_list_boards(
    workspaceId: String,
    workspaces: State<'_, WorkspaceService>,
) -> Result<Vec<BTreeMap<&'static str, String>>, String> {
    workspaces
        .list_boards(&workspaceId)
        .map(|items| items.into_iter().map(board_to_wire).collect())
        .map_err(workspace_error_code)
}

#[allow(non_snake_case)]
#[tauri::command]
pub fn desktop_api_create_board(
    workspaceId: String,
    title: String,
    workspaces: State<'_, WorkspaceService>,
) -> Result<BTreeMap<&'static str, String>, String> {
    workspaces
        .create_board(&workspaceId, &title)
        .map(board_to_wire)
        .map_err(workspace_error_code)
}

#[allow(non_snake_case)]
#[tauri::command]
pub fn desktop_api_open_board(
    workspaceId: String,
    boardId: String,
    workspaces: State<'_, WorkspaceService>,
) -> Result<BTreeMap<&'static str, String>, String> {
    workspaces
        .open_board(&workspaceId, &boardId)
        .map(board_to_wire)
        .map_err(workspace_error_code)
}


#[allow(non_snake_case)]
#[tauri::command]
pub fn desktop_api_list_columns(
    workspaceId: String,
    boardId: String,
    planner: State<'_, PlannerService>,
) -> Result<Vec<BTreeMap<&'static str, String>>, String> {
    planner
        .list_columns(&workspaceId, &boardId)
        .map(|items| items.into_iter().map(column_to_wire).collect())
        .map_err(planner_error_code)
}

#[allow(non_snake_case)]
#[tauri::command]
pub fn desktop_api_create_column(
    workspaceId: String,
    boardId: String,
    title: String,
    planner: State<'_, PlannerService>,
) -> Result<BTreeMap<&'static str, String>, String> {
    planner
        .create_column(&workspaceId, &boardId, &title)
        .map(column_to_wire)
        .map_err(planner_error_code)
}

#[allow(non_snake_case)]
#[tauri::command]
pub fn desktop_api_list_cards(
    workspaceId: String,
    boardId: String,
    includeArchived: bool,
    planner: State<'_, PlannerService>,
) -> Result<Vec<BTreeMap<&'static str, String>>, String> {
    planner
        .list_cards(&workspaceId, &boardId, includeArchived)
        .map(|items| items.into_iter().map(card_to_wire).collect())
        .map_err(planner_error_code)
}

#[allow(non_snake_case)]
#[tauri::command]
pub fn desktop_api_create_card(
    workspaceId: String,
    boardId: String,
    columnId: String,
    title: String,
    planner: State<'_, PlannerService>,
) -> Result<BTreeMap<&'static str, String>, String> {
    planner
        .create_card(&workspaceId, &boardId, &columnId, &title)
        .map(card_to_wire)
        .map_err(planner_error_code)
}

#[allow(non_snake_case)]
#[tauri::command]
pub fn desktop_api_move_card(
    workspaceId: String,
    cardId: String,
    targetColumnId: String,
    planner: State<'_, PlannerService>,
) -> Result<BTreeMap<&'static str, String>, String> {
    planner
        .move_card(&workspaceId, &cardId, &targetColumnId)
        .map(card_to_wire)
        .map_err(planner_error_code)
}

#[allow(non_snake_case)]
#[tauri::command]
pub fn desktop_api_swap_card_order(
    workspaceId: String,
    cardId: String,
    otherCardId: String,
    planner: State<'_, PlannerService>,
) -> Result<(), String> {
    planner
        .swap_card_order(&workspaceId, &cardId, &otherCardId)
        .map_err(planner_error_code)
}

#[allow(non_snake_case)]
#[tauri::command]
pub fn desktop_api_set_card_archived(
    workspaceId: String,
    cardId: String,
    archived: bool,
    planner: State<'_, PlannerService>,
) -> Result<BTreeMap<&'static str, String>, String> {
    planner
        .set_card_archived(&workspaceId, &cardId, archived)
        .map(card_to_wire)
        .map_err(planner_error_code)
}

#[allow(non_snake_case)]
#[tauri::command]
pub fn desktop_api_delete_card(
    workspaceId: String,
    cardId: String,
    planner: State<'_, PlannerService>,
) -> Result<(), String> {
    planner.delete_card(&workspaceId, &cardId).map_err(planner_error_code)
}

#[allow(non_snake_case)]
#[tauri::command]
pub fn desktop_api_list_checklists(
    workspaceId: String,
    cardId: String,
    planner: State<'_, PlannerService>,
) -> Result<Vec<BTreeMap<&'static str, String>>, String> {
    planner
        .list_checklists(&workspaceId, &cardId)
        .map(|items| items.into_iter().map(checklist_to_wire).collect())
        .map_err(planner_error_code)
}

#[allow(non_snake_case)]
#[tauri::command]
pub fn desktop_api_create_checklist(
    workspaceId: String,
    cardId: String,
    title: String,
    planner: State<'_, PlannerService>,
) -> Result<BTreeMap<&'static str, String>, String> {
    planner
        .create_checklist(&workspaceId, &cardId, &title)
        .map(checklist_to_wire)
        .map_err(planner_error_code)
}

#[allow(non_snake_case)]
#[tauri::command]
pub fn desktop_api_delete_checklist(
    workspaceId: String,
    checklistId: String,
    planner: State<'_, PlannerService>,
) -> Result<(), String> {
    planner
        .delete_checklist(&workspaceId, &checklistId)
        .map_err(planner_error_code)
}

#[allow(non_snake_case)]
#[tauri::command]
pub fn desktop_api_list_checklist_items(
    workspaceId: String,
    checklistId: String,
    planner: State<'_, PlannerService>,
) -> Result<Vec<BTreeMap<&'static str, String>>, String> {
    planner
        .list_checklist_items(&workspaceId, &checklistId)
        .map(|items| items.into_iter().map(checklist_item_to_wire).collect())
        .map_err(planner_error_code)
}

#[allow(non_snake_case)]
#[tauri::command]
pub fn desktop_api_create_checklist_item(
    workspaceId: String,
    checklistId: String,
    title: String,
    planner: State<'_, PlannerService>,
) -> Result<BTreeMap<&'static str, String>, String> {
    planner
        .create_checklist_item(&workspaceId, &checklistId, &title)
        .map(checklist_item_to_wire)
        .map_err(planner_error_code)
}

#[allow(non_snake_case)]
#[tauri::command]
pub fn desktop_api_set_checklist_item_done(
    workspaceId: String,
    itemId: String,
    done: bool,
    planner: State<'_, PlannerService>,
) -> Result<BTreeMap<&'static str, String>, String> {
    planner
        .set_checklist_item_done(&workspaceId, &itemId, done)
        .map(checklist_item_to_wire)
        .map_err(planner_error_code)
}

#[allow(non_snake_case)]
#[tauri::command]
pub fn desktop_api_delete_checklist_item(
    workspaceId: String,
    itemId: String,
    planner: State<'_, PlannerService>,
) -> Result<(), String> {
    planner
        .delete_checklist_item(&workspaceId, &itemId)
        .map_err(planner_error_code)
}

#[allow(non_snake_case)]
#[tauri::command]
pub fn desktop_api_pending_change_count(
    workspaceId: String,
    boardId: String,
    planner: State<'_, PlannerService>,
) -> Result<BTreeMap<&'static str, String>, String> {
    planner
        .pending_change_count(&workspaceId, &boardId)
        .map(|count| BTreeMap::from([("count", count.to_string())]))
        .map_err(planner_error_code)
}

#[cfg(test)]
mod tests {
    use super::{health_to_wire, profile_diagnostics_to_wire, vault_status_to_wire};
    use crate::{
        application::{
            system::HealthView,
            vault::{VaultState, VaultStatus},
        },
        infrastructure::linux::xdg::ProfileDiagnostics,
    };
    use std::path::PathBuf;

    #[test]
    fn adapter_maps_application_read_model_to_existing_wire_contract() {
        let payload = health_to_wire(HealthView {
            status: "ok",
            service: "native-test",
            version: "1.2.3",
            environment: "desktop",
        });

        assert_eq!(payload.get("status").map(String::as_str), Some("ok"));
        assert_eq!(payload.get("service").map(String::as_str), Some("native-test"));
        assert_eq!(payload.get("version").map(String::as_str), Some("1.2.3"));
        assert_eq!(payload.get("env").map(String::as_str), Some("desktop"));
    }

    #[test]
    fn diagnostics_adapter_exposes_only_resolved_paths_and_runtime_capability() {
        let payload = profile_diagnostics_to_wire(&ProfileDiagnostics {
            data_root: PathBuf::from("/tmp/data/p2pkanban"),
            config_root: PathBuf::from("/tmp/config/p2pkanban"),
            state_root: PathBuf::from("/tmp/state/p2pkanban"),
            cache_root: PathBuf::from("/tmp/cache/p2pkanban"),
            profile_database: PathBuf::from("/tmp/data/p2pkanban/profiles/default/profile.db"),
            runtime_activation_available: false,
        });
        assert_eq!(payload.get("runtimeActivation").map(String::as_str), Some("unavailable"));
        assert_eq!(payload.len(), 6);
    }

    #[test]
    fn vault_status_wire_exposes_capability_but_not_secret_operations() {
        let payload = vault_status_to_wire(VaultStatus::session_only(
            VaultState::ProviderUnavailable,
        ));
        assert_eq!(payload.get("mode").map(String::as_str), Some("session-only"));
        assert_eq!(
            payload.get("state").map(String::as_str),
            Some("provider-unavailable")
        );
        assert_eq!(payload.get("durable").map(String::as_str), Some("false"));
        assert_eq!(
            payload.get("passphraseFallbackAvailable").map(String::as_str),
            Some("true")
        );
        assert_eq!(payload.len(), 4);
    }
}
