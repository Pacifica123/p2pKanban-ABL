use std::collections::BTreeMap;

use tauri::State;

use crate::application::{system::HealthView, ApplicationServices};

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

#[tauri::command]
pub fn desktop_api_health(app: State<'_, ApplicationServices>) -> BTreeMap<&'static str, String> {
    health_to_wire(app.system().health())
}

#[cfg(test)]
mod tests {
    use super::health_to_wire;
    use crate::application::system::HealthView;

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
}
