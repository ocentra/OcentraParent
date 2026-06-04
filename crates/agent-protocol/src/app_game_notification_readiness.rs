use serde::{Deserialize, Serialize};

use crate::ActivityEvidenceRef;

pub const APP_GAME_NOTIFICATION_READINESS_CUSTODY_CHILD_DEVICE_QUERY_STORE: &str =
    "child-device-query-store";
pub const APP_GAME_NOTIFICATION_READINESS_STATUS_READY: &str = "notification-intent-ready";
pub const APP_GAME_NOTIFICATION_READINESS_STATUS_PARTIAL: &str = "notification-intent-partial";
pub const APP_GAME_NOTIFICATION_READINESS_STATUS_NO_ROWS: &str = "notification-intent-no-rows";
pub const APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED: &str = "time-limit-exceeded";
pub const APP_GAME_NOTIFICATION_READINESS_REASON_APPROVAL_REQUEST: &str = "approval-request";
pub const APP_GAME_NOTIFICATION_READINESS_REASON_SUSPICIOUS_UNKNOWN: &str = "suspicious-unknown";
pub const APP_GAME_NOTIFICATION_READINESS_REASON_MANUAL_REQUIRED: &str = "manual-required";
pub const APP_GAME_NOTIFICATION_READINESS_REASON_CAPABILITY_UNAVAILABLE: &str =
    "capability-unavailable";
pub const APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT: &str =
    "ready-for-local-intent";
pub const APP_GAME_NOTIFICATION_READINESS_STATE_MANUAL_REQUIRED: &str = "manual-required";
pub const APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE: &str = "unavailable";
pub const APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_TIME_LIMIT: &str =
    "minimal-alert:time-limit-exceeded";
pub const APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_APPROVAL_REQUEST: &str =
    "minimal-alert:approval-request";
pub const APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_SUSPICIOUS_UNKNOWN: &str =
    "minimal-alert:suspicious-unknown";
pub const APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_MANUAL_REQUIRED: &str =
    "minimal-alert:manual-required";
pub const APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_UNAVAILABLE: &str =
    "minimal-alert:capability-unavailable";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationReadinessRow {
    pub schema_version: u16,
    pub row_id: String,
    pub reason: String,
    pub readiness_state: String,
    pub row_count: u64,
    pub minimal_payload_ref: String,
    pub evidence_reference_ids: Vec<String>,
    pub evidence: Vec<ActivityEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationReadinessReadModel {
    pub schema_version: u16,
    pub generated_at: String,
    pub custody_label: String,
    pub capability_status: String,
    pub returned: u64,
    pub ready_intent_count: u64,
    pub manual_required_count: u64,
    pub unavailable_count: u64,
    pub provider_delivery_claimed: bool,
    pub provider_receipt_ingestion_claimed: bool,
    pub local_outbox_runtime_claimed: bool,
    pub scheduler_runtime_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub parent_ui_claimed: bool,
    pub child_delivery_claimed: bool,
    pub rows: Vec<AppGameNotificationReadinessRow>,
}
