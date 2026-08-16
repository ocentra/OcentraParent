use serde::{Deserialize, Serialize};

use super::ActivityReadModelState;
use crate::ActivityEvidenceRef;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityAppGameSourceStatusRow {
    pub source_kind: String,
    pub state: ActivityReadModelState,
    pub row_count: u64,
    pub last_observed_at: Option<String>,
    pub capability_status: String,
    pub evidence: Vec<ActivityEvidenceRef>,
}
