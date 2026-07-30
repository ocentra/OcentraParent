use ocentra_eventing::error::EventingError;
use serde::{Deserialize, Serialize};

use crate::policy_authority::{
    PolicyConflictDecision, PolicyConflictResolutionState, PolicyControlDecision,
    PolicyControlDecisionId, PolicyDecisionResolvedEvent, PolicyManualReviewState,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedPolicyDecision {
    pub decision_id: PolicyControlDecisionId,
    pub decision: PolicyControlDecision,
    pub conflict_decision: PolicyConflictDecision,
}

impl ResolvedPolicyDecision {
    pub fn for_delivery_grant(
        decision_id: impl Into<String>,
        decision: PolicyControlDecision,
        conflict_decision: PolicyConflictDecision,
    ) -> Result<Self, EventingError> {
        Ok(Self {
            decision_id: PolicyControlDecisionId::parse(decision_id)?,
            decision,
            conflict_decision,
        })
    }

    pub fn permits_execution(&self) -> bool {
        self.conflict_decision.resolution_state == PolicyConflictResolutionState::UseParentPolicy
            && self.conflict_decision.manual_review_state == PolicyManualReviewState::NotRequired
    }
}

impl PolicyDecisionResolvedEvent {
    pub fn resolved_decision(&self) -> ResolvedPolicyDecision {
        ResolvedPolicyDecision {
            decision_id: self.decision_id.clone(),
            decision: self.decision,
            conflict_decision: self.conflict_decision,
        }
    }
}
