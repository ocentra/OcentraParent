use serde::{Deserialize, Serialize};

pub const TRACKING_RETENTION_SETTINGS_MUTATION_STATE_ACCEPTED: &str = "accepted";
pub const TRACKING_RETENTION_SETTINGS_MUTATION_STATE_REJECTED: &str = "rejected";
pub const TRACKING_RETENTION_SETTINGS_MUTATION_REJECTION_INVALID_REQUEST: &str = "invalid-request";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingRetentionSettingsMutationRequest {
    pub request_id: String,
    pub intent_id: String,
    pub settings_kind: String,
    pub write_action: String,
    pub requested_value: String,
    pub evidence_reference_ids: Vec<String>,
    pub source_read_model_proof_refs: Vec<String>,
    pub writer_boundary_proof_refs: Vec<String>,
    pub audit_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingRetentionSettingsMutationResult {
    pub request_id: String,
    pub mutation_id: String,
    pub intent_id: String,
    pub settings_kind: String,
    pub write_action: String,
    pub requested_value: String,
    pub mutation_state: String,
    pub rejection_reason: Option<String>,
    pub service_mutation_executed: bool,
    pub durable_persistence_claimed: bool,
    pub portal_ui_claimed: bool,
    pub platform_runtime_claimed: bool,
    pub child_device_delivery_claimed: bool,
    pub provider_delivery_claimed: bool,
    pub notification_receipt_claimed: bool,
    pub physical_device_claimed: bool,
    pub authority_claimed: bool,
    pub product_claim_ready: bool,
    pub evidence_reference_ids: Vec<String>,
    pub source_read_model_proof_refs: Vec<String>,
    pub writer_boundary_proof_refs: Vec<String>,
    pub audit_refs: Vec<String>,
}
