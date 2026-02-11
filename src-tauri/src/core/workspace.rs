use crate::core::domain::{WorkspaceInfo};

pub struct Workspace {
    ws_info: WorkspaceInfo,
}

impl Workspace {
    pub fn new(ws_info: WorkspaceInfo) -> Self {
        Self { ws_info }
    }
}