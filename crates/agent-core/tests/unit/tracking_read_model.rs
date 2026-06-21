#![allow(clippy::expect_used, clippy::panic)]

use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind, LogFieldValue, LogFields, TrackingEvidenceRef, TrackingReadModel,
    ACTIVITY_SCHEMA_VERSION, TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS,
};

use ocentra_parent_agent_core::{
    activity_store::ActivityStore,
    tracking::tracking_read_model_for_store,
};

#[test]
fn activity_store_reports_tracking_read_model_from_ingested_events() {
    let read_model = tracking_read_model_with_mixed_events();

    assert_mixed_read_model_counts(&read_model);
    assert_mixed_read_model_latest_events(&read_model);
    assert_mixed_read_model_tombstone_row(&read_model);
    assert_mixed_read_model_active_product_surface_counts(&read_model);
}

fn tracking_read_model_with_mixed_events() -> TrackingReadModel {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let events = [
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
            constants::activity_store::TEST_TRACKING_EXPECTED_PLACE_EVENT_ID,
            constants::activity_store::TEST_TRACKING_EXPECTED_PLACE_OBSERVED_AT,
            ActivityEventKind::TrackingExpectedPlaceEvaluated,
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
    ];
    store
        .ingest_events(&events)
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    tracking_read_model_for_store(
        &store,
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT,
    )
    .expect(constants::error::ACTIVITY_STORE_QUERIES)
}

fn assert_mixed_read_model_counts(read_model: &TrackingReadModel) {
    assert_eq!(read_model.returned, 4);
    assert_eq!(read_model.active_rows, 3);
    assert_eq!(read_model.tombstone_rows, 1);
}

fn assert_mixed_read_model_latest_events(read_model: &TrackingReadModel) {
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
        Some(constants::activity_store::TEST_TRACKING_EXPECTED_PLACE_EVENT_ID)
    );
    assert_eq!(
        read_model
            .latest_active_observed_at
            .as_ref()
            .map(|value| value.as_str()),
        Some(constants::activity_store::TEST_TRACKING_EXPECTED_PLACE_OBSERVED_AT)
    );
    assert_eq!(
        read_model.capability_status,
        constants::activity_store::TEST_TRACKING_CAPABILITY_STATUS_RECENT
    );
    assert_eq!(
        read_model.deleted_evidence_reference_ids,
        vec![tracking_evidence_ref(
            constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID
        )]
    );
}

fn assert_mixed_read_model_tombstone_row(read_model: &TrackingReadModel) {
    assert_eq!(
        read_model.rows[0].evidence_reference_ids,
        vec![tracking_evidence_ref(
            constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID
        )]
    );
    assert_eq!(
        read_model.rows[0].kind,
        constants::activity_event_kind::TRACKING_RETENTION_DELETED
    );
    assert_eq!(
        read_model.rows[0].query_visibility,
        ocentra_parent_agent_protocol::TRACKING_READ_MODEL_ROW_VISIBILITY_TOMBSTONE
    );
    assert_eq!(
        read_model.rows[0]
            .deleted_at
            .as_ref()
            .map(|value| value.as_str()),
        Some(constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT)
    );
}

fn assert_mixed_read_model_active_product_surface_counts(read_model: &TrackingReadModel) {
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
        constants::activity_event_kind::TRACKING_EXPECTED_PLACE_EVALUATED,
        1,
    );
    assert_no_count(
        &read_model.active_kind_counts,
        constants::activity_event_kind::TRACKING_RETENTION_DELETED,
    );
    assert_count(
        &read_model.active_device_counts,
        constants::activity_store::TEST_REMOTE_DEVICE_ID,
        3,
    );
    assert_count(
        &read_model.active_capability_status_counts,
        constants::activity_store::TEST_TRACKING_CAPABILITY_STATUS_RECENT,
        3,
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
    assert_eq!(read_model.active_rows, 0);
    assert_eq!(read_model.tombstone_rows, 0);
    assert_eq!(read_model.rows.len(), 0);
    assert!(read_model.active_kind_counts.is_empty());
    assert!(read_model.active_device_counts.is_empty());
    assert!(read_model.active_capability_status_counts.is_empty());
    assert_eq!(read_model.latest_active_event_id, None);
    assert_eq!(read_model.latest_active_observed_at, None);
    assert!(read_model.deleted_evidence_reference_ids.is_empty());
    assert_eq!(
        read_model.capability_status,
        TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS
    );
}

fn tracking_evidence_ref(value: &str) -> TrackingEvidenceRef {
    TrackingEvidenceRef::parse(value).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn assert_count(
    counts: &[ocentra_parent_agent_protocol::TrackingReadModelCount],
    value: &str,
    count: u64,
) {
    let actual = counts
        .iter()
        .find(|entry| entry.value == value)
        .map(|entry| entry.count);
    assert_eq!(actual, Some(count));
}

fn assert_no_count(counts: &[ocentra_parent_agent_protocol::TrackingReadModelCount], value: &str) {
    assert!(!counts.iter().any(|entry| entry.value == value));
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
