use crate::{
    AppGameAdapterDispatchResultReadModel, AppGameAdapterDispatchResultRow,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_AUDIT_OWNED_PROCESS,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_ACCEPTED,
    APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_ACCEPTED,
    APP_GAME_ADAPTER_DISPATCH_DECISION_ELIGIBLE,
    APP_GAME_ADAPTER_DISPATCH_INTENT_OWNED_PROCESS_TIME_LIMIT,
    APP_GAME_ADAPTER_DISPATCH_OUTCOME_READY, APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL_ID,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_ROW_ID_PREFIX,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_ELIGIBLE,
    APP_GAME_ADAPTER_DISPATCH_RESULT_CLAIM_SCOPED_TIMER,
    APP_GAME_ADAPTER_DISPATCH_RESULT_CUSTODY_PREFLIGHT_AND_COMMAND,
    APP_GAME_ADAPTER_DISPATCH_RESULT_FALLBACK_SCOPED_TIMER,
    APP_GAME_ADAPTER_DISPATCH_RESULT_OWNED_PROCESS_ID,
    APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_ID, APP_GAME_ADAPTER_DISPATCH_RESULT_ROW_ID_PREFIX,
    APP_GAME_ADAPTER_DISPATCH_RESULT_STATUS_PARTIAL, APP_GAME_ADAPTER_DISPATCH_TIMER_OWNED_PROCESS,
    APP_GAME_ADAPTER_PRODUCT_NATIVE_APP, APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME,
    APP_GAME_PARENT_PLATFORM_WINDOWS, APP_GAME_SCHEMA_VERSION,
};

const GENERATED_AT: &str = "2026-06-08T10:44:00Z";
const SOURCE_PROOF_ENTRY_ID: &str = "windows-app-game-owned-process-time-limit";
const ADAPTER_CAPABILITY: &str = "app-game-owned-process-time-limit";

#[test]
fn app_game_adapter_dispatch_result_serializes_parent_safe_command_result_rows() {
    let row = AppGameAdapterDispatchResultRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id: format!("{APP_GAME_ADAPTER_DISPATCH_RESULT_ROW_ID_PREFIX}{SOURCE_PROOF_ENTRY_ID}"),
        source_dispatch_preflight_row_id: format!(
            "{APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_ROW_ID_PREFIX}{SOURCE_PROOF_ENTRY_ID}"
        ),
        source_proof_entry_id: SOURCE_PROOF_ENTRY_ID.to_string(),
        platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
        product_meanings: vec![
            APP_GAME_ADAPTER_PRODUCT_NATIVE_APP.to_string(),
            APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME.to_string(),
        ],
        adapter_capability: ADAPTER_CAPABILITY.to_string(),
        dispatch_preflight_state: APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_ELIGIBLE.to_string(),
        dispatch_decision: APP_GAME_ADAPTER_DISPATCH_DECISION_ELIGIBLE.to_string(),
        dispatch_intent_id: Some(
            APP_GAME_ADAPTER_DISPATCH_INTENT_OWNED_PROCESS_TIME_LIMIT.to_string(),
        ),
        dispatch_outcome_state: APP_GAME_ADAPTER_DISPATCH_OUTCOME_READY.to_string(),
        dispatch_command_result_state: APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_ACCEPTED
            .to_string(),
        dispatch_command_result_decision:
            APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_ACCEPTED.to_string(),
        enforcement_command_name: Some("agent.enforcement.execute".to_string()),
        enforcement_event_name: Some("agent.enforcement.audit.reported".to_string()),
        enforcement_action_mode: Some("terminate-process".to_string()),
        dispatch_command_result_id: Some(
            APP_GAME_ADAPTER_DISPATCH_RESULT_OWNED_PROCESS_ID.to_string(),
        ),
        dispatch_command_audit_refs: vec![
            APP_GAME_ADAPTER_DISPATCH_COMMAND_AUDIT_OWNED_PROCESS.to_string()
        ],
        dispatch_command_timer_refs: vec![APP_GAME_ADAPTER_DISPATCH_TIMER_OWNED_PROCESS.to_string()],
        manual_proof_requirements: vec![],
        claim_boundary: APP_GAME_ADAPTER_DISPATCH_RESULT_CLAIM_SCOPED_TIMER.to_string(),
        fallback_behavior: APP_GAME_ADAPTER_DISPATCH_RESULT_FALLBACK_SCOPED_TIMER.to_string(),
        adapter_dispatch_command_result_claimed: true,
        adapter_dispatch_executed_claimed: false,
        broad_installed_app_blocking_claimed: false,
        child_device_delivery_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        private_diagnostics_claimed: false,
        last_checked_at: GENERATED_AT.to_string(),
    };
    let read_model = AppGameAdapterDispatchResultReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        read_model_id: APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_ID.to_string(),
        generated_at: GENERATED_AT.to_string(),
        source_read_model_ids: vec![
            APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL_ID.to_string(),
            "agent.enforcement.execute".to_string(),
        ],
        custody_label: APP_GAME_ADAPTER_DISPATCH_RESULT_CUSTODY_PREFLIGHT_AND_COMMAND.to_string(),
        capability_status: APP_GAME_ADAPTER_DISPATCH_RESULT_STATUS_PARTIAL.to_string(),
        returned: 1,
        command_accepted_count: 1,
        blocked_before_command_count: 0,
        adapter_dispatch_command_result_claimed_count: 1,
        adapter_dispatch_executed_claimed_count: 0,
        broad_installed_app_blocking_claimed: false,
        child_device_delivery_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        private_diagnostics_claimed: false,
        rows: vec![row],
    };

    let serialized = serde_json::to_string(&read_model).expect("dispatch result serializes");

    assert!(serialized.contains(APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_ID));
    assert!(serialized.contains("agent.enforcement.execute"));
    assert!(serialized.contains("\"adapterDispatchExecutedClaimed\":false"));
}
