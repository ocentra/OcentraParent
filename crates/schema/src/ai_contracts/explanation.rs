use serde::{Deserialize, Serialize};

use super::{
    context::{AiEvidenceReference, AiPromptReference, AiRuleReference, AiRuntimeReference},
    identity::{
        AiEvidenceReferenceId, AiExplanationId, AiFamilyId, AiGraphReferenceId,
        AiMemoryReferenceId, AiRequestId, AiResultId, AiSchemaVersion, AiTimestamp,
    },
    memory::{AiGraphReference, AiMemoryReference},
    AiAuthorityBoundary, AiDegradedState, AiRedactionState, AiRetentionState, AiSafeText,
    AiValidationState,
};

mod citation;
mod document;
mod section;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiExplanationSurface {
    ParentReadModel,
    ChildSafetyInternal,
    AuditRecord,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiExplanationState {
    Ready,
    Degraded,
    Unavailable,
    ManualRequired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiExplanationCitation {
    evidence_reference_ids: Vec<AiEvidenceReferenceId>,
    memory_reference_ids: Vec<AiMemoryReferenceId>,
    graph_reference_ids: Vec<AiGraphReferenceId>,
    label: AiSafeText,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiExplanationSection {
    heading: AiSafeText,
    body: AiSafeText,
    citations: Vec<AiExplanationCitation>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiExplanation {
    schema_version: AiSchemaVersion,
    family_id: AiFamilyId,
    explanation_id: AiExplanationId,
    request_id: AiRequestId,
    result_id: AiResultId,
    surface: AiExplanationSurface,
    state: AiExplanationState,
    validation: AiValidationState,
    degraded_state: AiDegradedState,
    sections: Vec<AiExplanationSection>,
    evidence: Vec<AiEvidenceReference>,
    memory: Vec<AiMemoryReference>,
    graph: Vec<AiGraphReference>,
    rules: Vec<AiRuleReference>,
    prompt: AiPromptReference,
    runtime: Option<AiRuntimeReference>,
    authority_boundary: AiAuthorityBoundary,
    redaction: AiRedactionState,
    retention: AiRetentionState,
    generated_at: AiTimestamp,
}
