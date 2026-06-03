use std::fs::remove_file;

use ocentra_parent_agent_core::ActivityStore;
use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind, AgentCommandEnvelope, AgentCommandName, AgentEventName,
    AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute, LogFieldValue, LogFields,
    TrackingReadModel, ACTIVITY_SCHEMA_VERSION, AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{
    activity_report_env_lock::REPORT_ENV_LOCK, lan_pairing::LanPairingRuntime,
    websocket::handle_command_text_for_test,
};

#[tokio::test]
async fn tracking_read_model_command_reports_service_backed_sqlite_rows() {
    let _guard = REPORT_ENV_LOCK.lock().await;
    let store_path = temp_path(constants::activity_store::TEST_STORE_SUFFIX);
    cleanup_path(&store_path);
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);

    let store = ActivityStore::open(&store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
    store
        .ingest_events(&[
            tracking_activity_event(
                constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID,
                constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT,
                ActivityEventKind::LocationObserved,
                ActivityObserver::AndroidLocation,
                ActivitySubjectKind::Location,
                constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID,
            ),
            tracking_activity_event(
                constants::activity_store::TEST_TRACKING_GEOFENCE_EVENT_ID,
                constants::activity_store::TEST_TRACKING_GEOFENCE_OBSERVED_AT,
                ActivityEventKind::TrackingGeofenceTransitionEvaluated,
                ActivityObserver::TrackingEngine,
                ActivitySubjectKind::TrackingRule,
                constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID,
            ),
            tracking_activity_event(
                constants::activity_store::TEST_TRACKING_RETENTION_DELETE_EVENT_ID,
                constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT,
                ActivityEventKind::TrackingRetentionDeleted,
                ActivityObserver::TrackingEngine,
                ActivitySubjectKind::Retention,
                constants::activity_store::TEST_TRACKING_RETENTION_TOMBSTONE_EVIDENCE_REFERENCE_ID,
            ),
        ])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);

    let body =
        serde_json::to_string(&command_envelope()).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let read_model =
        tracking_read_model_payload(&event.payload[constants::field::ACTIVITY_TRACKING_READ_MODEL]);

    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    cleanup_path(&store_path);

    assert_eq!(
        event.event,
        AgentEventName::AgentActivityTrackingReadModelReported
    );
    assert_eq!(read_model.returned, 3);
    assert_eq!(
        read_model.latest_event_id.as_deref(),
        Some(constants::activity_store::TEST_TRACKING_RETENTION_DELETE_EVENT_ID)
    );
    assert_eq!(read_model.retention_tombstone_count, 1);
    assert_eq!(
        read_model.retention_tombstone_evidence_reference_ids[0],
        constants::activity_store::TEST_TRACKING_RETENTION_TOMBSTONE_EVIDENCE_REFERENCE_ID
    );
    assert_eq!(
        read_model.evidence_reference_ids[0],
        constants::activity_store::TEST_TRACKING_RETENTION_TOMBSTONE_EVIDENCE_REFERENCE_ID
    );
    assert_eq!(
        read_model.evidence_reference_ids[1],
        constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID
    );
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::ACTIVITY_TRACKING_READ_MODEL_REPORTED.to_string(),
        sent_at: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform:
                ocentra_parent_agent_protocol::policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
                    .to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentActivityTrackingReadModelGet,
        payload: LogFields::new(),
    }
}

fn tracking_activity_event(
    event_id: &str,
    observed_at: &str,
    kind: ActivityEventKind,
    observer: ActivityObserver,
    subject_kind: ActivitySubjectKind,
    evidence_reference_id: &str,
) -> ActivityEvent {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::CAPABILITY_STATUS.to_string(),
        LogFieldValue::String(
            constants::activity_store::TEST_TRACKING_CAPABILITY_STATUS_RECENT.to_string(),
        ),
    );
    fields.insert(
        constants::field::EVIDENCE_REFERENCE_IDS.to_string(),
        LogFieldValue::String(evidence_reference_id.to_string()),
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: event_id.to_string(),
        observed_at: observed_at.to_string(),
        source: ActivitySource {
            device_id: constants::activity_store::TEST_REMOTE_DEVICE_ID.to_string(),
            platform: constants::activity_store::TEST_TRACKING_PLATFORM_ANDROID.to_string(),
            observer,
            source_id: constants::activity_store::TEST_TRACKING_SOURCE_ID.to_string(),
        },
        kind,
        subject: ActivitySubject {
            kind: subject_kind,
            subject_id: constants::activity_store::TEST_TRACKING_SUBJECT_ID.to_string(),
            display_name: Some(constants::activity_store::TEST_TRACKING_SUBJECT_NAME.to_string()),
        },
        fields,
        evidence: Vec::new(),
    }
}

fn tracking_read_model_payload(value: &LogFieldValue) -> TrackingReadModel {
    match value {
        LogFieldValue::String(text) => {
            serde_json::from_str(text).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}

fn temp_path(suffix: &str) -> std::path::PathBuf {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::activity_store::FILE_EXTENSION);
    path
}

fn cleanup_path(path: &std::path::PathBuf) {
    let _ = remove_file(path);
    let mut wal_path = path.clone();
    wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(wal_path);
    let mut shm_path = path.clone();
    shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(shm_path);
}
