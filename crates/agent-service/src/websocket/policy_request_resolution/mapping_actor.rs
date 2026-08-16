use ocentra_parent_agent_protocol::transport::{
    PolicyRequestAssistantPreviewConfirmActorRole, PolicyRequestAssistantPreviewConfirmActorState,
};
use ocentra_policy_control_core::policy_source::{ParentPolicyActorRole, PolicySourceActorState};

pub(super) fn role(value: PolicyRequestAssistantPreviewConfirmActorRole) -> ParentPolicyActorRole {
    match value {
        PolicyRequestAssistantPreviewConfirmActorRole::Parent => ParentPolicyActorRole::Parent,
        PolicyRequestAssistantPreviewConfirmActorRole::CoParent => ParentPolicyActorRole::CoParent,
        PolicyRequestAssistantPreviewConfirmActorRole::Observer => ParentPolicyActorRole::Observer,
        PolicyRequestAssistantPreviewConfirmActorRole::Child => ParentPolicyActorRole::Child,
        PolicyRequestAssistantPreviewConfirmActorRole::Support => ParentPolicyActorRole::Support,
    }
}

pub(super) fn state(
    value: PolicyRequestAssistantPreviewConfirmActorState,
) -> PolicySourceActorState {
    match value {
        PolicyRequestAssistantPreviewConfirmActorState::Active => PolicySourceActorState::Active,
        PolicyRequestAssistantPreviewConfirmActorState::Revoked => PolicySourceActorState::Revoked,
    }
}
