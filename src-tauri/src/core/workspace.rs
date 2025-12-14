use std::io;
use std::path::Path;
use std::sync::Arc;
use anyhow::{Result, bail};

use crate::core::domain::{WorkspaceInfo};
use crate::core::infrastructure::{WorkspaceStorage, FsWorkspaceStorage};

pub struct Workspace {
    ws_info: WorkspaceInfo,
    storage: Arc<dyn WorkspaceStorage>
}

impl Workspace {
    // Opens or initalize workspace from provided folder
    pub fn open(root_path: impl AsRef<Path>) -> Result<Self> {
        let path: &Path = root_path.as_ref();

        if !path.exists() {
            bail!("Workspace path does not exist: {}", path.display());
        }

        if !path.is_dir() {
            bail!("Workspace path is not a directory: {}", path.display());
        }

        // Check metadata folder existence and access
        let metadata_folder = path.join(".initiation");
        match metadata_folder.metadata() {
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                //bail!("Metadata folder is missing at {}", metadata_folder.display())

                // Create metadata folder
                std::fs::create_dir_all(&metadata_folder)?
            }
            Err(e) => return Err(e.into()),
        };

        // Read metadata, check .wrkspce file and etc
        // Here just return workspace and create config

        let workspace_info = WorkspaceInfo::new(
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default(),
            path,
        );

        // Now just creating FS storage
        let workspace_storage = Arc::new(FsWorkspaceStorage::new(workspace_info.clone())?);

        Ok(Self { ws_info: workspace_info, storage: workspace_storage })
    }
}
