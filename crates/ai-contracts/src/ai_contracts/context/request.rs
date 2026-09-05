use super::AiEvidenceContextRequest;
use crate::ai_contracts::identity::AiTimestamp;
use crate::ai_contracts::{AiCustodyState, AiSafeText};

impl AiEvidenceContextRequest {
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
}
