use super::AiPolicyHandoff;
use crate::ai_contracts::identity::{AiPolicyReferenceId, AiRequestId, AiResultId};
use crate::ai_contracts::AiAuthorityBoundary;

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
