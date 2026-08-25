use std::collections::HashSet;

use super::{AiExplanation, AiExplanationState};
use crate::ai_contracts::context::{
    AiEvidenceReference, AiPromptReference, AiRuleReference, AiRuntimeReference,
};
use crate::ai_contracts::identity::{
    AiExplanationId, AiFamilyId, AiRequestId, AiResultId, AiSchemaVersion, AiTimestamp,
};
use crate::ai_contracts::memory::{AiGraphReference, AiMemoryReference};
use crate::ai_contracts::{
    validate_contract_schema_version, AiAuthorityBoundary, AiDegradedState, AiRedactionState,
    AiRetentionState, AiValidationState,
};

fn citations_are_grounded(
    sections: &[super::AiExplanationSection],
    evidence: &[AiEvidenceReference],
    memory: &[AiMemoryReference],
    graph: &[AiGraphReference],
) -> bool {
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
    sections.iter().all(|section| {
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
    })
}

fn family_references_match(
    family_id: &AiFamilyId,
    evidence: &[AiEvidenceReference],
    memory: &[AiMemoryReference],
    graph: &[AiGraphReference],
    rules: &[AiRuleReference],
) -> bool {
    !evidence.iter().any(|item| item.family_id() != family_id)
        && !memory.iter().any(|item| item.family_id() != family_id)
        && !graph.iter().any(|item| item.family_id() != family_id)
        && !rules.iter().any(|rule| rule.family_id() != family_id)
}

fn state_matches_validation(state: AiExplanationState, validation: AiValidationState) -> bool {
    matches!(
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
    )
}

impl AiExplanation {
    pub(crate) fn new(
        schema_version: AiSchemaVersion,
        family_id: AiFamilyId,
        explanation_id: AiExplanationId,
        request_id: AiRequestId,
        result_id: AiResultId,
        surface: super::AiExplanationSurface,
        state: AiExplanationState,
        validation: AiValidationState,
        degraded_state: AiDegradedState,
        sections: Vec<super::AiExplanationSection>,
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
        validate_contract_schema_version(&schema_version)?;
        let valid_content = !sections.is_empty()
            && citations_are_grounded(&sections, &evidence, &memory, &graph)
            && redaction.is_safe()
            && !matches!(
                retention,
                AiRetentionState::Deleted | AiRetentionState::Tombstoned
            )
            && generated_at.is_well_formed()
            && family_references_match(&family_id, &evidence, &memory, &graph, &rules);
        if !valid_content || !state_matches_validation(state, validation) {
            return Err("AI explanation is unredacted, ungrounded, or family-mismatched");
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

    pub fn sections(&self) -> &[super::AiExplanationSection] {
        &self.sections
    }

    pub fn authority_boundary(&self) -> AiAuthorityBoundary {
        self.authority_boundary
    }

    pub fn redaction(&self) -> AiRedactionState {
        self.redaction
    }
}
