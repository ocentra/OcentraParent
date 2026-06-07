use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingRetentionSettingsWriteRequest {
    pub schema_version: u16,
    pub command_id: String,
    pub settings_kind: String,
    pub requested_retention_window_hours: Option<u16>,
    pub requested_delete_after_alert_resolved: bool,
    pub requested_parent_export: bool,
    pub requested_remote_sync_enabled: bool,
    pub requested_remote_ai_enabled: bool,
    pub source_writer_intent_refs: Vec<String>,
    pub source_read_model_proof_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingRetentionSettingsWriteResult {
    pub schema_version: u16,
    pub command_id: String,
    pub settings_kind: String,
    pub write_state: String,
    pub accepted_at: String,
    pub source_writer_intent_refs: Vec<String>,
    pub source_read_model_proof_refs: Vec<String>,
    pub source_mutation_proof_refs: Vec<String>,
    pub applied_retention_window_hours: Option<u16>,
    pub applied_delete_after_alert_resolved: bool,
    pub parent_export_prepared: bool,
    pub remote_sync_enabled: bool,
    pub remote_ai_enabled: bool,
    pub local_service_state_revision: Option<u64>,
    pub local_service_state_snapshot_ref: String,
    pub durable_settings_store_ref: String,
    pub durable_settings_persisted: bool,
    pub command_transport_claimed: bool,
    pub service_write_preflight_claimed: bool,
    pub service_mutation_executed: bool,
    pub portal_writable_ui_claimed: bool,
    pub platform_runtime_claimed: bool,
    pub child_device_delivery_claimed: bool,
    pub provider_delivery_claimed: bool,
    pub notification_receipt_claimed: bool,
    pub physical_device_claimed: bool,
    pub authority_claimed: bool,
    pub product_claim_ready: bool,
}
