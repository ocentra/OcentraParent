use ocentra_eventing::error::EventingError;
use serde::{Deserialize, Serialize};

use crate::policy_authority::{
    PolicyConflictDecision, PolicyConflictResolutionState, PolicyControlAggregateId,
    PolicyControlDecision, PolicyControlDecisionId, PolicyDecisionResolvedEvent,
    PolicyManualReviewState,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedPolicyDecision {
    pub aggregate_id: PolicyControlAggregateId,
    pub decision_id: PolicyControlDecisionId,
    pub decision: PolicyControlDecision,
    pub conflict_decision: PolicyConflictDecision,
}

impl ResolvedPolicyDecision {
    pub fn for_delivery_grant(
        aggregate_id: impl Into<String>,
        decision_id: impl Into<String>,
        decision: PolicyControlDecision,
        conflict_decision: PolicyConflictDecision,
    ) -> Result<Self, EventingError> {
        Ok(Self {
            aggregate_id: PolicyControlAggregateId::parse(aggregate_id)?,
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
            aggregate_id: self.aggregate_id.clone(),
            decision_id: self.decision_id.clone(),
            decision: self.decision,
            conflict_decision: self.conflict_decision,
        }
    }
}
