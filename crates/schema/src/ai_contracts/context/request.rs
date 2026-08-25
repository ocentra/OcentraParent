use super::AiEvidenceContextRequest;
use crate::ai_contracts::identity::AiTimestamp;
use crate::ai_contracts::{AiCustodyState, AiSafeText};

impl AiEvidenceContextRequest {
    pub(crate) fn new(
        identity: super::AiSchemaIdentity,
        requested_evaluation: AiSafeText,
        requested_at: AiTimestamp,
        required_evidence: Vec<super::AiEvidenceKind>,
        allowed_custody: Vec<AiCustodyState>,
        parent_rules: Vec<super::AiRuleReference>,
        prompt: super::AiPromptReference,
        runtime: super::AiOwnerResolvedRuntime,
    ) -> Result<Self, &'static str> {
        if required_evidence.is_empty()
            || allowed_custody.is_empty()
            || !requested_at.is_well_formed()
            || parent_rules
                .iter()
                .any(|rule| rule.family_id() != identity.family())
        {
            return Err(
                "AI evidence context request has mismatched family or missing required data",
            );
        }
        Ok(Self {
            identity,
            requested_evaluation,
            requested_at,
            required_evidence,
            allowed_custody,
            parent_rules,
            prompt,
            runtime: runtime.into_runtime(),
        })
    }

    pub fn identity(&self) -> &super::AiSchemaIdentity {
        &self.identity
    }

    pub fn requested_evaluation(&self) -> &AiSafeText {
        &self.requested_evaluation
    }

    pub fn requested_at(&self) -> &AiTimestamp {
        &self.requested_at
    }

    pub fn required_evidence(&self) -> &[super::AiEvidenceKind] {
        &self.required_evidence
    }

    pub fn allowed_custody(&self) -> &[AiCustodyState] {
        &self.allowed_custody
    }

    pub fn parent_rules(&self) -> &[super::AiRuleReference] {
        &self.parent_rules
    }

    pub fn prompt(&self) -> &super::AiPromptReference {
        &self.prompt
    }

    pub(crate) fn runtime(&self) -> Option<&super::AiRuntimeReference> {
        self.runtime.as_ref()
    }
}
