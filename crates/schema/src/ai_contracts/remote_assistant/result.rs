use std::collections::HashSet;

use super::{AiRemoteAssistantRequest, AiRemoteAssistantResult};
use super::{AiRemoteAssistantSafetyBoundary, AiRemoteAssistantState};
use crate::ai_contracts::identity::{
    AiEvidenceReferenceId, AiRemoteAssistantResultId, AiSchemaVersion, AiTimestamp,
};
use crate::ai_contracts::{
    validate_contract_schema_version, AiDegradedState, AiRedactionState, AiRetentionState,
    AiSafeText, AiValidationState,
};

impl AiRemoteAssistantResult {
    pub(crate) fn from_authorized_request(
        schema_version: AiSchemaVersion,
        result_id: AiRemoteAssistantResultId,
        request: &AiRemoteAssistantRequest,
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
        let request_id = request.request_id().clone();
        let family_id = request.source_bundle().family_id().clone();
        let authorized_evidence = request.source_bundle().evidence_reference_ids();
        let cited_set: HashSet<&AiEvidenceReferenceId> =
            cited_evidence_reference_ids.iter().collect();
        let citations_bound = cited_set.len() == cited_evidence_reference_ids.len()
            && cited_evidence_reference_ids
                .iter()
                .all(|reference| authorized_evidence.contains(reference));
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
            || !citations_bound
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
            || !returned_at.is_at_or_after(request.requested_at())
        {
            return Err("AI remote result is not a safe, validated, request-bound result");
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

    pub fn request_id(&self) -> &super::super::identity::AiRemoteAssistantRequestId {
        &self.request_id
    }

    pub fn family_id(&self) -> &super::super::identity::AiFamilyId {
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
