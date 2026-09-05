use super::AiExplanationCitation;
use crate::ai_contracts::identity::{
    AiEvidenceReferenceId, AiGraphReferenceId, AiMemoryReferenceId,
};
use crate::ai_contracts::AiSafeText;

impl AiExplanationCitation {
    pub fn evidence_reference_ids(&self) -> &[AiEvidenceReferenceId] {
        &self.evidence_reference_ids
    }

    pub fn memory_reference_ids(&self) -> &[AiMemoryReferenceId] {
        &self.memory_reference_ids
    }

    pub fn graph_reference_ids(&self) -> &[AiGraphReferenceId] {
        &self.graph_reference_ids
    }

    pub fn label(&self) -> &AiSafeText {
        &self.label
    }

    pub fn is_grounded(&self) -> bool {
        !self.evidence_reference_ids.is_empty()
    }
}
