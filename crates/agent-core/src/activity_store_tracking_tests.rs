use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind, LogFieldValue, LogFields, TrackingReadModel, ACTIVITY_SCHEMA_VERSION,
    TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS, TRACKING_READ_MODEL_SURFACE_CHILD_CHECK_IN,
    TRACKING_READ_MODEL_SURFACE_LOCATION, TRACKING_READ_MODEL_SURFACE_RETENTION,
};

use super::{tracking_read_model_for_store, ActivityStore};

#[test]
fn activity_store_reports_tracking_read_model_from_ingested_events() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    store
        .ingest_events(&tracking_activity_events())
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let read_model = tracking_read_model_for_store(
        &store,
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT,
    )
    .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_tracking_read_model(&read_model);
}

fn assert_tracking_read_model(read_model: &TrackingReadModel) {
    assert_eq!(read_model.returned, 5);
    assert_eq!(read_model.active_rows, 4);
    assert_eq!(read_model.tombstone_rows, 1);
    assert_eq!(
        read_model.latest_event_id.as_deref(),
        Some(constants::activity_store::TEST_TRACKING_RETENTION_DELETE_EVENT_ID)
    );
    assert_eq!(
        read_model.latest_tombstone_event_id.as_deref(),
        Some(constants::activity_store::TEST_TRACKING_RETENTION_DELETE_EVENT_ID)
    );
    assert_eq!(
        read_model.capability_status,
        constants::activity_store::TEST_TRACKING_CAPABILITY_STATUS_RECENT
    );
    assert_eq!(
        read_model.deleted_evidence_reference_ids,
        vec![constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID.to_string()]
    );
    assert_eq!(
        read_model.rows[0].evidence_reference_ids,
        vec![constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID.to_string()]
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
        read_model.rows[0].deleted_at.as_deref(),
        Some(constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT)
    );
    assert_eq!(
        coverage(read_model, TRACKING_READ_MODEL_SURFACE_LOCATION).active_rows,
        1
    );
    assert_eq!(
        coverage(read_model, TRACKING_READ_MODEL_SURFACE_CHILD_CHECK_IN).active_rows,
        1
    );
    assert_eq!(
        coverage(read_model, TRACKING_READ_MODEL_SURFACE_RETENTION).tombstone_rows,
        1
    );
    assert!(!read_model.product_claim_state.physical_device_claimed);
    assert!(!read_model.product_claim_state.product_complete_claimed);
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
    assert!(read_model.deleted_evidence_reference_ids.is_empty());
    assert_eq!(
        read_model.capability_status,
        TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS
    );
    assert_eq!(read_model.coverage_rows.len(), 5);
    assert!(read_model
        .coverage_rows
        .iter()
        .all(|coverage| !coverage.ready_for_product_claim));
}

fn tracking_activity_events() -> Vec<ActivityEvent> {
    vec![
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
            constants::activity_store::TEST_TRACKING_CHECK_IN_EVENT_ID,
            constants::activity_store::TEST_TRACKING_CHECK_IN_OBSERVED_AT,
            ActivityEventKind::TrackingChildCheckInResponded,
            ActivityObserver::TrackingEngine,
            ActivitySubjectKind::CheckIn,
        ),
        tracking_activity_event(
            constants::activity_store::TEST_TRACKING_RETENTION_DELETE_EVENT_ID,
            constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT,
            ActivityEventKind::TrackingRetentionDeleted,
            ActivityObserver::TrackingEngine,
            ActivitySubjectKind::Retention,
        ),
    ]
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

fn coverage<'a>(
    read_model: &'a TrackingReadModel,
    surface: &str,
) -> &'a ocentra_parent_agent_protocol::TrackingReadModelCoverageRow {
    read_model
        .coverage_rows
        .iter()
        .find(|coverage| coverage.surface == surface)
        .expect(constants::error::ACTIVITY_STORE_QUERIES)
}
