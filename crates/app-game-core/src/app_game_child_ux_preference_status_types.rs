use ocentra_parent_agent_protocol::schema_domain_mirrors::{
    family::ParentEvidenceReference,
    notification::{
        NotificationLocalOutboxEntryId, NotificationLocalOutboxReference,
        NotificationLocalOutboxSchedulerEntryId, V3NotificationProviderChannel,
        V3NotificationRuleReasonCode,
    },
};

use crate::{
    app_game_child_ux_preference_preflight_types::{
        AppGameChildUxPreferencePreflightRow, AppGameChildUxPreferencePreflightStatus,
    },
    app_game_notification_parent_surface_intent::AppGameNotificationPreferenceStatusHandoffRow,
};

#[derive(Clone, Debug, PartialEq)]
pub struct AppGameChildUxPreferenceStatusInput {
    pub preflight_row: AppGameChildUxPreferencePreflightRow,
    pub handoff_row_id: NotificationLocalOutboxReference,
    pub notification_rule_ref: NotificationLocalOutboxReference,
    pub notification_intent_ref: NotificationLocalOutboxReference,
    pub delivery_attempt_ref: NotificationLocalOutboxReference,
    pub delivery_result_ref: NotificationLocalOutboxReference,
    pub retry_policy_ref: NotificationLocalOutboxReference,
    pub quiet_hours_policy_ref: NotificationLocalOutboxReference,
    pub escalation_policy_ref: NotificationLocalOutboxReference,
    pub parent_preference_ref: NotificationLocalOutboxReference,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppGameChildUxPreferenceStatusHandoffRow {
    pub handoff_row_id: NotificationLocalOutboxReference,
    pub source_preflight_row_id: NotificationLocalOutboxReference,
    pub source_preflight_status: AppGameChildUxPreferencePreflightStatus,
    pub source_scheduler_entry_id: NotificationLocalOutboxSchedulerEntryId,
    pub source_local_outbox_record_ref: Option<NotificationLocalOutboxEntryId>,
    pub source_provider_channel: V3NotificationProviderChannel,
    pub source_reason_code: V3NotificationRuleReasonCode,
    pub preference_status_handoff_row: AppGameNotificationPreferenceStatusHandoffRow,
    pub notification_rule_ref: NotificationLocalOutboxReference,
    pub notification_intent_ref: NotificationLocalOutboxReference,
    pub delivery_attempt_ref: NotificationLocalOutboxReference,
    pub retry_policy_ref: NotificationLocalOutboxReference,
    pub quiet_hours_policy_ref: NotificationLocalOutboxReference,
    pub escalation_policy_ref: NotificationLocalOutboxReference,
    pub parent_preference_ref: NotificationLocalOutboxReference,
    pub evidence_refs: Vec<ParentEvidenceReference>,
    pub policy_refs: Vec<NotificationLocalOutboxReference>,
    pub audit_refs: Vec<NotificationLocalOutboxReference>,
    pub manual_proof_requirements: Vec<NotificationLocalOutboxReference>,
    pub last_checked_at: String,
    pub parent_preference_mutation_runtime_claimed: bool,
    pub parent_frequency_control_ui_claimed: bool,
    pub quiet_hours_timer_runtime_claimed: bool,
    pub retry_execution_runtime_claimed: bool,
    pub provider_delivery_runtime_claimed: bool,
    pub provider_receipt_ingestion_claimed: bool,
    pub provider_credentials_claimed: bool,
    pub cloud_routing_claimed: bool,
    pub parent_notification_ui_claimed: bool,
    pub child_delivery_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub platform_enforcement_claimed: bool,
}
