use anyhow::Result;
use crate::core::{NodeId, NodeMeta};

// Meta control layer - everything we know about nodes and their meta
pub trait MetaStore {
    fn upsert_nodes(&self, nodes: &[NodeMeta]) -> Result<()>;
    fn get_node(&self, id: &NodeId) -> Result<Option<NodeMeta>>;
    fn list_nodes(&self) -> Result<Vec<NodeMeta>>;
    fn update_node(&self, node: &NodeMeta) -> Result<()>;
    fn delete_node(&self, id: &NodeId) -> Result<()>;
}
