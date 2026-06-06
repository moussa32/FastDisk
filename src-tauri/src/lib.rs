pub mod aggregator;
pub mod commands;
pub mod db;
pub mod events;
pub mod models;
pub mod repository;
pub mod scanner;

#[cfg(test)]
pub mod test_support;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|_app| {
            // FastDisk modules are registered here as implementation phases add commands.
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running FastDisk Viewer");
}
