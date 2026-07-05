#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;
use serde::{Deserialize, Serialize};

use super::parse_text_id;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ParentPolicyDocumentId(String);

impl ParentPolicyDocumentId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        parse_text_id(value, policy_control::source::FIELD_DOCUMENT_ID).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ParentPolicyDocumentId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<ParentPolicyDocumentId> for String {
    fn from(value: ParentPolicyDocumentId) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PolicyHouseholdId(String);

impl PolicyHouseholdId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        parse_text_id(value, policy_control::source::FIELD_HOUSEHOLD_ID).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for PolicyHouseholdId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<PolicyHouseholdId> for String {
    fn from(value: PolicyHouseholdId) -> Self {
        value.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct PolicyActorId(String);

impl PolicyActorId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        parse_text_id(value, policy_control::source::FIELD_ACTOR_ID).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for PolicyActorId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<PolicyActorId> for String {
    fn from(value: PolicyActorId) -> Self {
        value.0
    }
}
