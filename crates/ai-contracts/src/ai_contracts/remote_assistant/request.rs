use super::{AiRemoteAssistantRequest, AiRemoteAssistantState};
use crate::ai_contracts::identity::{AiRemoteAssistantRequestId, AiSchemaVersion};

impl AiRemoteAssistantRequest {
    pub fn schema_version(&self) -> &AiSchemaVersion {
        &self.schema_version
    }

    pub fn request_id(&self) -> &AiRemoteAssistantRequestId {
        &self.request_id
    }

    pub fn source_bundle(&self) -> &super::AiRemoteAssistantSourceBundle {
        &self.source_bundle
    }

    pub fn state(&self) -> AiRemoteAssistantState {
        self.state
    }
}
