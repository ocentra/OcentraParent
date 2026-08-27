use super::{AiEvidenceReference, AiReferenceValidationState};
use crate::ai_contracts::identity::AiEvidenceReferenceId;
use crate::ai_contracts::{AiCustodyState, AiRedactionState, AiRetentionState};

impl AiEvidenceReference {
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
