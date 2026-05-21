use serde::{Deserialize, Serialize};

use crate::{ParentEvidenceReference, PolicyDecision, PolicyTarget};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyPreviewReadModelRow {
    pub preview_id: String,
    pub source_event_id: String,
    pub observed_at: String,
    pub target: PolicyTarget,
    pub evidence_references: Vec<ParentEvidenceReference>,
    pub decision: PolicyDecision,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyPreviewReadModel {
    pub schema_version: String,
    pub generated_at: String,
    pub custody: String,
    pub limit: u64,
    pub returned: u64,
    pub capability_status: String,
    pub rows: Vec<PolicyPreviewReadModelRow>,
}
