use super::{AiOutputValidationState, AiResult};
use crate::ai_contracts::identity::{
    AiExplanationId, AiFamilyId, AiRequestId, AiResultId, AiSchemaVersion, AiTimestamp,
    AiWorkItemId,
};
use crate::ai_contracts::{
    validate_contract_schema_version, AiAuthorityBoundary, AiDegradedState, AiValidationState,
};

impl AiResult {
    pub(crate) fn new(
        schema_version: AiSchemaVersion,
        family_id: AiFamilyId,
        result_id: AiResultId,
        request_id: AiRequestId,
        work_item_id: AiWorkItemId,
        generated_at: AiTimestamp,
        validation: AiValidationState,
        output_validation: AiOutputValidationState,
        degraded_state: AiDegradedState,
        payload: Option<super::AiResultPayload>,
        explanation_id: Option<AiExplanationId>,
    ) -> Result<Self, &'static str> {
        validate_contract_schema_version(&schema_version)?;
        if !generated_at.is_well_formed()
            || payload
                .as_ref()
                .is_some_and(|payload| payload.family_id() != &family_id)
            || matches!(validation, AiValidationState::Accepted)
                && (!matches!(output_validation, AiOutputValidationState::SchemaValid)
                    || payload.is_none())
        {
            return Err("AI result validation, family, or payload state is inconsistent");
        }
        Ok(Self {
            schema_version,
            family_id,
            result_id,
            request_id,
            work_item_id,
            generated_at,
            validation,
            output_validation,
            degraded_state,
            payload,
            explanation_id,
            authority_boundary: AiAuthorityBoundary::EvidenceOnly,
        })
    }

    pub fn schema_version(&self) -> &AiSchemaVersion {
        &self.schema_version
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn result_id(&self) -> &AiResultId {
        &self.result_id
    }

    pub fn request_id(&self) -> &AiRequestId {
        &self.request_id
    }

    pub fn work_item_id(&self) -> &AiWorkItemId {
        &self.work_item_id
    }

    pub fn validation(&self) -> AiValidationState {
        self.validation
    }

    pub fn authority_boundary(&self) -> AiAuthorityBoundary {
        self.authority_boundary
    }
}
