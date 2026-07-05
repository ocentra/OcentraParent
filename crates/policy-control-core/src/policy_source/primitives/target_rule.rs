#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;
use serde::{Deserialize, Serialize};

use super::parse_text_id;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PolicyChildProfileId(String);

impl PolicyChildProfileId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        parse_text_id(value, policy_control::source::FIELD_CHILD_PROFILE_ID).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for PolicyChildProfileId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<PolicyChildProfileId> for String {
    fn from(value: PolicyChildProfileId) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PolicyDeviceId(String);

impl PolicyDeviceId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        parse_text_id(value, policy_control::source::FIELD_DEVICE_ID).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for PolicyDeviceId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<PolicyDeviceId> for String {
    fn from(value: PolicyDeviceId) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PolicyRuleId(String);

impl PolicyRuleId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        parse_text_id(value, policy_control::source::FIELD_RULE_ID).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for PolicyRuleId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<PolicyRuleId> for String {
    fn from(value: PolicyRuleId) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PolicyTargetReferenceId(String);

impl PolicyTargetReferenceId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        parse_text_id(value, policy_control::source::FIELD_TARGET_REFERENCE_ID).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for PolicyTargetReferenceId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<PolicyTargetReferenceId> for String {
    fn from(value: PolicyTargetReferenceId) -> Self {
        value.0
    }
}
