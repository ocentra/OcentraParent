use super::{
    constants, AppGameBoundaryReadModel, AppGameBoundaryReadModelRow,
    APP_GAME_BOUNDARY_KIND_AI_CLASSIFIER_RESULT, APP_GAME_BOUNDARY_KIND_EVIDENCE_CLAIM,
    APP_GAME_BOUNDARY_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    APP_GAME_BOUNDARY_READ_MODEL_STATUS_NO_ROWS, APP_GAME_SCHEMA_VERSION,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn app_game_boundary_read_model_serializes_counts_without_runtime_claims() {
    let read_model = AppGameBoundaryReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT
            .to_string(),
        custody_label: APP_GAME_BOUNDARY_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        capability_status: APP_GAME_BOUNDARY_READ_MODEL_STATUS_NO_ROWS.to_string(),
        returned: 0,
        evidence_claim_row_count: 0,
        identity_row_count: 0,
        approval_authority_row_count: 0,
        approval_action_result_row_count: 0,
        platform_authority_matrix_count: 0,
        platform_authority_row_count: 0,
        ai_classifier_result_row_count: 0,
        rows: Vec::new(),
    };

    let serialized =
        serde_json::to_value(read_model).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], APP_GAME_SCHEMA_VERSION);
    assert_eq!(
        serialized["custodyLabel"],
        APP_GAME_BOUNDARY_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE
    );
    assert_eq!(
        serialized["capabilityStatus"],
        APP_GAME_BOUNDARY_READ_MODEL_STATUS_NO_ROWS
    );
    assert_eq!(serialized["returned"], 0);
}

#[test]
fn app_game_boundary_read_model_row_serializes_boundary_kind_and_citations() {
    let row = AppGameBoundaryReadModelRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id: APP_GAME_BOUNDARY_KIND_EVIDENCE_CLAIM.to_string(),
        boundary_kind: APP_GAME_BOUNDARY_KIND_AI_CLASSIFIER_RESULT.to_string(),
        row_count: 2,
        evidence_reference_ids: vec![
            constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID.to_string(),
        ],
        evidence: Vec::new(),
    };

    let serialized =
        serde_json::to_value(row).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["boundaryKind"],
        APP_GAME_BOUNDARY_KIND_AI_CLASSIFIER_RESULT
    );
    assert_eq!(serialized["rowCount"], 2);
    assert_eq!(
        serialized["evidenceReferenceIds"][0],
        constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID
    );
}
