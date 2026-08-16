#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;
use serde::{Deserialize, Serialize};

use super::parse_text_id;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PolicyScheduleId(String);

impl PolicyScheduleId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        parse_text_id(value, policy_control::source::FIELD_SCHEDULE_ID).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for PolicyScheduleId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<PolicyScheduleId> for String {
    fn from(value: PolicyScheduleId) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PolicyTimezoneName(String);

impl PolicyTimezoneName {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        parse_text_id(value, policy_control::source::FIELD_TIMEZONE_NAME).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for PolicyTimezoneName {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<PolicyTimezoneName> for String {
    fn from(value: PolicyTimezoneName) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PolicyReasonCode(String);

impl PolicyReasonCode {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        parse_text_id(value, policy_control::source::FIELD_REASON_CODE).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for PolicyReasonCode {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<PolicyReasonCode> for String {
    fn from(value: PolicyReasonCode) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PolicyAuditReferenceId(String);

impl PolicyAuditReferenceId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        parse_text_id(value, policy_control::source::FIELD_AUDIT_REFERENCE_ID).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for PolicyAuditReferenceId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<PolicyAuditReferenceId> for String {
    fn from(value: PolicyAuditReferenceId) -> Self {
        value.0
    }
}
