use serde::{Deserialize, Serialize};

pub const APP_GAME_NOTIFICATION_PARENT_SURFACE_INPUT_FAMILY_MISMATCH: &str =
    "Expected app/game notification parent-surface inputs to use the same family ref";
pub const APP_GAME_NOTIFICATION_PARENT_SURFACE_INPUT_ROW_COUNT_MISMATCH: &str =
    "Expected app/game notification parent-surface inputs to have matching row counts";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationParentSurfaceIntentOptions {
    pub generated_at: String,
    pub intent_id: String,
    pub source_contract_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationFamilyReference {
    pub family_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationProviderStatusBoundaryEntry {
    pub provider_status: String,
    pub notification_status_ref: String,
    pub audit_refs: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationProviderStatusHandoffRow {
    pub handoff_row_id: String,
    pub source_scheduler_entry_ref: Option<String>,
    pub source_outbox_record_ref: Option<String>,
    pub provider_status_boundary_entry: AppGameNotificationProviderStatusBoundaryEntry,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationProviderStatusHandoffReadModel {
    pub handoff_id: String,
    pub family: AppGameNotificationFamilyReference,
    pub rows: Vec<AppGameNotificationProviderStatusHandoffRow>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationPreferenceStatusHandoffRow {
    pub handoff_row_id: String,
    pub source_scheduler_entry_ref: Option<String>,
    pub source_outbox_record_ref: Option<String>,
    pub notification_preference_status_entry: AppGameNotificationPreferenceStatusHandoffEntry,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationPreferenceStatusHandoffEntry {
    pub delivery_result_state: String,
    pub parent_preference_state: String,
    pub quiet_hours_decision: String,
    pub provider_channel: String,
    pub delivery_result_ref: String,
    pub audit_refs: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationPreferenceStatusHandoffReadModel {
    pub handoff_id: String,
    pub family: AppGameNotificationFamilyReference,
    pub rows: Vec<AppGameNotificationPreferenceStatusHandoffRow>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationParentSurfaceIntentRow {
    pub surface_row_id: String,
    pub source_provider_handoff_row_id: String,
    pub source_preference_handoff_row_id: String,
    pub source_scheduler_entry_ref: Option<String>,
    pub source_outbox_record_ref: Option<String>,
    pub provider_status: String,
    pub delivery_result_state: String,
    pub parent_preference_state: String,
    pub quiet_hours_decision: String,
    pub provider_channel: String,
    pub parent_surface_status: String,
    pub history_visibility: String,
    pub preference_visibility: String,
    pub drill_in_refs: Vec<String>,
    pub audit_refs: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
    pub minimal_surface_payload_boundary: String,
    pub sensitive_detail_included: bool,
    pub provider_delivery_claimed: bool,
    pub provider_receipt_claimed: bool,
    pub parent_preference_mutation_claimed: bool,
    pub child_delivery_claimed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationParentSurfaceIntentReadModel {
    pub schema_version: String,
    pub intent_id: String,
    pub generated_at: String,
    pub family: AppGameNotificationFamilyReference,
    pub source_provider_status_handoff_id: String,
    pub source_preference_status_handoff_id: String,
    pub source_contract_refs: Vec<String>,
    pub rows: Vec<AppGameNotificationParentSurfaceIntentRow>,
    pub manual_action_required_count: usize,
    pub unavailable_visible_count: usize,
    pub history_visible_count: usize,
    pub preference_setup_required_count: usize,
    pub parent_surface_non_claims: Vec<String>,
    pub parent_notification_ui_rendered: bool,
    pub parent_preference_ui_rendered: bool,
    pub parent_frequency_control_ui_rendered: bool,
    pub provider_delivery_runtime_claimed: bool,
    pub provider_receipt_ingestion_claimed: bool,
    pub provider_credentials_claimed: bool,
    pub cloud_routing_claimed: bool,
    pub child_delivery_claimed: bool,
    pub production_runtime_claimed: bool,
    pub production_durable_outbox_storage_claimed: bool,
    pub adapter_dispatch_claimed: bool,
}
