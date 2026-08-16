use ocentra_parent_agent_protocol::schema_domain_mirrors::{
    family::ParentTimestamp,
    notification::{
        NotificationLocalOutboxEntryId, NotificationLocalOutboxReference,
        NotificationLocalOutboxSchedulerRecord,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameNotificationSchedulerBridgeStatus {
    Scheduled,
    ManualRequired,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppGameNotificationSchedulerBridgeOptions {
    pub bridge_id: String,
    pub scheduler_now_at: ParentTimestamp,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationSchedulerBridgeRow {
    pub scheduler_bridge_record_id: String,
    pub status: AppGameNotificationSchedulerBridgeStatus,
    pub source_bridge_record_id: String,
    pub source_entry_id: Option<NotificationLocalOutboxEntryId>,
    pub scheduler_record: Option<NotificationLocalOutboxSchedulerRecord>,
    pub blocked_reason_refs: Vec<NotificationLocalOutboxReference>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationSchedulerBridgeReadModel {
    pub schema_version: u16,
    pub bridge_id: String,
    pub source_bridge_id: String,
    pub generated_at: ParentTimestamp,
    pub rows: Vec<AppGameNotificationSchedulerBridgeRow>,
    pub scheduled_count: u64,
    pub manual_required_count: u64,
    pub unavailable_count: u64,
    pub provider_delivery_runtime_claimed: bool,
    pub provider_receipt_ingestion_claimed: bool,
    pub retry_worker_runtime_claimed: bool,
    pub quiet_hours_timer_runtime_claimed: bool,
    pub production_durable_outbox_storage_claimed: bool,
    pub cloud_routing_claimed: bool,
    pub parent_notification_ui_claimed: bool,
    pub child_delivery_claimed: bool,
    pub adapter_dispatch_claimed: bool,
}
