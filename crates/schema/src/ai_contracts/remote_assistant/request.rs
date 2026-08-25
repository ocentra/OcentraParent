use super::{AiRemoteAssistantRequest, AiRemoteAssistantState};
use crate::ai_contracts::context::AiPromptReference;
use crate::ai_contracts::identity::{AiRemoteAssistantRequestId, AiSchemaVersion, AiTimestamp};
use crate::ai_contracts::validate_contract_schema_version;

impl AiRemoteAssistantRequest {
    pub(crate) fn submit(
        schema_version: AiSchemaVersion,
        request_id: AiRemoteAssistantRequestId,
        source_bundle: super::AiRemoteAssistantSourceBundle,
        prompt: AiPromptReference,
        runtime: super::AiRemoteAssistantOwnerResolvedRuntime,
        requested_at: AiTimestamp,
    ) -> Result<Self, &'static str> {
        validate_contract_schema_version(&schema_version)?;
        let authorization = source_bundle.authorization();
        let within_authorization_window = requested_at
            .is_at_or_after(authorization.authorized_at())
            && requested_at.is_before(authorization.expires_at());
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
            runtime: runtime.into_runtime(),
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

    pub fn source_bundle(&self) -> &super::AiRemoteAssistantSourceBundle {
        &self.source_bundle
    }

    pub(super) fn requested_at(&self) -> &AiTimestamp {
        &self.requested_at
    }

    pub fn state(&self) -> AiRemoteAssistantState {
        self.state
    }
}
