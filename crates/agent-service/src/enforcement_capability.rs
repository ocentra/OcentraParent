use ocentra_parent_agent_core::{
    enforcement_adapter::{
        app_block_control_capability, managed_browser_control_capability,
        network_control_capability, process_control_capability, timer_control_capability,
    },
    enforcement_app_time_limit::app_time_limit_capability,
};
use ocentra_parent_agent_protocol::activity::policy::PolicyAction;
use ocentra_parent_agent_protocol::activity::policy::PolicyTargetType;
use ocentra_parent_agent_protocol::enforcement::EnforcementCapabilityStatus;

pub(crate) struct EnforcementRequestedAtText<'a>(pub(crate) &'a str);

pub(crate) fn enforcement_capability_for_policy(
    action: PolicyAction,
    target_type: PolicyTargetType,
    requested_at: &EnforcementRequestedAtText<'_>,
) -> EnforcementCapabilityStatus {
    match (action, target_type) {
        (PolicyAction::AskParent, _) => timer_control_capability(requested_at.0),
        (PolicyAction::TimeLimit, _) => app_time_limit_capability(requested_at.0),
        (PolicyAction::Block, PolicyTargetType::Process) => {
            process_control_capability(requested_at.0)
        }
        (PolicyAction::Block, PolicyTargetType::App) => {
            app_block_control_capability(requested_at.0)
        }
        (PolicyAction::Block, PolicyTargetType::Domain | PolicyTargetType::Category) => {
            network_control_capability(requested_at.0)
        }
        (
            PolicyAction::Block,
            PolicyTargetType::Site | PolicyTargetType::Video | PolicyTargetType::Channel,
        ) => managed_browser_control_capability(requested_at.0),
        _ => process_control_capability(requested_at.0),
    }
}
