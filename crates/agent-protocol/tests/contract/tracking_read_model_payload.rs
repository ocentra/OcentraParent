use ocentra_parent_agent_protocol::{
    constants, tracking_read_model_payload, LogFieldValue, LogFields, TrackingEvidenceRef,
    TrackingReadModel, TrackingReadModelCapabilityStatus, TrackingReadModelCount,
    TrackingReadModelCountValue, TrackingReadModelCustodyLabel, TrackingReadModelDeviceId,
    TrackingReadModelEventId, TrackingReadModelGeneratedAt, TrackingReadModelKind,
    TrackingReadModelObservedAt, TrackingReadModelObserver, TrackingReadModelPlatform,
    TrackingReadModelQueryVisibility, TrackingReadModelRow, TrackingReadModelSubjectDisplayName,
    TrackingReadModelSubjectId, TrackingReadModelSubjectKind, ACTIVITY_QUERY_SCHEMA_VERSION,
    TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    TRACKING_READ_MODEL_FIELD_ACTIVE_DEVICE_COUNTS, TRACKING_READ_MODEL_FIELD_ACTIVE_KIND_COUNTS,
    TRACKING_READ_MODEL_FIELD_ACTIVE_ROWS, TRACKING_READ_MODEL_FIELD_LATEST_ACTIVE_EVENT_ID,
    TRACKING_READ_MODEL_FIELD_TOMBSTONE_ROWS, TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE,
};

#[test]
fn tracking_read_model_payload_contains_contract_json_and_latest_citations() {
    let payload = tracking_read_model_payload(&tracking_read_model_fixture());
    let read_model_json = string_payload(&payload, constants::field::ACTIVITY_TRACKING_READ_MODEL);
    let decoded: TrackingReadModel =
        serde_json::from_str(read_model_json).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(decoded.returned, 1);
    assert_latest_payload_fields(&payload);
    assert_active_count_payloads(&payload);
}

fn tracking_read_model_fixture() -> TrackingReadModel {
    TrackingReadModel {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        generated_at: generated_at(
            constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT,
        ),
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
        rows: vec![tracking_row()],
    }
}

fn assert_latest_payload_fields(payload: &LogFields) {
    assert_eq!(
        string_payload(payload, constants::field::EVIDENCE_REFERENCE_IDS),
        constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID
    );
    assert_eq!(
        string_payload(payload, constants::field::LATEST_EVENT_ID),
        constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID
    );
    assert_eq!(
        string_payload(payload, TRACKING_READ_MODEL_FIELD_LATEST_ACTIVE_EVENT_ID),
        constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID
    );
    assert_eq!(
        number_payload(payload, TRACKING_READ_MODEL_FIELD_ACTIVE_ROWS),
        1.0
    );
    assert_eq!(
        number_payload(payload, TRACKING_READ_MODEL_FIELD_TOMBSTONE_ROWS),
        0.0
    );
    assert_eq!(
        string_payload(payload, constants::field::QUERY_VISIBILITY),
        TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE
    );
}

fn assert_active_count_payloads(payload: &LogFields) {
    let kind_counts = count_payload(payload, TRACKING_READ_MODEL_FIELD_ACTIVE_KIND_COUNTS);
    let device_counts = count_payload(payload, TRACKING_READ_MODEL_FIELD_ACTIVE_DEVICE_COUNTS);
    assert_eq!(
        kind_counts[0].value,
        constants::activity_event_kind::LOCATION_OBSERVED
    );
    assert_eq!(kind_counts[0].count, 1);
    assert_eq!(
        device_counts[0].value,
        constants::activity_store::TEST_REMOTE_DEVICE_ID
    );
    assert_eq!(device_counts[0].count, 1);
}

fn tracking_row() -> TrackingReadModelRow {
    TrackingReadModelRow {
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

fn string_payload<'a>(payload: &'a LogFields, key: &str) -> &'a str {
    match payload.get(key) {
        Some(LogFieldValue::String(value)) => value.as_str(),
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}

fn number_payload(payload: &LogFields, key: &str) -> f64 {
    match payload.get(key) {
        Some(LogFieldValue::Number(value)) => *value,
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}

fn count_payload(payload: &LogFields, key: &str) -> Vec<TrackingReadModelCount> {
    serde_json::from_str(string_payload(payload, key))
        .expect(constants::error::AGENT_EVENT_SERIALIZES)
}
