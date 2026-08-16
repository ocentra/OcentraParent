use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    TrackingEvidenceRef, TrackingReadModelCapabilityStatus, TrackingReadModelCountValue,
    TrackingReadModelCustodyLabel, TrackingReadModelDeviceId, TrackingReadModelEventId,
    TrackingReadModelGeneratedAt, TrackingReadModelKind, TrackingReadModelObservedAt,
    TrackingReadModelObserver, TrackingReadModelPlatform, TrackingReadModelQueryVisibility,
    TrackingReadModelSubjectDisplayName, TrackingReadModelSubjectId, TrackingReadModelSubjectKind,
};
use ocentra_parent_agent_protocol::tracking::read_model::{
    TrackingReadModel, TrackingReadModelCount, TrackingReadModelRow,
    TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE, TRACKING_READ_MODEL_SCHEMA_VERSION,
    TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS,
};
use std::error::Error;

macro_rules! tracking_parse {
    ($ty:ty, $value:expr $(,)?) => {
        <$ty>::parse($value)?
    };
}

macro_rules! tracking_row {
    ($event_id_value:expr, $kind_value:expr $(,)?) => {{
        TrackingReadModelRow {
            schema_version: TRACKING_READ_MODEL_SCHEMA_VERSION,
            event_id: tracking_parse!(TrackingReadModelEventId, $event_id_value),
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
                constants::activity_observer::TRACKING_ENGINE,
            ),
            kind: tracking_parse!(TrackingReadModelKind, $kind_value),
            subject_kind: tracking_parse!(
                TrackingReadModelSubjectKind,
                constants::activity_subject_kind::TRACKING_RULE,
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
        }
    }};
}

#[test]
fn tracking_read_model_serializes_without_product_completion_claims() -> Result<(), Box<dyn Error>>
{
    let read_model = TrackingReadModel {
        schema_version: TRACKING_READ_MODEL_SCHEMA_VERSION,
        generated_at: tracking_parse!(
            TrackingReadModelGeneratedAt,
            constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT,
        ),
        custody_label: tracking_parse!(
            TrackingReadModelCustodyLabel,
            TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE,
        ),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: 0,
        active_rows: 0,
        tombstone_rows: 0,
        capability_status: tracking_parse!(
            TrackingReadModelCapabilityStatus,
            TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS,
        ),
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

    let serialized = serde_json::to_value(read_model)?;

    assert_eq!(
        serialized["schemaVersion"],
        TRACKING_READ_MODEL_SCHEMA_VERSION
    );
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

    Ok(())
}

#[test]
fn tracking_read_model_serializes_active_product_surface_counts() -> Result<(), Box<dyn Error>> {
    let read_model = TrackingReadModel {
        schema_version: TRACKING_READ_MODEL_SCHEMA_VERSION,
        generated_at: tracking_parse!(
            TrackingReadModelGeneratedAt,
            constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT,
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
        rows: Vec::new(),
    };

    let serialized = serde_json::to_value(read_model)?;

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

    Ok(())
}

#[test]
fn tracking_read_model_row_serializes_journal_citation_ids_and_visibility(
) -> Result<(), Box<dyn Error>> {
    let row = TrackingReadModelRow {
        schema_version: TRACKING_READ_MODEL_SCHEMA_VERSION,
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
    };

    let serialized = serde_json::to_value(row)?;

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

    Ok(())
}

#[test]
fn tracking_read_model_row_serializes_tracking_alert_and_parent_notification_kinds(
) -> Result<(), Box<dyn Error>> {
    let alert_row = tracking_row!(
        constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID,
        constants::activity_event_kind::TRACKING_ALERT_EVALUATED,
    );
    let notification_row = tracking_row!(
        constants::activity_store::TEST_TRACKING_GEOFENCE_EVENT_ID,
        constants::activity_event_kind::TRACKING_PARENT_NOTIFICATION_REQUESTED,
    );

    let serialized = serde_json::to_value(vec![alert_row, notification_row])?;

    assert_eq!(
        serialized[0]["kind"],
        constants::activity_event_kind::TRACKING_ALERT_EVALUATED
    );
    assert_eq!(
        serialized[1]["kind"],
        constants::activity_event_kind::TRACKING_PARENT_NOTIFICATION_REQUESTED
    );

    Ok(())
}
