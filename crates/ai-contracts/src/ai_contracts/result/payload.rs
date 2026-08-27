use super::AiResultPayload;
use crate::ai_contracts::context::AiEvidenceReference;
use crate::ai_contracts::identity::AiFamilyId;
use crate::ai_contracts::memory::{AiGraphReference, AiMemoryReference};

impl AiResultPayload {
    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn claims(&self) -> &[super::AiClaim] {
        &self.claims
    }

    pub fn evidence(&self) -> &[AiEvidenceReference] {
        &self.evidence
    }

    pub fn memory(&self) -> &[AiMemoryReference] {
        &self.memory
    }

    pub fn graph(&self) -> &[AiGraphReference] {
        &self.graph
    }
}
