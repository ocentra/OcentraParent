use super::AiClaim;
use crate::ai_contracts::identity::{
    AiEvidenceReferenceId, AiGraphReferenceId, AiMemoryReferenceId, AiResultId, AiRuleId,
    AiSubjectIdentity,
};
use crate::ai_contracts::AiAuthorityBoundary;

impl AiClaim {
    pub fn claim_id(&self) -> &AiResultId {
        &self.claim_id
    }

    pub fn subject(&self) -> &AiSubjectIdentity {
        &self.subject
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

    pub fn rule_reference_ids(&self) -> &[AiRuleId] {
        &self.rule_reference_ids
    }

    pub fn is_grounded(&self) -> bool {
        !self.evidence_reference_ids.is_empty()
    }

    pub fn authority_boundary(&self) -> AiAuthorityBoundary {
        self.authority_boundary
    }
}
