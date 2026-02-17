pub mod core;
pub mod api;

use crate::core::{WorkspaceService};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(WorkspaceService::new())
        .invoke_handler(tauri::generate_handler![
            greet,
            api::open_fs,
            api::list_nodes,
            api::load_note_text,
            api::create_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}