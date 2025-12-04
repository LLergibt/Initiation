use std::path::Path;
use std::sync::Arc;
use anyhow::Result;
use chrono::Utc;

use crate::core::domain::{WorkspaceInfo};
use crate::core::infrastructure::{WorkspaceStorage};

pub struct Workspace {
    ws_info: WorkspaceInfo,
    storage: Arc<dyn WorkspaceStorage>
}
