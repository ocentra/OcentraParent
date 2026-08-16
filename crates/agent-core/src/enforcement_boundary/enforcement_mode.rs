use ocentra_parent_agent_protocol::activity::policy::PolicyAction;
use ocentra_parent_agent_protocol::activity::policy::PolicyTargetType;
use ocentra_parent_agent_protocol::enforcement::{EnforcementIntent, EnforcementMode};

use super::EnforcementBoundaryRejection;

pub(super) fn enforcement_mode(
    intent: &EnforcementIntent,
) -> Result<EnforcementMode, EnforcementBoundaryRejection> {
    match intent.requested_action {
        PolicyAction::Allow | PolicyAction::Warn | PolicyAction::Unknown => {
            Ok(EnforcementMode::ObserveOnly)
        }
        PolicyAction::AskParent => Ok(EnforcementMode::AskParent),
        PolicyAction::TimeLimit => match intent.target.target_type {
            PolicyTargetType::App | PolicyTargetType::Process | PolicyTargetType::Device => {
                Ok(EnforcementMode::TimeLimit)
            }
            _ => Err(EnforcementBoundaryRejection::UnsupportedEnforcementCapability),
        },
        PolicyAction::Block => match intent.target.target_type {
            PolicyTargetType::Process => Ok(EnforcementMode::TerminateProcess),
            PolicyTargetType::App => Ok(EnforcementMode::BlockProcess),
            PolicyTargetType::Domain
            | PolicyTargetType::Site
            | PolicyTargetType::Category
            | PolicyTargetType::Video
            | PolicyTargetType::Channel => Ok(EnforcementMode::TemporaryBlock),
            _ => Err(EnforcementBoundaryRejection::PolicyTargetMismatch),
        },
    }
}
