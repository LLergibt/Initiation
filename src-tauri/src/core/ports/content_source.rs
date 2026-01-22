use anyhow::Result;
use std::{path::{Path, PathBuf}};

// Content control layer - works only with content, no meta, RW operation for files
pub trait ContentSource {
    fn read(&self, key: &Path) -> Result<Vec<u8>>;
    fn write(&self, key: &Path, data: &[u8]) -> Result<()>;
    fn delete(&self, key: &Path) -> Result<()>;
    fn list(&self, prefix: &Path) -> Result<Vec<PathBuf>>;
}