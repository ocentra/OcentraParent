use super::{AiPromptReference, AiRuleReference};
use crate::ai_contracts::identity::{
    AiEvidenceReferenceId, AiFamilyId, AiPolicyReferenceId, AiPromptTemplateId, AiPromptVersion,
    AiRuleId,
};
use crate::ai_contracts::AiSafeText;

impl AiRuleReference {
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
