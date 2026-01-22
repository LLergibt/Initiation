use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use parking_lot::RwLock;

use crate::core::{Workspace};
use crate::core::infrastructure::{WorkspaceStorage};

pub struct WorkspaceService {
    workspace: RwLock<Option<Workspace>>,
    storage: Arc<dyn WorkspaceStorage>
}
