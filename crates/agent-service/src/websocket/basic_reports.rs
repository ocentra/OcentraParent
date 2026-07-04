use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use ocentra_parent_agent_protocol::{
    constants,
    logging::{LogFieldValue, LogLevel},
    transport::{AgentCommandEnvelope, AgentEventEnvelope, AgentEventName},
};

use crate::{
    event_builder::build_event, fields::fields_from_pairs, snapshot::build_dev_log_snapshot,
};

const TEST_RUNTIME_STORE_FILE_PREFIX: &str = "ocentra-parent-agent-service-";
const TEST_RUNTIME_STORE_FILE_EXTENSION: &str = ".json";

pub(crate) fn build_dev_echo_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_event(
        constants::event_id::DEV_ECHOED,
        &command.message_id,
        command.source,
        AgentEventName::AgentDevEchoed,
        LogLevel::Info,
        command.payload,
        None,
    )
}

pub(crate) fn build_health_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_event(
        constants::event_id::HEALTH_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentHealthReported,
        LogLevel::Info,
        fields_from_pairs(vec![
            (constants::field::ONLINE, LogFieldValue::Boolean(true)),
            (
                constants::field::TRANSPORT,
                LogFieldValue::String(constants::value::TRANSPORT_WEBSOCKET.to_string()),
            ),
        ]),
        Some(build_dev_log_snapshot()),
    )
}

pub(crate) fn build_log_snapshot_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_event(
        constants::event_id::LOG_SNAPSHOT_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentLogSnapshotReported,
        LogLevel::Info,
        fields_from_pairs(vec![(
            constants::field::ENTRIES,
            LogFieldValue::Number(1.0),
        )]),
        Some(build_dev_log_snapshot()),
    )
}

pub(crate) fn build_watcher_status_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_event(
        constants::event_id::WATCH_STATUS_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentWatchStatusReported,
        LogLevel::Info,
        fields_from_pairs(vec![
            (constants::field::AVAILABLE, LogFieldValue::Boolean(false)),
            (
                constants::field::NOTE,
                LogFieldValue::String(constants::value::WATCHER_STATUS_ONLY.to_string()),
            ),
        ]),
        None,
    )
}

pub(crate) fn temp_runtime_store_path(prefix: &str) -> PathBuf {
    static TEST_RUNTIME_COUNTER: AtomicU64 = AtomicU64::new(0);

    let sequence = TEST_RUNTIME_COUNTER.fetch_add(1, Ordering::Relaxed);
    let mut file_name = String::from(TEST_RUNTIME_STORE_FILE_PREFIX);
    file_name.push_str(prefix);
    file_name.push('-');
    file_name.push_str(&std::process::id().to_string());
    file_name.push('-');
    file_name.push_str(&sequence.to_string());
    file_name.push_str(TEST_RUNTIME_STORE_FILE_EXTENSION);
    std::env::temp_dir().join(file_name)
}
