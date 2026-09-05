use super::{AiJournalPayloadKind, AiJournalPayloadReference};
use crate::ai_contracts::identity::{AiExplanationId, AiRequestId, AiResultId, AiWorkItemId};

impl AiJournalPayloadReference {
    pub fn payload_kind(&self) -> AiJournalPayloadKind {
        self.payload_kind
    }

    pub fn request_id(&self) -> &AiRequestId {
        &self.request_id
    }

    pub fn work_item_id(&self) -> Option<&AiWorkItemId> {
        self.work_item_id.as_ref()
    }

    pub fn result_id(&self) -> Option<&AiResultId> {
        self.result_id.as_ref()
    }

    pub fn explanation_id(&self) -> Option<&AiExplanationId> {
        self.explanation_id.as_ref()
    }

    pub fn content_digest(&self) -> &super::AiDigest {
        &self.content_digest
    }
}
