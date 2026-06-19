use ocentra_parent_agent_protocol::{
    constants, ActivityEvidenceKind, ActivityEvidenceRef, LogFieldValue, LogFields,
    TrackingReadModel,
};
use ocentra_tracking_core::read_model::tracking_read_model_for_connection;
use rusqlite::{params, Connection};

#[test]
fn tracking_read_model_includes_alert_and_parent_notification_rows() {
    let connection = tracking_connection();
    insert_tracking_row(
        &connection,
        constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID,
        constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT,
        constants::activity_event_kind::LOCATION_OBSERVED,
        constants::activity_observer::ANDROID_LOCATION,
        constants::activity_subject_kind::LOCATION,
        constants::activity_store::TEST_TRACKING_SUBJECT_ID,
        constants::activity_store::TEST_TRACKING_SUBJECT_NAME,
    );
    insert_tracking_row(
        &connection,
        "tracking-alert-evaluated-event-1",
        "2026-06-03T02:01:30Z",
        constants::activity_event_kind::TRACKING_ALERT_EVALUATED,
        constants::activity_observer::TRACKING_ENGINE,
        constants::activity_subject_kind::TRACKING_RULE,
        "tracking-alert-school",
        "School arrival alert",
    );
    insert_tracking_row(
        &connection,
        "tracking-parent-notification-event-1",
        "2026-06-03T02:03:30Z",
        constants::activity_event_kind::TRACKING_PARENT_NOTIFICATION_REQUESTED,
        constants::activity_observer::TRACKING_ENGINE,
        constants::activity_subject_kind::TRACKING_RULE,
        "tracking-parent-notification-school",
        "Parent notification request",
    );
    insert_tracking_row(
        &connection,
        constants::activity_store::TEST_TRACKING_RETENTION_DELETE_EVENT_ID,
        constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT,
        constants::activity_event_kind::TRACKING_RETENTION_DELETED,
        constants::activity_observer::TRACKING_ENGINE,
        constants::activity_subject_kind::RETENTION,
        constants::activity_store::TEST_TRACKING_SUBJECT_ID,
        constants::activity_store::TEST_TRACKING_SUBJECT_NAME,
    );

    let read_model = tracking_read_model_for_connection(
        &connection,
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT,
    )
    .expect(constants::error::ACTIVITY_STORE_QUERIES);

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
    assert_eq!(
        read_model.rows[1].kind,
        constants::activity_event_kind::TRACKING_PARENT_NOTIFICATION_REQUESTED
    );
    assert_eq!(
        read_model.rows[2].kind,
        constants::activity_event_kind::TRACKING_ALERT_EVALUATED
    );
    assert_eq!(
        read_model.rows[1].evidence_reference_ids[0],
        constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID
    );
    assert_eq!(
        read_model.rows[2].evidence_reference_ids[0],
        constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID
    );
}

#[test]
fn tracking_read_model_excludes_non_tracking_activity_rows() {
    let connection = tracking_connection();
    insert_tracking_row(
        &connection,
        "process-observed-event-1",
        constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT,
        constants::activity_event_kind::PROCESS_OBSERVED,
        constants::activity_observer::WINDOWS_PROCESS,
        constants::activity_subject_kind::PROCESS,
        constants::activity_store::TEST_PROCESS_SUBJECT_ID,
        constants::activity_store::TEST_PROCESS_SUBJECT_NAME,
    );

    let read_model = tracking_read_model_for_connection(
        &connection,
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT,
    )
    .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(read_model.returned, 0);
    assert_eq!(read_model.active_rows, 0);
    assert_eq!(read_model.tombstone_rows, 0);
    assert!(read_model.rows.is_empty());
    assert!(read_model.active_kind_counts.is_empty());
    assert_eq!(read_model.latest_active_event_id, None);
}

fn tracking_connection() -> Connection {
    let connection = Connection::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    connection
        .execute_batch(constants::sqlite::INITIALIZE_ACTIVITY_STORE)
        .expect(constants::error::ACTIVITY_STORE_OPENS);
    connection
}

fn insert_tracking_row(
    connection: &Connection,
    event_id: &str,
    observed_at: &str,
    kind: &str,
    observer: &str,
    subject_kind: &str,
    subject_id: &str,
    subject_display_name: &str,
) {
    let fields_json =
        serde_json::to_string(&tracking_fields()).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let evidence_json = serde_json::to_string(&tracking_evidence())
        .expect(constants::error::AGENT_EVENT_SERIALIZES);

    connection
        .execute(
            constants::sqlite::INSERT_ACTIVITY_EVENT,
            params![
                event_id,
                observed_at,
                constants::activity_store::TEST_REMOTE_DEVICE_ID,
                constants::activity_store::TEST_TRACKING_PLATFORM_ANDROID,
                observer,
                kind,
                subject_kind,
                subject_id,
                subject_display_name,
                fields_json,
                evidence_json
            ],
        )
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
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

fn assert_count(read_model: &TrackingReadModel, kind: &str, count: u64) {
    let actual = read_model
        .active_kind_counts
        .iter()
        .find(|entry| entry.value == kind)
        .map(|entry| entry.count);
    assert_eq!(actual, Some(count));
}
