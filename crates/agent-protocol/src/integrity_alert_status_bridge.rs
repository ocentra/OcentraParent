use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08IntegrityAlertState {
    #[serde(rename = "permission-loss")]
    PermissionLoss,
    #[serde(rename = "stale-heartbeat")]
    StaleHeartbeat,
    #[serde(rename = "stopped-or-removed")]
    StoppedOrRemoved,
    #[serde(rename = "tamper-manual-required")]
    TamperManualRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08IntegrityAlertParentVisibleStatus {
    #[serde(rename = "permission-action-required")]
    PermissionActionRequired,
    #[serde(rename = "agent-heartbeat-stale")]
    AgentHeartbeatStale,
    #[serde(rename = "agent-stopped-or-removed")]
    AgentStoppedOrRemoved,
    #[serde(rename = "tamper-review-required")]
    TamperReviewRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08IntegrityAlertNotificationIntentState {
    #[serde(rename = "intent-created")]
    IntentCreated,
    #[serde(rename = "manual-review-required")]
    ManualReviewRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08IntegrityAlertDeliveryState {
    #[serde(rename = "not-delivered-provider-not-configured")]
    NotDeliveredProviderNotConfigured,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08IntegrityAlertAuditState {
    #[serde(rename = "audit-ref-backed")]
    AuditRefBacked,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V08IntegrityAlertStatusBridgeEntry {
    pub schema_version: String,
    pub bridge_entry_id: String,
    pub integrity_alert_state: V08IntegrityAlertState,
    pub parent_visible_status: V08IntegrityAlertParentVisibleStatus,
    pub notification_intent_state: V08IntegrityAlertNotificationIntentState,
    pub delivery_state: V08IntegrityAlertDeliveryState,
    pub audit_state: V08IntegrityAlertAuditState,
    pub reason_code_ref: String,
    pub status_ref: String,
    pub notification_intent_refs: Vec<String>,
    pub notification_status_refs: Vec<String>,
    pub audit_refs: Vec<String>,
    pub integrity_refs: Vec<String>,
    pub drill_in_refs: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
    pub boundary: String,
    pub provider_delivery_claimed: bool,
    pub broad_blocking_claimed: bool,
    pub tamper_resistance_claimed: bool,
    pub mobile_enforcement_claimed: bool,
    pub stealth_persistence_claimed: bool,
    pub privilege_escalation_claimed: bool,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V08IntegrityAlertStatusBridgeReadModel {
    pub schema_version: String,
    pub read_model_id: String,
    pub generated_at: String,
    pub source_read_model_ids: Vec<String>,
    pub entries: Vec<V08IntegrityAlertStatusBridgeEntry>,
}
