use super::{AiClaim, AiResultKind};
use crate::ai_contracts::identity::{
    AiEvidenceReferenceId, AiGraphReferenceId, AiMemoryReferenceId, AiResultId, AiRuleId,
    AiSubjectIdentity,
};
use crate::ai_contracts::{AiAuthorityBoundary, AiConfidence, AiSafeText};

impl AiClaim {
    pub(crate) fn new(
        claim_id: AiResultId,
        result_kind: AiResultKind,
        subject: AiSubjectIdentity,
        label: AiSafeText,
        confidence: AiConfidence,
        evidence_reference_ids: Vec<AiEvidenceReferenceId>,
        memory_reference_ids: Vec<AiMemoryReferenceId>,
        graph_reference_ids: Vec<AiGraphReferenceId>,
        rule_reference_ids: Vec<AiRuleId>,
    ) -> Result<Self, &'static str> {
        if !matches!(result_kind, AiResultKind::NoClaim) && evidence_reference_ids.is_empty() {
            return Err("AI claims require at least one evidence reference");
        }
        Ok(Self {
            claim_id,
            result_kind,
            subject,
            label,
            confidence,
            evidence_reference_ids,
            memory_reference_ids,
            graph_reference_ids,
            rule_reference_ids,
            authority_boundary: AiAuthorityBoundary::EvidenceOnly,
        })
    }

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
