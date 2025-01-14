mod config;

use config::SnSConfiguration;
use log::warn;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _sns_conf: SnSConfiguration = SnSConfiguration::load();

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            warn!("Only a single instance of Shisō no Senshi can be running. Closing.");
            let _ = app
                .get_webview_window("main")
                .expect("Unable to switch to the running instance.")
                .set_focus();
        }))
        .plugin(tauri_plugin_os::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("An error occurred while starting Shisō no Sehshi.");
}
