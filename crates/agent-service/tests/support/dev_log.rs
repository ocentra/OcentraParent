#[path = "dev_log/message.rs"]
mod message;

use ocentra_parent_agent_protocol::logging::LogFieldValue as ProtocolLogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_logging_core::{
    dev_log::DevLogger,
    field::{LogFieldValue, LogFields as CoreLogFields},
    level::LogLevel,
    source::LogSource,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AgentLogMessageRef<'a>(pub &'a str);

pub trait AgentLogMessageSource {
    fn as_agent_log_message_ref(&self) -> AgentLogMessageRef<'_>;
}

impl AgentLogMessageSource for str {
    fn as_agent_log_message_ref(&self) -> AgentLogMessageRef<'_> {
        AgentLogMessageRef(self)
    }
}

pub fn write_agent_info(
    message: &(impl AgentLogMessageSource + ?Sized),
    fields: LogFields,
) -> std::io::Result<()> {
    write_agent_log(&LogLevel::Info, message, fields)
}

pub fn write_agent_warn(
    message: &(impl AgentLogMessageSource + ?Sized),
    fields: LogFields,
) -> std::io::Result<()> {
    write_agent_log(&LogLevel::Warn, message, fields)
}

pub fn write_agent_error(
    message: &(impl AgentLogMessageSource + ?Sized),
    fields: LogFields,
) -> std::io::Result<()> {
    write_agent_log(&LogLevel::Error, message, fields)
}

pub fn write_agent_debug(
    message: &(impl AgentLogMessageSource + ?Sized),
    fields: LogFields,
) -> std::io::Result<()> {
    write_agent_log(&LogLevel::Debug, message, fields)
}

pub fn write_agent_info_ref(
    message: AgentLogMessageRef<'_>,
    fields: LogFields,
) -> std::io::Result<()> {
    write_agent_info(&message, fields)
}

pub fn write_agent_warn_ref(
    message: AgentLogMessageRef<'_>,
    fields: LogFields,
) -> std::io::Result<()> {
    write_agent_warn(&message, fields)
}

pub fn write_agent_error_ref(
    message: AgentLogMessageRef<'_>,
    fields: LogFields,
) -> std::io::Result<()> {
    write_agent_error(&message, fields)
}

pub fn write_agent_debug_ref(
    message: AgentLogMessageRef<'_>,
    fields: LogFields,
) -> std::io::Result<()> {
    write_agent_debug(&message, fields)
}

fn write_agent_log(
    level: &LogLevel,
    message: &(impl AgentLogMessageSource + ?Sized),
    fields: LogFields,
) -> std::io::Result<()> {
    let logger = DevLogger::from_env(LogSource::AgentService)?;
    let core_fields = into_core_fields(fields);
    let message = message.as_agent_log_message_ref();
    match *level {
        LogLevel::Info => logger.info(message.0, core_fields),
        LogLevel::Warn => logger.warn(message.0, core_fields),
        LogLevel::Error => logger.error(message.0, core_fields),
        LogLevel::Debug => logger.debug(message.0, core_fields),
        LogLevel::Trace => logger.log(LogLevel::Trace, message.0, core_fields),
    }
    .map(|_| ())
}

fn into_core_fields(fields: LogFields) -> CoreLogFields {
    fields
        .into_inner()
        .into_iter()
        .map(|(key, value)| (key, into_core_field_value(value)))
        .collect()
}

fn into_core_field_value(value: ProtocolLogFieldValue) -> LogFieldValue {
    match value {
        ProtocolLogFieldValue::String(value) => LogFieldValue::String(value),
        ProtocolLogFieldValue::Number(value) => LogFieldValue::Number(value),
        ProtocolLogFieldValue::Boolean(value) => LogFieldValue::Boolean(value),
        ProtocolLogFieldValue::Null(value) => LogFieldValue::Null(value),
    }
}
