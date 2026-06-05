use super::{
    constants, AppGamePolicyEvaluationReadModel, AppGamePolicyEvaluationRow,
    APP_GAME_POLICY_EVALUATION_ADAPTER_NOT_DISPATCHED,
    APP_GAME_POLICY_EVALUATION_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    APP_GAME_POLICY_EVALUATION_DECISION_DRY_RUN_READY, APP_GAME_POLICY_EVALUATION_HANDOFF_DISABLED,
    APP_GAME_POLICY_EVALUATION_KIND_TIME_LIMIT,
    APP_GAME_POLICY_EVALUATION_POLICY_ACTION_TIME_LIMIT,
    APP_GAME_POLICY_EVALUATION_REASON_ADAPTER_DISPATCH_DISABLED,
    APP_GAME_POLICY_EVALUATION_REASON_READY, APP_GAME_POLICY_EVALUATION_REJECTION_NONE,
    APP_GAME_POLICY_EVALUATION_REQUESTED_ACTION_TIME_LIMIT,
    APP_GAME_POLICY_EVALUATION_STATUS_READY, APP_GAME_POLICY_READINESS_KIND_POLICY_EVIDENCE,
    APP_GAME_SCHEMA_VERSION,
};

#[test]
fn app_game_policy_evaluation_read_model_serializes_dry_run_without_adapter_claim() {
    let read_model = AppGamePolicyEvaluationReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT
            .to_string(),
        custody_label: APP_GAME_POLICY_EVALUATION_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        capability_status: APP_GAME_POLICY_EVALUATION_STATUS_READY.to_string(),
        returned: 1,
        policy_evaluation_ready: true,
        manual_review_required: false,
        dry_run: true,
        enforcement_handoff_state: APP_GAME_POLICY_EVALUATION_HANDOFF_DISABLED.to_string(),
        adapter_dispatch_claimed: false,
        readiness_row_count: 1,
        evaluated_row_count: 1,
        evidence_claim_row_count: 1,
        identity_row_count: 1,
        approval_authority_row_count: 1,
        approval_action_result_row_count: 0,
        platform_authority_row_count: 1,
        ai_classifier_result_row_count: 0,
        rows: vec![AppGamePolicyEvaluationRow {
            schema_version: APP_GAME_SCHEMA_VERSION,
            evaluation_id: APP_GAME_POLICY_EVALUATION_KIND_TIME_LIMIT.to_string(),
            evaluation_kind: APP_GAME_POLICY_EVALUATION_KIND_TIME_LIMIT.to_string(),
            requested_action: APP_GAME_POLICY_EVALUATION_REQUESTED_ACTION_TIME_LIMIT.to_string(),
            policy_action: APP_GAME_POLICY_EVALUATION_POLICY_ACTION_TIME_LIMIT.to_string(),
            decision_state: APP_GAME_POLICY_EVALUATION_DECISION_DRY_RUN_READY.to_string(),
            rejection_reason: APP_GAME_POLICY_EVALUATION_REJECTION_NONE.to_string(),
            reason_codes: vec![
                APP_GAME_POLICY_EVALUATION_REASON_READY.to_string(),
                APP_GAME_POLICY_EVALUATION_REASON_ADAPTER_DISPATCH_DISABLED.to_string(),
            ],
            required_readiness_kinds: vec![
                APP_GAME_POLICY_READINESS_KIND_POLICY_EVIDENCE.to_string()
            ],
            evidence_reference_ids: Vec::new(),
            evidence: Vec::new(),
            dry_run: true,
            enforcement_handoff_state: APP_GAME_POLICY_EVALUATION_HANDOFF_DISABLED.to_string(),
            adapter_dispatch_state: APP_GAME_POLICY_EVALUATION_ADAPTER_NOT_DISPATCHED.to_string(),
        }],
    };

    let serialized =
        serde_json::to_value(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], APP_GAME_SCHEMA_VERSION);
    assert_eq!(
        serialized["custodyLabel"],
        APP_GAME_POLICY_EVALUATION_CUSTODY_CHILD_DEVICE_QUERY_STORE
    );
    assert_eq!(serialized["dryRun"], true);
    assert_eq!(serialized["adapterDispatchClaimed"], false);
    assert_eq!(
        serialized["enforcementHandoffState"],
        APP_GAME_POLICY_EVALUATION_HANDOFF_DISABLED
    );
    assert_eq!(
        serialized["rows"][0]["adapterDispatchState"],
        APP_GAME_POLICY_EVALUATION_ADAPTER_NOT_DISPATCHED
    );
}
