use super::{
    constants, AppGamePolicyReadinessReadModel, AppGamePolicyReadinessRow,
    APP_GAME_POLICY_READINESS_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    APP_GAME_POLICY_READINESS_KIND_CATEGORY_CANDIDATE, APP_GAME_POLICY_READINESS_STATE_READY,
    APP_GAME_POLICY_READINESS_STATUS_PARTIAL, APP_GAME_SCHEMA_VERSION,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn app_game_policy_readiness_read_model_serializes_no_adapter_claim() {
    let read_model = AppGamePolicyReadinessReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT
            .to_string(),
        custody_label: APP_GAME_POLICY_READINESS_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        capability_status: APP_GAME_POLICY_READINESS_STATUS_PARTIAL.to_string(),
        returned: 1,
        policy_evaluation_ready: false,
        category_routing_ready: true,
        unknown_review_required: false,
        manual_review_required: true,
        adapter_dispatch_claimed: false,
        evidence_claim_row_count: 1,
        identity_row_count: 0,
        approval_authority_row_count: 0,
        approval_action_result_row_count: 0,
        platform_authority_row_count: 0,
        ai_classifier_result_row_count: 0,
        category_candidate_row_count: 1,
        unknown_review_row_count: 0,
        rows: vec![AppGamePolicyReadinessRow {
            schema_version: APP_GAME_SCHEMA_VERSION,
            row_id: APP_GAME_POLICY_READINESS_KIND_CATEGORY_CANDIDATE.to_string(),
            readiness_kind: APP_GAME_POLICY_READINESS_KIND_CATEGORY_CANDIDATE.to_string(),
            readiness_state: APP_GAME_POLICY_READINESS_STATE_READY.to_string(),
            row_count: 1,
            evidence_reference_ids: vec![
                constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID.to_string(),
            ],
            evidence: Vec::new(),
        }],
    };

    let serialized =
        serde_json::to_value(read_model).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], APP_GAME_SCHEMA_VERSION);
    assert_eq!(
        serialized["custodyLabel"],
        APP_GAME_POLICY_READINESS_CUSTODY_CHILD_DEVICE_QUERY_STORE
    );
    assert_eq!(
        serialized["capabilityStatus"],
        APP_GAME_POLICY_READINESS_STATUS_PARTIAL
    );
    assert_eq!(serialized["policyEvaluationReady"], false);
    assert_eq!(serialized["categoryRoutingReady"], true);
    assert_eq!(serialized["unknownReviewRequired"], false);
    assert_eq!(serialized["adapterDispatchClaimed"], false);
    assert_eq!(serialized["categoryCandidateRowCount"], 1);
    assert_eq!(serialized["unknownReviewRowCount"], 0);
    assert_eq!(
        serialized["rows"][0]["readinessKind"],
        APP_GAME_POLICY_READINESS_KIND_CATEGORY_CANDIDATE
    );
}
