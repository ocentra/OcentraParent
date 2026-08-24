use serde::{Deserialize, Serialize};

use super::{
    context::{AiPromptReference, AiRuntimeReference},
    identity::{
        AiActorIdentity, AiAuthorizationReferenceId, AiEvidenceReferenceId, AiFamilyId,
        AiRemoteAssistantRequestId, AiRemoteAssistantResultId, AiSchemaVersion, AiTimestamp,
    },
    validate_contract_schema_version, AiCustodyState, AiDegradedState, AiRedactionState,
    AiRetentionState, AiSafeText, AiUntrustedText, AiValidationState,
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

impl AiParentAuthorization {
    pub(crate) fn issue(
        authorization_reference_id: AiAuthorizationReferenceId,
        actor: AiActorIdentity,
        authorized_at: AiTimestamp,
        expires_at: AiTimestamp,
    ) -> Result<Self, &'static str> {
        if !actor.is_parent_authority()
            || actor.subject().is_none()
            || !authorized_at.precedes(&expires_at)
        {
            return Err("AI remote authorization is not parent-issued or has an invalid lifetime");
        }
        Ok(Self {
            authorization_reference_id,
            actor,
            authorized_at,
            expires_at,
        })
    }

    pub fn authorization_reference_id(&self) -> &AiAuthorizationReferenceId {
        &self.authorization_reference_id
    }

    pub fn actor(&self) -> &AiActorIdentity {
        &self.actor
    }

    pub fn family_id(&self) -> Option<&AiFamilyId> {
        self.actor.subject().map(|subject| subject.family_id())
    }

    pub fn authorized_at(&self) -> &AiTimestamp {
        &self.authorized_at
    }

    pub fn expires_at(&self) -> &AiTimestamp {
        &self.expires_at
    }
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

impl AiRemoteAssistantSourceBundle {
    pub(crate) fn new(
        family_id: AiFamilyId,
        evidence_reference_ids: Vec<AiEvidenceReferenceId>,
        authorization: AiParentAuthorization,
        custody: AiCustodyState,
        retention: AiRetentionState,
        redaction: AiRedactionState,
        redaction_policy: AiRemoteAssistantRedactionPolicy,
        safety_boundary: AiRemoteAssistantSafetyBoundary,
    ) -> Result<Self, &'static str> {
        if evidence_reference_ids.is_empty()
            || authorization.family_id() != Some(&family_id)
            || !matches!(custody, AiCustodyState::ParentAuthorizedRedacted)
            || !matches!(retention, AiRetentionState::Active)
            || !redaction.is_safe()
            || !matches!(
                safety_boundary,
                AiRemoteAssistantSafetyBoundary::OutsideChildSafetyBlockingPath
            )
        {
            return Err("AI remote source bundle is not parent-authorized and redacted");
        }
        Ok(Self {
            family_id,
            evidence_reference_ids,
            authorization,
            custody,
            retention,
            redaction,
            redaction_policy,
            safety_boundary,
        })
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn evidence_reference_ids(&self) -> &[AiEvidenceReferenceId] {
        &self.evidence_reference_ids
    }

    pub fn authorization(&self) -> &AiParentAuthorization {
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

impl AiRemoteAssistantRequest {
    pub(crate) fn submit(
        schema_version: AiSchemaVersion,
        request_id: AiRemoteAssistantRequestId,
        source_bundle: AiRemoteAssistantSourceBundle,
        prompt: AiPromptReference,
        runtime: Option<AiRuntimeReference>,
        requested_at: AiTimestamp,
    ) -> Result<Self, &'static str> {
        validate_contract_schema_version(&schema_version)?;
        let authorization = source_bundle.authorization();
        let within_authorization_window = authorization.authorized_at().as_str()
            <= requested_at.as_str()
            && requested_at.as_str() < authorization.expires_at().as_str();
        if !source_bundle.is_custody_safe()
            || !requested_at.is_well_formed()
            || !within_authorization_window
        {
            return Err("AI remote request is not fail-closed safe");
        }
        Ok(Self {
            schema_version,
            request_id,
            source_bundle,
            prompt,
            runtime,
            requested_at,
            state: AiRemoteAssistantState::Submitted,
        })
    }

    pub fn schema_version(&self) -> &AiSchemaVersion {
        &self.schema_version
    }

    pub fn request_id(&self) -> &AiRemoteAssistantRequestId {
        &self.request_id
    }

    pub fn source_bundle(&self) -> &AiRemoteAssistantSourceBundle {
        &self.source_bundle
    }

    pub fn state(&self) -> AiRemoteAssistantState {
        self.state
    }
}

/// An untrusted wire prompt. Its task cannot be converted into `AiSafeText`
/// here; a trusted owner must inspect it and provide an owner-issued prompt.
#[derive(Clone, PartialEq)]
pub struct AiRemoteAssistantWirePrompt {
    template_id: super::identity::AiPromptTemplateId,
    version: super::identity::AiPromptVersion,
    task: AiUntrustedText,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiRemoteAssistantWirePromptFields {
    template_id: super::identity::AiPromptTemplateId,
    version: super::identity::AiPromptVersion,
    task: AiUntrustedText,
}

impl<'de> Deserialize<'de> for AiRemoteAssistantWirePrompt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fields = AiRemoteAssistantWirePromptFields::deserialize(deserializer)?;
        Ok(Self {
            template_id: fields.template_id,
            version: fields.version,
            task: fields.task,
        })
    }
}

impl AiRemoteAssistantWirePrompt {
    pub fn template_id(&self) -> &super::identity::AiPromptTemplateId {
        &self.template_id
    }

    pub fn version(&self) -> &super::identity::AiPromptVersion {
        &self.version
    }
}

/// Untrusted wire metadata for a remote-assistant request. This deliberately
/// carries only an authorization reference, never parent authority itself.
/// A trusted owner must bind that reference and an owner-issued prompt to an
/// in-process `AiParentAuthorization` before creating
/// `AiRemoteAssistantRequest`.
#[derive(Clone, PartialEq)]
pub struct AiRemoteAssistantWireRequest {
    schema_version: AiSchemaVersion,
    request_id: AiRemoteAssistantRequestId,
    family_id: AiFamilyId,
    evidence_reference_ids: Vec<AiEvidenceReferenceId>,
    authorization_reference_id: AiAuthorizationReferenceId,
    custody: AiCustodyState,
    retention: AiRetentionState,
    redaction: AiRedactionState,
    redaction_policy: AiRemoteAssistantRedactionPolicy,
    safety_boundary: AiRemoteAssistantSafetyBoundary,
    prompt: AiRemoteAssistantWirePrompt,
    runtime: Option<AiRuntimeReference>,
    requested_at: AiTimestamp,
    state: AiRemoteAssistantState,
}

impl AiRemoteAssistantWireRequest {
    fn from_parts(
        schema_version: AiSchemaVersion,
        request_id: AiRemoteAssistantRequestId,
        family_id: AiFamilyId,
        evidence_reference_ids: Vec<AiEvidenceReferenceId>,
        authorization_reference_id: AiAuthorizationReferenceId,
        custody: AiCustodyState,
        retention: AiRetentionState,
        redaction: AiRedactionState,
        redaction_policy: AiRemoteAssistantRedactionPolicy,
        safety_boundary: AiRemoteAssistantSafetyBoundary,
        prompt: AiRemoteAssistantWirePrompt,
        runtime: Option<AiRuntimeReference>,
        requested_at: AiTimestamp,
        state: AiRemoteAssistantState,
    ) -> Result<Self, &'static str> {
        validate_contract_schema_version(&schema_version)?;
        if !matches!(state, AiRemoteAssistantState::Submitted)
            || evidence_reference_ids.is_empty()
            || !matches!(custody, AiCustodyState::ParentAuthorizedRedacted)
            || !matches!(retention, AiRetentionState::Active)
            || !redaction.is_safe()
            || !matches!(
                safety_boundary,
                AiRemoteAssistantSafetyBoundary::OutsideChildSafetyBlockingPath
            )
            || !requested_at.is_well_formed()
        {
            return Err("AI remote wire request is not fail-closed safe");
        }
        Ok(Self {
            schema_version,
            request_id,
            family_id,
            evidence_reference_ids,
            authorization_reference_id,
            custody,
            retention,
            redaction,
            redaction_policy,
            safety_boundary,
            prompt,
            runtime,
            requested_at,
            state,
        })
    }

    pub fn authorization_reference_id(&self) -> &AiAuthorizationReferenceId {
        &self.authorization_reference_id
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn request_id(&self) -> &AiRemoteAssistantRequestId {
        &self.request_id
    }

    pub(crate) fn authorize(
        self,
        authorization: AiParentAuthorization,
        prompt: AiPromptReference,
    ) -> Result<AiRemoteAssistantRequest, &'static str> {
        if authorization.authorization_reference_id() != &self.authorization_reference_id
            || authorization.family_id() != Some(&self.family_id)
            || prompt.template_id() != self.prompt.template_id()
            || prompt.version() != self.prompt.version()
        {
            return Err("AI remote wire authorization reference is not bound to the request");
        }
        let source_bundle = AiRemoteAssistantSourceBundle::new(
            self.family_id,
            self.evidence_reference_ids,
            authorization,
            self.custody,
            self.retention,
            self.redaction,
            self.redaction_policy,
            self.safety_boundary,
        )?;
        AiRemoteAssistantRequest::submit(
            self.schema_version,
            self.request_id,
            source_bundle,
            prompt,
            self.runtime,
            self.requested_at,
        )
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiRemoteAssistantWireRequestFields {
    schema_version: AiSchemaVersion,
    request_id: AiRemoteAssistantRequestId,
    family_id: AiFamilyId,
    evidence_reference_ids: Vec<AiEvidenceReferenceId>,
    authorization_reference_id: AiAuthorizationReferenceId,
    custody: AiCustodyState,
    retention: AiRetentionState,
    redaction: AiRedactionState,
    redaction_policy: AiRemoteAssistantRedactionPolicy,
    safety_boundary: AiRemoteAssistantSafetyBoundary,
    prompt: AiRemoteAssistantWirePrompt,
    runtime: Option<AiRuntimeReference>,
    requested_at: AiTimestamp,
    state: AiRemoteAssistantState,
}

impl<'de> Deserialize<'de> for AiRemoteAssistantWireRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fields = AiRemoteAssistantWireRequestFields::deserialize(deserializer)?;
        Self::from_parts(
            fields.schema_version,
            fields.request_id,
            fields.family_id,
            fields.evidence_reference_ids,
            fields.authorization_reference_id,
            fields.custody,
            fields.retention,
            fields.redaction,
            fields.redaction_policy,
            fields.safety_boundary,
            fields.prompt,
            fields.runtime,
            fields.requested_at,
            fields.state,
        )
        .map_err(serde::de::Error::custom)
    }
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

impl AiRemoteAssistantResult {
    pub(crate) fn new(
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
    ) -> Result<Self, &'static str> {
        validate_contract_schema_version(&schema_version)?;
        let valid_state = matches!(
            (state, validation),
            (
                AiRemoteAssistantState::Succeeded,
                AiValidationState::Accepted
            ) | (
                AiRemoteAssistantState::Degraded,
                AiValidationState::ManualRequired
            ) | (
                AiRemoteAssistantState::ManualRequired,
                AiValidationState::ManualRequired
            )
        );
        let valid_degraded_state = match state {
            AiRemoteAssistantState::Succeeded => matches!(degraded_state, AiDegradedState::None),
            AiRemoteAssistantState::Degraded | AiRemoteAssistantState::ManualRequired => {
                !matches!(degraded_state, AiDegradedState::None)
            }
            _ => false,
        };
        if !valid_state
            || !valid_degraded_state
            || (matches!(state, AiRemoteAssistantState::Succeeded) && answer.is_none())
            || (answer.is_some() && cited_evidence_reference_ids.is_empty())
            || !matches!(
                safety_boundary,
                AiRemoteAssistantSafetyBoundary::OutsideChildSafetyBlockingPath
            )
            || !redaction.is_safe()
            || matches!(
                retention,
                AiRetentionState::Deleted | AiRetentionState::Tombstoned
            )
            || !returned_at.is_well_formed()
        {
            return Err("AI remote result is not a safe, validated, outside-child-safety result");
        }
        Ok(Self {
            schema_version,
            result_id,
            request_id,
            family_id,
            state,
            validation,
            degraded_state,
            answer,
            cited_evidence_reference_ids,
            safety_boundary,
            redaction,
            retention,
            returned_at,
        })
    }

    pub fn result_id(&self) -> &AiRemoteAssistantResultId {
        &self.result_id
    }

    pub fn request_id(&self) -> &AiRemoteAssistantRequestId {
        &self.request_id
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn state(&self) -> AiRemoteAssistantState {
        self.state
    }

    pub fn answer(&self) -> Option<&AiSafeText> {
        self.answer.as_ref()
    }

    pub fn cited_evidence_reference_ids(&self) -> &[AiEvidenceReferenceId] {
        &self.cited_evidence_reference_ids
    }
}
