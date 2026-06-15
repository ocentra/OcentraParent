use serde::{Deserialize, Serialize};

use crate::event::ParentLogEvent;

pub const SNAPSHOT_SCHEMA_VERSION: u16 = 1;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogSnapshot {
    pub schema_version: u16,
    pub status: String,
    #[serde(default)]
    pub entries: Vec<ParentLogEvent>,
}
