use serde::{Deserialize, Serialize};

use super::{
    context::{AiEvidenceReference, AiPromptReference, AiRuleReference, AiRuntimeReference},
    identity::{
        AiEvidenceReferenceId, AiExplanationId, AiGraphReferenceId, AiMemoryReferenceId,
        AiRequestId, AiResultId, AiSchemaVersion, AiTimestamp,
    },
    memory::{AiGraphReference, AiMemoryReference},
    AiAuthorityBoundary, AiDegradedState, AiRedactionState, AiRetentionState, AiText,
    AiValidationState,
};

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
    pub evidence_reference_ids: Vec<AiEvidenceReferenceId>,
    pub memory_reference_ids: Vec<AiMemoryReferenceId>,
    pub graph_reference_ids: Vec<AiGraphReferenceId>,
    pub label: AiText,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiExplanationCitationFields {
    evidence_reference_ids: Vec<AiEvidenceReferenceId>,
    memory_reference_ids: Vec<AiMemoryReferenceId>,
    graph_reference_ids: Vec<AiGraphReferenceId>,
    label: AiText,
}

impl<'de> Deserialize<'de> for AiExplanationCitation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fields = AiExplanationCitationFields::deserialize(deserializer)?;
        if fields.evidence_reference_ids.is_empty() {
            return Err(serde::de::Error::custom(
                "AI explanation citations require at least one evidence reference",
            ));
        }
        Ok(Self {
            evidence_reference_ids: fields.evidence_reference_ids,
            memory_reference_ids: fields.memory_reference_ids,
            graph_reference_ids: fields.graph_reference_ids,
            label: fields.label,
        })
    }
}

impl AiExplanationCitation {
    pub fn is_grounded(&self) -> bool {
        !self.evidence_reference_ids.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiExplanationSection {
    pub heading: AiText,
    pub body: AiText,
    pub citations: Vec<AiExplanationCitation>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiExplanation {
    pub schema_version: AiSchemaVersion,
    pub explanation_id: AiExplanationId,
    pub request_id: AiRequestId,
    pub result_id: AiResultId,
    pub surface: AiExplanationSurface,
    pub state: AiExplanationState,
    pub validation: AiValidationState,
    pub degraded_state: AiDegradedState,
    pub sections: Vec<AiExplanationSection>,
    pub evidence: Vec<AiEvidenceReference>,
    pub memory: Vec<AiMemoryReference>,
    pub graph: Vec<AiGraphReference>,
    pub rules: Vec<AiRuleReference>,
    pub prompt: AiPromptReference,
    pub runtime: Option<AiRuntimeReference>,
    pub authority_boundary: AiAuthorityBoundary,
    pub redaction: AiRedactionState,
    pub retention: AiRetentionState,
    pub generated_at: AiTimestamp,
}
