use serde::{Deserialize, Serialize};

pub const APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL_ID: &str = "app-game-adapter-dispatch-result";
pub const APP_GAME_ADAPTER_DISPATCH_RESULT_CUSTODY_PREFLIGHT_AND_COMMAND: &str =
    "adapter-dispatch-preflight-and-enforcement-command-result";
pub const APP_GAME_ADAPTER_DISPATCH_RESULT_STATUS_PARTIAL: &str =
    "app-game-adapter-dispatch-command-result-partial";
pub const APP_GAME_ADAPTER_DISPATCH_RESULT_ROW_ID_PREFIX: &str =
    "app-game-adapter-dispatch-result-";
pub const APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_ACCEPTED: &str = "command-accepted";
pub const APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_BLOCKED: &str = "blocked-before-command";
pub const APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_MANUAL_REQUIRED: &str = "manual-required";
pub const APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_UNAVAILABLE: &str = "unavailable";
pub const APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_UNSUPPORTED: &str = "unsupported";
pub const APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_STATE_DEGRADED: &str = "degraded";
pub const APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_ACCEPTED: &str = "command-accepted";
pub const APP_GAME_ADAPTER_DISPATCH_COMMAND_RESULT_DECISION_BLOCKED: &str =
    "blocked-before-command";
pub const APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_STATE_RECORDED: &str =
    "service-local-audit-recorded";
pub const APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_STATE_BLOCKED: &str =
    "blocked-before-execution-audit";
pub const APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_RECORDED: &str =
    "service-local-audit-recorded";
pub const APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_DECISION_BLOCKED: &str =
    "blocked-before-execution-audit";
pub const APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_STATE_REPORTED: &str =
    "adapter-execution-reported";
pub const APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_STATE_MISSING: &str =
    "adapter-execution-evidence-missing";
pub const APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_STATE_BLOCKED: &str =
    "blocked-before-adapter-execution";
pub const APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_REPORTED: &str =
    "adapter-execution-reported";
pub const APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_MISSING: &str =
    "adapter-execution-evidence-missing";
pub const APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_DECISION_BLOCKED: &str =
    "blocked-before-adapter-execution";
pub const APP_GAME_ADAPTER_DISPATCH_ADAPTER_EXECUTION_REF_PREFIX: &str = "adapter-execution-audit-";
pub const APP_GAME_ADAPTER_DISPATCH_RESULT_OWNED_PROCESS_ID: &str =
    "app-game-dispatch-command-result-owned-process-time-limit";
pub const APP_GAME_ADAPTER_DISPATCH_COMMAND_AUDIT_OWNED_PROCESS: &str =
    "audit-owned-process-dispatch-command-accepted";
pub const APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_ID: &str =
    "app-game-adapter-dispatch-execution-audit-owned-process-time-limit";
pub const APP_GAME_ADAPTER_DISPATCH_EXECUTION_AUDIT_OWNED_PROCESS_REF: &str =
    "audit-owned-process-dispatch-service-local-execution-recorded";
pub const APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_COMMAND: &str = "agent.enforcement.execute";
pub const APP_GAME_ADAPTER_DISPATCH_EXECUTE_COMMAND: &str =
    "agent.activity.app-game.adapter-dispatch.execute";
pub const APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_EVENT: &str =
    "agent.enforcement.audit.reported";
pub const APP_GAME_ADAPTER_DISPATCH_RESULT_ENFORCEMENT_ACTION_MODE: &str = "terminate-process";
pub const APP_GAME_ADAPTER_DISPATCH_RESULT_READBACK_COMMAND: &str =
    "agent.activity.app-game.adapter-dispatch-result.read-model.get";
pub const APP_GAME_ADAPTER_DISPATCH_EXECUTE_TEST_COMMAND_ID: &str =
    "app-game-adapter-dispatch-execute-command";
pub const APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_COMMAND_ID: &str =
    "app-game-adapter-dispatch-result-command";
pub const APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_GENERATED_AT: &str = "2026-06-08T10:44:00Z";
pub const APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_SENT_AT: &str = "2026-06-08T10:44:01Z";
pub const APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_PORTAL_PEER: &str = "portal-dev";
pub const APP_GAME_ADAPTER_DISPATCH_RESULT_TEST_DEVICE_ID: &str = "child-device";
pub const APP_GAME_ADAPTER_DISPATCH_RESULT_CLAIM_SCOPED_TIMER: &str = "Dispatch command-result is limited to scoped Windows owned-process app/game time-limit rows and reuses agent.enforcement.execute.";
pub const APP_GAME_ADAPTER_DISPATCH_RESULT_CLAIM_BLOCKED: &str =
    "Adapter dispatch command-result is blocked before runtime for this app/game row.";
pub const APP_GAME_ADAPTER_DISPATCH_RESULT_FALLBACK_SCOPED_TIMER: &str =
    "Rows without scoped process/session identity stay blocked before dispatch command handoff.";
pub const APP_GAME_ADAPTER_DISPATCH_RESULT_FALLBACK_BLOCKED: &str =
    "The parent surface must route this row to manual review instead of dispatch.";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameAdapterDispatchResultRow {
    pub schema_version: u16,
    pub row_id: String,
    pub source_dispatch_preflight_row_id: String,
    pub source_proof_entry_id: String,
    pub platform: String,
    pub product_meanings: Vec<String>,
    pub adapter_capability: String,
    pub dispatch_preflight_state: String,
    pub dispatch_decision: String,
    pub dispatch_intent_id: Option<String>,
    pub dispatch_outcome_state: String,
    pub dispatch_command_result_state: String,
    pub dispatch_command_result_decision: String,
    pub enforcement_command_name: Option<String>,
    pub enforcement_event_name: Option<String>,
    pub enforcement_action_mode: Option<String>,
    pub dispatch_command_result_id: Option<String>,
    pub dispatch_command_audit_refs: Vec<String>,
    pub dispatch_command_timer_refs: Vec<String>,
    pub dispatch_execution_audit_state: String,
    pub dispatch_execution_audit_decision: String,
    pub dispatch_execution_audit_id: Option<String>,
    pub dispatch_execution_audit_refs: Vec<String>,
    pub dispatch_adapter_execution_state: String,
    pub dispatch_adapter_execution_decision: String,
    pub dispatch_adapter_execution_result_id: Option<String>,
    pub dispatch_adapter_execution_status: Option<String>,
    pub dispatch_adapter_execution_adapter_result_code: Option<String>,
    pub dispatch_adapter_execution_audit_event_id: Option<String>,
    pub dispatch_adapter_execution_refs: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
    pub claim_boundary: String,
    pub fallback_behavior: String,
    pub adapter_dispatch_command_result_claimed: bool,
    pub adapter_dispatch_executed_claimed: bool,
    pub service_local_execution_audit_claimed: bool,
    pub broad_installed_app_blocking_claimed: bool,
    pub child_device_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub provider_delivery_claimed: bool,
    pub private_diagnostics_claimed: bool,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameAdapterDispatchResultReadModel {
    pub schema_version: u16,
    pub read_model_id: String,
    pub generated_at: String,
    pub source_read_model_ids: Vec<String>,
    pub custody_label: String,
    pub capability_status: String,
    pub returned: u64,
    pub command_accepted_count: u64,
    pub blocked_before_command_count: u64,
    pub execution_audit_recorded_count: u64,
    pub blocked_before_execution_audit_count: u64,
    pub adapter_execution_reported_count: u64,
    pub adapter_execution_evidence_missing_count: u64,
    pub blocked_before_adapter_execution_count: u64,
    pub adapter_dispatch_command_result_claimed_count: u64,
    pub service_local_execution_audit_claimed_count: u64,
    pub adapter_dispatch_executed_claimed_count: u64,
    pub broad_installed_app_blocking_claimed: bool,
    pub child_device_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub provider_delivery_claimed: bool,
    pub private_diagnostics_claimed: bool,
    pub rows: Vec<AppGameAdapterDispatchResultRow>,
}
