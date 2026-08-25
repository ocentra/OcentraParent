use super::{AiOutputValidationState, AiResultPayload};
use crate::ai_contracts::identity::{
    AiDigest, AiExplanationId, AiFamilyId, AiRequestId, AiResultId, AiSchemaVersion, AiTimestamp,
    AiWorkItemId,
};
use crate::ai_contracts::{AiAuthorityBoundary, AiDegradedState, AiValidationState};

const AI_RESULT_DIGEST_DOMAIN: &[u8] = b"ocentra.ai.result.v1";

pub(super) fn digest_for(
    schema_version: &AiSchemaVersion,
    family_id: &AiFamilyId,
    result_id: &AiResultId,
    request_id: &AiRequestId,
    work_item_id: &AiWorkItemId,
    generated_at: &AiTimestamp,
    validation: AiValidationState,
    output_validation: AiOutputValidationState,
    degraded_state: AiDegradedState,
    payload: Option<&AiResultPayload>,
    explanation_id: Option<&AiExplanationId>,
    authority_boundary: AiAuthorityBoundary,
) -> Result<AiDigest, &'static str> {
    let binding = (
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
        authority_boundary,
    );
    let content = serde_json::to_vec(&binding)
        .map_err(|_| "AI result content cannot be canonically encoded")?;
    Ok(AiDigest::from_canonical_binding(
        AI_RESULT_DIGEST_DOMAIN,
        &[content.as_slice()],
    ))
}
