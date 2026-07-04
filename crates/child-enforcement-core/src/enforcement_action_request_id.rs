use ocentra_eventing::error::EventingError;
use serde::{Deserialize, Serialize};

use crate::enforcement_action_support::parse_enforcement_text_id;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct EnforcementActionRequestId(String);

impl EnforcementActionRequestId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        parse_enforcement_text_id(value, "child_enforcement.request_id").map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for EnforcementActionRequestId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<EnforcementActionRequestId> for String {
    fn from(value: EnforcementActionRequestId) -> Self {
        value.0
    }
}

impl std::fmt::Display for EnforcementActionRequestId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}
