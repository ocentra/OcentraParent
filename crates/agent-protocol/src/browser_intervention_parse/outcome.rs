use super::protocol_lookup;
use crate::{constants, BrowserInterventionOutcome};

impl BrowserInterventionOutcome {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::browser::INTERVENTION_OUTCOME_APPLIED,
                    Self::Applied,
                ),
                (
                    constants::browser::INTERVENTION_OUTCOME_ALLOWED,
                    Self::Allowed,
                ),
                (
                    constants::browser::INTERVENTION_OUTCOME_WARNED,
                    Self::Warned,
                ),
                (
                    constants::browser::INTERVENTION_OUTCOME_BLOCKED,
                    Self::Blocked,
                ),
                (
                    constants::browser::INTERVENTION_OUTCOME_REDIRECTED,
                    Self::Redirected,
                ),
                (
                    constants::browser::INTERVENTION_OUTCOME_APPROVAL_REQUIRED,
                    Self::ApprovalRequired,
                ),
                (constants::browser::INTERVENTION_OUTCOME_HELD, Self::Held),
                (
                    constants::browser::INTERVENTION_OUTCOME_TERMINATED,
                    Self::Terminated,
                ),
                (
                    constants::browser::INTERVENTION_OUTCOME_RELAUNCH_STARTED,
                    Self::RelaunchStarted,
                ),
                (
                    constants::browser::INTERVENTION_OUTCOME_MANUAL_REQUIRED,
                    Self::ManualRequired,
                ),
                (
                    constants::browser::INTERVENTION_OUTCOME_FAILED,
                    Self::Failed,
                ),
                (
                    constants::browser::INTERVENTION_OUTCOME_UNSUPPORTED,
                    Self::Unsupported,
                ),
                (
                    constants::browser::INTERVENTION_OUTCOME_MONITOR_ONLY,
                    Self::MonitorOnly,
                ),
            ],
        )
    }
}
