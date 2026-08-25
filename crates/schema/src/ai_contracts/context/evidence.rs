use super::{AiEvidenceReference, AiReferenceValidationState};
use crate::ai_contracts::identity::AiEvidenceReferenceId;
use crate::ai_contracts::{AiCustodyState, AiRedactionState, AiRetentionState};

impl AiEvidenceReference {
    pub(crate) fn new(
        evidence_reference_id: AiEvidenceReferenceId,
        family_id: super::AiFamilyId,
        evidence_kind: super::AiEvidenceKind,
        provenance: super::AiEvidenceProvenance,
        custody: AiCustodyState,
        retention: AiRetentionState,
        redaction: AiRedactionState,
        confidence: Option<super::AiConfidence>,
        validation: AiReferenceValidationState,
    ) -> Result<Self, &'static str> {
        if provenance.family_id() != &family_id {
            return Err("AI evidence provenance family does not match its reference");
        }
        let blocked = matches!(
            custody,
            AiCustodyState::Deleted | AiCustodyState::Unavailable
        ) || matches!(
            retention,
            AiRetentionState::Deleted | AiRetentionState::Tombstoned
        ) || matches!(redaction, AiRedactionState::RejectedPrivatePayload);
        if matches!(validation, AiReferenceValidationState::Validated) && blocked {
            return Err("AI evidence cannot be validated while custody or retention is blocked");
        }
        if matches!(validation, AiReferenceValidationState::Rejected)
            && !matches!(redaction, AiRedactionState::RejectedPrivatePayload)
            && !blocked
        {
            return Err("AI rejected evidence must carry a blocked or rejected state");
        }
        Ok(Self {
            evidence_reference_id,
            family_id,
            evidence_kind,
            provenance,
            custody,
            retention,
            redaction,
            confidence,
            validation,
        })
    }

    pub fn evidence_reference_id(&self) -> &AiEvidenceReferenceId {
        &self.evidence_reference_id
    }

    pub fn family_id(&self) -> &super::AiFamilyId {
        &self.family_id
    }

    pub fn evidence_kind(&self) -> super::AiEvidenceKind {
        self.evidence_kind
    }

    pub fn provenance(&self) -> &super::AiEvidenceProvenance {
        &self.provenance
    }

    pub fn custody(&self) -> AiCustodyState {
        self.custody
    }

    pub fn retention(&self) -> AiRetentionState {
        self.retention
    }

    pub fn redaction(&self) -> AiRedactionState {
        self.redaction
    }

    pub fn validation(&self) -> AiReferenceValidationState {
        self.validation
    }
}
