use super::protocol_lookup;
use crate::{constants, BrowserInterventionDecisionSource};

impl BrowserInterventionDecisionSource {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::browser::INTERVENTION_DECISION_SOURCE_PARENT_RULE,
                    Self::ParentRule,
                ),
                (
                    constants::browser::INTERVENTION_DECISION_SOURCE_PARENT_PORTAL,
                    Self::ParentPortal,
                ),
                (
                    constants::browser::INTERVENTION_DECISION_SOURCE_LOCAL_AI,
                    Self::LocalAi,
                ),
                (
                    constants::browser::INTERVENTION_DECISION_SOURCE_SYSTEM,
                    Self::System,
                ),
                (
                    constants::browser::INTERVENTION_DECISION_SOURCE_MANUAL_TEST,
                    Self::ManualTest,
                ),
                (
                    constants::browser::INTERVENTION_DECISION_SOURCE_UNKNOWN,
                    Self::Unknown,
                ),
            ],
        )
    }
}
