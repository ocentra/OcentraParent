use ocentra_parent_agent_protocol::schema_domain_mirrors::{
    family::ParentTimestamp, notification::NotificationLocalOutboxReference,
};

use crate::app_game_child_ux_provider_preflight_types::{
    AppGameChildUxProviderPreflightRow, AppGameChildUxProviderPreflightStatus,
};

#[derive(Clone, Debug, PartialEq)]
pub struct AppGameNotificationProviderPreflightBridgeOptions {
    pub bridge_id: String,
    pub generated_at: ParentTimestamp,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppGameNotificationProviderPreflightBridgeRow {
    pub preflight_bridge_record_id: String,
    pub status: AppGameChildUxProviderPreflightStatus,
    pub source_scheduler_bridge_record_id: String,
    pub preflight_row: Option<AppGameChildUxProviderPreflightRow>,
    pub blocked_reason_refs: Vec<NotificationLocalOutboxReference>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppGameNotificationProviderPreflightBridgeReadModel {
    pub schema_version: u16,
    pub bridge_id: String,
    pub source_bridge_id: String,
    pub generated_at: ParentTimestamp,
    pub rows: Vec<AppGameNotificationProviderPreflightBridgeRow>,
    pub provider_adapter_required_count: u64,
    pub manual_required_count: u64,
    pub unavailable_count: u64,
    pub provider_delivery_runtime_claimed: bool,
    pub provider_receipt_ingestion_claimed: bool,
    pub provider_credentials_claimed: bool,
    pub retry_worker_runtime_claimed: bool,
    pub quiet_hours_timer_runtime_claimed: bool,
    pub production_durable_outbox_storage_claimed: bool,
    pub cloud_routing_claimed: bool,
    pub parent_notification_ui_claimed: bool,
    pub child_delivery_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub platform_enforcement_claimed: bool,
}
