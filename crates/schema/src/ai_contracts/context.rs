use serde::{Deserialize, Serialize};

use super::{
    identity::{
        AiAdapterId, AiCapabilityId, AiChildProfileId, AiDeviceId, AiEvidenceReferenceId,
        AiFamilyId, AiPolicyReferenceId, AiPromptTemplateId, AiPromptVersion, AiRequestId,
        AiRuleId, AiRuntimeReferenceId, AiSchemaIdentity, AiSchemaVersion, AiSourceId, AiTimestamp,
    },
    memory::{AiGraphReference, AiMemoryReference},
    AiAuthorityBoundary, AiConfidence, AiCustodyState, AiDegradedState, AiRedactionState,
    AiRetentionState, AiText, AiValidationState,
};

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiEvidenceProvenance {
    pub provenance_kind: AiProvenanceKind,
    pub source_id: AiSourceId,
    pub adapter_id: AiAdapterId,
    pub source_schema_version: AiSchemaVersion,
    pub observed_at: AiTimestamp,
    pub ingested_at: Option<AiTimestamp>,
    pub source_evidence_reference_id: Option<AiEvidenceReferenceId>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiEvidenceReference {
    pub evidence_reference_id: AiEvidenceReferenceId,
    pub evidence_kind: AiEvidenceKind,
    pub provenance: AiEvidenceProvenance,
    pub custody: AiCustodyState,
    pub retention: AiRetentionState,
    pub redaction: AiRedactionState,
    pub confidence: Option<AiConfidence>,
    pub validation: AiReferenceValidationState,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiRuleReference {
    pub policy_reference_id: AiPolicyReferenceId,
    pub rule_id: AiRuleId,
    pub rule_version: AiSchemaVersion,
    pub source_evidence_reference_id: AiEvidenceReferenceId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiPromptReference {
    pub template_id: AiPromptTemplateId,
    pub version: AiPromptVersion,
    pub task: AiText,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiRuntimeReference {
    pub runtime_reference_id: AiRuntimeReferenceId,
    pub provider_id: AiProviderId,
    pub model_id: AiModelId,
    pub capability_ids: Vec<AiCapabilityId>,
    pub observed_at: AiTimestamp,
}

use super::identity::{AiModelId, AiProviderId};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiEvidenceContextRequest {
    pub identity: AiSchemaIdentity,
    pub requested_evaluation: AiText,
    pub requested_at: AiTimestamp,
    pub required_evidence: Vec<AiEvidenceKind>,
    pub allowed_custody: Vec<AiCustodyState>,
    pub parent_rules: Vec<AiRuleReference>,
    pub prompt: AiPromptReference,
    pub runtime: Option<AiRuntimeReference>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiEvidenceContext {
    pub schema_version: AiSchemaVersion,
    pub request_id: AiRequestId,
    pub family_id: AiFamilyId,
    pub child_profile_id: Option<AiChildProfileId>,
    pub device_id: Option<AiDeviceId>,
    pub evidence: Vec<AiEvidenceReference>,
    pub parent_rules: Vec<AiRuleReference>,
    pub memory: Vec<AiMemoryReference>,
    pub graph: Vec<AiGraphReference>,
    pub prompt: AiPromptReference,
    pub runtime: Option<AiRuntimeReference>,
    pub custody: Vec<AiCustodyState>,
    pub authority_boundary: AiAuthorityBoundary,
    pub degraded_state: AiDegradedState,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiEvidenceContextBuildResult {
    pub request_id: AiRequestId,
    pub state: AiContextBuildState,
    pub validation: AiValidationState,
    pub context: Option<AiEvidenceContext>,
    pub rejected_references: Vec<AiEvidenceReferenceId>,
    pub missing_evidence: Vec<AiEvidenceKind>,
    pub degraded_state: AiDegradedState,
}
