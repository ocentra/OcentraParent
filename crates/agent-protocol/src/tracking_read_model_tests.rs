use super::{
    constants, TrackingReadModel, TrackingReadModelCoverageRow, TrackingReadModelProductClaimState,
    TrackingReadModelRow, ACTIVITY_QUERY_SCHEMA_VERSION,
    TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    TRACKING_READ_MODEL_MISSING_PROOF_PLATFORM_REPLAY, TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE,
    TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS, TRACKING_READ_MODEL_SURFACE_LOCATION,
};

#[test]
fn tracking_read_model_serializes_without_product_completion_claims() {
    let read_model = TrackingReadModel {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT
            .to_string(),
        custody_label: TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: 0,
        active_rows: 0,
        tombstone_rows: 0,
        capability_status: TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS.to_string(),
        latest_event_id: None,
        latest_observed_at: None,
        latest_tombstone_event_id: None,
        latest_tombstone_observed_at: None,
        deleted_evidence_reference_ids: Vec::new(),
        coverage_rows: vec![coverage_row()],
        product_claim_state: product_claim_state(),
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
        serialized["coverageRows"][0]["surface"],
        TRACKING_READ_MODEL_SURFACE_LOCATION
    );
    assert_eq!(serialized["coverageRows"][0]["readyForProductClaim"], false);
    assert_eq!(
        serialized["productClaimState"]["productCompleteClaimed"],
        false
    );
}

#[test]
fn tracking_read_model_row_serializes_journal_citation_ids_and_visibility() {
    let row = TrackingReadModelRow {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        event_id: constants::activity_store::TEST_TRACKING_LOCATION_EVENT_ID.to_string(),
        observed_at: constants::activity_store::TEST_TRACKING_LOCATION_OBSERVED_AT.to_string(),
        device_id: constants::activity_store::TEST_REMOTE_DEVICE_ID.to_string(),
        platform: constants::activity_store::TEST_TRACKING_PLATFORM_ANDROID.to_string(),
        observer: constants::activity_observer::ANDROID_LOCATION.to_string(),
        kind: constants::activity_event_kind::LOCATION_OBSERVED.to_string(),
        subject_kind: constants::activity_subject_kind::LOCATION.to_string(),
        subject_id: constants::activity_store::TEST_TRACKING_SUBJECT_ID.to_string(),
        subject_display_name: Some(
            constants::activity_store::TEST_TRACKING_SUBJECT_NAME.to_string(),
        ),
        capability_status: Some(
            constants::activity_store::TEST_TRACKING_CAPABILITY_STATUS_RECENT.to_string(),
        ),
        query_visibility: TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE.to_string(),
        deleted_at: None,
        evidence_reference_ids: vec![
            constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID.to_string(),
        ],
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

fn coverage_row() -> TrackingReadModelCoverageRow {
    TrackingReadModelCoverageRow {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        surface: TRACKING_READ_MODEL_SURFACE_LOCATION.to_string(),
        active_rows: 0,
        tombstone_rows: 0,
        citation_count: 0,
        latest_event_id: None,
        latest_observed_at: None,
        ready_for_product_claim: false,
        missing_proof: TRACKING_READ_MODEL_MISSING_PROOF_PLATFORM_REPLAY.to_string(),
    }
}

fn product_claim_state() -> TrackingReadModelProductClaimState {
    TrackingReadModelProductClaimState {
        physical_device_claimed: false,
        provider_delivery_claimed: false,
        notification_delivery_claimed: false,
        child_device_runtime_claimed: false,
        ocentra_hosted_storage_claimed: false,
        product_complete_claimed: false,
    }
}
