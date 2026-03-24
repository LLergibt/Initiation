use std::path::{Path, PathBuf};
use chrono::Utc;

use crate::core::domain::node::{NodeId, NodeKind, NodeMeta};
use crate::core::domain::workspace::WorkspaceInfo;
use crate::core::errors::storage_error::{Result, StorageError};

use crate::core::ports::content_source::ContentSource;
use crate::core::ports::meta_store::MetaStore;
use crate::core::ports::workspace_storage::{ResyncReport, WorkspaceStorage};

use super::fs_content_source::FsContentSource;
use super::fs_meta_store_json::FsMetaStoreJson;

#[derive(Debug)]
pub struct FsWorkspaceStorage {
    root: PathBuf,
    content: FsContentSource,
    meta: FsMetaStoreJson,
}

impl FsWorkspaceStorage {
    pub fn open(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();

        if !root.exists() {
            std::fs::create_dir_all(&root)?;
        }

        let content = FsContentSource::new(&root);
        let meta = FsMetaStoreJson::open(&root)?;

        Ok(Self {root, content, meta})
    }

    fn workspace_name(&self) -> String {
        self.root
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("workspace")
            .to_string()
    }

    fn new_id(&self) -> NodeId {
        NodeId(uuid::Uuid::new_v4().as_u128())

    }

    fn ensure_note_kind(meta: &NodeMeta) -> Result<()> {
        if meta.kind != NodeKind::Note {
            return Err(StorageError::WrongKind {
                expected: "Note",
                actual: "Asset",
            });
        }
        Ok(())
    }

    fn ensure_asset_kind(meta: &NodeMeta) -> Result<()> {
        if meta.kind != NodeKind::Asset {
            return Err(StorageError::WrongKind {
                expected: "Asset",
                actual: "Note",
            });
        }
        Ok(())
    }
}

impl WorkspaceStorage for FsWorkspaceStorage {
    fn workspace_info(&self) -> Result<WorkspaceInfo> {
        Ok(WorkspaceInfo::new(self.workspace_name(), &self.root))
    }

    fn resync(&self) -> Result<ResyncReport> {
        // MVP: empty report
        Ok(ResyncReport {
            scanned_files: 0,
            created_meta: 0,
            updated_meta: 0,
            orphan_meta: 0,
            orphan_files: 0,
        })
    }

    fn list_nodes(&self) -> Result<Vec<NodeMeta>> {
        self.meta.list_nodes()
    }

    fn get_node_meta(&self, id: &NodeId) -> Result<Option<NodeMeta>> {
        self.meta.get_node(id)
    }

    fn find_node_by_path(&self, rel_path: &Path) -> Result<Option<NodeMeta>> {
        self.meta.find_by_path(rel_path)
    }

    fn load_note_text(&self, id: &NodeId) -> Result<String> {
        let meta = self.meta.get_node(id)?
            .ok_or(StorageError::NotFound { what: "node meta" })?;

        Self::ensure_note_kind(&meta)?;

        let bytes = self.content.read(&meta.path)?;
        let text = String::from_utf8(bytes)
            .map_err(|_| StorageError::Corrupted {
                details: "note is not valid UTF-8".to_string(),
            })?;
        Ok(text)
    }

    fn load_asset_bytes(&self, id: &NodeId) -> Result<Vec<u8>> {
        let meta = self.meta.get_node(id)?
            .ok_or(StorageError::NotFound { what: "node meta" })?;

        Self::ensure_asset_kind(&meta)?;

        self.content.read(&meta.path)
    }

    fn create_note(&self, rel_path: &Path, title: &str, initial_text: &str) -> Result<NodeMeta> {
        if self.meta.find_by_path(rel_path)?.is_some() {
            return Err(StorageError::PathConflict {
                details: format!("path already used: {}", rel_path.display()),
            });
        }

        // Create meta
        let now = Utc::now();
        let meta = NodeMeta {
            id: self.new_id(),
            kind: NodeKind::Note,
            path: rel_path.to_path_buf(),
            title: title.to_string(),
            tags: vec![],
            created_at: Some(now),
            updated_at: Some(now),
            color: None,
        };

        // Write content
        self.content.write(rel_path, initial_text.as_bytes())?;

        // Write meta
        self.meta.upsert_nodes(&[meta.clone()])?;

        Ok(meta)
    }

    fn create_asset(&self, rel_path: &Path, display_name: &str, bytes: &[u8]) -> Result<NodeMeta> {
        if self.meta.find_by_path(rel_path)?.is_some() {
            return Err(StorageError::PathConflict {
                details: format!("path already used: {}", rel_path.display()),
            });
        }

        let now = Utc::now();
        let meta = NodeMeta {
            id: self.new_id(),
            kind: NodeKind::Asset,
            path: rel_path.to_path_buf(),
            title: display_name.to_string(),
            tags: vec![],
            created_at: Some(now),
            updated_at: Some(now),
            color: None,
        };

        self.content.write(rel_path, bytes)?;
        self.meta.upsert_nodes(&[meta.clone()])?;

        Ok(meta)
    }

    fn save_note_text(&self, id: &NodeId, new_text: &str) -> Result<NodeMeta> {
        let mut meta = self.meta.get_node(id)?
            .ok_or(StorageError::NotFound { what: "node meta" })?;

        Self::ensure_note_kind(&meta)?;

        // write content first
        self.content.write(&meta.path, new_text.as_bytes())?;

        // then meta
        meta.updated_at = Some(Utc::now());
        self.meta.update_node(&meta)?;

        Ok(meta)
    }

    fn save_asset_bytes(&self, id: &NodeId, bytes: &[u8]) -> Result<NodeMeta> {
        let mut meta = self.meta.get_node(id)?
            .ok_or(StorageError::NotFound { what: "node meta" })?;

        Self::ensure_asset_kind(&meta)?;

        self.content.write(&meta.path, bytes)?;

        meta.updated_at = Some(Utc::now());
        self.meta.update_node(&meta)?;

        Ok(meta)
    }

    fn update_meta(&self, meta: &NodeMeta) -> Result<NodeMeta> {
        // TODO
        self.meta.update_node(meta)?;
        Ok(meta.clone())
    }

    fn rename_node(&self, id: &NodeId, new_rel_path: &Path) -> Result<NodeMeta> {
        if self.meta.find_by_path(new_rel_path)?.is_some() {
            return Err(StorageError::PathConflict {
                details: format!("path already used: {}", new_rel_path.display()),
            });
        }

        let mut meta = self.meta.get_node(id)?
            .ok_or(StorageError::NotFound { what: "node meta" })?;

        // FS move/rename
        self.content.rename(&meta.path, new_rel_path)?;

        meta.path = new_rel_path.to_path_buf();
        meta.updated_at = Some(Utc::now());
        self.meta.update_node(&meta)?;

        Ok(meta)
    }

    fn delete_node(&self, id: &NodeId) -> Result<()> {
        let meta = self.meta.get_node(id)?
            .ok_or(StorageError::NotFound { what: "node meta" })?;

        self.content.delete(&meta.path)?;
        self.meta.delete_node(id)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::path::Path;

    fn p(s: &str) -> &Path {
        Path::new(s)
    }

    #[test]
    fn open_creates_meta_dir_and_nodes_json() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let _st = FsWorkspaceStorage::open(root).unwrap();

        assert!(root.join(".initiation/meta").exists());
        assert!(root.join(".initiation/meta").join("nodes.json").exists());
    }

    #[test]
    fn create_and_load_note() {
        let dir = tempdir().unwrap();
        let st = FsWorkspaceStorage::open(dir.path()).unwrap();

        let meta = st.create_note(p("notes/a.md"), "A", "hello").unwrap();
        let text = st.load_note_text(&meta.id).unwrap();

        assert_eq!(text, "hello");
        assert_eq!(meta.kind, NodeKind::Note);
        assert_eq!(meta.path, p("notes/a.md"));
    }

    #[test]
    fn save_note_updates_content() {
        let dir = tempdir().unwrap();
        let st = FsWorkspaceStorage::open(dir.path()).unwrap();

        let meta = st.create_note(p("a.md"), "A", "v1").unwrap();
        st.save_note_text(&meta.id, "v2").unwrap();

        let text = st.load_note_text(&meta.id).unwrap();
        assert_eq!(text, "v2");
    }

    #[test]
    fn rename_note_moves_file_and_updates_meta() {
        let dir = tempdir().unwrap();
        let st = FsWorkspaceStorage::open(dir.path()).unwrap();

        let meta = st.create_note(p("a.md"), "A", "x").unwrap();
        let meta2 = st.rename_node(&meta.id, p("b.md")).unwrap();

        assert!(dir.path().join("b.md").exists());
        assert!(!dir.path().join("a.md").exists());
        assert_eq!(meta2.path, p("b.md"));

        let text = st.load_note_text(&meta.id).unwrap();
        assert_eq!(text, "x");
    }

    #[test]
    fn delete_note_removes_file_and_meta() {
        let dir = tempdir().unwrap();
        let st = FsWorkspaceStorage::open(dir.path()).unwrap();

        let meta = st.create_note(p("a.md"), "A", "x").unwrap();
        assert!(dir.path().join("a.md").exists());

        st.delete_node(&meta.id).unwrap();

        assert!(!dir.path().join("a.md").exists());
        assert!(st.get_node_meta(&meta.id).unwrap().is_none());
    }

    #[test]
    fn create_note_path_conflict_returns_pathconflict() {
        let dir = tempdir().unwrap();
        let st = FsWorkspaceStorage::open(dir.path()).unwrap();

        let _ = st.create_note(p("a.md"), "A", "1").unwrap();
        let err = st.create_note(p("a.md"), "A2", "2").unwrap_err();

        match err {
            StorageError::PathConflict { .. } => {}
            other => panic!("expected PathConflict, got {other:?}"),
        }
    }
}
