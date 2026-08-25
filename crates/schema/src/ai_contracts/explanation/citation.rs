use super::AiExplanationCitation;
use crate::ai_contracts::identity::{
    AiEvidenceReferenceId, AiGraphReferenceId, AiMemoryReferenceId,
};
use crate::ai_contracts::AiSafeText;

impl AiExplanationCitation {
    pub(crate) fn new(
        evidence_reference_ids: Vec<AiEvidenceReferenceId>,
        memory_reference_ids: Vec<AiMemoryReferenceId>,
        graph_reference_ids: Vec<AiGraphReferenceId>,
        label: AiSafeText,
    ) -> Result<Self, &'static str> {
        if evidence_reference_ids.is_empty() {
            return Err("AI explanation citations require evidence identity");
        }
        Ok(Self {
            evidence_reference_ids,
            memory_reference_ids,
            graph_reference_ids,
            label,
        })
    }

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
