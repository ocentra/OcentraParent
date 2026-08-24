use serde::{Deserialize, Serialize};

use super::{
    context::{AiPromptReference, AiRuntimeReference},
    identity::{
        AiActorIdentity, AiAuthorizationReferenceId, AiEvidenceReferenceId, AiFamilyId,
        AiRemoteAssistantRequestId, AiRemoteAssistantResultId, AiSchemaVersion, AiTimestamp,
    },
    AiCustodyState, AiDegradedState, AiRedactionState, AiRetentionState, AiText, AiValidationState,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiRemoteAssistantState {
    Disabled,
    AwaitingParentAuthorization,
    Authorized,
    Submitted,
    Succeeded,
    Degraded,
    ManualRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiRemoteAssistantSafetyBoundary {
    ParentReportOnly,
    OutsideChildSafetyBlockingPath,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiRemoteAssistantRedactionPolicy {
    ReferencesOnly,
    RedactedSummaries,
    NoChildPayload,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiParentAuthorization {
    pub authorization_reference_id: AiAuthorizationReferenceId,
    pub actor: AiActorIdentity,
    pub authorized_at: AiTimestamp,
    pub expires_at: AiTimestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiRemoteAssistantSourceBundle {
    pub family_id: AiFamilyId,
    pub evidence_reference_ids: Vec<AiEvidenceReferenceId>,
    pub authorization: AiParentAuthorization,
    pub custody: AiCustodyState,
    pub retention: AiRetentionState,
    pub redaction: AiRedactionState,
    pub redaction_policy: AiRemoteAssistantRedactionPolicy,
    pub safety_boundary: AiRemoteAssistantSafetyBoundary,
}

impl AiRemoteAssistantSourceBundle {
    pub fn excludes_raw_child_payload(&self) -> bool {
        let redacted = matches!(
            self.redaction,
            AiRedactionState::Redacted | AiRedactionState::FullyRedacted
        );
        redacted
            && matches!(
                self.redaction_policy,
                AiRemoteAssistantRedactionPolicy::ReferencesOnly
                    | AiRemoteAssistantRedactionPolicy::RedactedSummaries
                    | AiRemoteAssistantRedactionPolicy::NoChildPayload
            )
            && matches!(
                self.safety_boundary,
                AiRemoteAssistantSafetyBoundary::OutsideChildSafetyBlockingPath
                    | AiRemoteAssistantSafetyBoundary::ParentReportOnly
            )
    }

    pub fn is_custody_safe(&self) -> bool {
        !self.evidence_reference_ids.is_empty()
            && matches!(self.custody, AiCustodyState::ParentAuthorizedRedacted)
            && self.excludes_raw_child_payload()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiRemoteAssistantRequest {
    pub schema_version: AiSchemaVersion,
    pub request_id: AiRemoteAssistantRequestId,
    pub source_bundle: AiRemoteAssistantSourceBundle,
    pub prompt: AiPromptReference,
    pub runtime: Option<AiRuntimeReference>,
    pub requested_at: AiTimestamp,
    pub state: AiRemoteAssistantState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiRemoteAssistantResult {
    pub schema_version: AiSchemaVersion,
    pub result_id: AiRemoteAssistantResultId,
    pub request_id: AiRemoteAssistantRequestId,
    pub state: AiRemoteAssistantState,
    pub validation: AiValidationState,
    pub degraded_state: AiDegradedState,
    pub answer: Option<AiText>,
    pub cited_evidence_reference_ids: Vec<AiEvidenceReferenceId>,
    pub safety_boundary: AiRemoteAssistantSafetyBoundary,
    pub redaction: AiRedactionState,
    pub retention: AiRetentionState,
    pub returned_at: AiTimestamp,
}
