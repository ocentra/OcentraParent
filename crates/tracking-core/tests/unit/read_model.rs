use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::tracking::read_model::TrackingReadModel;
use ocentra_tracking_core::read_model::tracking_read_model_for_connection;
use rusqlite::{params, Connection};

#[derive(Clone, Copy)]
struct TrackingRowSeed {
    event_id: &'static str,
    observed_at: &'static str,
    kind: &'static str,
    observer: &'static str,
    subject_kind: &'static str,
    subject_id: &'static str,
    subject_display_name: &'static str,
}

#[test]
fn tracking_read_model_includes_alert_and_parent_notification_rows() {
    let connection = tracking_connection();
    insert_tracking_rows(&connection, &tracking_read_model_rows());

    let read_model = tracking_read_model_for_connection(
        &connection,
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT,
    )
    .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(read_model.returned, 4);
    assert_eq!(read_model.active_rows, 3);
    assert_eq!(read_model.tombstone_rows, 1);
    assert_eq!(
        read_model
            .latest_active_event_id
            .as_ref()
            .map(|value| value.as_str()),
        Some("tracking-parent-notification-event-1")
    );
    assert_count(
        &read_model,
        constants::activity_event_kind::TRACKING_ALERT_EVALUATED,
        1,
    );
    assert_count(
        &read_model,
        constants::activity_event_kind::TRACKING_PARENT_NOTIFICATION_REQUESTED,
        1,
    );
    assert_row_kind(
        &read_model,
        1,
        constants::activity_event_kind::TRACKING_PARENT_NOTIFICATION_REQUESTED,
    );
    assert_row_kind(
        &read_model,
        2,
        constants::activity_event_kind::TRACKING_ALERT_EVALUATED,
    );
    assert_row_evidence_reference_id(
        &read_model,
        1,
        constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID,
    );
    assert_row_evidence_reference_id(
        &read_model,
        2,
        constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID,
    );
}

#[test]
fn tracking_read_model_excludes_non_tracking_activity_rows() {
    let connection = tracking_connection();
    insert_tracking_row(
        &connection,
        TrackingRowSeed {
            event_id: "process-observed-event-1",
            observed_at: constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT,
            kind: constants::activity_event_kind::PROCESS_OBSERVED,
            observer: constants::activity_observer::WINDOWS_PROCESS,
            subject_kind: constants::activity_subject_kind::PROCESS,
            subject_id: constants::activity_store::TEST_PROCESS_SUBJECT_ID,
            subject_display_name: constants::activity_store::TEST_PROCESS_SUBJECT_NAME,
        },
    );

    let read_model = tracking_read_model_for_connection(
        &connection,
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT,
    )
    .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(read_model.returned, 0);
    assert_eq!(read_model.active_rows, 0);
    assert_eq!(read_model.tombstone_rows, 0);
    assert!(read_model.rows.is_empty());
    assert!(read_model.active_kind_counts.is_empty());
    assert_eq!(read_model.latest_active_event_id, None);
}

fn tracking_connection() -> Connection {
    let connection =
        Connection::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);
    connection
        .execute_batch(constants::sqlite::INITIALIZE_ACTIVITY_STORE)
        .expect_value(constants::error::ACTIVITY_STORE_OPENS);
    connection
}

fn insert_tracking_row(connection: &Connection, row: TrackingRowSeed) {
    let fields_json = serde_json::to_string(&tracking_fields())
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let evidence_json = serde_json::to_string(&tracking_evidence())
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    connection
        .execute(
            constants::sqlite::INSERT_ACTIVITY_EVENT,
            params![
                row.event_id,
                row.observed_at,
                constants::activity_store::TEST_REMOTE_DEVICE_ID,
                constants::activity_store::TEST_TRACKING_PLATFORM_ANDROID,
                row.observer,
                row.kind,
                row.subject_kind,
                row.subject_id,
                row.subject_display_name,
                fields_json,
                evidence_json
            ],
        )
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);
}

fn insert_tracking_rows(connection: &Connection, rows: &[TrackingRowSeed]) {
    for row in rows {
        insert_tracking_row(connection, *row);
    }
}

fn tracking_read_model_rows() -> [TrackingRowSeed; 4] {
    [
        TrackingRowSeed {
            event_id: constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID,
            observed_at: constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT,
            kind: constants::activity_event_kind::LOCATION_OBSERVED,
            observer: constants::activity_observer::ANDROID_LOCATION,
            subject_kind: constants::activity_subject_kind::LOCATION,
            subject_id: constants::activity_store::TEST_TRACKING_SUBJECT_ID,
            subject_display_name: constants::activity_store::TEST_TRACKING_SUBJECT_NAME,
        },
        TrackingRowSeed {
            event_id: "tracking-alert-evaluated-event-1",
            observed_at: "2026-06-03T02:01:30Z",
            kind: constants::activity_event_kind::TRACKING_ALERT_EVALUATED,
            observer: constants::activity_observer::TRACKING_ENGINE,
            subject_kind: constants::activity_subject_kind::TRACKING_RULE,
            subject_id: "tracking-alert-school",
            subject_display_name: "School arrival alert",
        },
        TrackingRowSeed {
            event_id: "tracking-parent-notification-event-1",
            observed_at: "2026-06-03T02:03:30Z",
            kind: constants::activity_event_kind::TRACKING_PARENT_NOTIFICATION_REQUESTED,
            observer: constants::activity_observer::TRACKING_ENGINE,
            subject_kind: constants::activity_subject_kind::TRACKING_RULE,
            subject_id: "tracking-parent-notification-school",
            subject_display_name: "Parent notification request",
        },
        TrackingRowSeed {
            event_id: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_EVENT_ID,
            observed_at: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT,
            kind: constants::activity_event_kind::TRACKING_RETENTION_DELETED,
            observer: constants::activity_observer::TRACKING_ENGINE,
            subject_kind: constants::activity_subject_kind::RETENTION,
            subject_id: constants::activity_store::TEST_TRACKING_SUBJECT_ID,
            subject_display_name: constants::activity_store::TEST_TRACKING_SUBJECT_NAME,
        },
    ]
}

fn tracking_fields() -> LogFields {
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
    fields
}

fn tracking_evidence() -> Vec<ActivityEvidenceRef> {
    vec![ActivityEvidenceRef {
        evidence_id: constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID.to_string(),
        kind: ActivityEvidenceKind::JournalEntry,
        digest: Some("sha256:tracking-read-model".to_string()),
        uri: None,
    }]
}

fn assert_count(read_model: &TrackingReadModel, kind: impl core::fmt::Display, count: u64) {
    let kind = kind.to_string();
    let actual = read_model
        .active_kind_counts
        .iter()
        .find(|entry| entry.value.as_str() == kind)
        .map(|entry| entry.count);
    assert_eq!(actual, Some(count));
}

fn assert_row_kind(
    read_model: &TrackingReadModel,
    row_index: usize,
    expected_kind: impl core::fmt::Display,
) {
    assert_eq!(
        read_model.rows[row_index].kind.as_str(),
        expected_kind.to_string()
    );
}

fn assert_row_evidence_reference_id(
    read_model: &TrackingReadModel,
    row_index: usize,
    expected_evidence_reference_id: impl core::fmt::Display,
) {
    assert_eq!(
        read_model.rows[row_index].evidence_reference_ids[0].as_str(),
        expected_evidence_reference_id.to_string()
    );
}
