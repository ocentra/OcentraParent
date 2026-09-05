use super::AiResult;
use crate::ai_contracts::identity::{
    AiDigest, AiFamilyId, AiRequestId, AiResultId, AiSchemaVersion, AiWorkItemId,
};
use crate::ai_contracts::AiAuthorityBoundary;
use crate::ai_contracts::AiValidationState;

impl AiResult {
    pub fn schema_version(&self) -> &AiSchemaVersion {
        &self.schema_version
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn result_id(&self) -> &AiResultId {
        &self.result_id
    }

    pub fn request_id(&self) -> &AiRequestId {
        &self.request_id
    }

    pub fn work_item_id(&self) -> &AiWorkItemId {
        &self.work_item_id
    }

    pub fn validation(&self) -> AiValidationState {
        self.validation
    }

    pub fn authority_boundary(&self) -> AiAuthorityBoundary {
        self.authority_boundary
    }

    pub fn digest(&self) -> &AiDigest {
        &self.digest
    }
}
