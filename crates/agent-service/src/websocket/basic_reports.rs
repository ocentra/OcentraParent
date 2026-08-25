use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use ocentra_parent_agent_protocol::{
    constants,
    logging::{LogFieldValue, LogLevel},
    transport::{AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, AgentRoute},
};

use super::health_nonce::health_event_id_suffix;
use crate::{
    event_builder::build_event, fields::fields_from_pairs, snapshot::build_dev_log_snapshot,
};

const TEST_RUNTIME_STORE_FILE_PREFIX: &str = "ocentra-parent-agent-service-";
const TEST_RUNTIME_STORE_FILE_EXTENSION: &str = ".json";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct TempRuntimeStorePrefix(pub(crate) &'static str);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TempRuntimeStorePath(pub(crate) PathBuf);

pub(crate) const BROWSER_POLICY_TEST_STORE_PREFIX: TempRuntimeStorePrefix =
    TempRuntimeStorePrefix(constants::browser_policy::TEST_STORE_FILE_PREFIX);
pub(crate) const SCREEN_SETTINGS_TEST_STORE_PREFIX: TempRuntimeStorePrefix =
    TempRuntimeStorePrefix(constants::screen_settings::TEST_STORE_FILE_PREFIX);

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
    let request_nonce_digest = super::health_nonce::request_nonce_digest(&command).0;
    build_event(
        health_event_id_suffix(&command).0,
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
            (
                constants::field::COMMAND_TARGET_ROUTE,
                LogFieldValue::String(
                    match &command.target.route {
                        AgentRoute::Localhost => constants::value::DEVICE_RUNTIME_ROUTE_LOCALHOST,
                        AgentRoute::LocalNetwork => {
                            constants::value::DEVICE_RUNTIME_ROUTE_LOCAL_NETWORK
                        }
                        AgentRoute::CloudRelay => {
                            constants::value::DEVICE_RUNTIME_ROUTE_CLOUD_RELAY
                        }
                    }
                    .to_string(),
                ),
            ),
            (
                constants::field::LAN_AUTHENTICATION_STATE,
                LogFieldValue::String(constants::value::LAN_AUTH_UNAUTHENTICATED.to_string()),
            ),
            (
                constants::field::REQUEST_NONCE_DIGEST,
                LogFieldValue::String(request_nonce_digest),
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

pub(crate) fn maybe_basic_report(command: AgentCommandEnvelope) -> Option<AgentEventEnvelope> {
    match command.command {
        ocentra_parent_agent_protocol::transport::AgentCommandName::AgentHealthCheck => {
            Some(build_health_report(command))
        }
        ocentra_parent_agent_protocol::transport::AgentCommandName::AgentLogSnapshotGet => {
            Some(build_log_snapshot_report(command))
        }
        ocentra_parent_agent_protocol::transport::AgentCommandName::AgentDevEcho => {
            Some(build_dev_echo_report(command))
        }
        ocentra_parent_agent_protocol::transport::AgentCommandName::AgentWatchStatusGet => {
            Some(build_watcher_status_report(command))
        }
        _ => None,
    }
}

pub(crate) fn temp_runtime_store_path(prefix: TempRuntimeStorePrefix) -> TempRuntimeStorePath {
    static TEST_RUNTIME_COUNTER: AtomicU64 = AtomicU64::new(0);

    let sequence = TEST_RUNTIME_COUNTER.fetch_add(1, Ordering::Relaxed);
    let mut file_name = String::from(TEST_RUNTIME_STORE_FILE_PREFIX);
    file_name.push_str(prefix.0);
    file_name.push('-');
    file_name.push_str(&std::process::id().to_string());
    file_name.push('-');
    file_name.push_str(&sequence.to_string());
    file_name.push_str(TEST_RUNTIME_STORE_FILE_EXTENSION);
    TempRuntimeStorePath(std::env::temp_dir().join(file_name))
}
