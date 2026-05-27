use ocentra_parent_agent_core::{
    app_block_control_capability, app_time_limit_capability, managed_browser_control_capability,
    network_control_capability, process_control_capability, timer_control_capability,
};
use ocentra_parent_agent_protocol::{EnforcementCapabilityStatus, PolicyAction, PolicyTargetType};

pub(crate) fn enforcement_capability_for_policy(
    action: PolicyAction,
    target_type: PolicyTargetType,
    requested_at: &str,
) -> EnforcementCapabilityStatus {
    match (action, target_type) {
        (PolicyAction::AskParent, _) => timer_control_capability(requested_at),
        (PolicyAction::TimeLimit, _) => app_time_limit_capability(requested_at),
        (PolicyAction::Block, PolicyTargetType::Process) => {
            process_control_capability(requested_at)
        }
        (PolicyAction::Block, PolicyTargetType::App) => app_block_control_capability(requested_at),
        (PolicyAction::Block, PolicyTargetType::Domain | PolicyTargetType::Category) => {
            network_control_capability(requested_at)
        }
        (
            PolicyAction::Block,
            PolicyTargetType::Site | PolicyTargetType::Video | PolicyTargetType::Channel,
        ) => managed_browser_control_capability(requested_at),
        _ => process_control_capability(requested_at),
    }
}
