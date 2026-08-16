use serde::{Deserialize, Serialize};

pub const APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL_ID: &str =
    "app-game-adapter-dispatch-preflight";
pub const APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_CUSTODY_EXECUTION_AND_POLICY_DISPATCH: &str =
    "adapter-execution-readiness-and-policy-dispatch";
pub const APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATUS_PARTIAL: &str =
    "app-game-adapter-dispatch-preflight-partial";
pub const APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_ROW_ID_PREFIX: &str =
    "app-game-adapter-dispatch-preflight-";
pub const APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_ELIGIBLE: &str = "dispatch-eligible";
pub const APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_BLOCKED: &str = "blocked-before-dispatch";
pub const APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_MANUAL_REQUIRED: &str = "manual-required";
pub const APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_UNAVAILABLE: &str = "unavailable";
pub const APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_UNSUPPORTED: &str = "unsupported";
pub const APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_STATE_DEGRADED: &str = "degraded";
pub const APP_GAME_ADAPTER_DISPATCH_DECISION_ELIGIBLE: &str = "dispatch-eligible";
pub const APP_GAME_ADAPTER_DISPATCH_DECISION_BLOCKED: &str = "blocked-before-dispatch";
pub const APP_GAME_ADAPTER_DISPATCH_OUTCOME_READY: &str = "dispatch-ready";
pub const APP_GAME_ADAPTER_DISPATCH_OUTCOME_MANUAL_REQUIRED: &str = "manual-required";
pub const APP_GAME_ADAPTER_DISPATCH_OUTCOME_UNAVAILABLE: &str = "unavailable";
pub const APP_GAME_ADAPTER_DISPATCH_OUTCOME_UNSUPPORTED: &str = "unsupported";
pub const APP_GAME_ADAPTER_DISPATCH_OUTCOME_DEGRADED: &str = "degraded";
pub const APP_GAME_ADAPTER_DISPATCH_INTENT_OWNED_PROCESS_TIME_LIMIT: &str =
    "dispatch-owned-process-time-limit";
pub const APP_GAME_ADAPTER_DISPATCH_EVIDENCE_OWNED_PROCESS: &str =
    "evidence-app-session-owned-process";
pub const APP_GAME_ADAPTER_DISPATCH_AUDIT_OWNED_PROCESS: &str =
    "audit-owned-process-dispatch-accepted";
pub const APP_GAME_ADAPTER_DISPATCH_TIMER_OWNED_PROCESS: &str = "timer-owned-process-active";
pub const APP_GAME_ADAPTER_DISPATCH_CLAIM_SCOPED_TIMER: &str =
    "Dispatch eligibility is limited to scoped Windows owned-process app/game time-limit rows.";
pub const APP_GAME_ADAPTER_DISPATCH_CLAIM_BLOCKED: &str =
    "Adapter dispatch is blocked before runtime for this app/game row.";
pub const APP_GAME_ADAPTER_DISPATCH_FALLBACK_SCOPED_TIMER: &str =
    "Rows without scoped process/session identity stay blocked before adapter dispatch.";
pub const APP_GAME_ADAPTER_DISPATCH_FALLBACK_BLOCKED: &str =
    "The parent surface must route this row to manual review instead of dispatch.";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameAdapterDispatchPreflightRow {
    pub schema_version: u16,
    pub row_id: String,
    pub source_execution_readiness_row_id: String,
    pub source_proof_entry_id: String,
    pub platform: String,
    pub product_meanings: Vec<String>,
    pub adapter_capability: String,
    pub adapter_execution_state: String,
    pub execution_decision: String,
    pub dispatch_preflight_state: String,
    pub dispatch_decision: String,
    pub dispatch_intent_id: Option<String>,
    pub dispatch_outcome_state: String,
    pub dispatch_evidence_refs: Vec<String>,
    pub host_capability_state: String,
    pub host_capability_evidence_refs: Vec<String>,
    pub host_capability_probe_refs: Vec<String>,
    pub dispatch_audit_refs: Vec<String>,
    pub dispatch_timer_refs: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
    pub claim_boundary: String,
    pub fallback_behavior: String,
    pub adapter_dispatch_eligible: bool,
    pub adapter_dispatch_executed_claimed: bool,
    pub broad_installed_app_blocking_claimed: bool,
    pub child_device_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub provider_delivery_claimed: bool,
    pub private_diagnostics_claimed: bool,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameAdapterDispatchPreflightReadModel {
    pub schema_version: u16,
    pub read_model_id: String,
    pub generated_at: String,
    pub source_read_model_ids: Vec<String>,
    pub custody_label: String,
    pub capability_status: String,
    pub returned: u64,
    pub dispatch_eligible_count: u64,
    pub blocked_before_dispatch_count: u64,
    pub adapter_dispatch_eligible_count: u64,
    pub adapter_dispatch_executed_claimed_count: u64,
    pub host_capability_available_count: u64,
    pub host_capability_not_detected_count: u64,
    pub host_capability_not_applicable_count: u64,
    pub host_capability_probe_ref_count: u64,
    pub broad_installed_app_blocking_claimed: bool,
    pub child_device_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub provider_delivery_claimed: bool,
    pub private_diagnostics_claimed: bool,
    pub rows: Vec<AppGameAdapterDispatchPreflightRow>,
}
