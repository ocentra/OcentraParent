use std::env;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{
    AgentIdentity, AgentLogEntry, AgentLogSnapshot, LogFieldValue, LogFields, LogLevel, LogSource,
};
use ocentra_parent_agent_protocol::LOG_SCHEMA_VERSION;

use crate::time::timestamp_now;

pub fn build_dev_log_snapshot() -> AgentLogSnapshot {
    let hostname = env::var(constants::env_var::COMPUTER_NAME)
        .or_else(|_| env::var(constants::env_var::HOSTNAME))
        .unwrap_or_else(|_| constants::value::UNKNOWN_HOST.to_string());
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::CAPTURE_ENABLED.to_string(),
        LogFieldValue::Boolean(capture_enabled()),
    );
    fields.insert(
        constants::field::POLICY_ENGINE_ENABLED.to_string(),
        LogFieldValue::Boolean(false),
    );
    fields.insert(
        constants::field::MODE.to_string(),
        LogFieldValue::String(constants::value::DEV_MODE.to_string()),
    );
    fields.insert(
        constants::field::PID.to_string(),
        LogFieldValue::Number(f64::from(std::process::id())),
    );
    fields.insert(
        constants::field::REMOTE_SYNC.to_string(),
        LogFieldValue::Null(()),
    );

    AgentLogSnapshot {
        schema_version: LOG_SCHEMA_VERSION,
        agent: AgentIdentity {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            hostname,
            platform: env::consts::OS.to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
        },
        entries: vec![AgentLogEntry {
            schema_version: LOG_SCHEMA_VERSION,
            id: constants::event_id::DEV_LOCALHOST_API_READY.to_string(),
            timestamp: timestamp_now(),
            level: LogLevel::Info,
            source: LogSource::AgentService,
            message: constants::value::LOCALHOST_API_REACHABLE.to_string(),
            fields,
        }],
    }
}

#[cfg(windows)]
fn capture_enabled() -> bool {
    true
}

#[cfg(not(windows))]
fn capture_enabled() -> bool {
    false
}
