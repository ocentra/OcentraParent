use ocentra_parent_agent_protocol::activity::policy::PolicyTargetType;
use ocentra_parent_agent_protocol::policy_constants;

use super::EnforcementPayloadError;
use super::EnforcementText;

pub(super) fn policy_target_type(
    value: &EnforcementText,
) -> Result<PolicyTargetType, EnforcementPayloadError> {
    match value.0.as_str() {
        policy_constants::TARGET_TYPE_APP => Ok(PolicyTargetType::App),
        policy_constants::TARGET_TYPE_PROCESS => Ok(PolicyTargetType::Process),
        policy_constants::TARGET_TYPE_WINDOW => Ok(PolicyTargetType::Window),
        policy_constants::TARGET_TYPE_DOMAIN => Ok(PolicyTargetType::Domain),
        policy_constants::TARGET_TYPE_SITE => Ok(PolicyTargetType::Site),
        policy_constants::TARGET_TYPE_CATEGORY => Ok(PolicyTargetType::Category),
        policy_constants::TARGET_TYPE_VIDEO => Ok(PolicyTargetType::Video),
        policy_constants::TARGET_TYPE_CHANNEL => Ok(PolicyTargetType::Channel),
        policy_constants::TARGET_TYPE_ACTIVITY_TYPE => Ok(PolicyTargetType::ActivityType),
        policy_constants::TARGET_TYPE_DEVICE => Ok(PolicyTargetType::Device),
        _ => Err(EnforcementPayloadError::CommandPayloadInvalid),
    }
}
