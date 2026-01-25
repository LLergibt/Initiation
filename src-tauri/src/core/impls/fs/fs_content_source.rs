use std::path::{Path, PathBuf};

use crate::core::errors::storage_error::{Result, StorageError};
use crate::core::ports::content_source::ContentSource;

#[derive(Debug, Clone)]
pub struct FsContentSource {
    root: PathBuf,
}

impl FsContentSource {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self { root: root.as_ref().to_path_buf() }
    }

    fn abs(&self, rel: &Path) -> PathBuf {
        self.root.join(rel)
    }

    fn ensure_parent_dir(abs: &Path) -> Result<()> {
        if let Some(parent) = abs.parent() {
            std::fs::create_dir_all(parent)?;
        }
        Ok(())
    }
}

impl ContentSource for FsContentSource {
    fn read(&self, key: &Path) -> Result<Vec<u8>> {
        Ok(std::fs::read(self.abs(key))?)
    }

    fn write(&self, key: &Path, data: &[u8]) -> Result<()> {
        let abs = self.abs(key);
        Self::ensure_parent_dir(&abs)?;
        std::fs::write(abs, data)?;
        Ok(())
    }

    fn delete(&self, key: &Path) -> Result<()> {
        let abs = self.abs(key);

        // 
        if abs.exists() {
            std::fs::remove_file(abs)?;
        }

        Ok(())
    }

    fn list(&self, prefix: &Path) -> Result<Vec<PathBuf>> {
        let abs_prefix = self.abs(prefix);

        if !abs_prefix.exists() {
            return Ok(Vec::new());
        }

        let mut out = Vec::new();

        for entry in walkdir::WalkDir::new(&abs_prefix)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let abs_path = entry.path();

                let rel = abs_path
                    .strip_prefix(&self.root)
                    .map_err(|_| StorageError::Corrupted {
                        details: "walkdir returned path outside workspace root".to_string(),
                    })?;

                out.push(rel.to_path_buf());
                
            }
        }

        Ok(out)
    }

    fn rename(&self, from: &Path, to: &Path) -> Result<()> {
        let abs_from = self.abs(from);
        let abs_to = self.abs(to);

        if !abs_from.exists() {
            return Err(StorageError::NotFound { what: "content file" });
        }

        Self::ensure_parent_dir(&abs_to)?;
        std::fs::rename(abs_from, abs_to)?;
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
    fn write_and_read_roundtrip() {
        let dir = tempdir().unwrap();
        let cs = FsContentSource::new(dir.path());

        cs.write(p("notes/a.md"), b"hello").unwrap();

        let bytes = cs.read(p("notes/a.md")).unwrap();
        assert_eq!(bytes, b"hello");
    }

    #[test]
    fn write_creates_parent_dirs() {
        let dir = tempdir().unwrap();
        let cs = FsContentSource::new(dir.path());

        cs.write(p("x/y/z.txt"), b"ok").unwrap();

        assert!(dir.path().join("x").join("y").join("z.txt").exists());
    }

    #[test]
    fn delete_is_idempotent() {
        let dir = tempdir().unwrap();
        let cs = FsContentSource::new(dir.path());

        // deleting missing file should be ok
        cs.delete(p("missing.bin")).unwrap();

        cs.write(p("a.bin"), b"1").unwrap();
        cs.delete(p("a.bin")).unwrap();
        cs.delete(p("a.bin")).unwrap(); // second delete also ok
    }

    #[test]
    fn rename_moves_file() {
        let dir = tempdir().unwrap();
        let cs = FsContentSource::new(dir.path());

        cs.write(p("a.txt"), b"data").unwrap();
        cs.rename(p("a.txt"), p("b.txt")).unwrap();

        assert!(!dir.path().join("a.txt").exists());
        assert!(dir.path().join("b.txt").exists());

        let bytes = cs.read(p("b.txt")).unwrap();
        assert_eq!(bytes, b"data");
    }

    #[test]
    fn list_returns_relative_paths() {
        let dir = tempdir().unwrap();
        let cs = FsContentSource::new(dir.path());

        cs.write(p("notes/a.md"), b"a").unwrap();
        cs.write(p("notes/sub/b.md"), b"b").unwrap();

        let mut list = cs.list(p("notes")).unwrap();
        list.sort();

        assert_eq!(list, vec![PathBuf::from("notes/a.md"), PathBuf::from("notes/sub/b.md")]);
    }
}
