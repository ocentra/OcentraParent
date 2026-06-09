use serde::{Deserialize, Serialize};

pub const APP_GAME_ADAPTER_EXECUTION_READINESS_READ_MODEL_ID: &str =
    "app-game-adapter-execution-readiness";
pub const APP_GAME_ADAPTER_EXECUTION_READINESS_CUSTODY_SUPPORTED_ADAPTER_RUNTIME_PROOF: &str =
    "supported-adapter-runtime-proof";
pub const APP_GAME_ADAPTER_EXECUTION_READINESS_STATUS_PARTIAL: &str =
    "app-game-adapter-execution-partial";
pub const APP_GAME_ADAPTER_PRODUCT_NATIVE_APP: &str = "native-app";
pub const APP_GAME_ADAPTER_PRODUCT_NATIVE_GAME: &str = "native-game";
pub const APP_GAME_ADAPTER_EXECUTION_ROW_ID_PREFIX: &str = "app-game-adapter-execution-";
pub const APP_GAME_ADAPTER_EXECUTION_STATE_PROVED_SCOPED: &str = "proved-scoped-execution";
pub const APP_GAME_ADAPTER_EXECUTION_STATE_MANUAL_REQUIRED: &str = "manual-required";
pub const APP_GAME_ADAPTER_EXECUTION_STATE_UNAVAILABLE: &str = "unavailable";
pub const APP_GAME_ADAPTER_EXECUTION_STATE_UNSUPPORTED: &str = "unsupported";
pub const APP_GAME_ADAPTER_EXECUTION_STATE_DEGRADED: &str = "degraded";
pub const APP_GAME_ADAPTER_EXECUTION_STATE_NOT_CLAIMED: &str = "not-claimed";
pub const APP_GAME_ADAPTER_EXECUTION_DECISION_ALLOWED: &str = "execution-allowed";
pub const APP_GAME_ADAPTER_EXECUTION_DECISION_BLOCKED: &str = "blocked-before-execution";
pub const APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE: &str = "available";
pub const APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED: &str = "not-detected";
pub const APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_APPLICABLE: &str = "not-applicable";
pub const APP_GAME_PARENT_PLATFORM_MACOS: &str = "macos";
pub const APP_GAME_PARENT_PLATFORM_IOS: &str = "ios";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameAdapterExecutionReadinessRow {
    pub schema_version: u16,
    pub row_id: String,
    pub source_proof_entry_id: String,
    pub platform: String,
    pub product_meanings: Vec<String>,
    pub adapter_capability: String,
    pub adapter_execution_state: String,
    pub execution_decision: String,
    pub runtime_boundary: String,
    pub target_identity_state: String,
    pub rollback_reference_state: String,
    pub audit_reference_state: String,
    pub evidence_refs: Vec<String>,
    pub host_capability_state: String,
    pub host_capability_evidence_refs: Vec<String>,
    pub host_capability_probe_refs: Vec<String>,
    pub linked_proof_artifacts: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
    pub claim_boundary: String,
    pub fallback_behavior: String,
    pub adapter_execution_claimed: bool,
    pub broad_installed_app_blocking_claimed: bool,
    pub child_device_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub provider_delivery_claimed: bool,
    pub private_diagnostics_claimed: bool,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameAdapterExecutionReadinessReadModel {
    pub schema_version: u16,
    pub read_model_id: String,
    pub generated_at: String,
    pub source_read_model_ids: Vec<String>,
    pub custody_label: String,
    pub capability_status: String,
    pub returned: u64,
    pub execution_allowed_count: u64,
    pub blocked_before_execution_count: u64,
    pub adapter_execution_claimed_count: u64,
    pub host_capability_available_count: u64,
    pub host_capability_not_detected_count: u64,
    pub host_capability_not_applicable_count: u64,
    pub host_capability_probe_ref_count: u64,
    pub broad_installed_app_blocking_claimed: bool,
    pub child_device_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub provider_delivery_claimed: bool,
    pub private_diagnostics_claimed: bool,
    pub rows: Vec<AppGameAdapterExecutionReadinessRow>,
}
