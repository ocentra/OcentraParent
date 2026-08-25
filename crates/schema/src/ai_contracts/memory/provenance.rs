use super::AiProvenanceLink;
use crate::ai_contracts::identity::{AiDigest, AiEvidenceReferenceId, AiFamilyId, AiResultId};

impl AiProvenanceLink {
    pub(crate) fn new(
        family_id: AiFamilyId,
        source_evidence_reference_ids: Vec<AiEvidenceReferenceId>,
        source_result_id: Option<AiResultId>,
        source_digest: Option<AiDigest>,
    ) -> Result<Self, &'static str> {
        if source_evidence_reference_ids.is_empty() && source_result_id.is_none() {
            return Err("AI provenance link requires evidence or result identity");
        }
        Ok(Self {
            family_id,
            source_evidence_reference_ids,
            source_result_id,
            source_digest,
        })
    }

    pub fn source_evidence_reference_ids(&self) -> &[AiEvidenceReferenceId] {
        &self.source_evidence_reference_ids
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn source_result_id(&self) -> Option<&AiResultId> {
        self.source_result_id.as_ref()
    }

    pub fn source_digest(&self) -> Option<&AiDigest> {
        self.source_digest.as_ref()
    }

    pub fn is_grounded(&self) -> bool {
        !self.source_evidence_reference_ids.is_empty() || self.source_result_id.is_some()
    }
}
