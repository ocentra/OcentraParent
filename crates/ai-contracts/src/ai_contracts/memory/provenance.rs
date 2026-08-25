use super::{AiProvenanceLink, AiResultProvenanceReceipt};
use crate::ai_contracts::identity::{AiDigest, AiEvidenceReferenceId, AiFamilyId, AiResultId};
use crate::ai_contracts::result::AiResult;

impl AiResultProvenanceReceipt {
    pub(crate) fn from_result(result: &AiResult) -> Self {
        Self {
            family_id: result.family_id().clone(),
            result_id: result.result_id().clone(),
            digest: result.digest().clone(),
        }
    }

    pub(crate) fn is_for_family(&self, family_id: &AiFamilyId) -> bool {
        &self.family_id == family_id
    }

    pub(crate) fn into_parts(self) -> (AiResultId, AiDigest) {
        (self.result_id, self.digest)
    }
}

impl AiProvenanceLink {
    pub(crate) fn new(
        family_id: AiFamilyId,
        source_evidence_reference_ids: Vec<AiEvidenceReferenceId>,
        source_result: Option<AiResultProvenanceReceipt>,
    ) -> Result<Self, &'static str> {
        if source_result
            .as_ref()
            .is_some_and(|receipt| !receipt.is_for_family(&family_id))
        {
            return Err("AI provenance result family does not match its memory family");
        }
        let (source_result_id, source_digest) = source_result
            .map(AiResultProvenanceReceipt::into_parts)
            .map_or((None, None), |(result_id, digest)| {
                (Some(result_id), Some(digest))
            });
        if source_evidence_reference_ids.is_empty() && source_result_id.is_none()
            || source_result_id.is_some() != source_digest.is_some()
            || source_digest
                .as_ref()
                .is_some_and(|digest| !digest.is_canonical())
        {
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
