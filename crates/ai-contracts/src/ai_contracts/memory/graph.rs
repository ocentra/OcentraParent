use super::AiGraphReference;
use super::{AiGraphEdgeKind, AiGraphNodeKind};
use crate::ai_contracts::identity::{AiFamilyId, AiGraphNodeId, AiGraphReferenceId};

impl AiGraphReference {
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
