use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{
    context::{AiEvidenceReference, AiPromptReference, AiRuleReference, AiRuntimeReference},
    identity::{
        AiEvidenceReferenceId, AiExplanationId, AiFamilyId, AiGraphReferenceId,
        AiMemoryReferenceId, AiRequestId, AiResultId, AiSchemaVersion, AiTimestamp,
    },
    memory::{AiGraphReference, AiMemoryReference},
    validate_contract_schema_version, AiAuthorityBoundary, AiDegradedState, AiRedactionState,
    AiRetentionState, AiSafeText, AiValidationState,
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
    evidence_reference_ids: Vec<AiEvidenceReferenceId>,
    memory_reference_ids: Vec<AiMemoryReferenceId>,
    graph_reference_ids: Vec<AiGraphReferenceId>,
    label: AiSafeText,
}

impl AiExplanationCitation {
    pub(crate) fn new(
        evidence_reference_ids: Vec<AiEvidenceReferenceId>,
        memory_reference_ids: Vec<AiMemoryReferenceId>,
        graph_reference_ids: Vec<AiGraphReferenceId>,
        label: AiSafeText,
    ) -> Result<Self, &'static str> {
        if evidence_reference_ids.is_empty() {
            return Err("AI explanation citations require evidence identity");
        }
        Ok(Self {
            evidence_reference_ids,
            memory_reference_ids,
            graph_reference_ids,
            label,
        })
    }

    pub fn evidence_reference_ids(&self) -> &[AiEvidenceReferenceId] {
        &self.evidence_reference_ids
    }

    pub fn memory_reference_ids(&self) -> &[AiMemoryReferenceId] {
        &self.memory_reference_ids
    }

    pub fn graph_reference_ids(&self) -> &[AiGraphReferenceId] {
        &self.graph_reference_ids
    }

    pub fn label(&self) -> &AiSafeText {
        &self.label
    }

    pub fn is_grounded(&self) -> bool {
        !self.evidence_reference_ids.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiExplanationSection {
    heading: AiSafeText,
    body: AiSafeText,
    citations: Vec<AiExplanationCitation>,
}

impl AiExplanationSection {
    pub(crate) fn new(
        heading: AiSafeText,
        body: AiSafeText,
        citations: Vec<AiExplanationCitation>,
    ) -> Result<Self, &'static str> {
        if citations.is_empty() {
            return Err("AI explanation section requires grounded citations");
        }
        Ok(Self {
            heading,
            body,
            citations,
        })
    }

    pub fn heading(&self) -> &AiSafeText {
        &self.heading
    }

    pub fn body(&self) -> &AiSafeText {
        &self.body
    }

    pub fn citations(&self) -> &[AiExplanationCitation] {
        &self.citations
    }
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

impl AiExplanation {
    pub(crate) fn new(
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
        redaction: AiRedactionState,
        retention: AiRetentionState,
        generated_at: AiTimestamp,
    ) -> Result<Self, &'static str> {
        let evidence_ids = evidence
            .iter()
            .map(|item| item.evidence_reference_id())
            .collect::<HashSet<_>>();
        let memory_ids = memory
            .iter()
            .map(|item| item.memory_reference_id())
            .collect::<HashSet<_>>();
        let graph_ids = graph
            .iter()
            .map(|item| item.graph_reference_id())
            .collect::<HashSet<_>>();
        let citations_grounded = sections.iter().all(|section| {
            section.citations().iter().all(|citation| {
                citation
                    .evidence_reference_ids()
                    .iter()
                    .all(|id| evidence_ids.contains(id))
                    && citation
                        .memory_reference_ids()
                        .iter()
                        .all(|id| memory_ids.contains(id))
                    && citation
                        .graph_reference_ids()
                        .iter()
                        .all(|id| graph_ids.contains(id))
            })
        });
        validate_contract_schema_version(&schema_version)?;
        if sections.is_empty()
            || !citations_grounded
            || !redaction.is_safe()
            || matches!(
                retention,
                AiRetentionState::Deleted | AiRetentionState::Tombstoned
            )
            || !generated_at.is_well_formed()
            || evidence.iter().any(|item| item.family_id() != &family_id)
            || memory.iter().any(|item| item.family_id() != &family_id)
            || graph.iter().any(|item| item.family_id() != &family_id)
            || rules.iter().any(|rule| rule.family_id() != &family_id)
        {
            return Err("AI explanation is unredacted, ungrounded, or family-mismatched");
        }
        let state_matches_validation = matches!(
            (state, validation),
            (AiExplanationState::Ready, AiValidationState::Accepted)
                | (
                    AiExplanationState::Degraded,
                    AiValidationState::ManualRequired
                )
                | (AiExplanationState::Unavailable, AiValidationState::Rejected)
                | (
                    AiExplanationState::ManualRequired,
                    AiValidationState::ManualRequired
                )
        );
        if !state_matches_validation {
            return Err("AI explanation state and validation are inconsistent");
        }
        Ok(Self {
            schema_version,
            family_id,
            explanation_id,
            request_id,
            result_id,
            surface,
            state,
            validation,
            degraded_state,
            sections,
            evidence,
            memory,
            graph,
            rules,
            prompt,
            runtime,
            authority_boundary: AiAuthorityBoundary::EvidenceOnly,
            redaction,
            retention,
            generated_at,
        })
    }

    pub fn schema_version(&self) -> &AiSchemaVersion {
        &self.schema_version
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn explanation_id(&self) -> &AiExplanationId {
        &self.explanation_id
    }

    pub fn request_id(&self) -> &AiRequestId {
        &self.request_id
    }

    pub fn result_id(&self) -> &AiResultId {
        &self.result_id
    }

    pub fn sections(&self) -> &[AiExplanationSection] {
        &self.sections
    }

    pub fn authority_boundary(&self) -> AiAuthorityBoundary {
        self.authority_boundary
    }

    pub fn redaction(&self) -> AiRedactionState {
        self.redaction
    }
}
