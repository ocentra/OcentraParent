use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AgentDeviceId(String);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AgentHostname(String);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AgentPlatform(String);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AgentServiceVersion(String);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LogEntryId(String);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LogTimestamp(String);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LogMessage(String);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LogRunId(String);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LogLaneId(String);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LogCommandId(String);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LogCorrelationId(String);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StackTrace(String);

fn parse_non_empty_text(value: impl Into<String>) -> Option<String> {
    let value = value.into();
    if value.trim().is_empty() {
        None
    } else {
        Some(value)
    }
}

macro_rules! impl_logging_text_identifier {
    ($name:ident) => {
        impl $name {
            pub fn parse(value: impl Into<String>) -> Option<Self> {
                parse_non_empty_text(value).map(Self)
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

impl_logging_text_identifier!(AgentDeviceId);
impl_logging_text_identifier!(AgentHostname);
impl_logging_text_identifier!(AgentPlatform);
impl_logging_text_identifier!(AgentServiceVersion);
impl_logging_text_identifier!(LogEntryId);
impl_logging_text_identifier!(LogTimestamp);
impl_logging_text_identifier!(LogMessage);
impl_logging_text_identifier!(LogRunId);
impl_logging_text_identifier!(LogLaneId);
impl_logging_text_identifier!(LogCommandId);
impl_logging_text_identifier!(LogCorrelationId);
impl_logging_text_identifier!(StackTrace);
