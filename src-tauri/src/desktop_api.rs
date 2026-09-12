use std::collections::BTreeMap;

#[tauri::command]
pub fn desktop_api_health() -> BTreeMap<&'static str, String> {
    BTreeMap::from([
        ("status", "ok".to_string()),
        ("service", "p2pkanban-arch-native".to_string()),
        ("version", env!("CARGO_PKG_VERSION").to_string()),
        ("env", "desktop".to_string()),
    ])
}
