#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "u64", into = "u64")]
pub struct PolicyVersion(u64);

impl PolicyVersion {
    pub fn new(value: u64) -> Result<Self, EventingError> {
        if value == 0 {
            return Err(EventingError::InvalidValue {
                field: policy_control::source::FIELD_POLICY_VERSION,
                value: value.to_string(),
            });
        }
        Ok(Self(value))
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

impl TryFrom<u64> for PolicyVersion {
    type Error = EventingError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<PolicyVersion> for u64 {
    fn from(value: PolicyVersion) -> Self {
        value.0
    }
}
