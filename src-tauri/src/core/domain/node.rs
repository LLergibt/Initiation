use std::path::PathBuf;
use chrono::{DateTime, Utc};

// use super::vault::VaultId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Note,
    Asset,
}

#[derive(Debug, Clone)]
pub struct NodeMeta {
    // pub vault_id: VaultId,
    pub kind: NodeKind,

    pub path: PathBuf,  // relative path inside the vault
    pub title: String,
    pub tags: Vec<String>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,

    pub color: Option<String>,

    pub content_hash: Option<String>, // maybe version?
}

#[derive(Debug, Clone)]
pub struct NoteContent {
    pub meta: NodeMeta,
    pub raw: String,
}
