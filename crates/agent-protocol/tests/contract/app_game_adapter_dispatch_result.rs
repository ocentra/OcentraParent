use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::AppGameAdapterDispatchResultReadModel;
use ocentra_parent_agent_protocol::AppGameAdapterDispatchResultRow;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_MISSING;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_STATE_MISSING;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_COMMAND_AUDIT_OWNED_PROCESS;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_ACCEPTED;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_ACCEPTED;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_DECISION_ELIGIBLE;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_RECORDED;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_ID;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_REF;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_STATE_RECORDED;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_INTENT_OWNED_PROCESS_TIME_LIMIT;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_OUTCOME_READY;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL_ID;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_ROW_ID_PREFIX;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_ELIGIBLE;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_RESULT_CLAIM_SCOPED_TIMER;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_RESULT_CUSTODY_PREFLIGHT_AND_COMMAND;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_RESULT_FALLBACK_SCOPED_TIMER;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_RESULT_OWNED_PROCESS_ID;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_ID;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_RESULT_ROW_ID_PREFIX;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_RESULT_STATUS_PARTIAL;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_DISPATCH_TIMER_OWNED_PROCESS;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_PRODUCT_NATIVE_APP;
use ocentra_parent_agent_protocol::APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME;
use ocentra_parent_agent_protocol::APP_GAME_PARENT_PLATFORM_WINDOWS;
use ocentra_parent_agent_protocol::APP_GAME_SCHEMA_VERSION;

const GENERATED_AT: &str = "2026-06-08T10:44:00Z";
const SOURCE_PROOF_ENTRY_ID: &str = "windows-app-game-owned-process-time-limit";
const ADAPTER_CAPABILITY: &str = "app-game-owned-process-time-limit";

#[test]
fn app_game_adapter_dispatch_result_serializes_parent_safe_command_result_rows() {
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
        execution_audit_recorded_count: 1,
        blocked_before_execution_audit_count: 0,
        adapter_execution_reported_count: 0,
        adapter_execution_evidence_missing_count: 1,
        blocked_before_adapter_execution_count: 0,
        adapter_dispatch_command_result_claimed_count: 1,
        service_local_execution_audit_claimed_count: 1,
        adapter_dispatch_executed_claimed_count: 0,
        broad_installed_app_blocking_claimed: false,
        child_device_delivery_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        private_diagnostics_claimed: false,
        rows: vec![scoped_command_result_row()],
    };

    let serialized = serde_json::to_value(&read_model).expect_value("dispatch result serializes");

    assert_eq!(
        serialized["readModelId"],
        APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_ID
    );
    assert_eq!(
        serialized["sourceReadModelIds"][1],
        "agent.enforcement.execute"
    );
    assert_eq!(
        serialized["rows"][0]["dispatchExecutionAuditId"],
        APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_ID
    );
    assert_eq!(
        serialized["rows"][0]["adapterDispatchExecutedClaimed"],
        false
    );
}

fn scoped_command_result_row() -> AppGameAdapterDispatchResultRow {
    AppGameAdapterDispatchResultRow {
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
        dispatch_execution_audit_state: APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_STATE_RECORDED
            .to_string(),
        dispatch_execution_audit_decision:
            APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_RECORDED.to_string(),
        dispatch_execution_audit_id: Some(
            APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_ID.to_string(),
        ),
        dispatch_execution_audit_refs: vec![
            APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_REF.to_string(),
        ],
        dispatch_adapter_execution_state: APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_STATE_MISSING
            .to_string(),
        dispatch_adapter_execution_decision:
            APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_MISSING.to_string(),
        dispatch_adapter_execution_result_id: None,
        dispatch_adapter_execution_status: None,
        dispatch_adapter_execution_adapter_result_code: None,
        dispatch_adapter_execution_audit_event_id: None,
        dispatch_adapter_execution_refs: vec![],
        manual_proof_requirements: vec![],
        claim_boundary: APP_GAME_ADAPTER_DISPATCH_RESULT_CLAIM_SCOPED_TIMER.to_string(),
        fallback_behavior: APP_GAME_ADAPTER_DISPATCH_RESULT_FALLBACK_SCOPED_TIMER.to_string(),
        adapter_dispatch_command_result_claimed: true,
        adapter_dispatch_executed_claimed: false,
        service_local_execution_audit_claimed: true,
        broad_installed_app_blocking_claimed: false,
        child_device_delivery_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        private_diagnostics_claimed: false,
        last_checked_at: GENERATED_AT.to_string(),
    }
}
