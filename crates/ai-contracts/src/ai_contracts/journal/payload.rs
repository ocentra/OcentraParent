use super::{AiJournalPayloadKind, AiJournalPayloadReceipt, AiJournalPayloadReference};
use crate::ai_contracts::identity::{AiExplanationId, AiRequestId, AiResultId, AiWorkItemId};

impl AiJournalPayloadReference {
    pub(crate) fn new(
        payload_kind: AiJournalPayloadKind,
        request_id: AiRequestId,
        work_item_id: Option<AiWorkItemId>,
        result_id: Option<AiResultId>,
        explanation_id: Option<AiExplanationId>,
        content: AiJournalPayloadReceipt,
    ) -> Result<Self, &'static str> {
        let valid = match payload_kind {
            AiJournalPayloadKind::WorkItem => {
                work_item_id.is_some() && result_id.is_none() && explanation_id.is_none()
            }
            AiJournalPayloadKind::EvidenceContext => {
                work_item_id.is_none() && result_id.is_none() && explanation_id.is_none()
            }
            AiJournalPayloadKind::Result => {
                work_item_id.is_some() && result_id.is_some() && explanation_id.is_none()
            }
            AiJournalPayloadKind::Explanation => {
                work_item_id.is_none() && result_id.is_some() && explanation_id.is_some()
            }
            AiJournalPayloadKind::RemoteAssistant => {
                work_item_id.is_none() && result_id.is_none() && explanation_id.is_none()
            }
        };
        valid
            .then_some(Self {
                payload_kind,
                request_id,
                work_item_id,
                result_id,
                explanation_id,
                content_digest: content.digest,
            })
            .ok_or("AI journal payload identities do not match the payload kind")
    }

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
