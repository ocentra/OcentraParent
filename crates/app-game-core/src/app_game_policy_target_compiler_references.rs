use ocentra_eventing::error::EventingError;
use serde::{Deserialize, Serialize};

const REFERENCE_FIELDS: [&str; 11] = [
    "app_game.compile_request_id",
    "app_game.compiled_decision_id",
    "app_game.target_ref",
    "app_game.device_id",
    "app_game.local_user_ref",
    "app_game.evidence_ref",
    "app_game.rule_ref",
    "app_game.schedule_ref",
    "app_game.capability_ref",
    "app_game.authority_ref",
    "app_game.audit_ref",
];

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct AppGamePolicyReference<const KIND: usize>(String);

impl<const KIND: usize> AppGamePolicyReference<KIND> {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(EventingError::EmptyValue {
                field: REFERENCE_FIELDS[KIND],
            });
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<const KIND: usize> TryFrom<String> for AppGamePolicyReference<KIND> {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl<const KIND: usize> From<AppGamePolicyReference<KIND>> for String {
    fn from(value: AppGamePolicyReference<KIND>) -> Self {
        value.0
    }
}

pub type AppGamePolicyCompileRequestId = AppGamePolicyReference<0>;
pub type AppGamePolicyCompiledDecisionId = AppGamePolicyReference<1>;
pub type AppGamePolicyTargetRef = AppGamePolicyReference<2>;
pub type AppGamePolicyDeviceId = AppGamePolicyReference<3>;
pub type AppGamePolicyLocalUserRef = AppGamePolicyReference<4>;
pub type AppGamePolicyEvidenceRef = AppGamePolicyReference<5>;
pub type AppGamePolicyRuleRef = AppGamePolicyReference<6>;
pub type AppGamePolicyScheduleRef = AppGamePolicyReference<7>;
pub type AppGamePolicyCapabilityRef = AppGamePolicyReference<8>;
pub type AppGamePolicyAuthorityRef = AppGamePolicyReference<9>;
pub type AppGamePolicyAuditRef = AppGamePolicyReference<10>;
