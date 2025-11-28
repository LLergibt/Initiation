use std::path::PathBuf;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VaultId(pub String);

#[derive(Debug, Clone)]
pub struct VaultConfig {
    pub name: String,
    pub root_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Vault {
    pub config: VaultConfig,
    pub opened_at: DateTime<Utc>,
}
