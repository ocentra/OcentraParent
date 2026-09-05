use super::{AiContextBuildState, AiEvidenceContext, AiEvidenceContextBuildResult};
use crate::ai_contracts::identity::{AiRequestId, AiSchemaVersion};
use crate::ai_contracts::{AiAuthorityBoundary, AiValidationState};

impl AiEvidenceContext {
    pub fn schema_version(&self) -> &AiSchemaVersion {
        &self.schema_version
    }

    pub fn request_id(&self) -> &AiRequestId {
        &self.request_id
    }

    pub fn family_id(&self) -> &super::AiFamilyId {
        &self.family_id
    }

    pub fn evidence(&self) -> &[super::AiEvidenceReference] {
        &self.evidence
    }

    pub fn memory(&self) -> &[super::AiMemoryReference] {
        &self.memory
    }

    pub fn graph(&self) -> &[super::AiGraphReference] {
        &self.graph
    }

    pub fn authority_boundary(&self) -> AiAuthorityBoundary {
        self.authority_boundary
    }
}

impl AiEvidenceContextBuildResult {
    pub fn request_id(&self) -> &AiRequestId {
        &self.request_id
    }

    pub fn state(&self) -> AiContextBuildState {
        self.state
    }

    pub fn validation(&self) -> AiValidationState {
        self.validation
    }

    pub fn context(&self) -> Option<&AiEvidenceContext> {
        self.context.as_ref()
    }
}
