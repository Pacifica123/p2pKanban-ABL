use std::collections::BTreeMap;

use tauri::State;

use crate::{
    application::{
        system::HealthView,
        vault::{VaultMode, VaultService, VaultStatus},
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

fn vault_status_to_wire(status: VaultStatus) -> BTreeMap<&'static str, String> {
    let mode = match status.mode {
        VaultMode::SessionOnly => "session-only",
    };
    BTreeMap::from([
        ("mode", mode.to_owned()),
        ("durable", status.durable.to_string()),
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

#[cfg(test)]
mod tests {
    use super::{health_to_wire, profile_diagnostics_to_wire, vault_status_to_wire};
    use crate::{
        application::{
            system::HealthView,
            vault::{VaultMode, VaultStatus},
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
        let payload = vault_status_to_wire(VaultStatus {
            mode: VaultMode::SessionOnly,
            durable: false,
        });
        assert_eq!(payload.get("mode").map(String::as_str), Some("session-only"));
        assert_eq!(payload.get("durable").map(String::as_str), Some("false"));
        assert_eq!(payload.len(), 2);
    }
}
