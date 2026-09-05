use super::{AiMemoryReference, AiMemoryReferenceKind};
use crate::ai_contracts::identity::{AiFamilyId, AiMemoryReferenceId};

impl AiMemoryReference {
    pub fn memory_reference_id(&self) -> &AiMemoryReferenceId {
        &self.memory_reference_id
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn kind(&self) -> AiMemoryReferenceKind {
        self.kind
    }

    pub fn provenance(&self) -> &super::AiProvenanceLink {
        &self.provenance
    }
}
