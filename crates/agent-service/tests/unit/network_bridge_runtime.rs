extern crate ocentra_parent_agent_service as agent_service_lib;
extern crate self as ocentra_parent_agent_service;

use chrono::{DateTime, SecondsFormat};
use std::{
    fs::{create_dir_all, read_to_string, remove_file},
    path::{Path, PathBuf},
};

use ocentra_eventing::{error::EventingError, ids::EventId};
use ocentra_parent_agent_core::{
    activity_store::ActivityStore, network_capture::NetworkObservation,
    network_capture_event::network_observation_event,
    network_event_runtime::network_runtime_event_ids_for_source_event,
};
use ocentra_parent_agent_protocol::activity_capture::{
    ActivityCaptureCapabilityStatus, ActivityNetworkProtocol, ActivityNetworkTcpState,
};

#[path = "../support/test_invariants.rs"]
mod test_invariants;
#[path = "../support/test_text.rs"]
mod test_text;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentMessageTarget, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use test_invariants::require_ok;
use test_support::{lock_activity_report_env_for_test, network_runtime_journal_path_for_test};

#[tokio::test]
async fn network_runtime_spine_reuses_the_same_journal_path() {
    let path = network_runtime_journal_path_for_test();

    network_runtime_delivery::initialize_network_runtime_spine(&path)
        .await
        .expect("first durable spine initialization must succeed");
    network_runtime_delivery::initialize_network_runtime_spine(&path)
        .await
        .expect("same journal path must be idempotently reusable");
}

#[tokio::test]
async fn network_runtime_spine_rejects_a_different_journal_path() {
    let path = network_runtime_journal_path_for_test();
    network_runtime_delivery::initialize_network_runtime_spine(&path)
        .await
        .expect("durable spine must initialize before mismatch check");

    let mut different_path = path.as_path().to_path_buf();
    different_path.set_file_name("different-network-runtime-journal.ndjson");
    let error = network_runtime_delivery::initialize_network_runtime_spine(
        &ocentra_parent_agent_core::network_event_runtime::NetworkRuntimeJournalPath::new(
            different_path,
        ),
    )
    .await
    .expect_err("a different journal path must not reuse the process-global spine");

    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: constants::network_flow::NETWORK_RUNTIME_SPINE_FIELD,
            value: constants::network_flow::NETWORK_RUNTIME_SPINE_JOURNAL_PATH_MISMATCH.to_string(),
        }
    );
}

#[path = "../support/network_bridge_test_support.rs"]
pub mod test_support;

#[path = "../../src/activity_capture.rs"]
mod activity_capture;
#[path = "../../src/activity_network_flow_payload.rs"]
mod activity_network_flow_payload;
#[path = "../support/activity_report_env_lock.rs"]
mod activity_report_env_lock;
#[path = "../../src/activity_store_path.rs"]
mod activity_store_path;
#[path = "../../src/dev_log.rs"]
mod dev_log;
#[path = "../../src/event_builder.rs"]
mod event_builder;
#[path = "../../src/fields.rs"]
mod fields;
#[path = "../../src/network_android_vpn_service_gate_status_bridge.rs"]
mod network_android_vpn_service_gate_status_bridge;
#[path = "../../src/network_apple_network_extension_gate_status_bridge.rs"]
mod network_apple_network_extension_gate_status_bridge;
#[path = "../../src/network_flow_digest.rs"]
mod network_flow_digest;
#[path = "../../src/network_flow_digest_indicators.rs"]
mod network_flow_digest_indicators;
#[path = "../../src/network_flow_digest_rollups.rs"]
mod network_flow_digest_rollups;
#[path = "../../src/network_linux_nftables_lab_status_bridge.rs"]
mod network_linux_nftables_lab_status_bridge;
#[path = "../../src/network_live_capture_execution_bridge.rs"]
mod network_live_capture_execution_bridge;
#[path = "../../src/network_live_capture_readiness_bridge.rs"]
mod network_live_capture_readiness_bridge;
#[path = "../../src/network_remote_delivery_status_cross_process.rs"]
mod network_remote_delivery_status_cross_process;
#[path = "../../src/network_remote_delivery_status_payload.rs"]
mod network_remote_delivery_status_payload;
#[path = "../../src/network_runtime_delivery.rs"]
mod network_runtime_delivery;
#[path = "../../src/network_runtime_stream_event_payloads.rs"]
mod network_runtime_stream_event_payloads;
#[path = "../../src/network_runtime_stream_event_values.rs"]
mod network_runtime_stream_event_values;
#[path = "../../src/network_runtime_stream_events.rs"]
mod network_runtime_stream_events;
#[path = "../../src/network_runtime_stream_payload.rs"]
mod network_runtime_stream_payload;
#[path = "../../src/network_windows_firewall_lab_status_bridge.rs"]
mod network_windows_firewall_lab_status_bridge;
#[path = "../../src/network_windows_wfp_gate_status_bridge.rs"]
mod network_windows_wfp_gate_status_bridge;
#[path = "../../src/time.rs"]
mod time;

#[path = "network_android_vpn_service_gate_status_bridge_tests.rs"]
mod network_android_vpn_service_gate_status_bridge_tests;
#[path = "network_apple_network_extension_gate_status_bridge_tests.rs"]
mod network_apple_network_extension_gate_status_bridge_tests;
#[path = "network_flow_digest_tests.rs"]
mod network_flow_digest_tests;
#[path = "network_flow_payload_tests.rs"]
mod network_flow_payload_tests;
#[path = "network_linux_nftables_lab_status_bridge_tests.rs"]
mod network_linux_nftables_lab_status_bridge_tests;
#[path = "network_live_capture_readiness_bridge_tests.rs"]
mod network_live_capture_readiness_bridge_tests;
#[path = "network_remote_delivery_status_service_tests.rs"]
mod network_remote_delivery_status_service_tests;
#[path = "network_runtime_delivery_tests.rs"]
mod network_runtime_delivery_tests;
#[path = "network_runtime_stream_tests.rs"]
mod network_runtime_stream_tests;
#[path = "network_windows_firewall_lab_status_bridge_tests.rs"]
mod network_windows_firewall_lab_status_bridge_tests;
#[path = "network_windows_wfp_gate_status_bridge_tests.rs"]
mod network_windows_wfp_gate_status_bridge_tests;

#[test]
fn network_bridge_runtime_links_report_builders_and_time_helpers() {
    let command = command_envelope();

    let android = network_android_vpn_service_gate_status_bridge::build_network_android_vpn_service_gate_status_report(
        command.clone(),
    );
    assert_eq!(
        android.event,
        ocentra_parent_agent_protocol::transport::AgentEventName::AgentNetworkAndroidVpnServiceGateStatusReported
    );

    let apple = network_apple_network_extension_gate_status_bridge::build_network_apple_network_extension_gate_status_report(
        command.clone(),
    );
    assert_eq!(
        apple.event,
        ocentra_parent_agent_protocol::transport::AgentEventName::AgentNetworkAppleNetworkExtensionGateStatusReported
    );

    let linux =
        network_linux_nftables_lab_status_bridge::build_network_linux_nftables_lab_status_report(
            command.clone(),
        );
    assert_eq!(
        linux.event,
        ocentra_parent_agent_protocol::transport::AgentEventName::AgentNetworkLinuxNftablesLabStatusReported
    );

    let live_capture =
        network_live_capture_readiness_bridge::build_network_live_capture_status_report(
            command.clone(),
        );
    assert_eq!(
        live_capture.event,
        ocentra_parent_agent_protocol::transport::AgentEventName::AgentNetworkLiveCaptureStatusReported
    );

    let firewall = network_windows_firewall_lab_status_bridge::build_network_windows_firewall_lab_status_report(
        command.clone(),
    );
    assert_eq!(
        firewall.event,
        ocentra_parent_agent_protocol::transport::AgentEventName::AgentNetworkWindowsFirewallLabStatusReported
    );

    let wfp = network_windows_wfp_gate_status_bridge::build_network_windows_wfp_gate_status_report(
        command,
    );
    assert_eq!(
        wfp.event,
        ocentra_parent_agent_protocol::transport::AgentEventName::AgentNetworkWindowsWfpGateStatusReported
    );

    let runtime_report =
        network_runtime_stream_payload::NetworkRuntimeServiceStreamReport::default();
    let runtime_payload =
        network_runtime_stream_payload::network_runtime_event_chain_stream_payload(&runtime_report);
    assert_eq!(
        runtime_payload.get(constants::field::NETWORK_RUNTIME_EVENT_CHAIN_STREAM),
        Some(&ocentra_parent_agent_protocol::logging::LogFieldValue::String("[]".to_string()))
    );

    let timestamp_now: String = time::timestamp_now();
    let timestamp_from_epoch: String = time::timestamp_from_epoch_seconds(0);
    let timestamp_after_epoch: String = time::timestamp_after_epoch_seconds(0, 1);
    let parsed_timestamp_now = require_ok(
        DateTime::parse_from_rfc3339(&timestamp_now),
        "timestamp_now must use RFC3339 formatting",
    );
    assert_eq!(
        parsed_timestamp_now.to_rfc3339_opts(SecondsFormat::Millis, true),
        timestamp_now
    );
    assert_eq!(timestamp_from_epoch, "1970-01-01T00:00:00.000Z");
    assert_eq!(timestamp_after_epoch, "1970-01-01T00:00:01.000Z");
    assert_eq!(
        event_builder::portal_peer().peer_id,
        constants::peer::PORTAL_DEV
    );
}

#[tokio::test]
async fn network_bridge_runtime_links_remote_delivery_report_builder() {
    let event =
        network_remote_delivery_status_payload::build_network_remote_delivery_status_report(
            command_envelope(),
        )
        .await;

    assert_eq!(
        event.event,
        ocentra_parent_agent_protocol::transport::AgentEventName::AgentNetworkRemoteDeliveryStatusReported
    );
}

#[test]
fn network_bridge_runtime_links_test_invariant_helpers() {
    let decoded: serde_json::Value =
        test_invariants::require_json_decode("{\"timestamp\":\"ok\"}", "json decodes");
    assert_eq!(decoded["timestamp"], "ok");

    assert_eq!(
        test_invariants::require_some(Some("linked"), "option links"),
        "linked"
    );

    let mut payload = LogFields::new();
    payload.insert(
        constants::field::NETWORK_RUNTIME_EVENT_CHAIN_STREAM.to_string(),
        ocentra_parent_agent_protocol::logging::LogFieldValue::String("[]".to_string()),
    );
    assert_eq!(
        test_invariants::require_log_string_field(
            Some(&test_invariants::log_field(
                &payload,
                constants::field::NETWORK_RUNTIME_EVENT_CHAIN_STREAM,
                "log field links",
            )),
            "string log field links",
        ),
        "[]"
    );
}

#[tokio::test]
async fn network_runtime_composes_persisted_capture_with_exact_runtime_observation() {
    let _guard = lock_activity_report_env_for_test().await;
    let suffix = format!("{}-{}", std::process::id(), "wp09-composition");
    let (journal_path, key_path, store_path) = network_runtime_service_artifact_paths(&suffix);
    remove_network_runtime_service_artifacts(&journal_path, &key_path, &store_path);

    let observed_at = format!("2026-08-16T20:21:{:02}.000Z", std::process::id() % 60);
    let observation = NetworkObservation {
        status: ActivityCaptureCapabilityStatus::Available,
        protocol: Some(ActivityNetworkProtocol::Tcp),
        local_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
        local_port: Some(constants::activity_store::TEST_NETWORK_LOCAL_PORT),
        destination_ip: Some(constants::activity_store::TEST_NETWORK_DESTINATION_IP.to_string()),
        destination_port: Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT),
        destination_domain: Some(constants::activity_store::TEST_NETWORK_DOMAIN.to_string()),
        tcp_state: Some(ActivityNetworkTcpState::Established),
        pid: Some(4242),
        process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
        associated_pid_count: 3,
    };
    let activity_event = network_observation_event(observation.clone(), &observed_at, 77);
    let source_event_id = EventId::parse(activity_event.event_id.clone())
        .expect("persisted capture event ID must be a valid domain EventId");
    let activity_evidence_id = activity_event.evidence[0].evidence_id.clone();

    activity_capture::record_activity_events_to_paths(
        &journal_path,
        &key_path,
        &store_path,
        std::slice::from_ref(&activity_event),
    )
    .expect("activity event must persist through the real capture boundary");
    let store = ActivityStore::open(&store_path).expect("activity SQLite store must open");
    let read_model = store
        .network_flow_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            &observed_at,
        )
        .expect("persisted network event must produce a read model");
    assert_eq!(read_model.rows.len(), 1);
    assert_eq!(read_model.rows[0].event_id, activity_event.event_id);
    assert_eq!(read_model.rows[0].associated_pid_count, Some(3));
    assert!(read_model.rows[0]
        .evidence
        .iter()
        .any(|reference| reference.evidence_id == activity_evidence_id));
    drop(store);

    let runtime_path = network_runtime_journal_path_for_test();
    network_runtime_delivery::initialize_network_runtime_spine(&runtime_path)
        .await
        .expect("durable runtime spine must initialize before publication");
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);
    let before_publish = journal_envelope_row_count(runtime_path.as_path());
    let before_publish_lines = journal_line_count(runtime_path.as_path());
    network_runtime_delivery::reconcile_retained_network_runtime()
        .await
        .expect("restart reconciliation must preserve the persisted PID count");
    let after_reconciliation = journal_envelope_row_count(runtime_path.as_path());
    assert_eq!(after_reconciliation - before_publish, 3);
    let captured = activity_capture::capture_events::NetworkCaptureObservation {
        source_event_id: activity_event.event_id.clone(),
        observed_at: observed_at.clone(),
        observation: observation.clone(),
    };
    network_runtime_delivery::publish_captured_network_observations(&[captured.clone()])
        .await
        .expect("exact persisted capture observation must publish to the durable runtime");
    let after_publish = journal_envelope_row_count(runtime_path.as_path());
    assert_eq!(after_publish, after_reconciliation);
    let after_publish_lines = journal_line_count(runtime_path.as_path());
    assert_eq!(after_publish_lines - before_publish_lines, 9);

    assert_durable_runtime_projection(
        &read_model,
        &source_event_id,
        &observation,
        &activity_event.event_id,
    )
    .await;

    network_runtime_delivery::publish_captured_network_observations(&[captured])
        .await
        .expect("retrying the exact capture must remain idempotent");
    let after_retry = journal_envelope_row_count(runtime_path.as_path());
    assert_eq!(after_retry, after_publish);
    assert_eq!(
        journal_line_count(runtime_path.as_path()),
        after_publish_lines
    );

    remove_network_runtime_service_artifacts(&journal_path, &key_path, &store_path);
    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
}

#[tokio::test]
async fn startup_reconciliation_rejects_malformed_persisted_row_without_journal_mutation() {
    let _guard = lock_activity_report_env_for_test().await;
    let suffix = format!("{}-{}", std::process::id(), "wp09-reconciliation");
    let (journal_path, key_path, store_path) = network_runtime_service_artifact_paths(&suffix);
    remove_network_runtime_service_artifacts(&journal_path, &key_path, &store_path);

    let observed_at = format!("2026-08-16T20:22:{:02}.000Z", std::process::id() % 60);
    let observation = NetworkObservation {
        status: ActivityCaptureCapabilityStatus::Available,
        protocol: Some(ActivityNetworkProtocol::Tcp),
        local_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
        local_port: Some(constants::activity_store::TEST_NETWORK_LOCAL_PORT),
        destination_ip: Some(constants::activity_store::TEST_NETWORK_DESTINATION_IP.to_string()),
        destination_port: Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT),
        destination_domain: Some(constants::activity_store::TEST_NETWORK_DOMAIN.to_string()),
        tcp_state: Some(ActivityNetworkTcpState::Established),
        pid: Some(4242),
        process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
        associated_pid_count: 1,
    };
    let activity_event = network_observation_event(observation, &observed_at, 78);
    activity_capture::record_activity_events_to_paths(
        &journal_path,
        &key_path,
        &store_path,
        std::slice::from_ref(&activity_event),
    )
    .expect("valid activity event must persist before corruption is introduced");

    let store = ActivityStore::open(&store_path).expect("activity SQLite store must open");
    let updated = store
        .connection_for_test()
        .execute(
            "UPDATE activity_events SET observed_at = ?1 WHERE event_id = ?2",
            ["not-rfc3339", activity_event.event_id.as_str()],
        )
        .expect("persisted canonical timestamp must be mutated for the restart regression");
    assert_eq!(updated, 1);
    drop(store);

    let runtime_path = network_runtime_journal_path_for_test();
    network_runtime_delivery::initialize_network_runtime_spine(&runtime_path)
        .await
        .expect("durable runtime spine must initialize before restart reconciliation");
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);
    let before_lines = journal_line_count(runtime_path.as_path());
    let before_envelopes = journal_envelope_row_count(runtime_path.as_path());

    let error = network_runtime_delivery::reconcile_retained_network_runtime()
        .await
        .expect_err("malformed persisted timestamp must fail startup reconciliation");
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: constants::field::OBSERVED_AT,
            value: constants::network_flow::NETWORK_RUNTIME_STARTUP_RECONCILIATION_FAILURE
                .to_string(),
        }
    );
    assert_eq!(journal_line_count(runtime_path.as_path()), before_lines);
    assert_eq!(
        journal_envelope_row_count(runtime_path.as_path()),
        before_envelopes
    );

    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    remove_network_runtime_service_artifacts(&journal_path, &key_path, &store_path);
}

fn network_runtime_service_artifact_paths(suffix: &str) -> (PathBuf, PathBuf, PathBuf) {
    let artifact_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("target")
        .join("test-artifacts")
        .join("network-runtime-service");
    create_dir_all(&artifact_dir).expect("network runtime service artifact directory must exist");
    let stem = artifact_dir.join(format!("ocentra-{suffix}"));
    (
        stem.with_extension("ndjson"),
        stem.with_extension("key"),
        stem.with_extension("db"),
    )
}

fn remove_network_runtime_service_artifacts(
    journal_path: &Path,
    key_path: &Path,
    store_path: &Path,
) {
    for path in [journal_path, key_path, store_path] {
        let _ = remove_file(path);
    }

    let mut wal_path = store_path.to_path_buf();
    wal_path.set_extension("db-wal");
    let _ = remove_file(wal_path);

    let mut shm_path = store_path.to_path_buf();
    shm_path.set_extension("db-shm");
    let _ = remove_file(shm_path);

    let mut append_lock_path = journal_path.to_path_buf();
    if let Some(file_name) = journal_path.file_name() {
        let mut lock_name = file_name.to_os_string();
        lock_name.push(".append.lock");
        append_lock_path.set_file_name(lock_name);
        let _ = remove_file(append_lock_path);
    }
}

async fn assert_durable_runtime_projection(
    read_model: &ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowReadModel,
    source_event_id: &EventId,
    observation: &NetworkObservation,
    source_evidence_ref: &str,
) {
    let report = network_runtime_delivery::durable_network_runtime_projection(read_model)
        .await
        .expect("durable runtime projection must replay");
    let expected_ids = network_runtime_event_ids_for_source_event(source_event_id, observation)
        .expect("expected runtime IDs must derive from the valid source ID");
    let actual_ids = report
        .stored_events
        .iter()
        .map(|event| event.event_id.clone())
        .collect::<Vec<_>>();
    assert_eq!(actual_ids, expected_ids);
    assert_eq!(
        report
            .stored_events
            .iter()
            .map(|event| event.contract.event_type.as_str())
            .collect::<Vec<_>>(),
        vec![
            constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED,
            constants::network_flow::EVENT_NETWORK_DOMAIN_OBSERVED,
            constants::network_flow::EVENT_NETWORK_ACTIVITY_CLASSIFIED,
        ]
    );
    for event in &report.stored_events {
        let payload = event
            .decode::<ocentra_parent_agent_core::network_event_runtime::NetworkRuntimeEventPayload>(
            )
            .expect("owned runtime payload must decode");
        assert_eq!(payload.payload().evidence_ref, source_evidence_ref);
        assert_eq!(
            payload.payload().associated_pid_count,
            observation.associated_pid_count
        );
        assert_ne!(
            event.contract.event_type.as_str(),
            constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED
        );
    }
}

fn journal_envelope_row_count(path: &std::path::Path) -> usize {
    read_to_string(path)
        .map(|text| {
            text.lines()
                .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
                .filter(|record| record.get("envelope").is_some())
                .count()
        })
        .unwrap_or_default()
}

fn journal_line_count(path: &std::path::Path) -> usize {
    read_to_string(path)
        .map(|text| text.lines().filter(|line| !line.trim().is_empty()).count())
        .unwrap_or_default()
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::NETWORK_RUNTIME_EVENT_CHAIN_STREAM_REPORTED.to_string(),
        sent_at: time::timestamp_from_epoch_seconds(0),
        source: event_builder::portal_peer(),
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentNetworkRuntimeEventChainStreamGet,
        payload: LogFields::new(),
    }
}
