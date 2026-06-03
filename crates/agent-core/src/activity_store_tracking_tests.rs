use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind, LogFieldValue, LogFields, ACTIVITY_SCHEMA_VERSION,
    TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS,
};

use super::{tracking_read_model_for_store, ActivityStore};

#[test]
fn activity_store_reports_tracking_read_model_from_ingested_events() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let location = tracking_activity_event(
        constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID,
        constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT,
        ActivityEventKind::LocationObserved,
        ActivityObserver::AndroidLocation,
        ActivitySubjectKind::Location,
    );
    let geofence = tracking_activity_event(
        constants::activity_store::TEST_TRACKING_GEOFENCE_EVENT_ID,
        constants::activity_store::TEST_TRACKING_GEOFENCE_OBSERVED_AT,
        ActivityEventKind::TrackingGeofenceTransitionEvaluated,
        ActivityObserver::TrackingEngine,
        ActivitySubjectKind::TrackingRule,
    );
    let expected_place = tracking_activity_event(
        constants::activity_store::TEST_TRACKING_EXPECTED_PLACE_EVENT_ID,
        constants::activity_store::TEST_TRACKING_EXPECTED_PLACE_OBSERVED_AT,
        ActivityEventKind::TrackingExpectedPlaceEvaluated,
        ActivityObserver::TrackingEngine,
        ActivitySubjectKind::TrackingRule,
    );

    store
        .ingest_events(&[location, geofence, expected_place])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = tracking_read_model_for_store(
        &store,
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT,
    )
    .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(read_model.returned, 3);
    assert_eq!(
        read_model.latest_event_id.as_deref(),
        Some(constants::activity_store::TEST_TRACKING_EXPECTED_PLACE_EVENT_ID)
    );
    assert_eq!(
        read_model.capability_status,
        constants::activity_store::TEST_TRACKING_CAPABILITY_STATUS_RECENT
    );
    assert_eq!(
        read_model.rows[0].evidence_reference_ids,
        vec![constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID.to_string()]
    );
    assert_eq!(
        read_model.rows[0].kind,
        constants::activity_event_kind::TRACKING_EXPECTED_PLACE_EVALUATED
    );
}

#[test]
fn activity_store_reports_empty_tracking_read_model_without_inventing_rows() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);

    let read_model = tracking_read_model_for_store(
        &store,
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT,
    )
    .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(read_model.returned, 0);
    assert_eq!(read_model.rows.len(), 0);
    assert_eq!(
        read_model.capability_status,
        TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS
    );
}

fn tracking_activity_event(
    event_id: &str,
    observed_at: &str,
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
