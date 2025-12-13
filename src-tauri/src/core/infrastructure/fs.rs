use std::fs;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use anyhow::{Result, Context, bail};
use chrono::Utc;
use parking_lot::RwLock;

use crate::core::{WorkspaceInfo, NodeId, NodeKind, NodeMeta, NodeContent};
use super::{ContentSource, MetaStore, WorkspaceStorage};

#[derive(Debug)]
pub struct FsContentSource {
    root: PathBuf,
}

impl FsContentSource {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self { root: root.as_ref().to_path_buf() }
    }

    fn resolve(&self, key: &str) -> PathBuf {
        self.root.join(key)
    }
}

impl ContentSource for FsContentSource {
    fn read(&self, key: &str) -> Result<Vec<u8>> {
        let path = self.resolve(key);
        fs::read(&path)
            .with_context(|| format!("Failed to read content from {}", path.display()))
    }

    fn write(&self, key: &str, data: &[u8]) -> Result<()> {
        let path = self.resolve(key);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create parent dirs for {}", path.display()))?;
        }
        fs::write(&path, data)
            .with_context(|| format!("Failed to write content into {}", path.display()))
    }

    fn delete(&self, key: &str) -> Result<()> {
        let path = self.resolve(key);
        match fs::remove_file(&path) {
            Ok(_) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e).with_context(|| format!("Failed to delete {}", path.display())),
        }
    }

    fn list(&self, prefix: &str) -> Result<Vec<String>> {
        let base = self.resolve(prefix);
        if !base.exists() {
            return Ok(vec![]);
        }

        let mut found = Vec::new();
        let mut stack = vec![base];

        while let Some(path) = stack.pop() {
            if path.is_dir() {
                for entry in fs::read_dir(&path)
                    .with_context(|| format!("Failed to list directory {}", path.display()))?
                {
                    stack.push(entry?.path());
                }
            } else if path.is_file() {
                let rel = path.strip_prefix(&self.root)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .to_string();
                found.push(rel);
            }
        }

        Ok(found)
    }
}

#[derive(Debug)]
pub struct FsMetaStore {
    file_path: PathBuf,
    nodes: RwLock<Vec<NodeMeta>>,
}

impl FsMetaStore {
    pub fn new(root: impl AsRef<Path>) -> Result<Self> {
        let file_path = root.as_ref().join(".initiation").join("meta.json");
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to ensure metadata directory at {}", parent.display()))?;
        }

        let existing = Self::read_from_disk(&file_path)?;
        Ok(Self { file_path, nodes: RwLock::new(existing) })
    }

    fn read_from_disk(path: &Path) -> Result<Vec<NodeMeta>> {
        match fs::read_to_string(path) {
            Ok(raw) => {
                let parsed: Vec<NodeMeta> = serde_json::from_str(&raw)
                    .with_context(|| format!("Failed to parse metadata file {}", path.display()))?;
                Ok(parsed)
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(e) => Err(e).with_context(|| format!("Failed to read metadata file {}", path.display())),
        }
    }

    fn persist(&self, snapshot: &[NodeMeta]) -> Result<()> {
        let data = serde_json::to_string_pretty(snapshot)
            .context("Failed to serialize node metadata")?;
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to prepare metadata folder {}", parent.display()))?;
        }
        fs::write(&self.file_path, data)
            .with_context(|| format!("Failed to write metadata to {}", self.file_path.display()))
    }
}

impl MetaStore for FsMetaStore {
    fn upsert_nodes(&self, nodes: &[NodeMeta]) -> Result<()> {
        let mut cache = self.nodes.write();
        for node in nodes {
            if let Some(existing) = cache.iter_mut().find(|n| n.id == node.id) {
                *existing = node.clone();
            } else {
                cache.push(node.clone());
            }
        }
        self.persist(&cache)
    }

    fn get_node(&self, id: &NodeId) -> Result<Option<NodeMeta>> {
        let cache = self.nodes.read();
        Ok(cache.iter().find(|n| &n.id == id).cloned())
    }

    fn list_nodes(&self) -> Result<Vec<NodeMeta>> {
        let cache = self.nodes.read();
        Ok(cache.clone())
    }

    fn update_node(&self, node: &NodeMeta) -> Result<()> {
        let mut cache = self.nodes.write();
        if let Some(existing) = cache.iter_mut().find(|n| n.id == node.id) {
            *existing = node.clone();
            self.persist(&cache)
        } else {
            bail!("Node with id {} not found in metadata", node.id.0);
        }
    }

    fn delete_node(&self, id: &NodeId) -> Result<()> {
        let mut cache = self.nodes.write();
        let before = cache.len();
        cache.retain(|n| &n.id != id);
        if before != cache.len() {
            self.persist(&cache)
        } else {
            Ok(())
        }
    }
}

pub struct FsWorkspaceStorage {
    workspace: WorkspaceInfo,
    content: Arc<dyn ContentSource + Send + Sync>,
    meta: Arc<dyn MetaStore + Send + Sync>,
}

impl FsWorkspaceStorage {
    pub fn new(workspace: WorkspaceInfo) -> Result<Self> {
        let root = workspace.root_path.clone();
        let content: Arc<dyn ContentSource + Send + Sync> = Arc::new(FsContentSource::new(root.clone()));
        let meta: Arc<dyn MetaStore + Send + Sync> = Arc::new(FsMetaStore::new(&root)?);

        Ok(Self { workspace, content, meta })
    }

    fn root(&self) -> &Path {
        &self.workspace.root_path
    }

    fn validate_relative(path: &str) -> Result<PathBuf> {
        let path = Path::new(path);
        if path.is_absolute() || path.components().any(|c| c == Component::ParentDir) {
            bail!("Path must be relative to workspace: {}", path.display());
        }
        Ok(path.to_path_buf())
    }

    fn infer_kind(path: &Path) -> NodeKind {
        match path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase().as_str() {
            "md" | "markdown" => NodeKind::Note,
            _ => NodeKind::Asset,
        }
    }
}

impl WorkspaceStorage for FsWorkspaceStorage {
    fn resync(&self) -> Result<()> {
        let metas = self.meta.list_nodes()?;
        for meta in metas {
            let full_path = self.root().join(&meta.path);
            if !full_path.exists() {
                self.meta.delete_node(&meta.id)?;
            } else if let Ok(info) = fs::metadata(&full_path) {
                let mut updated = meta.clone();
                updated.size_bytes = Some(info.len());
                self.meta.update_node(&updated)?;
            }
        }
        Ok(())
    }

    fn list_nodes(&self) -> Result<Vec<NodeMeta>> {
        self.meta.list_nodes()
    }

    fn get_node_meta(&self, id: &NodeId) -> Result<Option<NodeMeta>> {
        self.meta.get_node(id)
    }

    fn load_node(&self, id: &NodeId) -> Result<NodeContent> {
        let meta = self.meta.get_node(id)?
            .ok_or_else(|| anyhow::anyhow!("Node {} not found", id.0))?;
        let data = self.content.read(meta.path.to_string_lossy().as_ref())?;
        let raw = String::from_utf8(data)
            .context("Failed to decode node content as UTF-8")?;
        Ok(NodeContent { meta, raw })
    }

    fn create_node(&self, relative_path: &str, title: &str, initial_body: &str) -> Result<NodeContent> {
        let rel = Self::validate_relative(relative_path)?;
        let full_path = self.root().join(&rel);

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directories for {}", full_path.display()))?;
        }

        self.content.write(rel.to_string_lossy().as_ref(), initial_body.as_bytes())?;

        let kind = Self::infer_kind(&rel);
        let now = Utc::now();
        let unique = now.timestamp_nanos_opt().unwrap_or_else(|| now.timestamp_micros() * 1_000);
        let size = fs::metadata(&full_path).ok().map(|m| m.len());
        let meta = NodeMeta {
            id: NodeId(format!("node-{}", unique)),
            kind,
            path: rel,
            title: title.to_string(),
            tags: Vec::new(),
            created_at: Some(now),
            updated_at: Some(now),
            color: None,
            size_bytes: size,
            content_hash: None,
        };

        self.meta.upsert_nodes(&[meta.clone()])?;

        Ok(NodeContent { meta, raw: initial_body.to_string() })
    }

    fn save_node(&self, id: &NodeId, new_raw: &str) -> Result<NodeContent> {
        let mut meta = self.meta.get_node(id)?
            .ok_or_else(|| anyhow::anyhow!("Node {} not found", id.0))?;

        self.content.write(meta.path.to_string_lossy().as_ref(), new_raw.as_bytes())?;

        meta.updated_at = Some(Utc::now());
        meta.size_bytes = fs::metadata(self.root().join(&meta.path)).ok().map(|m| m.len());

        self.meta.update_node(&meta)?;

        Ok(NodeContent { meta, raw: new_raw.to_string() })
    }

    fn rename_node(&self, id: &NodeId, new_rel_path: &str) -> Result<NodeMeta> {
        let mut meta = self.meta.get_node(id)?
            .ok_or_else(|| anyhow::anyhow!("Node {} not found", id.0))?;

        let new_rel = Self::validate_relative(new_rel_path)?;
        let old_full = self.root().join(&meta.path);
        let new_full = self.root().join(&new_rel);

        if let Some(parent) = new_full.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create destination directories for {}", new_full.display()))?;
        }

        fs::rename(&old_full, &new_full)
            .with_context(|| format!("Failed to rename {} to {}", old_full.display(), new_full.display()))?;

        meta.path = new_rel;
        meta.title = Path::new(new_rel_path)
            .file_stem()
            .and_then(|n| n.to_str())
            .unwrap_or(&meta.title)
            .to_string();
        meta.updated_at = Some(Utc::now());

        self.meta.update_node(&meta)?;

        Ok(meta)
    }

    fn delete_node(&self, id: &NodeId) -> Result<()> {
        let meta = self.meta.get_node(id)?
            .ok_or_else(|| anyhow::anyhow!("Node {} not found", id.0))?;
        let full_path = self.root().join(&meta.path);
        match fs::remove_file(&full_path) {
            Ok(_) => {},
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {},
            Err(e) => return Err(e).with_context(|| format!("Failed to delete {}", full_path.display())),
        }

        self.meta.delete_node(id)
    }
}
