use std::path::{Path, PathBuf};
use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct WorkspaceInfo {
    pub name: String,
    pub root_path: PathBuf,

    // pub opened_at: DateTime<Local>,
}

impl WorkspaceInfo {
    pub fn new(name: impl Into<String>, root_path: impl AsRef<Path>) -> Self {
        Self {
            name: name.into(),
            root_path: root_path.as_ref().to_path_buf()
            // opened_at: Local::now()
        }
    }
}
