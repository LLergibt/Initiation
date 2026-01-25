use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

use crate::core::domain::node::{NodeId, NodeMeta};
use crate::core::errors::storage_error::{Result, StorageError};
use crate::core::ports::meta_store::MetaStore;

#[derive(Debug)]
pub struct FsMetaStoreJson {
    meta_dir: PathBuf,
    nodes_path: PathBuf,
    cache: RwLock<HashMap<NodeId, NodeMeta>>,
}

impl FsMetaStoreJson {
    pub fn open(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref();

        let meta_dir = root.join(".initiation/meta");
        std::fs::create_dir_all(&meta_dir);

        let nodes_path = meta_dir.join("nodes.json");

        if !nodes_path.exists() {
            std::fs::write(&nodes_path, b"{}");
        }

        let bytes = std::fs::read(&nodes_path)?;
        let parsed: HashMap<NodeId, NodeMeta> = serde_json::from_slice(&bytes)
            .map_err(|e| StorageError::Corrupted { 
                details: format!("cannot parse nodes.json: {e}"), })?;
        
        Ok(Self {
            meta_dir,
            nodes_path,
            cache: RwLock::new(parsed),
        })
    }

    //TODO: atomic write to temp file
    fn flush(&self) -> Result<()> {
        let map = self.cache.read().unwrap();

        let bytes = serde_json::to_vec_pretty(&*map)?; // serde_json::Error -> StorageError::Serde через From
        std::fs::write(&self.nodes_path, bytes)?;
        Ok(())
    }

    fn find_id_by_path_locked(map: &HashMap<NodeId, NodeMeta>, rel_path: &Path) -> Option<NodeId> {
        map.iter()
            .find(|(_, meta)| meta.path == rel_path)
            .map(|(id, _)| id.clone())
    }
}

impl MetaStore for FsMetaStoreJson {
    fn upsert_nodes(&self, nodes: &[NodeMeta]) -> Result<()> {
        {
            let mut map = self.cache.write().unwrap();
            for n in nodes {
                map.insert(n.id.clone(), n.clone());
            }
        }

        self.flush()
    }

    fn get_node(&self, id: &NodeId) -> Result<Option<NodeMeta>> {
        let map = self.cache.read().unwrap();
        Ok(map.get(id).cloned())
    }

    fn list_nodes(&self) -> Result<Vec<NodeMeta>> {
        let map = self.cache.read().unwrap();
        Ok(map.values().cloned().collect())
    }

    fn update_node(&self, node: &NodeMeta) -> Result<()> {
        {
            let mut map = self.cache.write().unwrap();
            map.insert(node.id.clone(), node.clone());
        }
        self.flush()
    }

    fn delete_node(&self, id: &NodeId) -> Result<()> {
        {
            let mut map = self.cache.write().unwrap();
            map.remove(id);
        }
        self.flush()
    }

    fn find_by_path(&self, rel_path: &Path) -> Result<Option<NodeMeta>> {
        let map = self.cache.read().unwrap();
        Ok(map.values().find(|m| m.path==rel_path).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::path::Path;
    use chrono::Utc;

    fn make_meta(id: u128, path: &str, title: &str) -> NodeMeta {
        NodeMeta {
            id: NodeId(id),
            kind: crate::core::domain::node::NodeKind::Note,
            path: PathBuf::from(path),
            title: title.to_string(),
            tags: vec!["t".to_string()],
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            color: None,
        }
    }

    #[test]
    fn open_creates_meta_files() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        let _ms = FsMetaStoreJson::open(root).unwrap();

        assert!(root.join("meta").exists());
        assert!(root.join("meta").join("nodes.json").exists());
    }

    #[test]
    fn upsert_get_list_delete_roundtrip() {
        let dir = tempdir().unwrap();
        let ms = FsMetaStoreJson::open(dir.path()).unwrap();

        let a = make_meta(1, "notes/a.md", "A");
        let b = make_meta(2, "notes/b.md", "B");

        ms.upsert_nodes(&[a.clone(), b.clone()]).unwrap();

        let got_a = ms.get_node(&a.id).unwrap().unwrap();
        assert_eq!(got_a.path, a.path);

        let all = ms.list_nodes().unwrap();
        assert_eq!(all.len(), 2);

        ms.delete_node(&a.id).unwrap();
        assert!(ms.get_node(&a.id).unwrap().is_none());
        assert_eq!(ms.list_nodes().unwrap().len(), 1);
    }

    #[test]
    fn find_by_path_works() {
        let dir = tempdir().unwrap();
        let ms = FsMetaStoreJson::open(dir.path()).unwrap();

        let a = make_meta(10, "x.md", "X");
        ms.upsert_nodes(&[a.clone()]).unwrap();

        let found = ms.find_by_path(Path::new("x.md")).unwrap().unwrap();
        assert_eq!(found.id, a.id);

        let none = ms.find_by_path(Path::new("missing.md")).unwrap();
        assert!(none.is_none());
    }

    #[test]
    fn persists_to_disk_and_reopens() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        {
            let ms = FsMetaStoreJson::open(root).unwrap();
            let a = make_meta(777, "notes/a.md", "A");
            ms.upsert_nodes(&[a]).unwrap();
        }

        // reopen should load existing nodes.json
        let ms2 = FsMetaStoreJson::open(root).unwrap();
        let got = ms2.get_node(&NodeId(777)).unwrap().unwrap();
        assert_eq!(got.title, "A");
    }
}
