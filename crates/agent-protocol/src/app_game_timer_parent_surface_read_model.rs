use serde::{Deserialize, Serialize};

use crate::ActivityEvidenceRef;

pub const APP_GAME_TIMER_PARENT_SURFACE_CUSTODY_CHILD_DEVICE_QUERY_STORE: &str =
    "child-device-query-store";
pub const APP_GAME_TIMER_PARENT_SURFACE_STATUS_READY: &str = "timer-parent-surface-ready";
pub const APP_GAME_TIMER_PARENT_SURFACE_STATUS_PARTIAL: &str = "timer-parent-surface-partial";
pub const APP_GAME_TIMER_PARENT_SURFACE_STATUS_NO_ROWS: &str = "timer-parent-surface-no-rows";
pub const APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP: &str = "native-app";
pub const APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME: &str = "native-game";
pub const APP_GAME_TIMER_PARENT_SURFACE_STATE_READY_FOR_PARENT_SURFACE: &str =
    "ready-for-parent-surface";
pub const APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_SOURCE_FRESHNESS: &str =
    "blocked-by-source-freshness";
pub const APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_COMPILER_DECISION: &str =
    "blocked-by-compiler-decision";
pub const APP_GAME_TIMER_PARENT_SURFACE_STATE_RUNTIME_MANUAL_REQUIRED: &str =
    "runtime-manual-required";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AppGameTimerParentSurfaceRow {
    pub schema_version: u16,
    pub row_id: String,
    pub target_domain: String,
    pub timer_surface_state: String,
    pub row_count: u64,
    pub evidence_reference_ids: Vec<String>,
    pub evidence: Vec<ActivityEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AppGameTimerParentSurfaceChildUxLocalArtifactRecord {
    pub schema_version: u16,
    pub artifact_reference_id: String,
    pub source_result_id: String,
    pub target_domain: String,
    pub child_reason_reference_ids: Vec<String>,
    pub child_status_reference_ids: Vec<String>,
    pub child_delivery_claimed: bool,
    pub notification_delivery_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub raw_private_source_rows_included: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AppGameTimerParentSurfaceChildUxParentSurfaceIntentRecord {
    pub schema_version: u16,
    pub parent_surface_intent_reference_id: String,
    pub source_result_id: String,
    pub source_artifact_reference_id: String,
    pub target_domain: String,
    pub history_visibility: String,
    pub parent_surface_status: String,
    pub preference_visibility: String,
    pub drill_in_reference_ids: Vec<String>,
    pub manual_proof_reference_ids: Vec<String>,
    pub sensitive_detail_included: bool,
    pub parent_notification_ui_rendered: bool,
    pub parent_preference_mutation_claimed: bool,
    pub provider_delivery_claimed: bool,
    pub child_delivery_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub raw_private_source_rows_included: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AppGameTimerParentSurfaceChildUxParentPreferenceSetupRecord {
    pub schema_version: u16,
    pub parent_preference_setup_reference_id: String,
    pub source_parent_surface_intent_reference_id: String,
    pub source_result_id: String,
    pub source_artifact_reference_id: String,
    pub target_domain: String,
    pub draft_status: String,
    pub parent_preference_setup_request_status: String,
    pub parent_preference_setup_request_reference_ids: Vec<String>,
    pub drill_in_reference_ids: Vec<String>,
    pub manual_proof_reference_ids: Vec<String>,
    pub parent_preference_ui_rendered: bool,
    pub parent_frequency_control_ui_rendered: bool,
    pub parent_preference_mutation_claimed: bool,
    pub notification_rule_mutation_claimed: bool,
    pub provider_delivery_claimed: bool,
    pub child_delivery_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub raw_private_source_rows_included: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AppGameTimerParentSurfaceReadModel {
    pub schema_version: u16,
    pub generated_at: String,
    pub custody_label: String,
    pub capability_status: String,
    pub returned: u64,
    pub ready_for_parent_surface_count: u64,
    pub blocked_by_source_freshness_count: u64,
    pub blocked_by_compiler_decision_count: u64,
    pub runtime_manual_required_count: u64,
    pub control_action_result_count: u64,
    pub control_action_result_reference_ids: Vec<String>,
    pub control_action_result_statuses: Vec<String>,
    pub control_action_result_capability_states: Vec<String>,
    pub control_action_result_enforcement_statuses: Vec<String>,
    pub child_facing_reason_reference_ids: Vec<String>,
    pub child_facing_status_reference_ids: Vec<String>,
    pub child_ux_handoff_ready_count: u64,
    pub child_ux_handoff_blocked_count: u64,
    pub child_ux_handoff_reference_ids: Vec<String>,
    pub child_ux_local_handoff_artifact_record_count: u64,
    pub child_ux_local_handoff_artifact_skipped_count: u64,
    pub child_ux_local_handoff_artifact_reference_ids: Vec<String>,
    pub child_ux_local_handoff_artifact_records:
        Vec<AppGameTimerParentSurfaceChildUxLocalArtifactRecord>,
    pub child_ux_parent_surface_intent_manual_action_required_count: u64,
    pub child_ux_parent_surface_intent_unavailable_visible_count: u64,
    pub child_ux_parent_surface_intent_history_visible_count: u64,
    pub child_ux_parent_surface_intent_preference_setup_required_count: u64,
    pub child_ux_parent_surface_intent_reference_ids: Vec<String>,
    pub child_ux_parent_surface_intent_records:
        Vec<AppGameTimerParentSurfaceChildUxParentSurfaceIntentRecord>,
    pub child_ux_parent_preference_setup_draft_ready_count: u64,
    pub child_ux_parent_preference_setup_unavailable_visible_count: u64,
    pub child_ux_parent_preference_setup_reference_ids: Vec<String>,
    pub child_ux_parent_preference_setup_request_ready_count: u64,
    pub child_ux_parent_preference_setup_request_unavailable_visible_count: u64,
    pub child_ux_parent_preference_setup_request_reference_ids: Vec<String>,
    pub child_ux_parent_preference_setup_records:
        Vec<AppGameTimerParentSurfaceChildUxParentPreferenceSetupRecord>,
    pub timer_runtime_claimed: bool,
    pub scheduler_persistence_claimed: bool,
    pub durable_scheduler_storage_claimed: bool,
    pub audit_runtime_claimed: bool,
    pub rollback_runtime_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub child_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub raw_private_source_rows_included: bool,
    pub rows: Vec<AppGameTimerParentSurfaceRow>,
}
