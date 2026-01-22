use std::io;
use std::path::Path;
use std::sync::Arc;
use anyhow::{Result, bail};

use crate::core::domain::{WorkspaceInfo};

pub struct Workspace {
    ws_info: WorkspaceInfo,
}
