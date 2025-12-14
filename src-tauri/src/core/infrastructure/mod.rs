use anyhow::Result;
use crate::core::{NodeId, NodeMeta, NodeContent};

pub mod fs;
pub use fs::{FsContentSource, FsMetaStore, FsWorkspaceStorage};

// Content control layer - works only with content, no meta, RW operation for files
pub trait ContentSource {
    fn read(&self, key: &str) -> Result<Vec<u8>>;
    fn write(&self, key: &str, data: &[u8]) -> Result<()>;
    fn delete(&self, key: &str) -> Result<()>;
    fn list(&self, prefix: &str) -> Result<Vec<String>>;
}

// Meta control layer - everything we know about nodes and their meta
pub trait MetaStore {
    fn upsert_nodes(&self, nodes: &[NodeMeta]) -> Result<()>;
    fn get_node(&self, id: &NodeId) -> Result<Option<NodeMeta>>;
    fn list_nodes(&self) -> Result<Vec<NodeMeta>>;
    fn update_node(&self, node: &NodeMeta) -> Result<()>;
    fn delete_node(&self, id: &NodeId) -> Result<()>;
}

// Nodes and its content
pub trait WorkspaceStorage: Send + Sync {
    fn resync(&self) -> Result<()>; // rescan or synchronize
    fn list_nodes(&self) -> Result<Vec<NodeMeta>>;
    fn get_node_meta(&self, id: &NodeId) -> Result<Option<NodeMeta>>;
    fn load_node(&self, id: &NodeId) -> Result<NodeContent>;
    fn create_node(&self, relative_path: &str, title: &str, initial_body: &str) -> Result<NodeContent>;
    fn save_node(&self, id: &NodeId, new_raw: &str) -> Result<NodeContent>;
    fn rename_node(&self, id: &NodeId, new_rel_path: &str) -> Result<NodeMeta>;
    fn delete_node(&self, id: &NodeId) -> Result<()>;
}
