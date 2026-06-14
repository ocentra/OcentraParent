use ocentra_parent_agent_protocol::{constants, ACTIVITY_QUERY_SCHEMA_VERSION};

use ocentra_parent_agent_protocol::{
    TrackingEvidenceRef, TrackingReadModel, TrackingReadModelCapabilityStatus,
    TrackingReadModelCount, TrackingReadModelCountValue, TrackingReadModelCustodyLabel,
    TrackingReadModelDeviceId, TrackingReadModelEventId, TrackingReadModelGeneratedAt,
    TrackingReadModelKind, TrackingReadModelObservedAt, TrackingReadModelObserver,
    TrackingReadModelPlatform, TrackingReadModelQueryVisibility, TrackingReadModelRow,
    TrackingReadModelSubjectDisplayName, TrackingReadModelSubjectId, TrackingReadModelSubjectKind,
    TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE, TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS,
};

#[test]
fn tracking_read_model_serializes_without_product_completion_claims() {
    let read_model = TrackingReadModel {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        generated_at: generated_at(
            constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT,
        ),
        custody_label: custody_label(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: 0,
        active_rows: 0,
        tombstone_rows: 0,
        capability_status: capability_status(TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS),
        latest_event_id: None,
        latest_observed_at: None,
        latest_active_event_id: None,
        latest_active_observed_at: None,
        latest_tombstone_event_id: None,
        latest_tombstone_observed_at: None,
        active_kind_counts: Vec::new(),
        active_device_counts: Vec::new(),
        active_capability_status_counts: Vec::new(),
        deleted_evidence_reference_ids: Vec::new(),
        rows: Vec::new(),
    };

    let serialized =
        serde_json::to_value(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], ACTIVITY_QUERY_SCHEMA_VERSION);
    assert_eq!(
        serialized["custodyLabel"],
        TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE
    );
    assert_eq!(
        serialized["capabilityStatus"],
        TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS
    );
    assert_eq!(serialized["rows"].as_array().map(Vec::len), Some(0));
    assert_eq!(serialized["activeRows"], 0);
    assert_eq!(serialized["tombstoneRows"], 0);
    assert_eq!(
        serialized["activeKindCounts"].as_array().map(Vec::len),
        Some(0)
    );
    assert_eq!(
        serialized["activeDeviceCounts"].as_array().map(Vec::len),
        Some(0)
    );
    assert_eq!(
        serialized["activeCapabilityStatusCounts"]
            .as_array()
            .map(Vec::len),
        Some(0)
    );
}

#[test]
fn tracking_read_model_serializes_active_product_surface_counts() {
    let read_model = TrackingReadModel {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        generated_at: generated_at(constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT),
        custody_label: custody_label(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: 1,
        active_rows: 1,
        tombstone_rows: 0,
        capability_status: capability_status(
            constants::activity_store::TEST_TRACKING_CAPABILITY_STATUS_RECENT,
        ),
        latest_event_id: Some(event_id(
            constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID,
        )),
        latest_observed_at: Some(observed_at(
            constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT,
        )),
        latest_active_event_id: Some(event_id(
            constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID,
        )),
        latest_active_observed_at: Some(observed_at(
            constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT,
        )),
        latest_tombstone_event_id: None,
        latest_tombstone_observed_at: None,
        active_kind_counts: vec![TrackingReadModelCount {
            value: count_value(constants::activity_event_kind::LOCATION_OBSERVED),
            count: 1,
        }],
        active_device_counts: vec![TrackingReadModelCount {
            value: count_value(constants::activity_store::TEST_REMOTE_DEVICE_ID),
            count: 1,
        }],
        active_capability_status_counts: vec![TrackingReadModelCount {
            value: count_value(constants::activity_store::TEST_TRACKING_CAPABILITY_STATUS_RECENT),
            count: 1,
        }],
        deleted_evidence_reference_ids: Vec::new(),
        rows: Vec::new(),
    };

    let serialized =
        serde_json::to_value(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["latestActiveEventId"],
        constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID
    );
    assert_eq!(
        serialized["activeKindCounts"][0]["value"],
        constants::activity_event_kind::LOCATION_OBSERVED
    );
    assert_eq!(serialized["activeKindCounts"][0]["count"], 1);
    assert_eq!(
        serialized["activeDeviceCounts"][0]["value"],
        constants::activity_store::TEST_REMOTE_DEVICE_ID
    );
    assert_eq!(
        serialized["activeCapabilityStatusCounts"][0]["value"],
        constants::activity_store::TEST_TRACKING_CAPABILITY_STATUS_RECENT
    );
}

#[test]
fn tracking_read_model_row_serializes_journal_citation_ids_and_visibility() {
    let row = TrackingReadModelRow {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        event_id: event_id(constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID),
        observed_at: observed_at(constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT),
        device_id: device_id(constants::activity_store::TEST_REMOTE_DEVICE_ID),
        platform: platform(constants::activity_store::TEST_TRACKING_PLATFORM_ANDROID),
        observer: observer(constants::activity_observer::ANDROID_LOCATION),
        kind: kind(constants::activity_event_kind::LOCATION_OBSERVED),
        subject_kind: subject_kind(constants::activity_subject_kind::LOCATION),
        subject_id: subject_id(constants::activity_store::TEST_TRACKING_SUBJECT_ID),
        subject_display_name: Some(subject_display_name(
            constants::activity_store::TEST_TRACKING_SUBJECT_NAME,
        )),
        capability_status: Some(capability_status(
            constants::activity_store::TEST_TRACKING_CAPABILITY_STATUS_RECENT,
        )),
        query_visibility: query_visibility(TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE),
        deleted_at: None,
        evidence_reference_ids: vec![evidence_ref(
            constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID,
        )],
        deleted_evidence_reference_ids: Vec::new(),
        evidence: Vec::new(),
    };

    let serialized = serde_json::to_value(row).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["evidenceReferenceIds"][0],
        constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID
    );
    assert_eq!(
        serialized["kind"],
        constants::activity_event_kind::LOCATION_OBSERVED
    );
    assert_eq!(
        serialized["queryVisibility"],
        TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE
    );
    assert!(serialized["deletedAt"].is_null());
}

#[test]
fn tracking_read_model_row_serializes_tracking_alert_and_parent_notification_kinds() {
    let alert_row = tracking_row(
        constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID,
        constants::activity_event_kind::TRACKING_ALERT_EVALUATED,
    );
    let notification_row = tracking_row(
        constants::activity_store::TEST_TRACKING_GEOFENCE_EVENT_ID,
        constants::activity_event_kind::TRACKING_PARENT_NOTIFICATION_REQUESTED,
    );

    let serialized = serde_json::to_value(vec![alert_row, notification_row])
        .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized[0]["kind"],
        constants::activity_event_kind::TRACKING_ALERT_EVALUATED
    );
    assert_eq!(
        serialized[1]["kind"],
        constants::activity_event_kind::TRACKING_PARENT_NOTIFICATION_REQUESTED
    );
}

fn tracking_row(event_id_value: &str, kind_value: &str) -> TrackingReadModelRow {
    TrackingReadModelRow {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        event_id: event_id(event_id_value),
        observed_at: observed_at(constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT),
        device_id: device_id(constants::activity_store::TEST_REMOTE_DEVICE_ID),
        platform: platform(constants::activity_store::TEST_TRACKING_PLATFORM_ANDROID),
        observer: observer(constants::activity_observer::TRACKING_ENGINE),
        kind: kind(kind_value),
        subject_kind: subject_kind(constants::activity_subject_kind::TRACKING_RULE),
        subject_id: subject_id(constants::activity_store::TEST_TRACKING_SUBJECT_ID),
        subject_display_name: Some(subject_display_name(
            constants::activity_store::TEST_TRACKING_SUBJECT_NAME,
        )),
        capability_status: Some(capability_status(
            constants::activity_store::TEST_TRACKING_CAPABILITY_STATUS_RECENT,
        )),
        query_visibility: query_visibility(TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE),
        deleted_at: None,
        evidence_reference_ids: vec![evidence_ref(
            constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID,
        )],
        deleted_evidence_reference_ids: Vec::new(),
        evidence: Vec::new(),
    }
}

fn generated_at(value: &str) -> TrackingReadModelGeneratedAt {
    TrackingReadModelGeneratedAt::parse(value).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn custody_label() -> TrackingReadModelCustodyLabel {
    TrackingReadModelCustodyLabel::parse(TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE)
        .expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn capability_status(value: &str) -> TrackingReadModelCapabilityStatus {
    TrackingReadModelCapabilityStatus::parse(value).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn event_id(value: &str) -> TrackingReadModelEventId {
    TrackingReadModelEventId::parse(value).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn observed_at(value: &str) -> TrackingReadModelObservedAt {
    TrackingReadModelObservedAt::parse(value).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn count_value(value: &str) -> TrackingReadModelCountValue {
    TrackingReadModelCountValue::parse(value).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn device_id(value: &str) -> TrackingReadModelDeviceId {
    TrackingReadModelDeviceId::parse(value).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn platform(value: &str) -> TrackingReadModelPlatform {
    TrackingReadModelPlatform::parse(value).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn observer(value: &str) -> TrackingReadModelObserver {
    TrackingReadModelObserver::parse(value).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn kind(value: &str) -> TrackingReadModelKind {
    TrackingReadModelKind::parse(value).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn subject_kind(value: &str) -> TrackingReadModelSubjectKind {
    TrackingReadModelSubjectKind::parse(value).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn subject_id(value: &str) -> TrackingReadModelSubjectId {
    TrackingReadModelSubjectId::parse(value).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn subject_display_name(value: &str) -> TrackingReadModelSubjectDisplayName {
    TrackingReadModelSubjectDisplayName::parse(value)
        .expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn query_visibility(value: &str) -> TrackingReadModelQueryVisibility {
    TrackingReadModelQueryVisibility::parse(value).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn evidence_ref(value: &str) -> TrackingEvidenceRef {
    TrackingEvidenceRef::parse(value).expect(constants::error::AGENT_EVENT_SERIALIZES)
}
