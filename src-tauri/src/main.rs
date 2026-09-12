mod application;
mod desktop_api;
mod domain;
mod infrastructure;
mod navigation_policy;

use infrastructure::linux::{
    instance::{self, InstanceRole, SecondaryInstance, ACTIVATE_MAIN_V1},
    xdg::{DesktopPaths, XdgEnvironment},
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

    let activation_receiver = primary.take_activation_receiver();
    let diagnostics = prepared.diagnostics();

    tauri::Builder::default()
        .manage(primary)
        .manage(diagnostics)
        .manage(application::ApplicationServices::desktop())
        .invoke_handler(tauri::generate_handler![
            desktop_api::desktop_api_health,
            desktop_api::desktop_api_profile_diagnostics
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
