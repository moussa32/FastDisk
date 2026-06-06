pub mod aggregator;
pub mod commands;
pub mod db;
pub mod events;
pub mod models;
pub mod repository;
pub mod scanner;

#[cfg(test)]
pub mod test_support;

use commands::scan::{cancel_scan, get_scan_session, start_scan, AppState};
use commands::tree::get_children;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState::new(default_database_path()))
        .invoke_handler(tauri::generate_handler![
            start_scan,
            cancel_scan,
            get_scan_session,
            get_children
        ])
        .setup(|_app| {
            // FastDisk modules are registered here as implementation phases add commands.
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running FastDisk Viewer");
}

fn default_database_path() -> std::path::PathBuf {
    std::env::temp_dir().join("fastdisk-viewer.sqlite")
}
