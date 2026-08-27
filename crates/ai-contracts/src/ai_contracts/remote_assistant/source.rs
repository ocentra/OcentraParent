use super::{
    AiRemoteAssistantRedactionPolicy, AiRemoteAssistantSafetyBoundary,
    AiRemoteAssistantSourceBundle,
};
use crate::ai_contracts::identity::{AiEvidenceReferenceId, AiFamilyId};
use crate::ai_contracts::AiCustodyState;

impl AiRemoteAssistantSourceBundle {
    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn evidence_reference_ids(&self) -> &[AiEvidenceReferenceId] {
        &self.evidence_reference_ids
    }

    pub fn authorization(&self) -> &super::AiParentAuthorization {
        &self.authorization
    }

    pub fn safety_boundary(&self) -> AiRemoteAssistantSafetyBoundary {
        self.safety_boundary
    }

    pub fn excludes_raw_child_payload(&self) -> bool {
        self.redaction.is_safe()
            && matches!(
                self.redaction_policy,
                AiRemoteAssistantRedactionPolicy::ReferencesOnly
                    | AiRemoteAssistantRedactionPolicy::RedactedSummaries
                    | AiRemoteAssistantRedactionPolicy::NoChildPayload
            )
            && matches!(self.custody, AiCustodyState::ParentAuthorizedRedacted)
            && matches!(
                self.safety_boundary,
                AiRemoteAssistantSafetyBoundary::OutsideChildSafetyBlockingPath
            )
    }

    pub fn is_custody_safe(&self) -> bool {
        !self.evidence_reference_ids.is_empty() && self.excludes_raw_child_payload()
    }
}
