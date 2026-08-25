use super::{AiOwnerResolvedRuntime, AiRuntimeReference};
use crate::ai_contracts::identity::{
    AiCapabilityId, AiModelId, AiProviderId, AiRuntimeReferenceId, AiTimestamp,
};

impl AiRuntimeReference {
    fn issue(
        runtime_reference_id: AiRuntimeReferenceId,
        provider_id: AiProviderId,
        model_id: AiModelId,
        capability_ids: Vec<AiCapabilityId>,
        observed_at: AiTimestamp,
    ) -> Result<Self, &'static str> {
        if capability_ids.is_empty() || !observed_at.is_well_formed() {
            return Err("AI runtime reference requires capabilities and a well-formed timestamp");
        }
        Ok(Self {
            runtime_reference_id,
            provider_id,
            model_id,
            capability_ids,
            observed_at,
        })
    }

    pub(crate) fn runtime_reference_id(&self) -> &AiRuntimeReferenceId {
        &self.runtime_reference_id
    }

    pub(crate) fn provider_id(&self) -> &AiProviderId {
        &self.provider_id
    }

    pub(crate) fn model_id(&self) -> &AiModelId {
        &self.model_id
    }

    pub(crate) fn capability_ids(&self) -> &[AiCapabilityId] {
        &self.capability_ids
    }

    pub(crate) fn observed_at(&self) -> &AiTimestamp {
        &self.observed_at
    }
}

impl AiOwnerResolvedRuntime {
    pub(crate) fn absent() -> Self {
        Self { runtime: None }
    }

    pub(crate) fn issue(
        runtime_reference_id: AiRuntimeReferenceId,
        provider_id: AiProviderId,
        model_id: AiModelId,
        capability_ids: Vec<AiCapabilityId>,
        observed_at: AiTimestamp,
    ) -> Result<Self, &'static str> {
        Ok(Self {
            runtime: Some(AiRuntimeReference::issue(
                runtime_reference_id,
                provider_id,
                model_id,
                capability_ids,
                observed_at,
            )?),
        })
    }

    pub(crate) fn into_runtime(self) -> Option<AiRuntimeReference> {
        self.runtime
    }
}
