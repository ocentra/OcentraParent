use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::schema_domain_mirrors::{
    family::ParentTimestamp,
    notification::{
        NotificationLocalOutboxEntryId, NotificationLocalOutboxReference,
        NotificationLocalOutboxState, V3NotificationProviderChannel,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameNotificationAuditHistoryStatus {
    QueuedLocal,
    ManualRequired,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppGameNotificationAuditHistoryOptions {
    pub handoff_id: String,
    pub recorded_at: ParentTimestamp,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationAuditHistoryEntry {
    pub audit_entry_id: String,
    pub status: AppGameNotificationAuditHistoryStatus,
    pub recorded_at: ParentTimestamp,
    pub source_bridge_record_id: String,
    pub source_readiness_row_id: String,
    pub source_entry_id: Option<NotificationLocalOutboxEntryId>,
    pub source_outbox_state: Option<NotificationLocalOutboxState>,
    pub provider_channel: Option<V3NotificationProviderChannel>,
    pub source_reason: String,
    pub audit_refs: Vec<NotificationLocalOutboxReference>,
    pub evidence_refs: Vec<ActivityEvidenceRef>,
    pub policy_refs: Vec<NotificationLocalOutboxReference>,
    pub blocked_reason_refs: Vec<NotificationLocalOutboxReference>,
    pub provider_send_created: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationAuditHistoryReadModel {
    pub schema_version: u16,
    pub handoff_id: String,
    pub source_bridge_id: String,
    pub recorded_at: ParentTimestamp,
    pub entries: Vec<AppGameNotificationAuditHistoryEntry>,
    pub queued_local_count: u64,
    pub manual_required_count: u64,
    pub unavailable_count: u64,
    pub provider_delivery_runtime_claimed: bool,
    pub provider_receipt_ingestion_claimed: bool,
    pub retry_worker_runtime_claimed: bool,
    pub quiet_hours_timer_runtime_claimed: bool,
    pub production_durable_history_claimed: bool,
    pub parent_notification_history_ui_claimed: bool,
    pub child_delivery_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub cloud_routing_claimed: bool,
}
