use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{
    context::{AiEvidenceReference, AiPromptReference, AiRuleReference, AiRuntimeReference},
    identity::{
        AiEvidenceReferenceId, AiExplanationId, AiFamilyId, AiGraphReferenceId,
        AiMemoryReferenceId, AiPolicyReferenceId, AiRequestId, AiResultId, AiRuleId,
        AiSchemaVersion, AiSubjectIdentity, AiTimestamp, AiWorkItemId,
    },
    memory::{AiGraphReference, AiMemoryReference},
    validate_contract_schema_version, AiAuthorityBoundary, AiConfidence, AiDegradedState,
    AiSafeText, AiValidationState,
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

impl AiClaim {
    pub(crate) fn new(
        claim_id: AiResultId,
        result_kind: AiResultKind,
        subject: AiSubjectIdentity,
        label: AiSafeText,
        confidence: AiConfidence,
        evidence_reference_ids: Vec<AiEvidenceReferenceId>,
        memory_reference_ids: Vec<AiMemoryReferenceId>,
        graph_reference_ids: Vec<AiGraphReferenceId>,
        rule_reference_ids: Vec<AiRuleId>,
    ) -> Result<Self, &'static str> {
        if !matches!(result_kind, AiResultKind::NoClaim) && evidence_reference_ids.is_empty() {
            return Err("AI claims require at least one evidence reference");
        }
        Ok(Self {
            claim_id,
            result_kind,
            subject,
            label,
            confidence,
            evidence_reference_ids,
            memory_reference_ids,
            graph_reference_ids,
            rule_reference_ids,
            authority_boundary: AiAuthorityBoundary::EvidenceOnly,
        })
    }

    pub fn claim_id(&self) -> &AiResultId {
        &self.claim_id
    }

    pub fn subject(&self) -> &AiSubjectIdentity {
        &self.subject
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

    pub fn rule_reference_ids(&self) -> &[AiRuleId] {
        &self.rule_reference_ids
    }

    pub fn is_grounded(&self) -> bool {
        !self.evidence_reference_ids.is_empty()
    }

    pub fn authority_boundary(&self) -> AiAuthorityBoundary {
        self.authority_boundary
    }
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

impl AiResultPayload {
    pub(crate) fn new(
        family_id: AiFamilyId,
        claims: Vec<AiClaim>,
        summary: Option<AiSafeText>,
        evidence: Vec<AiEvidenceReference>,
        memory: Vec<AiMemoryReference>,
        graph: Vec<AiGraphReference>,
        rules: Vec<AiRuleReference>,
        prompt: AiPromptReference,
        runtime: Option<AiRuntimeReference>,
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
        let rule_ids = rules
            .iter()
            .map(|rule| rule.rule_id())
            .collect::<HashSet<_>>();
        let claims_grounded = claims.iter().all(|claim| {
            claim
                .evidence_reference_ids()
                .iter()
                .all(|id| evidence_ids.contains(id))
                && claim
                    .memory_reference_ids()
                    .iter()
                    .all(|id| memory_ids.contains(id))
                && claim
                    .graph_reference_ids()
                    .iter()
                    .all(|id| graph_ids.contains(id))
                && claim
                    .rule_reference_ids()
                    .iter()
                    .all(|id| rule_ids.contains(id))
        });
        if evidence.iter().any(|item| item.family_id() != &family_id)
            || memory.iter().any(|item| item.family_id() != &family_id)
            || graph.iter().any(|item| item.family_id() != &family_id)
            || rules.iter().any(|rule| rule.family_id() != &family_id)
            || claims
                .iter()
                .any(|claim| claim.subject().family_id() != &family_id)
            || !claims_grounded
        {
            return Err("AI result payload contains a family-mismatched identity");
        }
        Ok(Self {
            family_id,
            claims,
            summary,
            evidence,
            memory,
            graph,
            rules,
            prompt,
            runtime,
        })
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn claims(&self) -> &[AiClaim] {
        &self.claims
    }

    pub fn evidence(&self) -> &[AiEvidenceReference] {
        &self.evidence
    }

    pub fn memory(&self) -> &[AiMemoryReference] {
        &self.memory
    }

    pub fn graph(&self) -> &[AiGraphReference] {
        &self.graph
    }
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

impl AiResult {
    pub(crate) fn new(
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
    ) -> Result<Self, &'static str> {
        validate_contract_schema_version(&schema_version)?;
        if !generated_at.is_well_formed()
            || payload
                .as_ref()
                .is_some_and(|payload| payload.family_id() != &family_id)
            || matches!(validation, AiValidationState::Accepted)
                && (!matches!(output_validation, AiOutputValidationState::SchemaValid)
                    || payload.is_none())
        {
            return Err("AI result validation, family, or payload state is inconsistent");
        }
        Ok(Self {
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
            authority_boundary: AiAuthorityBoundary::EvidenceOnly,
        })
    }

    pub fn schema_version(&self) -> &AiSchemaVersion {
        &self.schema_version
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn result_id(&self) -> &AiResultId {
        &self.result_id
    }

    pub fn request_id(&self) -> &AiRequestId {
        &self.request_id
    }

    pub fn work_item_id(&self) -> &AiWorkItemId {
        &self.work_item_id
    }

    pub fn validation(&self) -> AiValidationState {
        self.validation
    }

    pub fn authority_boundary(&self) -> AiAuthorityBoundary {
        self.authority_boundary
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiPolicyHandoff {
    result_id: AiResultId,
    request_id: AiRequestId,
    policy_reference_ids: Vec<AiPolicyReferenceId>,
    authority_boundary: AiAuthorityBoundary,
}

impl AiPolicyHandoff {
    pub(crate) fn new(
        result_id: AiResultId,
        request_id: AiRequestId,
        policy_reference_ids: Vec<AiPolicyReferenceId>,
    ) -> Result<Self, &'static str> {
        if policy_reference_ids.is_empty() {
            return Err("AI policy handoff requires at least one policy reference");
        }
        Ok(Self {
            result_id,
            request_id,
            policy_reference_ids,
            authority_boundary: AiAuthorityBoundary::DeterministicPolicyRequired,
        })
    }

    pub fn result_id(&self) -> &AiResultId {
        &self.result_id
    }

    pub fn request_id(&self) -> &AiRequestId {
        &self.request_id
    }

    pub fn policy_reference_ids(&self) -> &[AiPolicyReferenceId] {
        &self.policy_reference_ids
    }

    pub fn authority_boundary(&self) -> AiAuthorityBoundary {
        self.authority_boundary
    }
}
