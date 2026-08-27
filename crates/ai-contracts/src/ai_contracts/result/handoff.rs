use super::AiPolicyHandoff;
use crate::ai_contracts::identity::{AiPolicyReferenceId, AiRequestId, AiResultId};
use crate::ai_contracts::AiAuthorityBoundary;

impl AiPolicyHandoff {
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
