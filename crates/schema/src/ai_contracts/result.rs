use serde::{Deserialize, Serialize};

use super::context::{AiEvidenceReference, AiPromptReference, AiRuleReference, AiRuntimeReference};
use super::identity::{
    AiEvidenceReferenceId, AiExplanationId, AiFamilyId, AiGraphReferenceId, AiMemoryReferenceId,
    AiPolicyReferenceId, AiRequestId, AiResultId, AiRuleId, AiSchemaVersion, AiSubjectIdentity,
    AiTimestamp, AiWorkItemId,
};
use super::memory::{AiGraphReference, AiMemoryReference};
use super::{AiAuthorityBoundary, AiConfidence, AiDegradedState, AiSafeText, AiValidationState};

mod claim;
mod document;
mod handoff;
mod payload;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiResultKind {
    Observation,
    Classification,
    Summary,
    Explanation,
    NoClaim,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiOutputValidationState {
    SchemaValid,
    SchemaInvalid,
    EvidenceMissing,
    ConfidenceInvalid,
    PolicyHandoffRequired,
    ManualRequired,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiClaim {
    claim_id: AiResultId,
    result_kind: AiResultKind,
    subject: AiSubjectIdentity,
    label: AiSafeText,
    confidence: AiConfidence,
    evidence_reference_ids: Vec<AiEvidenceReferenceId>,
    memory_reference_ids: Vec<AiMemoryReferenceId>,
    graph_reference_ids: Vec<AiGraphReferenceId>,
    rule_reference_ids: Vec<AiRuleId>,
    authority_boundary: AiAuthorityBoundary,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiResultPayload {
    family_id: AiFamilyId,
    claims: Vec<AiClaim>,
    summary: Option<AiSafeText>,
    evidence: Vec<AiEvidenceReference>,
    memory: Vec<AiMemoryReference>,
    graph: Vec<AiGraphReference>,
    rules: Vec<AiRuleReference>,
    prompt: AiPromptReference,
    runtime: Option<AiRuntimeReference>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiResult {
    schema_version: AiSchemaVersion,
    family_id: AiFamilyId,
    result_id: AiResultId,
    request_id: AiRequestId,
    work_item_id: AiWorkItemId,
    generated_at: AiTimestamp,
    validation: AiValidationState,
    output_validation: AiOutputValidationState,
    degraded_state: AiDegradedState,
    payload: Option<AiResultPayload>,
    explanation_id: Option<AiExplanationId>,
    authority_boundary: AiAuthorityBoundary,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiPolicyHandoff {
    result_id: AiResultId,
    request_id: AiRequestId,
    policy_reference_ids: Vec<AiPolicyReferenceId>,
    authority_boundary: AiAuthorityBoundary,
}
