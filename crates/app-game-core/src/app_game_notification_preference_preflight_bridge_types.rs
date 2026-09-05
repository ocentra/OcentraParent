use ocentra_parent_agent_protocol::schema_domain_mirrors::{
    family::ParentTimestamp, notification::NotificationLocalOutboxReference,
};

use crate::app_game_child_ux_preference_preflight_types::{
    AppGameChildUxPreferencePreflightRow, AppGameChildUxPreferencePreflightStatus,
};

#[derive(Clone, Debug, PartialEq)]
pub struct AppGameNotificationPreferencePreflightBridgeOptions {
    pub bridge_id: String,
    pub generated_at: ParentTimestamp,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppGameNotificationPreferencePreflightBridgeRow {
    pub preflight_bridge_record_id: String,
    pub status: AppGameChildUxPreferencePreflightStatus,
    pub source_scheduler_bridge_record_id: String,
    pub preflight_row: Option<AppGameChildUxPreferencePreflightRow>,
    pub blocked_reason_refs: Vec<NotificationLocalOutboxReference>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppGameNotificationPreferencePreflightBridgeReadModel {
    pub schema_version: u16,
    pub bridge_id: String,
    pub source_bridge_id: String,
    pub generated_at: ParentTimestamp,
    pub rows: Vec<AppGameNotificationPreferencePreflightBridgeRow>,
    pub parent_preference_required_count: u64,
    pub manual_required_count: u64,
    pub unavailable_count: u64,
    pub parent_preference_mutation_runtime_claimed: bool,
    pub parent_frequency_control_ui_claimed: bool,
    pub quiet_hours_timer_runtime_claimed: bool,
    pub provider_delivery_runtime_claimed: bool,
    pub provider_receipt_ingestion_claimed: bool,
    pub provider_credentials_claimed: bool,
    pub retry_worker_runtime_claimed: bool,
    pub production_durable_outbox_storage_claimed: bool,
    pub cloud_routing_claimed: bool,
    pub parent_notification_ui_claimed: bool,
    pub child_delivery_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub platform_enforcement_claimed: bool,
}
