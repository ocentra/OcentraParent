use serde::{Deserialize, Serialize};

pub const V08_NOTIFICATION_PROVIDER_STATUS_BOUNDARY_SCHEMA_VERSION: &str = "v0.8";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum V08NotificationProviderStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08NotificationProviderStatusProofState {
    #[serde(rename = "queued-contract-only")]
    QueuedContractOnly,
    #[serde(rename = "delivery-receipt-required")]
    DeliveryReceiptRequired,
    #[serde(rename = "failure-contract-only")]
    FailureContractOnly,
    #[serde(rename = "provider-unavailable-contract")]
    ProviderUnavailableContract,
    #[serde(rename = "manual-action-required")]
    ManualActionRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum V08NotificationQuietHoursReadiness {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "defer-noncritical")]
    DeferNoncritical,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum V08NotificationEscalationReadiness {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "waiting-window")]
    WaitingWindow,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08NotificationProviderDeliveryClaim {
    #[serde(rename = "not-implemented")]
    NotImplemented,
    #[serde(rename = "not-observed")]
    NotObserved,
    #[serde(rename = "receipt-required")]
    ReceiptRequired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V08NotificationProviderStatusBoundaryEntry {
    pub schema_version: String,
    pub status_entry_id: String,
    pub provider_status: V08NotificationProviderStatus,
    pub status_proof_state: V08NotificationProviderStatusProofState,
    pub quiet_hours_readiness: V08NotificationQuietHoursReadiness,
    pub escalation_readiness: V08NotificationEscalationReadiness,
    pub delivery_claim_state: V08NotificationProviderDeliveryClaim,
    pub notification_intent_ref: String,
    pub notification_status_ref: String,
    pub provider_attempt_ref: String,
    pub audit_refs: Vec<String>,
    pub preference_refs: Vec<String>,
    pub readiness_refs: Vec<String>,
    pub provider_receipt_refs: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
    pub minimal_payload_boundary: String,
    pub provider_delivery_implemented: bool,
    pub provider_delivery_observed: bool,
    pub delivered_notification_claimed: bool,
    pub sensitive_provider_payload_claimed: bool,
    pub provider_stores_child_evidence_claimed: bool,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V08NotificationProviderStatusBoundaryReadModel {
    pub schema_version: String,
    pub read_model_id: String,
    pub generated_at: String,
    pub source_read_model_ids: Vec<String>,
    pub entries: Vec<V08NotificationProviderStatusBoundaryEntry>,
}
