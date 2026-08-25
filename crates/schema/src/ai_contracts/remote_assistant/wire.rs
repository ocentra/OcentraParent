use serde::Deserialize;

use super::{
    AiParentAuthorization, AiRemoteAssistantOwnerResolvedSource, AiRemoteAssistantRequest,
    AiRemoteAssistantWirePrompt, AiRemoteAssistantWireRequest,
};
use crate::ai_contracts::context::{AiPromptReference, AiRuntimeReference};
use crate::ai_contracts::identity::{
    AiAuthorizationReferenceId, AiFamilyId, AiPromptTemplateId, AiPromptVersion,
    AiRemoteAssistantRequestId, AiSchemaVersion, AiTimestamp,
};
use crate::ai_contracts::{validate_contract_schema_version, AiUntrustedText};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiRemoteAssistantWirePromptFields {
    template_id: AiPromptTemplateId,
    version: AiPromptVersion,
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
    pub fn template_id(&self) -> &AiPromptTemplateId {
        &self.template_id
    }

    pub fn version(&self) -> &AiPromptVersion {
        &self.version
    }
}

impl AiRemoteAssistantWireRequest {
    fn from_parts(
        schema_version: AiSchemaVersion,
        request_id: AiRemoteAssistantRequestId,
        family_id: AiFamilyId,
        authorization_reference_id: AiAuthorizationReferenceId,
        prompt: AiRemoteAssistantWirePrompt,
        runtime: Option<AiRuntimeReference>,
        requested_at: AiTimestamp,
        state: super::AiRemoteAssistantState,
    ) -> Result<Self, &'static str> {
        validate_contract_schema_version(&schema_version)?;
        if !matches!(state, super::AiRemoteAssistantState::Submitted)
            || !requested_at.is_well_formed()
        {
            return Err("AI remote wire request is not fail-closed safe");
        }
        Ok(Self {
            schema_version,
            request_id,
            family_id,
            authorization_reference_id,
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

    pub fn wire_requested_at(&self) -> &AiTimestamp {
        &self.requested_at
    }

    pub(crate) fn authorize(
        self,
        authorization: AiParentAuthorization,
        prompt: AiPromptReference,
        source: AiRemoteAssistantOwnerResolvedSource,
        trusted_now: AiTimestamp,
    ) -> Result<AiRemoteAssistantRequest, &'static str> {
        if authorization.authorization_reference_id() != &self.authorization_reference_id
            || authorization.family_id() != Some(&self.family_id)
            || source.request_id() != &self.request_id
            || source.authorization_reference_id() != &self.authorization_reference_id
            || source.family_id() != &self.family_id
            || prompt.template_id() != self.prompt.template_id()
            || prompt.version() != self.prompt.version()
        {
            return Err("AI remote wire authorization reference is not bound to the request");
        }
        let source_bundle =
            super::AiRemoteAssistantSourceBundle::from_owner_resolved(source, authorization)?;
        AiRemoteAssistantRequest::submit(
            self.schema_version,
            self.request_id,
            source_bundle,
            prompt,
            self.runtime,
            trusted_now,
        )
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiRemoteAssistantWireRequestFields {
    schema_version: AiSchemaVersion,
    request_id: AiRemoteAssistantRequestId,
    family_id: AiFamilyId,
    authorization_reference_id: AiAuthorizationReferenceId,
    prompt: AiRemoteAssistantWirePrompt,
    runtime: Option<AiRuntimeReference>,
    requested_at: AiTimestamp,
    state: super::AiRemoteAssistantState,
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
            fields.authorization_reference_id,
            fields.prompt,
            fields.runtime,
            fields.requested_at,
            fields.state,
        )
        .map_err(serde::de::Error::custom)
    }
}
