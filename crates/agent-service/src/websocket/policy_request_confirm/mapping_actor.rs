use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmActorRole;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmActorState;
use ocentra_policy_control_core::policy_source::ParentPolicyActorRole as CoreParentPolicyActorRole;
use ocentra_policy_control_core::policy_source::PolicySourceActorState as CorePolicySourceActorState;

use super::ActorRoleProtocolText;

const ACTOR_ROLE_PROTOCOL_PARENT: &str = "parent";
const ACTOR_ROLE_PROTOCOL_CO_PARENT: &str = "co-parent";
const ACTOR_ROLE_PROTOCOL_OBSERVER: &str = "observer";
const ACTOR_ROLE_PROTOCOL_CHILD: &str = "child";
const ACTOR_ROLE_PROTOCOL_SUPPORT: &str = "support";

pub(super) fn actor_role_protocol(
    role: PolicyRequestAssistantPreviewConfirmActorRole,
) -> ActorRoleProtocolText {
    if matches!(role, PolicyRequestAssistantPreviewConfirmActorRole::Parent) {
        ActorRoleProtocolText(ACTOR_ROLE_PROTOCOL_PARENT)
    } else if matches!(
        role,
        PolicyRequestAssistantPreviewConfirmActorRole::CoParent
    ) {
        ActorRoleProtocolText(ACTOR_ROLE_PROTOCOL_CO_PARENT)
    } else if matches!(
        role,
        PolicyRequestAssistantPreviewConfirmActorRole::Observer
    ) {
        ActorRoleProtocolText(ACTOR_ROLE_PROTOCOL_OBSERVER)
    } else if matches!(role, PolicyRequestAssistantPreviewConfirmActorRole::Child) {
        ActorRoleProtocolText(ACTOR_ROLE_PROTOCOL_CHILD)
    } else {
        ActorRoleProtocolText(ACTOR_ROLE_PROTOCOL_SUPPORT)
    }
}

pub(super) fn map_actor_role(
    role: PolicyRequestAssistantPreviewConfirmActorRole,
) -> CoreParentPolicyActorRole {
    if matches!(role, PolicyRequestAssistantPreviewConfirmActorRole::Parent) {
        CoreParentPolicyActorRole::Parent
    } else if matches!(
        role,
        PolicyRequestAssistantPreviewConfirmActorRole::CoParent
    ) {
        CoreParentPolicyActorRole::CoParent
    } else if matches!(
        role,
        PolicyRequestAssistantPreviewConfirmActorRole::Observer
    ) {
        CoreParentPolicyActorRole::Observer
    } else if matches!(role, PolicyRequestAssistantPreviewConfirmActorRole::Child) {
        CoreParentPolicyActorRole::Child
    } else {
        CoreParentPolicyActorRole::Support
    }
}

pub(super) fn map_actor_state(
    state: PolicyRequestAssistantPreviewConfirmActorState,
) -> CorePolicySourceActorState {
    if matches!(
        state,
        PolicyRequestAssistantPreviewConfirmActorState::Active
    ) {
        CorePolicySourceActorState::Active
    } else {
        CorePolicySourceActorState::Revoked
    }
}
