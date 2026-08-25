use super::{AiPromptReference, AiRuleReference};
use crate::ai_contracts::identity::{
    AiEvidenceReferenceId, AiFamilyId, AiPolicyReferenceId, AiPromptTemplateId, AiPromptVersion,
    AiRuleId, AiSchemaVersion,
};
use crate::ai_contracts::{validate_contract_schema_version, AiSafeText};

impl AiRuleReference {
    pub(crate) fn new(
        policy_reference_id: AiPolicyReferenceId,
        family_id: AiFamilyId,
        rule_id: AiRuleId,
        rule_version: AiSchemaVersion,
        source_evidence_reference_id: AiEvidenceReferenceId,
    ) -> Result<Self, &'static str> {
        validate_contract_schema_version(&rule_version)?;
        Ok(Self {
            policy_reference_id,
            family_id,
            rule_id,
            rule_version,
            source_evidence_reference_id,
        })
    }

    pub fn policy_reference_id(&self) -> &AiPolicyReferenceId {
        &self.policy_reference_id
    }

    pub fn rule_id(&self) -> &AiRuleId {
        &self.rule_id
    }

    pub fn family_id(&self) -> &AiFamilyId {
        &self.family_id
    }

    pub fn source_evidence_reference_id(&self) -> &AiEvidenceReferenceId {
        &self.source_evidence_reference_id
    }
}

impl AiPromptReference {
    pub(crate) fn new(
        template_id: AiPromptTemplateId,
        version: AiPromptVersion,
        task: AiSafeText,
    ) -> Result<Self, &'static str> {
        Ok(Self {
            template_id,
            version,
            task,
        })
    }

    pub fn template_id(&self) -> &AiPromptTemplateId {
        &self.template_id
    }

    pub fn version(&self) -> &AiPromptVersion {
        &self.version
    }

    pub fn task(&self) -> &AiSafeText {
        &self.task
    }
}
