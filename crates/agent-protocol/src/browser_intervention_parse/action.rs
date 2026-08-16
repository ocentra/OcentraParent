use super::protocol_lookup;
use crate::{constants, BrowserInterventionAction};

impl BrowserInterventionAction {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (constants::browser::INTERVENTION_ACTION_ALLOW, Self::Allow),
                (constants::browser::INTERVENTION_ACTION_WARN, Self::Warn),
                (constants::browser::INTERVENTION_ACTION_BLOCK, Self::Block),
                (
                    constants::browser::INTERVENTION_ACTION_REDIRECT,
                    Self::Redirect,
                ),
                (
                    constants::browser::INTERVENTION_ACTION_TIME_LIMIT,
                    Self::TimeLimit,
                ),
                (
                    constants::browser::INTERVENTION_ACTION_ASK_PARENT,
                    Self::AskParent,
                ),
                (
                    constants::browser::INTERVENTION_ACTION_APPROVAL_HOLD,
                    Self::ApprovalHold,
                ),
                (
                    constants::browser::INTERVENTION_ACTION_CHECKING_HOLD,
                    Self::CheckingHold,
                ),
                (
                    constants::browser::INTERVENTION_ACTION_TERMINATE_PROCESS,
                    Self::TerminateProcess,
                ),
                (
                    constants::browser::INTERVENTION_ACTION_RELAUNCH_MANAGED,
                    Self::RelaunchManaged,
                ),
                (
                    constants::browser::INTERVENTION_ACTION_MONITOR,
                    Self::Monitor,
                ),
                (
                    constants::browser::INTERVENTION_ACTION_UNKNOWN,
                    Self::Unknown,
                ),
            ],
        )
    }
}
