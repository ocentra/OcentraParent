use serde::{Deserialize, Serialize};

use super::{
    identity::{
        AiDigest, AiEvidenceReferenceId, AiFamilyId, AiGraphNodeId, AiGraphReferenceId,
        AiMemoryReferenceId, AiResultId, AiTimestamp,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiProvenanceLink {
    family_id: AiFamilyId,
    source_evidence_reference_ids: Vec<AiEvidenceReferenceId>,
    source_result_id: Option<AiResultId>,
    source_digest: Option<AiDigest>,
}

impl AiProvenanceLink {
    pub(crate) fn new(
        family_id: AiFamilyId,
        source_evidence_reference_ids: Vec<AiEvidenceReferenceId>,
        source_result_id: Option<AiResultId>,
        source_digest: Option<AiDigest>,
    ) -> Result<Self, &'static str> {
        if source_evidence_reference_ids.is_empty() && source_result_id.is_none() {
            return Err("AI provenance link requires evidence or result identity");
        }
        Ok(Self {
            family_id,
            source_evidence_reference_ids,
            source_result_id,
            source_digest,
        })
    }

    pub fn source_evidence_reference_ids(&self) -> &[AiEvidenceReferenceId] {
        &self.source_evidence_reference_ids
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn source_result_id(&self) -> Option<&AiResultId> {
        self.source_result_id.as_ref()
    }

    pub fn source_digest(&self) -> Option<&AiDigest> {
        self.source_digest.as_ref()
    }

    pub fn is_grounded(&self) -> bool {
        !self.source_evidence_reference_ids.is_empty() || self.source_result_id.is_some()
    }
}

/// Owner-issued memory custody and provenance metadata. It has no public wire
/// deserializer because custody state is not established by enum shape alone.
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

impl AiMemoryReference {
    pub(crate) fn new(
        memory_reference_id: AiMemoryReferenceId,
        family_id: AiFamilyId,
        kind: AiMemoryReferenceKind,
        provenance: AiProvenanceLink,
        generated_at: AiTimestamp,
        expires_at: Option<AiTimestamp>,
        confidence: AiConfidence,
        custody: AiCustodyState,
        retention: AiRetentionState,
    ) -> Result<Self, &'static str> {
        if provenance.family_id() != &family_id {
            return Err("AI memory provenance family does not match its reference");
        }
        if !generated_at.is_well_formed()
            || expires_at
                .as_ref()
                .is_some_and(|expires| !generated_at.precedes(expires))
            || matches!(
                custody,
                AiCustodyState::Deleted | AiCustodyState::Unavailable
            )
            || matches!(
                retention,
                AiRetentionState::Deleted | AiRetentionState::Tombstoned
            )
        {
            return Err("AI memory reference has invalid time, custody, or retention state");
        }
        Ok(Self {
            memory_reference_id,
            family_id,
            kind,
            provenance,
            generated_at,
            expires_at,
            confidence,
            custody,
            retention,
        })
    }

    pub fn memory_reference_id(&self) -> &AiMemoryReferenceId {
        &self.memory_reference_id
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn kind(&self) -> AiMemoryReferenceKind {
        self.kind
    }

    pub fn provenance(&self) -> &AiProvenanceLink {
        &self.provenance
    }
}

fn valid_graph_semantics(node_kind: AiGraphNodeKind, edge_kind: AiGraphEdgeKind) -> bool {
    match edge_kind {
        AiGraphEdgeKind::GovernedBy => matches!(node_kind, AiGraphNodeKind::PolicyRule),
        AiGraphEdgeKind::Supports => matches!(
            node_kind,
            AiGraphNodeKind::Evidence | AiGraphNodeKind::Result | AiGraphNodeKind::Memory
        ),
        AiGraphEdgeKind::DerivedFrom => matches!(
            node_kind,
            AiGraphNodeKind::Evidence | AiGraphNodeKind::Result | AiGraphNodeKind::Memory
        ),
        AiGraphEdgeKind::RelatedTo => true,
    }
}

/// Owner-issued graph grounding and custody metadata. It has no public wire
/// deserializer because graph identity and custody must be bound together.
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
    generated_at: AiTimestamp,
    expires_at: Option<AiTimestamp>,
    custody: AiCustodyState,
    retention: AiRetentionState,
}

impl AiGraphReference {
    pub(crate) fn new(
        graph_reference_id: AiGraphReferenceId,
        family_id: AiFamilyId,
        node_kind: AiGraphNodeKind,
        target_node_id: AiGraphNodeId,
        edge_kind: AiGraphEdgeKind,
        source_memory_reference_id: Option<AiMemoryReferenceId>,
        source_evidence_reference_ids: Vec<AiEvidenceReferenceId>,
        source_result_id: Option<AiResultId>,
        generated_at: AiTimestamp,
        expires_at: Option<AiTimestamp>,
        custody: AiCustodyState,
        retention: AiRetentionState,
    ) -> Result<Self, &'static str> {
        let grounded = !source_evidence_reference_ids.is_empty()
            || source_memory_reference_id.is_some()
            || source_result_id.is_some();
        if !grounded
            || !valid_graph_semantics(node_kind, edge_kind)
            || !generated_at.is_well_formed()
            || expires_at
                .as_ref()
                .is_some_and(|expires| !generated_at.precedes(expires))
            || matches!(
                custody,
                AiCustodyState::Deleted | AiCustodyState::Unavailable
            )
            || matches!(
                retention,
                AiRetentionState::Deleted | AiRetentionState::Tombstoned
            )
        {
            return Err("AI graph reference is ungrounded or has invalid semantics/state");
        }
        Ok(Self {
            graph_reference_id,
            family_id,
            node_kind,
            target_node_id,
            edge_kind,
            source_memory_reference_id,
            source_evidence_reference_ids,
            source_result_id,
            generated_at,
            expires_at,
            custody,
            retention,
        })
    }

    pub fn graph_reference_id(&self) -> &AiGraphReferenceId {
        &self.graph_reference_id
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn node_kind(&self) -> AiGraphNodeKind {
        self.node_kind
    }

    pub fn target_node_id(&self) -> &AiGraphNodeId {
        &self.target_node_id
    }

    pub fn edge_kind(&self) -> AiGraphEdgeKind {
        self.edge_kind
    }

    pub fn is_grounded(&self) -> bool {
        self.source_memory_reference_id.is_some()
            || !self.source_evidence_reference_ids.is_empty()
            || self.source_result_id.is_some()
    }
}
