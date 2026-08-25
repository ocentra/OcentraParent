use super::{AiMemoryReference, AiMemoryReferenceKind};
use crate::ai_contracts::identity::{AiFamilyId, AiMemoryReferenceId, AiTimestamp};
use crate::ai_contracts::{AiConfidence, AiCustodyState, AiRetentionState};

impl AiMemoryReference {
    pub(crate) fn new(
        memory_reference_id: AiMemoryReferenceId,
        family_id: AiFamilyId,
        kind: AiMemoryReferenceKind,
        provenance: super::AiProvenanceLink,
        generated_at: AiTimestamp,
        expires_at: Option<AiTimestamp>,
        confidence: AiConfidence,
        custody: AiCustodyState,
        retention: AiRetentionState,
    ) -> Result<Self, &'static str> {
        if provenance.family_id() != &family_id {
            return Err("AI memory provenance family does not match its reference");
        }
        if !generated_at.is_well_formed()
            || expires_at
                .as_ref()
                .is_some_and(|expires| !generated_at.precedes(expires))
            || matches!(
                custody,
                AiCustodyState::Deleted | AiCustodyState::Unavailable
            )
            || matches!(
                retention,
                AiRetentionState::Deleted | AiRetentionState::Tombstoned
            )
        {
            return Err("AI memory reference has invalid time, custody, or retention state");
        }
        Ok(Self {
            memory_reference_id,
            family_id,
            kind,
            provenance,
            generated_at,
            expires_at,
            confidence,
            custody,
            retention,
        })
    }

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
