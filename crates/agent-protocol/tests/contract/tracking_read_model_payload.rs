use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::read_model::{
    TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    TRACKING_READ_MODEL_FIELD_ACTIVE_DEVICE_COUNTS, TRACKING_READ_MODEL_FIELD_ACTIVE_KIND_COUNTS,
    TRACKING_READ_MODEL_FIELD_ACTIVE_ROWS, TRACKING_READ_MODEL_FIELD_LATEST_ACTIVE_EVENT_ID,
    TRACKING_READ_MODEL_FIELD_TOMBSTONE_ROWS, TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE,
};
use ocentra_parent_agent_protocol::tracking::read_model_payload::tracking_read_model_payload;
use ocentra_parent_agent_protocol::LogFieldValue;
use ocentra_parent_agent_protocol::LogFields;
use ocentra_parent_agent_protocol::TrackingEvidenceRef;
use ocentra_parent_agent_protocol::TrackingReadModel;
use ocentra_parent_agent_protocol::TrackingReadModelCapabilityStatus;
use ocentra_parent_agent_protocol::TrackingReadModelCount;
use ocentra_parent_agent_protocol::TrackingReadModelCountValue;
use ocentra_parent_agent_protocol::TrackingReadModelCustodyLabel;
use ocentra_parent_agent_protocol::TrackingReadModelDeviceId;
use ocentra_parent_agent_protocol::TrackingReadModelEventId;
use ocentra_parent_agent_protocol::TrackingReadModelGeneratedAt;
use ocentra_parent_agent_protocol::TrackingReadModelKind;
use ocentra_parent_agent_protocol::TrackingReadModelObservedAt;
use ocentra_parent_agent_protocol::TrackingReadModelObserver;
use ocentra_parent_agent_protocol::TrackingReadModelPlatform;
use ocentra_parent_agent_protocol::TrackingReadModelQueryVisibility;
use ocentra_parent_agent_protocol::TrackingReadModelRow;
use ocentra_parent_agent_protocol::TrackingReadModelSubjectDisplayName;
use ocentra_parent_agent_protocol::TrackingReadModelSubjectId;
use ocentra_parent_agent_protocol::TrackingReadModelSubjectKind;
use ocentra_parent_agent_protocol::ACTIVITY_QUERY_SCHEMA_VERSION;
use std::error::Error;
use std::io;

macro_rules! tracking_payload_string {
    ($payload:expr, $field:expr $(,)?) => {{
        match payload_value($payload, $field) {
            Some(LogFieldValue::String(text)) => Ok(text.as_str()),
            _ => Err(io::Error::other("tracking payload missing string field")),
        }
    }};
}

macro_rules! tracking_parse {
    ($ty:ty, $value:expr $(,)?) => {
        <$ty>::parse($value)?
    };
}

#[test]
fn tracking_read_model_payload_contains_contract_json_and_latest_citations(
) -> Result<(), Box<dyn Error>> {
    let payload = tracking_read_model_payload(&tracking_read_model_fixture()?);
    let read_model_json = tracking_payload_string!(
        &payload,
        TrackingReadModelPayloadField::ActivityTrackingReadModel
    )?;
    let decoded: TrackingReadModel = serde_json::from_str(read_model_json)?;

    assert_eq!(decoded.returned, 1);
    assert_latest_payload_fields(&payload)?;
    assert_active_count_payloads(&payload)?;

    Ok(())
}

fn tracking_read_model_fixture() -> Result<TrackingReadModel, Box<dyn Error>> {
    Ok(TrackingReadModel {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        generated_at: tracking_parse!(
            TrackingReadModelGeneratedAt,
            constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT,
        ),
        custody_label: tracking_parse!(
            TrackingReadModelCustodyLabel,
            TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE,
        ),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: 1,
        active_rows: 1,
        tombstone_rows: 0,
        capability_status: tracking_parse!(
            TrackingReadModelCapabilityStatus,
            constants::activity_store::TEST_TRACKING_CAPABILITY_STATUS_RECENT,
        ),
        latest_event_id: Some(tracking_parse!(
            TrackingReadModelEventId,
            constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID,
        )),
        latest_observed_at: Some(tracking_parse!(
            TrackingReadModelObservedAt,
            constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT,
        )),
        latest_active_event_id: Some(tracking_parse!(
            TrackingReadModelEventId,
            constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID,
        )),
        latest_active_observed_at: Some(tracking_parse!(
            TrackingReadModelObservedAt,
            constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT,
        )),
        latest_tombstone_event_id: None,
        latest_tombstone_observed_at: None,
        active_kind_counts: vec![TrackingReadModelCount {
            value: tracking_parse!(
                TrackingReadModelCountValue,
                constants::activity_event_kind::LOCATION_OBSERVED,
            ),
            count: 1,
        }],
        active_device_counts: vec![TrackingReadModelCount {
            value: tracking_parse!(
                TrackingReadModelCountValue,
                constants::activity_store::TEST_REMOTE_DEVICE_ID,
            ),
            count: 1,
        }],
        active_capability_status_counts: vec![TrackingReadModelCount {
            value: tracking_parse!(
                TrackingReadModelCountValue,
                constants::activity_store::TEST_TRACKING_CAPABILITY_STATUS_RECENT,
            ),
            count: 1,
        }],
        deleted_evidence_reference_ids: Vec::new(),
        rows: vec![tracking_row()?],
    })
}

fn assert_latest_payload_fields(payload: &LogFields) -> Result<(), Box<dyn Error>> {
    assert_eq!(
        tracking_payload_string!(payload, TrackingReadModelPayloadField::EvidenceReferenceIds)?,
        constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID
    );
    assert_eq!(
        tracking_payload_string!(payload, TrackingReadModelPayloadField::LatestEventId)?,
        constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID
    );
    assert_eq!(
        tracking_payload_string!(payload, TrackingReadModelPayloadField::LatestActiveEventId)?,
        constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID
    );
    assert_eq!(
        payload_number(payload, TrackingReadModelPayloadField::ActiveRows)?,
        1.0
    );
    assert_eq!(
        payload_number(payload, TrackingReadModelPayloadField::TombstoneRows)?,
        0.0
    );
    assert_eq!(
        tracking_payload_string!(payload, TrackingReadModelPayloadField::QueryVisibility)?,
        TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE
    );

    Ok(())
}

fn assert_active_count_payloads(payload: &LogFields) -> Result<(), Box<dyn Error>> {
    let kind_counts: Vec<TrackingReadModelCount> =
        payload_counts(payload, TrackingReadModelPayloadField::ActiveKindCounts)?;
    let device_counts: Vec<TrackingReadModelCount> =
        payload_counts(payload, TrackingReadModelPayloadField::ActiveDeviceCounts)?;
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

    Ok(())
}

fn tracking_row() -> Result<TrackingReadModelRow, Box<dyn Error>> {
    Ok(TrackingReadModelRow {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        event_id: tracking_parse!(
            TrackingReadModelEventId,
            constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID,
        ),
        observed_at: tracking_parse!(
            TrackingReadModelObservedAt,
            constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT,
        ),
        device_id: tracking_parse!(
            TrackingReadModelDeviceId,
            constants::activity_store::TEST_REMOTE_DEVICE_ID,
        ),
        platform: tracking_parse!(
            TrackingReadModelPlatform,
            constants::activity_store::TEST_TRACKING_PLATFORM_ANDROID,
        ),
        observer: tracking_parse!(
            TrackingReadModelObserver,
            constants::activity_observer::ANDROID_LOCATION,
        ),
        kind: tracking_parse!(
            TrackingReadModelKind,
            constants::activity_event_kind::LOCATION_OBSERVED,
        ),
        subject_kind: tracking_parse!(
            TrackingReadModelSubjectKind,
            constants::activity_subject_kind::LOCATION,
        ),
        subject_id: tracking_parse!(
            TrackingReadModelSubjectId,
            constants::activity_store::TEST_TRACKING_SUBJECT_ID,
        ),
        subject_display_name: Some(tracking_parse!(
            TrackingReadModelSubjectDisplayName,
            constants::activity_store::TEST_TRACKING_SUBJECT_NAME,
        )),
        capability_status: Some(tracking_parse!(
            TrackingReadModelCapabilityStatus,
            constants::activity_store::TEST_TRACKING_CAPABILITY_STATUS_RECENT,
        )),
        query_visibility: tracking_parse!(
            TrackingReadModelQueryVisibility,
            TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE,
        ),
        deleted_at: None,
        evidence_reference_ids: vec![tracking_parse!(
            TrackingEvidenceRef,
            constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID,
        )],
        deleted_evidence_reference_ids: Vec::new(),
        evidence: Vec::new(),
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TrackingReadModelPayloadField {
    ActivityTrackingReadModel,
    EvidenceReferenceIds,
    LatestEventId,
    LatestActiveEventId,
    ActiveRows,
    TombstoneRows,
    QueryVisibility,
    ActiveKindCounts,
    ActiveDeviceCounts,
}

impl TrackingReadModelPayloadField {
    const KEYS: [&'static str; 9] = [
        constants::field::ACTIVITY_TRACKING_READ_MODEL,
        constants::field::EVIDENCE_REFERENCE_IDS,
        constants::field::LATEST_EVENT_ID,
        TRACKING_READ_MODEL_FIELD_LATEST_ACTIVE_EVENT_ID,
        TRACKING_READ_MODEL_FIELD_ACTIVE_ROWS,
        TRACKING_READ_MODEL_FIELD_TOMBSTONE_ROWS,
        constants::field::QUERY_VISIBILITY,
        TRACKING_READ_MODEL_FIELD_ACTIVE_KIND_COUNTS,
        TRACKING_READ_MODEL_FIELD_ACTIVE_DEVICE_COUNTS,
    ];
}

fn payload_counts(
    payload: &LogFields,
    field: TrackingReadModelPayloadField,
) -> Result<Vec<TrackingReadModelCount>, Box<dyn Error>> {
    Ok(serde_json::from_str(tracking_payload_string!(
        payload, field
    )?)?)
}

fn payload_number(
    payload: &LogFields,
    field: TrackingReadModelPayloadField,
) -> Result<f64, Box<dyn Error>> {
    match payload_value(payload, field) {
        Some(LogFieldValue::Number(number)) => Ok(*number),
        _ => Err(io::Error::other("tracking payload missing numeric field").into()),
    }
}

fn payload_value(
    payload: &LogFields,
    field: TrackingReadModelPayloadField,
) -> Option<&LogFieldValue> {
    let key_ref = TrackingReadModelPayloadField::KEYS[field as usize];
    payload.get(key_ref)
}
