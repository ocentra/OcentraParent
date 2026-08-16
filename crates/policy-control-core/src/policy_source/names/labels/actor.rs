#![forbid(unsafe_code)]

use crate::policy_source::{ParentPolicyActorRole, PolicySourceActorState};
use ocentra_parent_agent_protocol::constants::policy_control;

pub(super) fn policy_actor_role_name(role: ParentPolicyActorRole) -> &'static str {
    match role {
        ParentPolicyActorRole::Parent => policy_control::source::ROLE_PARENT,
        ParentPolicyActorRole::CoParent => policy_control::source::ROLE_CO_PARENT,
        ParentPolicyActorRole::Observer => policy_control::source::ROLE_OBSERVER,
        ParentPolicyActorRole::Child => policy_control::source::ROLE_CHILD,
        ParentPolicyActorRole::Support => policy_control::source::ROLE_SUPPORT,
    }
}

pub(super) fn policy_actor_state_name(state: PolicySourceActorState) -> &'static str {
    match state {
        PolicySourceActorState::Active => policy_control::source::ACTOR_STATE_ACTIVE,
        PolicySourceActorState::Revoked => policy_control::source::ACTOR_STATE_REVOKED,
    }
}
