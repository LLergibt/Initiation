use std::path::Path;
use serde::{Serialize, Deserialize};

use crate::core::errors::storage_error::Result;
use crate::core::{NodeId, NodeMeta, WorkspaceInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export, export_to = "../../bindings/")]
pub struct ResyncReport {
    pub scanned_files: u64,
    pub created_meta: u64,
    pub updated_meta: u64,
    pub orphan_meta: u64,
    pub orphan_files: u64,
}

pub trait WorkspaceStorage: Send + Sync {
    fn workspace_info(&self) -> Result<WorkspaceInfo>;

    fn resync(&self) -> Result<ResyncReport>;

    fn list_nodes(&self) -> Result<Vec<NodeMeta>>;
    fn get_node_meta(&self, id: &NodeId) -> Result<Option<NodeMeta>>;
    fn find_node_by_path(&self, rel_path: &Path) -> Result<Option<NodeMeta>>;

    fn load_note_text(&self, id: &NodeId) -> Result<String>;
    fn load_asset_bytes(&self, id: &NodeId) -> Result<Vec<u8>>;

    fn create_note(&self, rel_path: &Path, title: &str, initial_text: &str) -> Result<NodeMeta>;
    fn create_asset(&self, rel_path: &Path, display_name: &str, bytes: &[u8]) -> Result<NodeMeta>;

    fn save_note_text(&self, id: &NodeId, new_text: &str) -> Result<NodeMeta>;
    fn save_asset_bytes(&self, id: &NodeId, bytes: &[u8]) -> Result<NodeMeta>;

    fn update_meta(&self, meta: &NodeMeta) -> Result<NodeMeta>;

    fn rename_node(&self, id: &NodeId, new_rel_path: &Path) -> Result<NodeMeta>;
    fn delete_node(&self, id: &NodeId) -> Result<()>;
}