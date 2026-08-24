use serde::{Deserialize, Serialize};

use super::{
    identity::{
        AiDigest, AiEvidenceReferenceId, AiGraphReferenceId, AiMemoryReferenceId, AiResultId,
        AiTimestamp,
    },
    AiConfidence, AiCustodyState, AiRetentionState,
};

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiProvenanceLink {
    pub source_evidence_reference_ids: Vec<AiEvidenceReferenceId>,
    pub source_result_id: Option<AiResultId>,
    pub source_digest: Option<AiDigest>,
}

impl AiProvenanceLink {
    pub fn is_grounded(&self) -> bool {
        !self.source_evidence_reference_ids.is_empty() || self.source_result_id.is_some()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiMemoryReference {
    pub memory_reference_id: AiMemoryReferenceId,
    pub kind: AiMemoryReferenceKind,
    pub provenance: AiProvenanceLink,
    pub generated_at: AiTimestamp,
    pub expires_at: Option<AiTimestamp>,
    pub confidence: AiConfidence,
    pub custody: AiCustodyState,
    pub retention: AiRetentionState,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiMemoryReferenceFields {
    memory_reference_id: AiMemoryReferenceId,
    kind: AiMemoryReferenceKind,
    provenance: AiProvenanceLink,
    generated_at: AiTimestamp,
    expires_at: Option<AiTimestamp>,
    confidence: AiConfidence,
    custody: AiCustodyState,
    retention: AiRetentionState,
}

impl<'de> Deserialize<'de> for AiMemoryReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fields = AiMemoryReferenceFields::deserialize(deserializer)?;
        if !fields.provenance.is_grounded() {
            return Err(serde::de::Error::custom(
                "AI memory reference requires source evidence or result provenance",
            ));
        }
        Ok(Self {
            memory_reference_id: fields.memory_reference_id,
            kind: fields.kind,
            provenance: fields.provenance,
            generated_at: fields.generated_at,
            expires_at: fields.expires_at,
            confidence: fields.confidence,
            custody: fields.custody,
            retention: fields.retention,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiGraphReference {
    pub graph_reference_id: AiGraphReferenceId,
    pub node_kind: AiGraphNodeKind,
    pub edge_kind: AiGraphEdgeKind,
    pub source_memory_reference_id: Option<AiMemoryReferenceId>,
    pub source_evidence_reference_ids: Vec<AiEvidenceReferenceId>,
    pub source_result_id: Option<AiResultId>,
    pub generated_at: AiTimestamp,
    pub expires_at: Option<AiTimestamp>,
    pub custody: AiCustodyState,
    pub retention: AiRetentionState,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiGraphReferenceFields {
    graph_reference_id: AiGraphReferenceId,
    node_kind: AiGraphNodeKind,
    edge_kind: AiGraphEdgeKind,
    source_memory_reference_id: Option<AiMemoryReferenceId>,
    source_evidence_reference_ids: Vec<AiEvidenceReferenceId>,
    source_result_id: Option<AiResultId>,
    generated_at: AiTimestamp,
    expires_at: Option<AiTimestamp>,
    custody: AiCustodyState,
    retention: AiRetentionState,
}

impl<'de> Deserialize<'de> for AiGraphReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fields = AiGraphReferenceFields::deserialize(deserializer)?;
        let grounded = !fields.source_evidence_reference_ids.is_empty()
            || fields.source_memory_reference_id.is_some()
            || fields.source_result_id.is_some();
        if !grounded {
            return Err(serde::de::Error::custom(
                "AI graph reference requires source evidence, memory, or result provenance",
            ));
        }
        Ok(Self {
            graph_reference_id: fields.graph_reference_id,
            node_kind: fields.node_kind,
            edge_kind: fields.edge_kind,
            source_memory_reference_id: fields.source_memory_reference_id,
            source_evidence_reference_ids: fields.source_evidence_reference_ids,
            source_result_id: fields.source_result_id,
            generated_at: fields.generated_at,
            expires_at: fields.expires_at,
            custody: fields.custody,
            retention: fields.retention,
        })
    }
}

impl AiGraphReference {
    pub fn is_grounded(&self) -> bool {
        !self.source_evidence_reference_ids.is_empty()
            || self.source_memory_reference_id.is_some()
            || self.source_result_id.is_some()
    }
}
