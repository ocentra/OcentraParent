#![forbid(unsafe_code)]

use crate::policy_source::{
    ParentPolicyActorRole, PolicySourceActorState, PolicySourceStatus, PolicySourceSurface,
};

mod actor;
mod status;
mod surface;

pub(super) fn policy_surface_name(surface: PolicySourceSurface) -> &'static str {
    surface::policy_surface_name(surface)
}

pub(super) fn policy_actor_role_name(role: ParentPolicyActorRole) -> &'static str {
    actor::policy_actor_role_name(role)
}

pub(super) fn policy_actor_state_name(state: PolicySourceActorState) -> &'static str {
    actor::policy_actor_state_name(state)
}

pub(super) fn policy_status_name(status: PolicySourceStatus) -> &'static str {
    status::policy_status_name(status)
}
