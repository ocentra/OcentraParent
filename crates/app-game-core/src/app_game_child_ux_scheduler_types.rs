use ocentra_parent_agent_protocol::schema_domain_mirrors::{
    family::ParentTimestamp,
    notification::{
        NotificationLocalOutboxEntryId, NotificationLocalOutboxPayloadPreview,
        NotificationLocalOutboxRecord, NotificationLocalOutboxReference,
        NotificationLocalOutboxSchedulerEntryId, NotificationLocalOutboxSchedulerRecord,
        NotificationLocalOutboxState,
    },
};

#[derive(Clone, Debug, PartialEq)]
pub struct AppGameChildUxSchedulerInput {
    pub source_record: NotificationLocalOutboxRecord,
    pub scheduler_entry_id: NotificationLocalOutboxSchedulerEntryId,
    pub scheduler_decision_ref: NotificationLocalOutboxReference,
    pub scheduler_artifact_ref: NotificationLocalOutboxReference,
    pub scheduler_now_at: ParentTimestamp,
    pub scheduler_payload_preview: NotificationLocalOutboxPayloadPreview,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AppGameChildUxSchedulerRoute {
    DueLocal(Box<NotificationLocalOutboxSchedulerRecord>),
    Blocked {
        source_entry_id: NotificationLocalOutboxEntryId,
        source_state: NotificationLocalOutboxState,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppGameChildUxSchedulerPersistResult {
    Inserted,
    AlreadyPresent,
}
