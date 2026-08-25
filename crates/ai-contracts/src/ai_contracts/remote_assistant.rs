use serde::{Deserialize, Serialize};

use super::{
    context::{AiPromptReference, AiRuntimeReference},
    identity::{
        AiActorIdentity, AiAuthorizationReferenceId, AiEvidenceReferenceId, AiFamilyId,
        AiRemoteAssistantRequestId, AiRemoteAssistantResultId, AiSchemaVersion, AiTimestamp,
    },
    AiCustodyState, AiDegradedState, AiRedactionState, AiRetentionState, AiSafeText,
    AiUntrustedText, AiValidationState,
};

mod authorization;
mod request;
mod result;
mod source;
mod wire;

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

impl AiRemoteAssistantState {
    pub(super) fn binding_label(self) -> &'static [u8] {
        match self {
            Self::Disabled => b"disabled",
            Self::AwaitingParentAuthorization => b"awaiting-parent-authorization",
            Self::Authorized => b"authorized",
            Self::Submitted => b"submitted",
            Self::Succeeded => b"succeeded",
            Self::Degraded => b"degraded",
            Self::ManualRequired => b"manual-required",
        }
    }
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

/// In-process parent authorization. It is owner-issued and serialize-only;
/// wire callers receive a reference and cannot deserialize this authority.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiParentAuthorization {
    authorization_reference_id: AiAuthorizationReferenceId,
    actor: AiActorIdentity,
    authorized_at: AiTimestamp,
    expires_at: AiTimestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiRemoteAssistantSourceBundle {
    family_id: AiFamilyId,
    evidence_reference_ids: Vec<AiEvidenceReferenceId>,
    authorization: AiParentAuthorization,
    custody: AiCustodyState,
    retention: AiRetentionState,
    redaction: AiRedactionState,
    redaction_policy: AiRemoteAssistantRedactionPolicy,
    safety_boundary: AiRemoteAssistantSafetyBoundary,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiRemoteAssistantRequest {
    schema_version: AiSchemaVersion,
    request_id: AiRemoteAssistantRequestId,
    source_bundle: AiRemoteAssistantSourceBundle,
    prompt: AiPromptReference,
    runtime: Option<AiRuntimeReference>,
    requested_at: AiTimestamp,
    state: AiRemoteAssistantState,
}

/// An untrusted wire prompt. Authorization consumes this exact task through an
/// owner-held redaction receipt; it cannot be silently replaced by another
/// trusted prompt.
#[derive(Clone, PartialEq)]
pub struct AiRemoteAssistantWirePrompt {
    template_id: super::identity::AiPromptTemplateId,
    version: super::identity::AiPromptVersion,
    task: AiUntrustedText,
}

/// Untrusted wire metadata for a remote-assistant request. Only the
/// authorization reference crosses this boundary. Evidence and custody are
/// resolved by an owner adapter and cannot be supplied by the wire caller.
#[derive(Clone, PartialEq)]
pub struct AiRemoteAssistantWireRequest {
    schema_version: AiSchemaVersion,
    request_id: AiRemoteAssistantRequestId,
    family_id: AiFamilyId,
    authorization_reference_id: AiAuthorizationReferenceId,
    prompt: AiRemoteAssistantWirePrompt,
    requested_at: AiTimestamp,
    state: AiRemoteAssistantState,
}

/// Owner-resolved source metadata. Wire requests intentionally cannot create
/// this value; an owner adapter must resolve the evidence IDs and custody
/// receipt from its own durable state before authorization can proceed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AiRemoteAssistantOwnerResolvedSource {
    request_id: AiRemoteAssistantRequestId,
    family_id: AiFamilyId,
    authorization_reference_id: AiAuthorizationReferenceId,
    evidence_reference_ids: Vec<AiEvidenceReferenceId>,
    custody: AiCustodyState,
    retention: AiRetentionState,
    redaction: AiRedactionState,
    redaction_policy: AiRemoteAssistantRedactionPolicy,
    safety_boundary: AiRemoteAssistantSafetyBoundary,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiRemoteAssistantResult {
    schema_version: AiSchemaVersion,
    result_id: AiRemoteAssistantResultId,
    request_id: AiRemoteAssistantRequestId,
    family_id: AiFamilyId,
    state: AiRemoteAssistantState,
    validation: AiValidationState,
    degraded_state: AiDegradedState,
    answer: Option<AiSafeText>,
    cited_evidence_reference_ids: Vec<AiEvidenceReferenceId>,
    safety_boundary: AiRemoteAssistantSafetyBoundary,
    redaction: AiRedactionState,
    retention: AiRetentionState,
    returned_at: AiTimestamp,
}
