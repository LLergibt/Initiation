use anyhow::Result;
use crate::core::{NodeId};

pub trait IndexPort {
    fn update_node(&self, id: &NodeId, raw: &str) -> Result<()>;
    fn backlinks(&self, id: &NodeId) -> Result<Vec<NodeId>>;
}