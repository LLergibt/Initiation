use std::{path::Path, sync::Arc, sync::RwLock};

use crate::core::{NodeId, NodeMeta};
use crate::core::errors::storage_error::{Result, StorageError};

use crate::core::impls::fs::fs_workspace_storage::FsWorkspaceStorage;
use crate::core::{WorkspaceInfo, ports::workspace_storage::{ResyncReport, WorkspaceStorage}};
use crate::core::validation::validate_rel_path;

struct WorkspaceState {
    info: WorkspaceInfo,
    storage: Arc<dyn WorkspaceStorage>,
}

pub struct WorkspaceService {
    state: RwLock<Option<WorkspaceState>>,
}

impl WorkspaceService {
    pub fn new() -> Self {
        Self {
            state: RwLock::new(None),
        }
    }

    fn storage(&self) -> Result<Arc<dyn WorkspaceStorage>> {
        let state = self.state.read().map_err(|_| StorageError::Other {
            details: "lock poisoned".to_string(),
        })?;
        state
            .as_ref()
            .map(|s| s.storage.clone())
            .ok_or(StorageError::Other {
                details: "workspace is not open".to_string(),
            })
    }

    pub fn open(&self, info: WorkspaceInfo, storage: impl WorkspaceStorage + 'static + Send + Sync) -> Result<WorkspaceInfo> {
        let mut state = self.state.write().map_err(|_| StorageError::Other {
            details: "lock poisoned".to_string(),
        })?;
        *state = Some(WorkspaceState {
            info: info.clone(),
            storage: Arc::new(storage),
        });
        Ok(info)
    }

    pub fn open_fs(&self, root: impl AsRef<Path>) -> Result<WorkspaceInfo> {
        let fs_storage = FsWorkspaceStorage::open(root)?;
        let info = fs_storage.workspace_info()?;
        self.open(info, fs_storage)
    }

    pub fn is_open(&self) -> bool {
        self.state
            .read()
            .map(|s| s.is_some())
            .unwrap_or(false)
    }

    pub fn close(&self) {
        if let Ok(mut state) = self.state.write() {
            *state = None;
        }
    }

    pub fn list_nodes(&self) -> Result<Vec<NodeMeta>> {
        self.storage()?.list_nodes()
    }

    pub fn get_node_meta(&self, id: &NodeId) -> Result<Option<NodeMeta>> {
        self.storage()?.get_node_meta(id)
    }

    pub fn find_node_by_path(&self, rel_path: &Path) -> Result<Option<NodeMeta>> {
        validate_rel_path(rel_path)?;
        self.storage()?.find_node_by_path(rel_path)
    }

    pub fn load_note_text(&self, id: &NodeId) -> Result<String> {
        self.storage()?.load_note_text(id)
    }

    pub fn load_asset_bytes(&self, id: &NodeId) -> Result<Vec<u8>> {
        self.storage()?.load_asset_bytes(id)
    }

    pub fn create_note(&self, rel_path: &Path, title: &str, text: &str) -> Result<NodeMeta> {
        validate_rel_path(rel_path)?;
        self.storage()?.create_note(rel_path, title, text)
    }

    pub fn create_asset(&self, rel_path: &Path, display_name: &str, bytes: &[u8]) -> Result<NodeMeta> {
        validate_rel_path(rel_path)?;
        self.storage()?.create_asset(rel_path, display_name, bytes)
    }

    pub fn save_note_text(&self, id: &NodeId, text: &str) -> Result<NodeMeta> {
        self.storage()?.save_note_text(id, text)
    }

    pub fn save_asset_bytes(&self, id: &NodeId, bytes: &[u8]) -> Result<NodeMeta> {
        self.storage()?.save_asset_bytes(id, bytes)
    }

    pub fn update_meta(&self, meta: &NodeMeta) -> Result<NodeMeta> {
        self.storage()?.update_meta(meta)
    }

    pub fn rename_node(&self, id: &NodeId, new_rel_path: &Path) -> Result<NodeMeta> {
        validate_rel_path(new_rel_path)?;
        self.storage()?.rename_node(id, new_rel_path)
    }

    pub fn delete_node(&self, id: &NodeId) -> Result<()> {
        self.storage()?.delete_node(id)
    }

    pub fn resync(&self) -> Result<ResyncReport> {
        self.storage()?.resync()
    }
}