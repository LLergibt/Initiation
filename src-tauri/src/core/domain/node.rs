use std::path::PathBuf;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeKind {
    Note,
    Asset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMeta {
    pub id: NodeId,
    pub kind: NodeKind,
    pub path: PathBuf,  // relative path inside the vault
    pub title: String,
    pub tags: Vec<String>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,

    pub color: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeContent {
    pub meta: NodeMeta,
    pub raw: String,
}
