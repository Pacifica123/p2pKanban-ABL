use std::collections::BTreeMap;

use tauri::State;

use crate::{
    application::{system::HealthView, ApplicationServices},
    infrastructure::linux::xdg::ProfileDiagnostics,
};

/// IPC DTO mapping stays in the platform adapter. Application/domain modules
/// never return transport envelopes or Tauri-specific types.
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

#[cfg(test)]
mod tests {
    use super::{health_to_wire, profile_diagnostics_to_wire};
    use crate::{
        application::system::HealthView,
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
}
