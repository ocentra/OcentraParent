use serde::{Deserialize, Serialize};

use super::{
    identity::{
        AiAdapterId, AiCapabilityId, AiChildProfileId, AiDeviceId, AiEvidenceReferenceId,
        AiFamilyId, AiModelId, AiPolicyReferenceId, AiPromptTemplateId, AiPromptVersion,
        AiProviderId, AiRequestId, AiResultId, AiRuleId, AiRuntimeReferenceId, AiSchemaIdentity,
        AiSchemaVersion, AiSourceId, AiTimestamp,
    },
    memory::{AiGraphReference, AiMemoryReference},
    AiAuthorityBoundary, AiConfidence, AiCustodyState, AiDegradedState, AiRedactionState,
    AiRetentionState, AiSafeText, AiValidationState,
};

mod evidence;
mod provenance;
mod request;
mod result;
mod rule_prompt;
mod runtime;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiEvidenceKind {
    Browser,
    App,
    Game,
    Network,
    ScreenSummary,
    Activity,
    ParentRule,
    Audit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiProvenanceKind {
    DirectObservation,
    DerivedFromEvidence,
    DerivedFromResult,
    ParentAuthoredRule,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiContextBuildState {
    Ready,
    Partial,
    Rejected,
    ManualRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiReferenceValidationState {
    Validated,
    MissingSource,
    CustodyBlocked,
    Stale,
    Rejected,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiEvidenceProvenance {
    provenance_kind: AiProvenanceKind,
    family_id: AiFamilyId,
    source_id: AiSourceId,
    adapter_id: AiAdapterId,
    source_schema_version: AiSchemaVersion,
    observed_at: AiTimestamp,
    ingested_at: Option<AiTimestamp>,
    source_evidence_reference_id: Option<AiEvidenceReferenceId>,
    source_result_id: Option<AiResultId>,
    source_rule_id: Option<AiRuleId>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiEvidenceReference {
    evidence_reference_id: AiEvidenceReferenceId,
    family_id: AiFamilyId,
    evidence_kind: AiEvidenceKind,
    provenance: AiEvidenceProvenance,
    custody: AiCustodyState,
    retention: AiRetentionState,
    redaction: AiRedactionState,
    confidence: Option<AiConfidence>,
    validation: AiReferenceValidationState,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiRuleReference {
    policy_reference_id: AiPolicyReferenceId,
    family_id: AiFamilyId,
    rule_id: AiRuleId,
    rule_version: AiSchemaVersion,
    source_evidence_reference_id: AiEvidenceReferenceId,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiPromptReference {
    template_id: AiPromptTemplateId,
    version: AiPromptVersion,
    task: AiSafeText,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiRuntimeReference {
    runtime_reference_id: AiRuntimeReferenceId,
    provider_id: AiProviderId,
    model_id: AiModelId,
    capability_ids: Vec<AiCapabilityId>,
    observed_at: AiTimestamp,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiEvidenceContextRequest {
    identity: AiSchemaIdentity,
    requested_evaluation: AiSafeText,
    requested_at: AiTimestamp,
    required_evidence: Vec<AiEvidenceKind>,
    allowed_custody: Vec<AiCustodyState>,
    parent_rules: Vec<AiRuleReference>,
    prompt: AiPromptReference,
    runtime: Option<AiRuntimeReference>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiEvidenceContext {
    schema_version: AiSchemaVersion,
    request_id: AiRequestId,
    family_id: AiFamilyId,
    child_profile_id: Option<AiChildProfileId>,
    device_id: Option<AiDeviceId>,
    evidence: Vec<AiEvidenceReference>,
    parent_rules: Vec<AiRuleReference>,
    memory: Vec<AiMemoryReference>,
    graph: Vec<AiGraphReference>,
    prompt: AiPromptReference,
    runtime: Option<AiRuntimeReference>,
    custody: Vec<AiCustodyState>,
    authority_boundary: AiAuthorityBoundary,
    degraded_state: AiDegradedState,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiEvidenceContextBuildResult {
    request_id: AiRequestId,
    state: AiContextBuildState,
    validation: AiValidationState,
    context: Option<AiEvidenceContext>,
    rejected_references: Vec<AiEvidenceReferenceId>,
    missing_evidence: Vec<AiEvidenceKind>,
    degraded_state: AiDegradedState,
}
