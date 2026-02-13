use std::path::{Path, Component};
use crate::core::errors::storage_error::{Result, StorageError};

pub fn validate_rel_path(rel_path: &Path) -> Result<()> {

    if rel_path.as_os_str().is_empty() {
        return Err(StorageError::InvalidInput { details: "empty path".to_string(), })
    }
    
    if rel_path.is_absolute() {
        return Err(StorageError::InvalidInput {
            details: "rel_path must be relative".to_string(),
        });
    }
    if rel_path
        .components()
        .any(|c| matches!(c, Component::ParentDir))
    {
        return Err(StorageError::InvalidInput {
            details: "rel_path must not contain ..".to_string(),
        });
    }
    Ok(())
}
