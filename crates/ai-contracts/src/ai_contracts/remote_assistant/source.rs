use super::{
    AiParentAuthorization, AiRemoteAssistantOwnerResolvedSource, AiRemoteAssistantRedactionPolicy,
    AiRemoteAssistantSafetyBoundary, AiRemoteAssistantSourceBundle,
};
use crate::ai_contracts::identity::{
    AiAuthorizationReferenceId, AiEvidenceReferenceId, AiFamilyId, AiRemoteAssistantRequestId,
};
use crate::ai_contracts::{AiCustodyState, AiRedactionState, AiRetentionState};

impl AiRemoteAssistantSourceBundle {
    pub(super) fn from_owner_resolved(
        source: AiRemoteAssistantOwnerResolvedSource,
        authorization: AiParentAuthorization,
    ) -> Result<Self, &'static str> {
        if authorization.family_id() != Some(&source.family_id)
            || authorization.authorization_reference_id() != &source.authorization_reference_id
        {
            return Err("AI remote source bundle is not parent-authorized and redacted");
        }
        Ok(Self {
            family_id: source.family_id,
            evidence_reference_ids: source.evidence_reference_ids,
            authorization,
            custody: source.custody,
            retention: source.retention,
            redaction: source.redaction,
            redaction_policy: source.redaction_policy,
            safety_boundary: source.safety_boundary,
        })
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn evidence_reference_ids(&self) -> &[AiEvidenceReferenceId] {
        &self.evidence_reference_ids
    }

    pub fn authorization(&self) -> &super::AiParentAuthorization {
        &self.authorization
    }

    pub fn safety_boundary(&self) -> AiRemoteAssistantSafetyBoundary {
        self.safety_boundary
    }

    pub fn excludes_raw_child_payload(&self) -> bool {
        self.redaction.is_safe()
            && matches!(
                self.redaction_policy,
                AiRemoteAssistantRedactionPolicy::ReferencesOnly
                    | AiRemoteAssistantRedactionPolicy::RedactedSummaries
                    | AiRemoteAssistantRedactionPolicy::NoChildPayload
            )
            && matches!(self.custody, AiCustodyState::ParentAuthorizedRedacted)
            && matches!(
                self.safety_boundary,
                AiRemoteAssistantSafetyBoundary::OutsideChildSafetyBlockingPath
            )
    }

    pub fn is_custody_safe(&self) -> bool {
        !self.evidence_reference_ids.is_empty() && self.excludes_raw_child_payload()
    }
}

impl AiRemoteAssistantOwnerResolvedSource {
    pub(crate) fn from_owner(
        request_id: AiRemoteAssistantRequestId,
        family_id: AiFamilyId,
        authorization_reference_id: AiAuthorizationReferenceId,
        evidence_reference_ids: Vec<AiEvidenceReferenceId>,
        custody: AiCustodyState,
        retention: AiRetentionState,
        redaction: AiRedactionState,
        redaction_policy: AiRemoteAssistantRedactionPolicy,
        safety_boundary: AiRemoteAssistantSafetyBoundary,
    ) -> Result<Self, &'static str> {
        if evidence_reference_ids.is_empty()
            || !matches!(custody, AiCustodyState::ParentAuthorizedRedacted)
            || !matches!(retention, AiRetentionState::Active)
            || !redaction.is_safe()
            || !matches!(
                safety_boundary,
                AiRemoteAssistantSafetyBoundary::OutsideChildSafetyBlockingPath
            )
        {
            return Err("AI owner source resolution is not a safe redacted custody binding");
        }
        Ok(Self {
            request_id,
            family_id,
            authorization_reference_id,
            evidence_reference_ids,
            custody,
            retention,
            redaction,
            redaction_policy,
            safety_boundary,
        })
    }

    pub(super) fn request_id(&self) -> &AiRemoteAssistantRequestId {
        &self.request_id
    }

    pub(super) fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub(super) fn authorization_reference_id(&self) -> &AiAuthorizationReferenceId {
        &self.authorization_reference_id
    }
}
