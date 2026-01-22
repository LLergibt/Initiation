use std::sync::Arc;

use parking_lot::RwLock;

use crate::core::{Workspace};
use crate::core::ports::{WorkspaceStorage};
pub struct WorkspaceService {
    workspace: RwLock<Option<Workspace>>,
    storage: Arc<dyn WorkspaceStorage>
}
