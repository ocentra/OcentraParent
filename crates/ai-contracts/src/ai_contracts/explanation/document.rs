use super::AiExplanation;
use crate::ai_contracts::identity::{
    AiExplanationId, AiFamilyId, AiRequestId, AiResultId, AiSchemaVersion,
};
use crate::ai_contracts::{AiAuthorityBoundary, AiRedactionState};

impl AiExplanation {
    pub fn schema_version(&self) -> &AiSchemaVersion {
        &self.schema_version
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn explanation_id(&self) -> &AiExplanationId {
        &self.explanation_id
    }

    pub fn request_id(&self) -> &AiRequestId {
        &self.request_id
    }

    pub fn result_id(&self) -> &AiResultId {
        &self.result_id
    }

    pub fn sections(&self) -> &[super::AiExplanationSection] {
        &self.sections
    }

    pub fn authority_boundary(&self) -> AiAuthorityBoundary {
        self.authority_boundary
    }

    pub fn redaction(&self) -> AiRedactionState {
        self.redaction
    }
}
