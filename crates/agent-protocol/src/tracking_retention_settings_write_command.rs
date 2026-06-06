use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingRetentionSettingsWriteResult {
    pub schema_version: u16,
    pub command_id: String,
    pub settings_kind: String,
    pub write_state: String,
    pub accepted_at: String,
    pub source_mutation_proof_refs: Vec<String>,
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
