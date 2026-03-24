use std::path::PathBuf;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(ts_rs::TS)]
#[ts(type = "string", export, export_to = "../../bindings/")]
pub struct NodeId(pub u128);

impl Serialize for NodeId {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&format!("{:032x}", self.0))
    }
}

impl<'de> Deserialize<'de> for NodeId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let hex = String::deserialize(d)?;
        let n = u128::from_str_radix(&hex, 16).map_err(serde::de::Error::custom)?;
        Ok(NodeId(n))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export, export_to = "../../bindings/")]
pub enum NodeKind {
    Note,
    Asset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export, export_to = "../../bindings/")]
pub struct NodeMeta {
    pub id: NodeId,
    pub kind: NodeKind,
    pub path: PathBuf,  // relative path inside the vault
    pub title: String,
    pub tags: Vec<String>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,

    pub color: Option<String>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(ts_rs::TS)]
#[ts(export, export_to = "../../bindings/")]
pub struct NodeContent {
    pub meta: NodeMeta,
    pub raw: String,
}
