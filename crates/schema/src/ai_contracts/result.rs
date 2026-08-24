use serde::{Deserialize, Serialize};

use super::{
    context::{AiEvidenceReference, AiPromptReference, AiRuleReference, AiRuntimeReference},
    identity::{
        AiEvidenceReferenceId, AiExplanationId, AiGraphReferenceId, AiMemoryReferenceId,
        AiRequestId, AiResultId, AiSchemaVersion, AiTimestamp, AiWorkItemId,
    },
    memory::{AiGraphReference, AiMemoryReference},
    AiAuthorityBoundary, AiConfidence, AiDegradedState, AiText, AiValidationState,
};

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
    pub claim_id: AiResultId,
    pub result_kind: AiResultKind,
    pub subject: super::identity::AiSubjectIdentity,
    pub label: AiText,
    pub confidence: AiConfidence,
    pub evidence_reference_ids: Vec<AiEvidenceReferenceId>,
    pub memory_reference_ids: Vec<AiMemoryReferenceId>,
    pub graph_reference_ids: Vec<AiGraphReferenceId>,
    pub rule_reference_ids: Vec<super::identity::AiRuleId>,
    pub authority_boundary: AiAuthorityBoundary,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiClaimFields {
    claim_id: AiResultId,
    result_kind: AiResultKind,
    subject: super::identity::AiSubjectIdentity,
    label: AiText,
    confidence: AiConfidence,
    evidence_reference_ids: Vec<AiEvidenceReferenceId>,
    memory_reference_ids: Vec<AiMemoryReferenceId>,
    graph_reference_ids: Vec<AiGraphReferenceId>,
    rule_reference_ids: Vec<super::identity::AiRuleId>,
    authority_boundary: AiAuthorityBoundary,
}

impl<'de> Deserialize<'de> for AiClaim {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fields = AiClaimFields::deserialize(deserializer)?;
        if !matches!(fields.result_kind, AiResultKind::NoClaim)
            && fields.evidence_reference_ids.is_empty()
        {
            return Err(serde::de::Error::custom(
                "AI claims require at least one evidence reference",
            ));
        }
        Ok(Self {
            claim_id: fields.claim_id,
            result_kind: fields.result_kind,
            subject: fields.subject,
            label: fields.label,
            confidence: fields.confidence,
            evidence_reference_ids: fields.evidence_reference_ids,
            memory_reference_ids: fields.memory_reference_ids,
            graph_reference_ids: fields.graph_reference_ids,
            rule_reference_ids: fields.rule_reference_ids,
            authority_boundary: fields.authority_boundary,
        })
    }
}

impl AiClaim {
    pub fn is_grounded(&self) -> bool {
        !self.evidence_reference_ids.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiResultPayload {
    pub claims: Vec<AiClaim>,
    pub summary: Option<AiText>,
    pub evidence: Vec<AiEvidenceReference>,
    pub memory: Vec<AiMemoryReference>,
    pub graph: Vec<AiGraphReference>,
    pub rules: Vec<AiRuleReference>,
    pub prompt: AiPromptReference,
    pub runtime: Option<AiRuntimeReference>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiResult {
    pub schema_version: AiSchemaVersion,
    pub result_id: AiResultId,
    pub request_id: AiRequestId,
    pub work_item_id: AiWorkItemId,
    pub generated_at: AiTimestamp,
    pub validation: AiValidationState,
    pub output_validation: AiOutputValidationState,
    pub degraded_state: AiDegradedState,
    pub payload: Option<AiResultPayload>,
    pub explanation_id: Option<AiExplanationId>,
    pub authority_boundary: AiAuthorityBoundary,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiPolicyHandoff {
    pub result_id: AiResultId,
    pub request_id: AiRequestId,
    pub policy_reference_ids: Vec<super::identity::AiPolicyReferenceId>,
    pub authority_boundary: AiAuthorityBoundary,
}
