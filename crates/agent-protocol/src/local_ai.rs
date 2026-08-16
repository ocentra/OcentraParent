use serde::{Deserialize, Serialize};

use crate::local_ai_runtime::{lifecycle::LocalAiDegradedState, status::LocalModelRuntimeStatus};
use crate::policy_constants;

use super::policy::{ParentEvidenceReference, PolicyAction};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalAiUnknownState {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "missing-evidence")]
    MissingEvidence,
    #[serde(rename = "low-confidence")]
    LowConfidence,
    #[serde(rename = "model-unavailable")]
    ModelUnavailable,
    #[serde(rename = "policy-conflict")]
    PolicyConflict,
}

impl LocalAiUnknownState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::None => policy_constants::UNKNOWN_NONE,
            Self::MissingEvidence => policy_constants::UNKNOWN_MISSING_EVIDENCE,
            Self::LowConfidence => policy_constants::UNKNOWN_LOW_CONFIDENCE,
            Self::ModelUnavailable => policy_constants::UNKNOWN_MODEL_UNAVAILABLE,
            Self::PolicyConflict => policy_constants::UNKNOWN_POLICY_CONFLICT,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalAiMemoryReferenceKind {
    #[serde(rename = "evidence-memory")]
    EvidenceMemory,
    #[serde(rename = "recent-activity")]
    RecentActivity,
    #[serde(rename = "policy-memory")]
    PolicyMemory,
    #[serde(rename = "semantic-memory")]
    SemanticMemory,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalAiGraphReferenceKind {
    #[serde(rename = "graph-entity")]
    GraphEntity,
    #[serde(rename = "graph-edge")]
    GraphEdge,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalAiMemoryReference {
    pub memory_reference_id: String,
    pub kind: LocalAiMemoryReferenceKind,
    pub source_evidence_references: Vec<ParentEvidenceReference>,
    pub source_policy_version: Option<String>,
    pub generated_at: String,
    pub confidence: f64,
    pub derived_index_version: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalAiGraphReference {
    pub graph_reference_id: String,
    pub kind: LocalAiGraphReferenceKind,
    pub source_evidence_references: Vec<ParentEvidenceReference>,
    pub source_policy_version: Option<String>,
    pub generated_at: String,
    pub confidence: f64,
    pub derived_index_version: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalAiSafetyResult {
    pub schema_version: String,
    pub result_id: String,
    pub request_id: String,
    pub action: PolicyAction,
    pub confidence: f64,
    pub unknown_state: LocalAiUnknownState,
    pub degraded_state: LocalAiDegradedState,
    pub reason_codes: Vec<String>,
    pub explanation_reference: Option<String>,
    pub evidence_references: Vec<ParentEvidenceReference>,
    pub parent_rule_references: Vec<String>,
    pub memory_references: Vec<LocalAiMemoryReference>,
    pub graph_references: Vec<LocalAiGraphReference>,
    pub model_runtime: LocalModelRuntimeStatus,
    pub prompt_version: String,
    pub expires_at: Option<String>,
}
