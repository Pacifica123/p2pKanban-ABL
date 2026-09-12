mod application;
mod desktop_api;
mod domain;
mod infrastructure;
mod navigation_policy;

use tauri::{
    webview::{NewWindowResponse, WebviewWindowBuilder},
    WebviewUrl,
};

fn main() {
    tauri::Builder::default()
        .manage(application::ApplicationServices::desktop())
        .invoke_handler(tauri::generate_handler![desktop_api::desktop_api_health])
        .setup(|app| {
            WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                .title("p2pKanban")
                .inner_size(1180.0, 760.0)
                .min_inner_size(820.0, 560.0)
                .devtools(false)
                .on_navigation(navigation_policy::allows_top_level_navigation)
                .on_new_window(|_, _| NewWindowResponse::Deny)
                .on_download(|_, _| false)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run p2pKanban Arch-native shell");
}
