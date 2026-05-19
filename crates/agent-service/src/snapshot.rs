use std::env;

use ocentra_parent_agent_protocol::{
    constants, AgentIdentity, AgentLogEntry, AgentLogSnapshot, LogFieldValue, LogFields, LogLevel,
    LogSource, LOG_SCHEMA_VERSION,
};

use crate::time::timestamp_now;

pub fn build_dev_log_snapshot() -> AgentLogSnapshot {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::CAPTURE_ENABLED.to_string(),
        LogFieldValue::Boolean(false),
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
            hostname: hostname(),
            platform: env::consts::OS.to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
        },
        entries: vec![AgentLogEntry {
            id: constants::event_id::DEV_LOCALHOST_API_READY.to_string(),
            timestamp: timestamp_now(),
            level: LogLevel::Info,
            source: LogSource::AgentService,
            message: constants::value::LOCALHOST_API_REACHABLE.to_string(),
            fields,
        }],
    }
}

fn hostname() -> String {
    env::var(constants::env_var::COMPUTER_NAME)
        .or_else(|_| env::var(constants::env_var::HOSTNAME))
        .unwrap_or_else(|_| constants::value::UNKNOWN_HOST.to_string())
}

#[cfg(test)]
mod tests {
    use super::{build_dev_log_snapshot, constants};

    #[test]
    fn build_dev_log_snapshot_uses_protocol_owned_constants() {
        let snapshot = build_dev_log_snapshot();

        assert_eq!(snapshot.agent.device_id, constants::peer::LOCAL_DEV_AGENT);
        assert!(snapshot.entries[0]
            .fields
            .contains_key(constants::field::CAPTURE_ENABLED));
    }
}
