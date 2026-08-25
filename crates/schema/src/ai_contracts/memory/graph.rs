use super::{AiGraphEdgeKind, AiGraphNodeKind, AiGraphReference};
use crate::ai_contracts::identity::{
    AiEvidenceReferenceId, AiFamilyId, AiGraphNodeId, AiGraphReferenceId, AiMemoryReferenceId,
    AiResultId, AiTimestamp,
};
use crate::ai_contracts::{AiCustodyState, AiRetentionState};

fn valid_graph_semantics(node_kind: AiGraphNodeKind, edge_kind: AiGraphEdgeKind) -> bool {
    match edge_kind {
        AiGraphEdgeKind::GovernedBy => matches!(node_kind, AiGraphNodeKind::PolicyRule),
        AiGraphEdgeKind::Supports | AiGraphEdgeKind::DerivedFrom => matches!(
            node_kind,
            AiGraphNodeKind::Evidence | AiGraphNodeKind::Result | AiGraphNodeKind::Memory
        ),
        AiGraphEdgeKind::RelatedTo => true,
    }
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

    pub(crate) fn source_memory_reference_id(&self) -> Option<&AiMemoryReferenceId> {
        self.source_memory_reference_id.as_ref()
    }

    pub(crate) fn source_evidence_reference_ids(&self) -> &[AiEvidenceReferenceId] {
        &self.source_evidence_reference_ids
    }

    pub(crate) fn is_grounding_safe(&self) -> bool {
        !matches!(
            self.custody,
            AiCustodyState::Deleted | AiCustodyState::Unavailable
        ) && matches!(self.retention, AiRetentionState::Active)
    }
}
