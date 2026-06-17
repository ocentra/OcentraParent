use ocentra_parent_agent_protocol::{LogFieldValue as ProtocolLogFieldValue, LogFields};
use ocentra_parent_logging_core::{
    dev_log::DevLogger,
    field::{LogFieldValue, LogFields as CoreLogFields},
    level::LogLevel,
    source::LogSource,
};

// Compatibility local-dev writer until WP04 migrates agent-service logging into crates/logging-core.
pub fn write_agent_info(message: &str, fields: LogFields) -> std::io::Result<()> {
    write_agent_log(LogLevel::Info, message, fields)
}

pub fn write_agent_warn(message: &str, fields: LogFields) -> std::io::Result<()> {
    write_agent_log(LogLevel::Warn, message, fields)
}

pub fn write_agent_error(message: &str, fields: LogFields) -> std::io::Result<()> {
    write_agent_log(LogLevel::Error, message, fields)
}

pub fn write_agent_debug(message: &str, fields: LogFields) -> std::io::Result<()> {
    write_agent_log(LogLevel::Debug, message, fields)
}

fn write_agent_log(level: LogLevel, message: &str, fields: LogFields) -> std::io::Result<()> {
    let logger = DevLogger::from_env(LogSource::AgentService)?;
    let core_fields = into_core_fields(fields);
    match level {
        LogLevel::Info => logger.info(message, core_fields),
        LogLevel::Warn => logger.warn(message, core_fields),
        LogLevel::Error => logger.error(message, core_fields),
        LogLevel::Debug => logger.debug(message, core_fields),
        LogLevel::Trace => logger.log(LogLevel::Trace, message, core_fields),
    }
    .map(|_| ())
}

fn into_core_fields(fields: LogFields) -> CoreLogFields {
    fields
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
