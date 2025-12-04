use std::path::PathBuf;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct WorkspaceInfo {
    pub name: String,
    pub root_path: PathBuf,

    pub opened_at: DateTime<Utc>,
}
