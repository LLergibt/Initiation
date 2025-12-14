use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use parking_lot::RwLock;

use crate::core::{Workspace, WorkspaceInfo};
use crate::core::infrastructure::{ContentSource, MetaStore};

pub struct WorkspaceService {
    workspace: RwLock<Option<Workspace>>,
    content: Arc<dyn ContentSource>,
    meta: Arc<dyn MetaStore>,
}

// API Layer, called from Tauri command
impl WorkspaceService {

}
