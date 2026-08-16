use ocentra_parent_agent_protocol::{
    notification_provider_status_boundary::V08NotificationProviderStatusBoundaryEntry,
    schema_domain_mirrors::notification::{
        NotificationLocalOutboxEntryId, NotificationLocalOutboxReference,
        NotificationLocalOutboxSchedulerEntryId, V3NotificationProviderChannel,
    },
};

use crate::app_game_child_ux_provider_preflight_types::{
    AppGameChildUxProviderPreflightRow, AppGameChildUxProviderPreflightStatus,
};

#[derive(Clone, Debug, PartialEq)]
pub struct AppGameChildUxProviderStatusInput {
    pub preflight_row: AppGameChildUxProviderPreflightRow,
    pub handoff_row_id: NotificationLocalOutboxReference,
    pub status_entry_id: NotificationLocalOutboxReference,
    pub notification_intent_ref: NotificationLocalOutboxReference,
    pub notification_status_ref: NotificationLocalOutboxReference,
    pub provider_attempt_ref: NotificationLocalOutboxReference,
    pub preference_refs: Vec<NotificationLocalOutboxReference>,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppGameChildUxProviderStatusHandoffRow {
    pub handoff_row_id: NotificationLocalOutboxReference,
    pub source_preflight_row_id: NotificationLocalOutboxReference,
    pub source_preflight_status: AppGameChildUxProviderPreflightStatus,
    pub source_scheduler_entry_id: NotificationLocalOutboxSchedulerEntryId,
    pub source_local_outbox_record_ref: Option<NotificationLocalOutboxEntryId>,
    pub source_provider_channel: Option<V3NotificationProviderChannel>,
    pub provider_status_boundary_entry: V08NotificationProviderStatusBoundaryEntry,
    pub manual_proof_requirements: Vec<NotificationLocalOutboxReference>,
    pub provider_delivery_runtime_claimed: bool,
    pub provider_receipt_ingestion_claimed: bool,
    pub provider_credentials_claimed: bool,
    pub cloud_routing_claimed: bool,
    pub parent_notification_ui_claimed: bool,
    pub child_delivery_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub platform_enforcement_claimed: bool,
}
