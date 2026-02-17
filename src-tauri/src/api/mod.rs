use std::path::Path;

use tauri::State;

use crate::core::{NodeId, NodeMeta, WorkspaceService};

#[tauri::command]
pub fn open_fs(state: State<WorkspaceService>, root: String) -> Result<(), String> {
    state.open_fs(root).map(|_| ()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_nodes(state: State<WorkspaceService>) -> Result<Vec<NodeMeta>, String> {
    state.list_nodes().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_note_text(state: State<WorkspaceService>, id: u128) -> Result<String, String> {
    state
        .load_note_text(&NodeId(id))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_note(
    state: State<WorkspaceService>,
    rel_path: String,
    title: String,
    text: String,
) -> Result<NodeMeta, String> {
    state
        .create_note(Path::new(&rel_path), &title, &text)
        .map_err(|e| e.to_string())
}
