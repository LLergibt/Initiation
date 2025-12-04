use std::path::PathBuf;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Note,
    Asset,
}

#[derive(Debug, Clone)]
pub struct NodeMeta {
    pub id: NodeId,
    pub kind: NodeKind,
    pub path: PathBuf,  // relative path inside the vault
    pub title: String,
    pub tags: Vec<String>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,

    pub color: Option<String>,

    pub size_bytes: Option<u64>, 
    pub content_hash: Option<String>, // maybe version?
}

#[derive(Debug, Clone)]
pub struct NodeContent {
    pub meta: NodeMeta,
    pub raw: String,
}
