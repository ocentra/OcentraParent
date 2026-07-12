use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmAction;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmRequestKind;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmTargetKind;
use ocentra_policy_control_core::policy_request::PolicyRequestKind as CorePolicyRequestKind;
use ocentra_policy_control_core::policy_source::PolicyRuleAction as CorePolicyRuleAction;
use ocentra_policy_control_core::policy_source::PolicyTargetKind as CorePolicyTargetKind;

pub(super) fn map_request_kind(
    kind: PolicyRequestAssistantPreviewConfirmRequestKind,
) -> CorePolicyRequestKind {
    if matches!(
        kind,
        PolicyRequestAssistantPreviewConfirmRequestKind::AskParent
    ) {
        CorePolicyRequestKind::AskParent
    } else if matches!(
        kind,
        PolicyRequestAssistantPreviewConfirmRequestKind::BonusTime
    ) {
        CorePolicyRequestKind::BonusTime
    } else {
        CorePolicyRequestKind::TemporaryOverride
    }
}

pub(super) fn map_target_kind(
    kind: PolicyRequestAssistantPreviewConfirmTargetKind,
) -> CorePolicyTargetKind {
    if matches!(
        kind,
        PolicyRequestAssistantPreviewConfirmTargetKind::ChildProfile
    ) {
        CorePolicyTargetKind::ChildProfile
    } else if matches!(kind, PolicyRequestAssistantPreviewConfirmTargetKind::Device) {
        CorePolicyTargetKind::Device
    } else if matches!(kind, PolicyRequestAssistantPreviewConfirmTargetKind::App) {
        CorePolicyTargetKind::App
    } else if matches!(kind, PolicyRequestAssistantPreviewConfirmTargetKind::Site) {
        CorePolicyTargetKind::Site
    } else if matches!(
        kind,
        PolicyRequestAssistantPreviewConfirmTargetKind::Category
    ) {
        CorePolicyTargetKind::Category
    } else {
        CorePolicyTargetKind::Resource
    }
}

pub(super) fn map_requested_action(
    action: PolicyRequestAssistantPreviewConfirmAction,
) -> CorePolicyRuleAction {
    if matches!(action, PolicyRequestAssistantPreviewConfirmAction::Allow) {
        CorePolicyRuleAction::Allow
    } else if matches!(action, PolicyRequestAssistantPreviewConfirmAction::Warn) {
        CorePolicyRuleAction::Warn
    } else if matches!(
        action,
        PolicyRequestAssistantPreviewConfirmAction::AskParent
    ) {
        CorePolicyRuleAction::AskParent
    } else if matches!(
        action,
        PolicyRequestAssistantPreviewConfirmAction::TimeLimit
    ) {
        CorePolicyRuleAction::TimeLimit
    } else {
        CorePolicyRuleAction::Block
    }
}
