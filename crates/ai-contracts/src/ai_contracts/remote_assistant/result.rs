use super::{AiRemoteAssistantResult, AiRemoteAssistantState};
use crate::ai_contracts::identity::{AiEvidenceReferenceId, AiRemoteAssistantResultId};
use crate::ai_contracts::AiSafeText;

impl AiRemoteAssistantResult {
    pub fn result_id(&self) -> &AiRemoteAssistantResultId {
        &self.result_id
    }

    pub fn request_id(&self) -> &super::super::identity::AiRemoteAssistantRequestId {
        &self.request_id
    }

    pub fn family_id(&self) -> &super::super::identity::AiFamilyId {
        &self.family_id
    }

    pub fn state(&self) -> AiRemoteAssistantState {
        self.state
    }

    pub fn answer(&self) -> Option<&AiSafeText> {
        self.answer.as_ref()
    }

    pub fn cited_evidence_reference_ids(&self) -> &[AiEvidenceReferenceId] {
        &self.cited_evidence_reference_ids
    }
}
