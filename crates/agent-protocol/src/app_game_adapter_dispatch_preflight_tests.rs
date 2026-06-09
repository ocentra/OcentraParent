use crate::{
    AppGameAdapterDispatchPreflightReadModel, AppGameAdapterDispatchPreflightRow,
    APP_GAME_ADAPTER_DISPATCH_AUDIT_OWNED_PROCESS, APP_GAME_ADAPTER_DISPATCH_CLAIM_SCOPED_TIMER,
    APP_GAME_ADAPTER_DISPATCH_DECISION_ELIGIBLE, APP_GAME_ADAPTER_DISPATCH_EVIDENCE_OWNED_PROCESS,
    APP_GAME_ADAPTER_DISPATCH_FALLBACK_SCOPED_TIMER,
    APP_GAME_ADAPTER_DISPATCH_INTENT_OWNED_PROCESS_TIME_LIMIT,
    APP_GAME_ADAPTER_DISPATCH_OUTCOME_READY, APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL_ID,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_ROW_ID_PREFIX,
    APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_ELIGIBLE,
    APP_GAME_ADAPTER_DISPATCH_TIMER_OWNED_PROCESS, APP_GAME_ADAPTER_EXECUTION_DECISION_ALLOWED,
    APP_GAME_ADAPTER_EXECUTION_ROW_ID_PREFIX, APP_GAME_ADAPTER_EXECUTION_STATE_PROVED_SCOPED,
    APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE, APP_GAME_ADAPTER_PRODUCT_NATIVE_APP,
    APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME, APP_GAME_PARENT_PLATFORM_WINDOWS,
    APP_GAME_SCHEMA_VERSION,
};

use crate::constants::v08_supported_adapter_runtime_proof::REF_ADAPTER_CAPABILITY_STATE;

const GENERATED_AT: &str = "2026-06-08T10:16:00Z";
const SOURCE_PROOF_ENTRY_ID: &str = "windows-app-game-owned-process-time-limit";
const ADAPTER_CAPABILITY: &str = "app-game-owned-process-time-limit";

const APP_GAME_ADAPTER_DISPATCH_EXECUTED_CLAIM_FALSE: bool = false;

#[test]
fn app_game_adapter_dispatch_preflight_serializes_parent_safe_rows() {
    let row = AppGameAdapterDispatchPreflightRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id: format!(
            "{APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_ROW_ID_PREFIX}{SOURCE_PROOF_ENTRY_ID}"
        ),
        source_execution_readiness_row_id: format!(
            "{APP_GAME_ADAPTER_EXECUTION_ROW_ID_PREFIX}{SOURCE_PROOF_ENTRY_ID}"
        ),
        source_proof_entry_id: SOURCE_PROOF_ENTRY_ID.to_string(),
        platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
        product_meanings: vec![
            APP_GAME_ADAPTER_PRODUCT_NATIVE_APP.to_string(),
            APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME.to_string(),
        ],
        adapter_capability: ADAPTER_CAPABILITY.to_string(),
        adapter_execution_state: APP_GAME_ADAPTER_EXECUTION_STATE_PROVED_SCOPED.to_string(),
        execution_decision: APP_GAME_ADAPTER_EXECUTION_DECISION_ALLOWED.to_string(),
        dispatch_preflight_state: APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_ELIGIBLE.to_string(),
        dispatch_decision: APP_GAME_ADAPTER_DISPATCH_DECISION_ELIGIBLE.to_string(),
        dispatch_intent_id: Some(
            APP_GAME_ADAPTER_DISPATCH_INTENT_OWNED_PROCESS_TIME_LIMIT.to_string(),
        ),
        dispatch_outcome_state: APP_GAME_ADAPTER_DISPATCH_OUTCOME_READY.to_string(),
        dispatch_evidence_refs: vec![APP_GAME_ADAPTER_DISPATCH_EVIDENCE_OWNED_PROCESS.to_string()],
        host_capability_state: APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE.to_string(),
        host_capability_evidence_refs: vec![REF_ADAPTER_CAPABILITY_STATE.to_string()],
        host_capability_probe_refs: vec!["windows-host-local-probe-ref".to_string()],
        dispatch_audit_refs: vec![APP_GAME_ADAPTER_DISPATCH_AUDIT_OWNED_PROCESS.to_string()],
        dispatch_timer_refs: vec![APP_GAME_ADAPTER_DISPATCH_TIMER_OWNED_PROCESS.to_string()],
        manual_proof_requirements: vec![],
        claim_boundary: APP_GAME_ADAPTER_DISPATCH_CLAIM_SCOPED_TIMER.to_string(),
        fallback_behavior: APP_GAME_ADAPTER_DISPATCH_FALLBACK_SCOPED_TIMER.to_string(),
        adapter_dispatch_eligible: true,
        adapter_dispatch_executed_claimed: APP_GAME_ADAPTER_DISPATCH_EXECUTED_CLAIM_FALSE,
        broad_installed_app_blocking_claimed: false,
        child_device_delivery_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        private_diagnostics_claimed: false,
        last_checked_at: GENERATED_AT.to_string(),
    };
    let read_model = AppGameAdapterDispatchPreflightReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        read_model_id: APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL_ID.to_string(),
        generated_at: GENERATED_AT.to_string(),
        source_read_model_ids: vec![],
        custody_label: "adapter-execution-readiness-and-policy-dispatch".to_string(),
        capability_status: "app-game-adapter-dispatch-preflight-partial".to_string(),
        returned: 1,
        dispatch_eligible_count: 1,
        blocked_before_dispatch_count: 0,
        adapter_dispatch_eligible_count: 1,
        adapter_dispatch_executed_claimed_count: 0,
        host_capability_available_count: 1,
        host_capability_not_detected_count: 0,
        host_capability_not_applicable_count: 0,
        host_capability_probe_ref_count: 1,
        broad_installed_app_blocking_claimed: false,
        child_device_delivery_claimed: false,
        platform_enforcement_claimed: false,
        provider_delivery_claimed: false,
        private_diagnostics_claimed: false,
        rows: vec![row],
    };

    let serialized = serde_json::to_string(&read_model).expect("dispatch preflight serializes");

    assert!(serialized.contains(APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL_ID));
    assert!(serialized.contains(APP_GAME_ADAPTER_DISPATCH_INTENT_OWNED_PROCESS_TIME_LIMIT));
    assert!(
        serialized.contains("\"hostCapabilityEvidenceRefs\":[\"adapter-capability-state-ref\"]")
    );
    assert!(serialized.contains("\"hostCapabilityProbeRefs\":[\"windows-host-local-probe-ref\"]"));
    assert!(serialized.contains("\"hostCapabilityState\":\"available\""));
    assert!(serialized.contains("\"hostCapabilityAvailableCount\":1"));
    assert!(serialized.contains("\"hostCapabilityProbeRefCount\":1"));
    assert!(serialized.contains("\"adapterDispatchExecutedClaimed\":false"));
}
