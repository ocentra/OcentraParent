use ocentra_parent_agent_protocol::activity::policy::PolicyAction;
use ocentra_parent_agent_protocol::policy_constants;

use super::EnforcementPayloadError;
use super::EnforcementText;

pub(super) fn policy_action(
    value: &EnforcementText,
) -> Result<PolicyAction, EnforcementPayloadError> {
    match value.0.as_str() {
        policy_constants::ACTION_ALLOW => Ok(PolicyAction::Allow),
        policy_constants::ACTION_WARN => Ok(PolicyAction::Warn),
        policy_constants::ACTION_BLOCK => Ok(PolicyAction::Block),
        policy_constants::ACTION_TIME_LIMIT => Ok(PolicyAction::TimeLimit),
        policy_constants::ACTION_ASK_PARENT => Ok(PolicyAction::AskParent),
        policy_constants::ACTION_UNKNOWN => Ok(PolicyAction::Unknown),
        _ => Err(EnforcementPayloadError::CommandPayloadInvalid),
    }
}
