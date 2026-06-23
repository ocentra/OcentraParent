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

const ACTOR_ROLE_PROTOCOL_PARENT: &str = "parent";
const ACTOR_ROLE_PROTOCOL_CO_PARENT: &str = "co-parent";
const ACTOR_ROLE_PROTOCOL_OBSERVER: &str = "observer";
const ACTOR_ROLE_PROTOCOL_CHILD: &str = "child";
const ACTOR_ROLE_PROTOCOL_SUPPORT: &str = "support";

pub(super) fn actor_role_protocol(
    role: PolicyRequestAssistantPreviewConfirmActorRole,
) -> &'static str {
    match role {
        PolicyRequestAssistantPreviewConfirmActorRole::Parent => ACTOR_ROLE_PROTOCOL_PARENT,
        PolicyRequestAssistantPreviewConfirmActorRole::CoParent => ACTOR_ROLE_PROTOCOL_CO_PARENT,
        PolicyRequestAssistantPreviewConfirmActorRole::Observer => ACTOR_ROLE_PROTOCOL_OBSERVER,
        PolicyRequestAssistantPreviewConfirmActorRole::Child => ACTOR_ROLE_PROTOCOL_CHILD,
        PolicyRequestAssistantPreviewConfirmActorRole::Support => ACTOR_ROLE_PROTOCOL_SUPPORT,
    }
}

pub(super) fn map_request_origin(origin: ProtocolPolicyRequestOrigin) -> CorePolicyRequestOrigin {
    match origin {
        ProtocolPolicyRequestOrigin::Child => CorePolicyRequestOrigin::Child,
        ProtocolPolicyRequestOrigin::AssistantDraft => CorePolicyRequestOrigin::AssistantDraft,
    }
}

pub(super) fn map_request_kind(
    kind: PolicyRequestAssistantPreviewConfirmRequestKind,
) -> CorePolicyRequestKind {
    match kind {
        PolicyRequestAssistantPreviewConfirmRequestKind::AskParent => {
            CorePolicyRequestKind::AskParent
        }
        PolicyRequestAssistantPreviewConfirmRequestKind::BonusTime => {
            CorePolicyRequestKind::BonusTime
        }
        PolicyRequestAssistantPreviewConfirmRequestKind::TemporaryOverride => {
            CorePolicyRequestKind::TemporaryOverride
        }
    }
}

pub(super) fn map_target_kind(
    kind: PolicyRequestAssistantPreviewConfirmTargetKind,
) -> CorePolicyTargetKind {
    match kind {
        PolicyRequestAssistantPreviewConfirmTargetKind::ChildProfile => {
            CorePolicyTargetKind::ChildProfile
        }
        PolicyRequestAssistantPreviewConfirmTargetKind::Device => CorePolicyTargetKind::Device,
        PolicyRequestAssistantPreviewConfirmTargetKind::App => CorePolicyTargetKind::App,
        PolicyRequestAssistantPreviewConfirmTargetKind::Site => CorePolicyTargetKind::Site,
        PolicyRequestAssistantPreviewConfirmTargetKind::Category => CorePolicyTargetKind::Category,
        PolicyRequestAssistantPreviewConfirmTargetKind::Resource => CorePolicyTargetKind::Resource,
    }
}

pub(super) fn map_requested_action(
    action: PolicyRequestAssistantPreviewConfirmAction,
) -> CorePolicyRuleAction {
    match action {
        PolicyRequestAssistantPreviewConfirmAction::Allow => CorePolicyRuleAction::Allow,
        PolicyRequestAssistantPreviewConfirmAction::Warn => CorePolicyRuleAction::Warn,
        PolicyRequestAssistantPreviewConfirmAction::AskParent => CorePolicyRuleAction::AskParent,
        PolicyRequestAssistantPreviewConfirmAction::TimeLimit => CorePolicyRuleAction::TimeLimit,
        PolicyRequestAssistantPreviewConfirmAction::Block => CorePolicyRuleAction::Block,
    }
}

pub(super) fn map_actor_role(
    role: PolicyRequestAssistantPreviewConfirmActorRole,
) -> CoreParentPolicyActorRole {
    match role {
        PolicyRequestAssistantPreviewConfirmActorRole::Parent => CoreParentPolicyActorRole::Parent,
        PolicyRequestAssistantPreviewConfirmActorRole::CoParent => {
            CoreParentPolicyActorRole::CoParent
        }
        PolicyRequestAssistantPreviewConfirmActorRole::Observer => {
            CoreParentPolicyActorRole::Observer
        }
        PolicyRequestAssistantPreviewConfirmActorRole::Child => CoreParentPolicyActorRole::Child,
        PolicyRequestAssistantPreviewConfirmActorRole::Support => {
            CoreParentPolicyActorRole::Support
        }
    }
}

pub(super) fn map_actor_state(
    state: PolicyRequestAssistantPreviewConfirmActorState,
) -> CorePolicySourceActorState {
    match state {
        PolicyRequestAssistantPreviewConfirmActorState::Active => {
            CorePolicySourceActorState::Active
        }
        PolicyRequestAssistantPreviewConfirmActorState::Revoked => {
            CorePolicySourceActorState::Revoked
        }
    }
}

pub(super) fn map_confirmation_state(
    state: ProtocolPolicyAssistantConfirmationState,
) -> CorePolicyAssistantConfirmationState {
    match state {
        ProtocolPolicyAssistantConfirmationState::NotRequired => {
            CorePolicyAssistantConfirmationState::NotRequired
        }
        ProtocolPolicyAssistantConfirmationState::ParentConfirmationRequired => {
            CorePolicyAssistantConfirmationState::ParentConfirmationRequired
        }
        ProtocolPolicyAssistantConfirmationState::ParentConfirmed => {
            CorePolicyAssistantConfirmationState::ParentConfirmed
        }
    }
}

pub(super) fn map_request_status(status: ProtocolPolicyRequestStatus) -> CorePolicyRequestStatus {
    match status {
        ProtocolPolicyRequestStatus::PreviewOnly => CorePolicyRequestStatus::PreviewOnly,
        ProtocolPolicyRequestStatus::PendingParentReview => {
            CorePolicyRequestStatus::PendingParentReview
        }
        ProtocolPolicyRequestStatus::Approved => CorePolicyRequestStatus::Approved,
        ProtocolPolicyRequestStatus::Denied => CorePolicyRequestStatus::Denied,
        ProtocolPolicyRequestStatus::Modified => CorePolicyRequestStatus::Modified,
        ProtocolPolicyRequestStatus::Expired => CorePolicyRequestStatus::Expired,
        ProtocolPolicyRequestStatus::ReplayRejected => CorePolicyRequestStatus::PreviewOnly,
    }
}

pub(super) fn map_protocol_request_status(
    status: CorePolicyRequestStatus,
) -> ProtocolPolicyRequestStatus {
    match status {
        CorePolicyRequestStatus::PreviewOnly => ProtocolPolicyRequestStatus::PreviewOnly,
        CorePolicyRequestStatus::PendingParentReview => {
            ProtocolPolicyRequestStatus::PendingParentReview
        }
        CorePolicyRequestStatus::Approved => ProtocolPolicyRequestStatus::Approved,
        CorePolicyRequestStatus::Denied => ProtocolPolicyRequestStatus::Denied,
        CorePolicyRequestStatus::Modified => ProtocolPolicyRequestStatus::Modified,
        CorePolicyRequestStatus::Expired => ProtocolPolicyRequestStatus::Expired,
        CorePolicyRequestStatus::ReplayRejected => ProtocolPolicyRequestStatus::ReplayRejected,
    }
}

pub(super) fn map_protocol_confirmation_state(
    state: CorePolicyAssistantConfirmationState,
) -> ProtocolPolicyAssistantConfirmationState {
    match state {
        CorePolicyAssistantConfirmationState::NotRequired => {
            ProtocolPolicyAssistantConfirmationState::NotRequired
        }
        CorePolicyAssistantConfirmationState::ParentConfirmationRequired => {
            ProtocolPolicyAssistantConfirmationState::ParentConfirmationRequired
        }
        CorePolicyAssistantConfirmationState::ParentConfirmed => {
            ProtocolPolicyAssistantConfirmationState::ParentConfirmed
        }
    }
}
