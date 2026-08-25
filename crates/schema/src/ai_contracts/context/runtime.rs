use serde::Deserialize;

use super::AiRuntimeReference;
use crate::ai_contracts::identity::{
    AiCapabilityId, AiModelId, AiProviderId, AiRuntimeReferenceId, AiTimestamp,
};

impl AiRuntimeReference {
    pub fn new(
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

    pub fn runtime_reference_id(&self) -> &AiRuntimeReferenceId {
        &self.runtime_reference_id
    }

    pub fn provider_id(&self) -> &AiProviderId {
        &self.provider_id
    }

    pub fn model_id(&self) -> &AiModelId {
        &self.model_id
    }

    pub fn capability_ids(&self) -> &[AiCapabilityId] {
        &self.capability_ids
    }

    pub fn observed_at(&self) -> &AiTimestamp {
        &self.observed_at
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiRuntimeReferenceFields {
    runtime_reference_id: AiRuntimeReferenceId,
    provider_id: AiProviderId,
    model_id: AiModelId,
    capability_ids: Vec<AiCapabilityId>,
    observed_at: AiTimestamp,
}

impl<'de> Deserialize<'de> for AiRuntimeReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fields = AiRuntimeReferenceFields::deserialize(deserializer)?;
        Self::new(
            fields.runtime_reference_id,
            fields.provider_id,
            fields.model_id,
            fields.capability_ids,
            fields.observed_at,
        )
        .map_err(serde::de::Error::custom)
    }
}
