use crate::core::domain::node::NodeId;
use crate::core::errors::storage_error::Result;


pub trait IndexPort {
    fn update_node(&self, id: &NodeId, raw: &str) -> Result<()>;
    fn backlinks(&self, id: &NodeId) -> Result<Vec<NodeId>>;
    fn remove_node(&self, id: &NodeId) -> Result<()>;
}