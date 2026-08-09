use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmAction;
use ocentra_policy_control_core::policy_source::PolicyRuleAction;

pub(super) fn map(value: PolicyRequestAssistantPreviewConfirmAction) -> PolicyRuleAction {
    match value {
        PolicyRequestAssistantPreviewConfirmAction::Allow => PolicyRuleAction::Allow,
        PolicyRequestAssistantPreviewConfirmAction::Warn => PolicyRuleAction::Warn,
        PolicyRequestAssistantPreviewConfirmAction::AskParent => PolicyRuleAction::AskParent,
        PolicyRequestAssistantPreviewConfirmAction::TimeLimit => PolicyRuleAction::TimeLimit,
        PolicyRequestAssistantPreviewConfirmAction::Block => PolicyRuleAction::Block,
    }
}
