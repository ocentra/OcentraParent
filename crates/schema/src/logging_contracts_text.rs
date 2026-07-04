use std::fmt::{Display, Formatter};

use super::{
    AgentDeviceId, AgentHostname, AgentPlatform, AgentServiceVersion, LogCommandId,
    LogCorrelationId, LogEntryId, LogLaneId, LogMessage, LogRunId, LogTimestamp, StackTrace,
};

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
