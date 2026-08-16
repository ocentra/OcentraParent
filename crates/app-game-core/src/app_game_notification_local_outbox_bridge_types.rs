use ocentra_parent_agent_protocol::app_game_notification_readiness::AppGameNotificationReadinessRow;
use ocentra_parent_agent_protocol::enforcement::ParentActionReference;
use ocentra_parent_agent_protocol::schema_domain_mirrors::family::{
    FamilyReference, ParentDeviceReference, ParentTimestamp,
};
use ocentra_parent_agent_protocol::schema_domain_mirrors::notification::{
    NotificationLocalOutboxRecord, NotificationLocalOutboxReference, V3NotificationProviderChannel,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameNotificationLocalOutboxBridgeStatus {
    Linked,
    ManualRequired,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppGameNotificationLocalOutboxBridgeOptions {
    pub bridge_id: String,
    pub generated_at: ParentTimestamp,
    pub family: FamilyReference,
    pub device: ParentDeviceReference,
    pub parent_action: ParentActionReference,
    pub provider_channel: V3NotificationProviderChannel,
    pub outbox_root_ref: NotificationLocalOutboxReference,
    pub outbox_file_ref: NotificationLocalOutboxReference,
    pub local_data_path_ref: NotificationLocalOutboxReference,
    pub policy_refs: Vec<NotificationLocalOutboxReference>,
    pub audit_refs: Vec<NotificationLocalOutboxReference>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationLocalOutboxBridgeRow {
    pub bridge_record_id: String,
    pub status: AppGameNotificationLocalOutboxBridgeStatus,
    pub source: AppGameNotificationReadinessRow,
    pub outbox_record: Option<NotificationLocalOutboxRecord>,
    pub blocked_reason_refs: Vec<NotificationLocalOutboxReference>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameNotificationLocalOutboxBridgeReadModel {
    pub schema_version: u16,
    pub bridge_id: String,
    pub generated_at: ParentTimestamp,
    pub family: FamilyReference,
    pub outbox_root_ref: NotificationLocalOutboxReference,
    pub policy_refs: Vec<NotificationLocalOutboxReference>,
    pub audit_refs: Vec<NotificationLocalOutboxReference>,
    pub rows: Vec<AppGameNotificationLocalOutboxBridgeRow>,
    pub linked_record_count: u64,
    pub manual_required_count: u64,
    pub unavailable_count: u64,
    pub provider_delivery_runtime_claimed: bool,
    pub provider_receipt_ingestion_claimed: bool,
    pub scheduler_runtime_claimed: bool,
    pub cloud_routing_claimed: bool,
    pub parent_notification_ui_claimed: bool,
    pub child_delivery_claimed: bool,
    pub adapter_dispatch_claimed: bool,
}
