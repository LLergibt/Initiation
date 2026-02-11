use std::{path::Path, sync::Arc, sync::RwLock};

use crate::core::{NodeId, NodeMeta};
use crate::core::domain::workspace;
use crate::core::errors::storage_error::{Result, StorageError};

use crate::core::impls::fs::fs_workspace_storage::{self, FsWorkspaceStorage};
use crate::core::{Workspace, WorkspaceInfo, ports::workspace_storage::WorkspaceStorage};

pub struct WorkspaceService {
    workspace: RwLock<Option<Workspace>>,
    storage: RwLock<Option<Arc<dyn WorkspaceStorage>>>
}

impl WorkspaceService {
    pub fn new() -> Self {
        Self {
            workspace: RwLock::new(None),
            storage: RwLock::new(None)

        }
    }

    fn storage(&self) -> Result<Arc<dyn WorkspaceStorage>> {
        let guard = self.storage.read().map_err(|_| StorageError::Other { 
            details: "storage locked".to_string(),
        })?;

        guard.clone().ok_or(StorageError::Other { 
            details: "workspace is not open".to_string(), 
        })
    }

    pub fn open_fs(&self, root: impl AsRef<Path>) -> Result<WorkspaceInfo> {
        let fs_workspace_storage = FsWorkspaceStorage::open(root)?;
        
        let ws_info = fs_workspace_storage.workspace_info()?;
        let mut workspace = self.workspace.write().unwrap();
        *workspace = Some(Workspace::new(ws_info.clone()));

        let mut storage = self.storage.write().unwrap();
        *storage = Some(Arc::new(fs_workspace_storage));


        Ok(ws_info)
    }

    pub fn is_open(&self) -> Result<bool> {
        Ok(!self.workspace.read().unwrap().is_none())
    }

    pub fn close(&self) -> Result<()> {

        let mut workspace = self.workspace.write().unwrap();
        *workspace = None;

        let mut storage = self.storage.write().unwrap();
        *storage = None;

        Ok(())
    }

    pub fn list_nodes(&self) -> Result<Vec<NodeMeta>> {
        self.storage()?.list_nodes()
    }

    pub fn load_note_text(&self, id: &NodeId) -> Result<String> {
        // Maybe needed some checks?
        
        self.storage()?.load_note_text(id)
    }

    pub fn create_note(&self, rel_path: &Path, title: &str, text: &str) -> Result<NodeMeta> {
        // Check path (path not exists)

        if self.storage()?.workspace_info()?.root_path.join(rel_path).exists() {
            return Err(StorageError::AlreadyExists { 
                what: "node", 
            })
        }

        self.storage()?.create_note(rel_path, title, text)
    }

    pub fn save_note_text(&self, id: &NodeId, text: &str) -> Result<NodeMeta> {     // Return NodeMeta?
        self.storage()?.save_note_text(id, text)
    }

}