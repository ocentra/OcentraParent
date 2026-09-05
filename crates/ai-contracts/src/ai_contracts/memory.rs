use serde::{Deserialize, Serialize};

use super::identity::{
    AiDigest, AiEvidenceReferenceId, AiFamilyId, AiGraphNodeId, AiGraphReferenceId,
    AiMemoryReferenceId, AiResultId, AiTimestamp,
};
use super::{AiConfidence, AiCustodyState, AiRetentionState};

mod graph;
mod provenance;
mod reference;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiMemoryReferenceKind {
    RecentActivity,
    EvidenceMemory,
    SemanticMemory,
    PolicyMemory,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiGraphNodeKind {
    Evidence,
    Activity,
    Result,
    Memory,
    PolicyRule,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiGraphEdgeKind {
    Supports,
    DerivedFrom,
    RelatedTo,
    GovernedBy,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiProvenanceLink {
    family_id: AiFamilyId,
    source_evidence_reference_ids: Vec<AiEvidenceReferenceId>,
    source_result_id: Option<AiResultId>,
    source_digest: Option<AiDigest>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiMemoryReference {
    memory_reference_id: AiMemoryReferenceId,
    family_id: AiFamilyId,
    kind: AiMemoryReferenceKind,
    provenance: AiProvenanceLink,
    generated_at: AiTimestamp,
    expires_at: Option<AiTimestamp>,
    confidence: AiConfidence,
    custody: AiCustodyState,
    retention: AiRetentionState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiGraphReference {
    graph_reference_id: AiGraphReferenceId,
    family_id: AiFamilyId,
    node_kind: AiGraphNodeKind,
    target_node_id: AiGraphNodeId,
    edge_kind: AiGraphEdgeKind,
    source_memory_reference_id: Option<AiMemoryReferenceId>,
    source_evidence_reference_ids: Vec<AiEvidenceReferenceId>,
    source_result_id: Option<AiResultId>,
    source_result_digest: Option<AiDigest>,
    generated_at: AiTimestamp,
    expires_at: Option<AiTimestamp>,
    custody: AiCustodyState,
    retention: AiRetentionState,
}
