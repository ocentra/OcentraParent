#[path = "mapping_action.rs"]
mod action;
#[path = "mapping_actor.rs"]
mod actor;
#[path = "mapping_decision.rs"]
mod decision;

use ocentra_parent_agent_protocol::transport::{
    PolicyRequestAssistantPreviewConfirmAction, PolicyRequestAssistantPreviewConfirmActorRole,
    PolicyRequestAssistantPreviewConfirmActorState, PolicyRequestParentResolutionDecision,
};
use ocentra_policy_control_core::policy_request::PolicyApprovalDecision;
use ocentra_policy_control_core::policy_source::{
    ParentPolicyActorRole, PolicyRuleAction, PolicySourceActorState,
};

pub(crate) fn decision(value: PolicyRequestParentResolutionDecision) -> PolicyApprovalDecision {
    decision::map(value)
}

pub(crate) fn action(value: PolicyRequestAssistantPreviewConfirmAction) -> PolicyRuleAction {
    action::map(value)
}

pub(crate) fn actor_role(
    value: PolicyRequestAssistantPreviewConfirmActorRole,
) -> ParentPolicyActorRole {
    actor::role(value)
}

pub(crate) fn actor_state(
    value: PolicyRequestAssistantPreviewConfirmActorState,
) -> PolicySourceActorState {
    actor::state(value)
}
