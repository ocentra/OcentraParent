use super::{AiContextBuildState, AiEvidenceContext, AiEvidenceContextBuildResult};
use crate::ai_contracts::identity::{AiRequestId, AiSchemaVersion};
use crate::ai_contracts::{
    validate_contract_schema_version, AiAuthorityBoundary, AiCustodyState, AiDegradedState,
    AiValidationState,
};

impl AiEvidenceContext {
    pub(crate) fn new(
        schema_version: AiSchemaVersion,
        request_id: AiRequestId,
        family_id: super::AiFamilyId,
        child_profile_id: Option<super::AiChildProfileId>,
        device_id: Option<super::AiDeviceId>,
        evidence: Vec<super::AiEvidenceReference>,
        parent_rules: Vec<super::AiRuleReference>,
        memory: Vec<super::AiMemoryReference>,
        graph: Vec<super::AiGraphReference>,
        prompt: super::AiPromptReference,
        runtime: Option<super::AiRuntimeReference>,
        custody: Vec<AiCustodyState>,
        authority_boundary: AiAuthorityBoundary,
        degraded_state: AiDegradedState,
    ) -> Result<Self, &'static str> {
        validate_contract_schema_version(&schema_version)?;
        if evidence.iter().any(|item| item.family_id() != &family_id)
            || parent_rules
                .iter()
                .any(|rule| rule.family_id() != &family_id)
            || memory.iter().any(|item| item.family_id() != &family_id)
            || graph.iter().any(|item| item.family_id() != &family_id)
        {
            return Err("AI evidence context contains a family-mismatched reference");
        }
        if !matches!(authority_boundary, AiAuthorityBoundary::EvidenceOnly) {
            return Err("AI evidence context cannot mint policy authority");
        }
        if custody.is_empty() {
            return Err("AI evidence context must declare custody states");
        }
        Ok(Self {
            schema_version,
            request_id,
            family_id,
            child_profile_id,
            device_id,
            evidence,
            parent_rules,
            memory,
            graph,
            prompt,
            runtime,
            custody,
            authority_boundary,
            degraded_state,
        })
    }

    pub fn schema_version(&self) -> &AiSchemaVersion {
        &self.schema_version
    }

    pub fn request_id(&self) -> &AiRequestId {
        &self.request_id
    }

    pub fn family_id(&self) -> &super::AiFamilyId {
        &self.family_id
    }

    pub fn evidence(&self) -> &[super::AiEvidenceReference] {
        &self.evidence
    }

    pub fn memory(&self) -> &[super::AiMemoryReference] {
        &self.memory
    }

    pub fn graph(&self) -> &[super::AiGraphReference] {
        &self.graph
    }

    pub fn authority_boundary(&self) -> AiAuthorityBoundary {
        self.authority_boundary
    }
}

impl AiEvidenceContextBuildResult {
    pub(crate) fn new(
        request_id: AiRequestId,
        state: AiContextBuildState,
        validation: AiValidationState,
        context: Option<AiEvidenceContext>,
        rejected_references: Vec<super::AiEvidenceReferenceId>,
        missing_evidence: Vec<super::AiEvidenceKind>,
        degraded_state: AiDegradedState,
    ) -> Result<Self, &'static str> {
        if context
            .as_ref()
            .is_some_and(|context| context.request_id() != &request_id)
        {
            return Err("AI context build result request identity does not match its context");
        }
        match (state, validation, context.is_some()) {
            (AiContextBuildState::Ready, AiValidationState::Accepted, true)
            | (AiContextBuildState::Partial, AiValidationState::ManualRequired, true)
            | (AiContextBuildState::Rejected, AiValidationState::Rejected, false)
            | (AiContextBuildState::ManualRequired, AiValidationState::ManualRequired, _) => {}
            _ => return Err("AI context build state and validation are inconsistent"),
        }
        Ok(Self {
            request_id,
            state,
            validation,
            context,
            rejected_references,
            missing_evidence,
            degraded_state,
        })
    }

    pub fn request_id(&self) -> &AiRequestId {
        &self.request_id
    }

    pub fn state(&self) -> AiContextBuildState {
        self.state
    }

    pub fn validation(&self) -> AiValidationState {
        self.validation
    }

    pub fn context(&self) -> Option<&AiEvidenceContext> {
        self.context.as_ref()
    }
}
