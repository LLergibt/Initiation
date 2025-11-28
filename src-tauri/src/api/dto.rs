use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct VaultInfoDto {
    pub name: String,
    pub root_path: String,
    pub opened_at: String,
    pub total_nodes: u32,
    pub notes_count: u32,
    pub assets_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct NoteListItemDto {
    pub id: String,
    pub title: String,
    pub path: String,
    pub tags: Vec<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct NoteContentDto {
    pub id: String,
    pub title: String,
    pub path: String,
    pub raw: String,
    pub html: String,
    pub tags: Vec<String>,
    pub pinned: bool,
}