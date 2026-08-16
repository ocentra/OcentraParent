use std::path::PathBuf as TestPathBuf;
use std::primitive::str as TestStr;
use std::string::String as TestString;
use std::{error::Error, fs::remove_file, io::Error as IoError, path::Path};

use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::tracking::{
    identifiers::TrackingEvidenceRef, read_model::TrackingReadModel,
};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::ACTIVITY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

use crate::activity_report_env_lock::REPORT_ENV_LOCK;
use ocentra_parent_agent_service::test_support::handle_local_command_text_for_test;

type TestResult = Result<(), Box<dyn Error>>;

#[tokio::test]
async fn tracking_read_model_command_reports_service_backed_sqlite_rows() -> TestResult {
    let _guard = REPORT_ENV_LOCK.lock().await;
    let store_path = temp_path(constants::activity_store::TEST_STORE_SUFFIX);
    cleanup_path(&store_path);
    std::env::set_var(constants::env_var::ACTIVITY_DB_PATH, &store_path);

    let test_result: TestResult = async {
        seed_tracking_store(&store_path)?;

        let body = serde_json::to_string(&command_envelope())?;
        let event =
            handle_local_command_text_for_test(crate::test_text::TestText::from_display(body))
                .await;
        let read_model = tracking_read_model_payload(&crate::test_invariants::log_field(
            &event.payload,
            constants::field::ACTIVITY_TRACKING_READ_MODEL,
            constants::error::AGENT_EVENT_SERIALIZES,
        ))?;

        assert_eq!(
            event.event,
            AgentEventName::AgentActivityTrackingReadModelReported
        );
        assert_eq!(read_model.returned, 5);
        assert_eq!(read_model.active_rows, 4);
        assert_eq!(read_model.tombstone_rows, 1);
        assert_service_read_model_latest_events(&read_model)?;
        assert_service_read_model_tombstone_row(&read_model);
        assert_service_read_model_active_product_surface_counts(&read_model);

        Ok(())
    }
    .await;

    std::env::remove_var(constants::env_var::ACTIVITY_DB_PATH);
    cleanup_path(&store_path);
    test_result
}

fn seed_tracking_store(store_path: &Path) -> TestResult {
    let store = ActivityStore::open(store_path).map_err(|error| {
        IoError::other(format!(
            "{}: {error:?}",
            constants::error::ACTIVITY_STORE_OPENS
        ))
    })?;
    store
        .ingest_events(&[
            tracking_activity_event(
                constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID,
                constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT,
                ActivityEventKind::LocationObserved,
                ActivityObserver::AndroidLocation,
                ActivitySubjectKind::Location,
            ),
            tracking_activity_event(
                constants::activity_store::TEST_TRACKING_GEOFENCE_EVENT_ID,
                constants::activity_store::TEST_TRACKING_GEOFENCE_OBSERVED_AT,
                ActivityEventKind::TrackingGeofenceTransitionEvaluated,
                ActivityObserver::TrackingEngine,
                ActivitySubjectKind::TrackingRule,
            ),
            tracking_activity_event(
                "tracking-alert-evaluated-event-1",
                "2026-06-03T02:02:30Z",
                ActivityEventKind::TrackingAlertEvaluated,
                ActivityObserver::TrackingEngine,
                ActivitySubjectKind::TrackingRule,
            ),
            tracking_activity_event(
                "tracking-parent-notification-event-1",
                "2026-06-03T02:03:30Z",
                ActivityEventKind::TrackingParentNotificationRequested,
                ActivityObserver::TrackingEngine,
                ActivitySubjectKind::TrackingRule,
            ),
            tracking_activity_event(
                constants::activity_store::TEST_TRACKING_RETENTION_DELETE_EVENT_ID,
                constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT,
                ActivityEventKind::TrackingRetentionDeleted,
                ActivityObserver::TrackingEngine,
                ActivitySubjectKind::Retention,
            ),
        ])
        .map_err(|error| {
            IoError::other(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_INGESTS
            ))
        })?;

    Ok(())
}

fn assert_service_read_model_latest_events(read_model: &TrackingReadModel) -> TestResult {
    let deleted_evidence_reference_id =
        TrackingEvidenceRef::parse(constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID)
            .map_err(|error| {
                IoError::other(format!(
                    "{}: {error:?}",
                    constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID
                ))
            })?;

    assert_eq!(
        read_model
            .latest_event_id
            .as_ref()
            .map(|value| value.as_str()),
        Some(constants::activity_store::TEST_TRACKING_RETENTION_DELETE_EVENT_ID)
    );
    assert_eq!(
        read_model
            .latest_tombstone_event_id
            .as_ref()
            .map(|value| value.as_str()),
        Some(constants::activity_store::TEST_TRACKING_RETENTION_DELETE_EVENT_ID)
    );
    assert_eq!(
        read_model
            .latest_active_event_id
            .as_ref()
            .map(|value| value.as_str()),
        Some("tracking-parent-notification-event-1")
    );
    assert_eq!(
        read_model.deleted_evidence_reference_ids,
        vec![deleted_evidence_reference_id]
    );

    Ok(())
}

fn assert_service_read_model_tombstone_row(read_model: &TrackingReadModel) {
    assert_eq!(
        read_model.rows[0].evidence_reference_ids[0],
        constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID
    );
    assert_eq!(
        read_model.rows[0].query_visibility,
        ocentra_parent_agent_protocol::tracking::read_model::TRACKING_READ_MODEL_ROW_VISIBILITY_TOMBSTONE
    );
}

fn assert_service_read_model_active_product_surface_counts(read_model: &TrackingReadModel) {
    assert_count(
        &read_model.active_kind_counts,
        constants::activity_event_kind::LOCATION_OBSERVED,
        1,
    );
    assert_count(
        &read_model.active_kind_counts,
        constants::activity_event_kind::TRACKING_GEOFENCE_TRANSITION_EVALUATED,
        1,
    );
    assert_count(
        &read_model.active_kind_counts,
        constants::activity_event_kind::TRACKING_ALERT_EVALUATED,
        1,
    );
    assert_count(
        &read_model.active_kind_counts,
        constants::activity_event_kind::TRACKING_PARENT_NOTIFICATION_REQUESTED,
        1,
    );
    assert_count(
        &read_model.active_device_counts,
        constants::activity_store::TEST_REMOTE_DEVICE_ID,
        4,
    );
    assert_count(
        &read_model.active_capability_status_counts,
        constants::activity_store::TEST_TRACKING_CAPABILITY_STATUS_RECENT,
        4,
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
    event_id: &TestStr,
    observed_at: &TestStr,
    kind: ActivityEventKind,
    observer: ActivityObserver,
    subject_kind: ActivitySubjectKind,
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
        LogFieldValue::String(
            constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID.to_string(),
        ),
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

fn tracking_read_model_payload(value: &LogFieldValue) -> Result<TrackingReadModel, Box<dyn Error>> {
    match value {
        LogFieldValue::String(text) => Ok(serde_json::from_str(text)?),
        _ => Err(Box::new(IoError::other(
            constants::error::AGENT_EVENT_SERIALIZES,
        ))),
    }
}

fn assert_count(
    counts: &[ocentra_parent_agent_protocol::tracking::read_model::TrackingReadModelCount],
    value: &TestStr,
    count: u64,
) {
    let actual = counts
        .iter()
        .find(|entry| entry.value == value)
        .map(|entry| entry.count);
    assert_eq!(actual, Some(count));
}

fn temp_path(suffix: impl AsRef<TestStr>) -> TestPathBuf {
    let suffix = suffix.as_ref();
    let mut name = TestString::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);

    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::activity_store::FILE_EXTENSION);
    path
}

fn cleanup_path(path: &Path) {
    let _ = remove_file(path);
    let mut wal_path = path.to_path_buf();
    wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(wal_path);
    let mut shm_path = path.to_path_buf();
    shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(shm_path);
}
