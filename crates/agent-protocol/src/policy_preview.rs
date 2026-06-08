use serde::{Deserialize, Serialize};

use crate::{LocalAiParentRuleContextRef, ParentEvidenceReference, PolicyDecision, PolicyTarget};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyPreviewNetworkEvidenceMapping {
    pub evidence_grade: String,
    pub requested_action: String,
    pub mapped_action: String,
    pub mode: String,
    pub adapter_action_authorized: bool,
    pub enforcement_command_authorized: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyPreviewReadModelRow {
    pub preview_id: String,
    pub source_event_id: String,
    pub observed_at: String,
    pub target: PolicyTarget,
    pub evidence_references: Vec<ParentEvidenceReference>,
    pub parent_rule_context_references: Vec<LocalAiParentRuleContextRef>,
    pub decision: PolicyDecision,
    pub network_evidence_mapping: Option<PolicyPreviewNetworkEvidenceMapping>,
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
