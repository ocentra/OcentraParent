use ocentra_parent_agent_protocol::activity::policy_preview::PolicyAssistantConfirmationState as CorePolicyAssistantConfirmationState;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyAssistantConfirmationState as ProtocolPolicyAssistantConfirmationState;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestOrigin as CorePolicyRequestOrigin;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestOrigin as ProtocolPolicyRequestOrigin;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus as CorePolicyRequestStatus;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus as ProtocolPolicyRequestStatus;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmAction;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmActorRole;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmActorState;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmRequestKind;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmTargetKind;
use ocentra_policy_control_core::policy_request::PolicyRequestKind as CorePolicyRequestKind;
use ocentra_policy_control_core::policy_source::ParentPolicyActorRole as CoreParentPolicyActorRole;
use ocentra_policy_control_core::policy_source::PolicyRuleAction as CorePolicyRuleAction;
use ocentra_policy_control_core::policy_source::PolicySourceActorState as CorePolicySourceActorState;
use ocentra_policy_control_core::policy_source::PolicyTargetKind as CorePolicyTargetKind;

#[path = "policy_request_confirm/mapping_actor.rs"]
mod mapping_actor;
#[path = "policy_request_confirm/mapping_request.rs"]
mod mapping_request;
#[path = "policy_request_confirm/mapping_status.rs"]
mod mapping_status;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ActorRoleProtocolText(pub(super) &'static str);

pub(super) fn actor_role_protocol(
    role: PolicyRequestAssistantPreviewConfirmActorRole,
) -> ActorRoleProtocolText {
    mapping_actor::actor_role_protocol(role)
}

pub(super) fn map_request_origin(origin: ProtocolPolicyRequestOrigin) -> CorePolicyRequestOrigin {
    mapping_status::map_request_origin(origin)
}

pub(super) fn map_request_kind(
    kind: PolicyRequestAssistantPreviewConfirmRequestKind,
) -> CorePolicyRequestKind {
    mapping_request::map_request_kind(kind)
}

pub(super) fn map_target_kind(
    kind: PolicyRequestAssistantPreviewConfirmTargetKind,
) -> CorePolicyTargetKind {
    mapping_request::map_target_kind(kind)
}

pub(super) fn map_requested_action(
    action: PolicyRequestAssistantPreviewConfirmAction,
) -> CorePolicyRuleAction {
    mapping_request::map_requested_action(action)
}

pub(super) fn map_actor_role(
    role: PolicyRequestAssistantPreviewConfirmActorRole,
) -> CoreParentPolicyActorRole {
    mapping_actor::map_actor_role(role)
}

pub(super) fn map_actor_state(
    state: PolicyRequestAssistantPreviewConfirmActorState,
) -> CorePolicySourceActorState {
    mapping_actor::map_actor_state(state)
}

pub(super) fn map_confirmation_state(
    state: ProtocolPolicyAssistantConfirmationState,
) -> CorePolicyAssistantConfirmationState {
    mapping_status::map_confirmation_state(state)
}

pub(super) fn map_request_status(status: ProtocolPolicyRequestStatus) -> CorePolicyRequestStatus {
    mapping_status::map_request_status(status)
}

pub(super) fn map_protocol_request_status(
    status: CorePolicyRequestStatus,
) -> ProtocolPolicyRequestStatus {
    mapping_status::map_protocol_request_status(status)
}

pub(super) fn map_protocol_confirmation_state(
    state: CorePolicyAssistantConfirmationState,
) -> ProtocolPolicyAssistantConfirmationState {
    mapping_status::map_protocol_confirmation_state(state)
}
