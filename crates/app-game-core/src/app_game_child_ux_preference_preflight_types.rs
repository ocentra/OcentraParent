use ocentra_parent_agent_protocol::schema_domain_mirrors::{
    family::ParentEvidenceReference,
    notification::{
        NotificationLocalOutboxEntryId, NotificationLocalOutboxRecord,
        NotificationLocalOutboxReference, NotificationLocalOutboxSchedulerEntryId,
        NotificationLocalOutboxSchedulerRecord, NotificationLocalOutboxSchedulerState,
        NotificationLocalOutboxSeverity, V3NotificationProviderChannel,
        V3NotificationRuleReasonCode,
    },
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppGameChildUxPreferencePreflightStatus {
    ParentPreferenceRequired,
    ManualRequired,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppGameChildUxPreferencePreflightInput {
    pub scheduler_record: NotificationLocalOutboxSchedulerRecord,
    pub source_outbox_record: NotificationLocalOutboxRecord,
    pub preflight_row_id: NotificationLocalOutboxReference,
    pub parent_preference_requirement_ref: NotificationLocalOutboxReference,
    pub notification_frequency_requirement_ref: NotificationLocalOutboxReference,
    pub quiet_hours_requirement_ref: NotificationLocalOutboxReference,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppGameChildUxPreferencePreflightRow {
    pub preflight_row_id: NotificationLocalOutboxReference,
    pub source_scheduler_entry_id: NotificationLocalOutboxSchedulerEntryId,
    pub source_scheduler_state: NotificationLocalOutboxSchedulerState,
    pub status: AppGameChildUxPreferencePreflightStatus,
    pub source_local_outbox_record_ref: Option<NotificationLocalOutboxEntryId>,
    pub source_outbox_file_ref: Option<NotificationLocalOutboxReference>,
    pub local_data_path_ref: Option<NotificationLocalOutboxReference>,
    pub scheduler_decision_ref: NotificationLocalOutboxReference,
    pub scheduler_artifact_ref: NotificationLocalOutboxReference,
    pub provider_channel: Option<V3NotificationProviderChannel>,
    pub reason_code: Option<V3NotificationRuleReasonCode>,
    pub severity: Option<NotificationLocalOutboxSeverity>,
    pub evidence_refs: Vec<ParentEvidenceReference>,
    pub policy_refs: Vec<NotificationLocalOutboxReference>,
    pub audit_refs: Vec<NotificationLocalOutboxReference>,
    pub parent_preference_requirement_refs: Vec<NotificationLocalOutboxReference>,
    pub notification_frequency_requirement_refs: Vec<NotificationLocalOutboxReference>,
    pub quiet_hours_requirement_refs: Vec<NotificationLocalOutboxReference>,
    pub manual_proof_requirements: Vec<NotificationLocalOutboxReference>,
    pub parent_preference_mutation_runtime_claimed: bool,
    pub parent_frequency_control_ui_claimed: bool,
    pub quiet_hours_timer_runtime_claimed: bool,
    pub provider_delivery_runtime_claimed: bool,
    pub provider_receipt_ingestion_claimed: bool,
    pub provider_credentials_claimed: bool,
    pub cloud_routing_claimed: bool,
    pub parent_notification_ui_claimed: bool,
    pub child_delivery_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub platform_enforcement_claimed: bool,
}
