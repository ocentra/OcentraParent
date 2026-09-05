use ocentra_parent_agent_protocol::schema_domain_mirrors::notification::{
    NotificationLocalOutboxEntryId, NotificationLocalOutboxSchedulerEntryId,
    NotificationLocalOutboxSchedulerRecord,
};

#[derive(Clone, Copy)]
pub(super) enum DuplicateSchedulerRecordIdentity {
    SchedulerEntry,
    SourceEntry,
}

struct DuplicateSchedulerRecordIdentityDescriptor {
    label: &'static str,
    field: &'static str,
}

impl DuplicateSchedulerRecordIdentity {
    pub(super) const ALL: [Self; 2] = [Self::SchedulerEntry, Self::SourceEntry];

    pub(super) fn duplicate(
        self,
        record: &NotificationLocalOutboxSchedulerRecord,
    ) -> NotificationLocalOutboxSchedulerRecord {
        let mut duplicate = record.clone();
        match self {
            Self::SchedulerEntry => {
                duplicate.source_entry_id = NotificationLocalOutboxEntryId::from(
                    "app-game-notification-outbox:bridge-58:alternate-source",
                );
            }
            Self::SourceEntry => {
                duplicate.scheduler_entry_id = NotificationLocalOutboxSchedulerEntryId::from(
                    "app-game-notification-scheduler:bridge-59:alternate-entry",
                );
            }
        }
        duplicate
    }

    pub(super) fn label(self) -> &'static str {
        self.descriptor().label
    }

    fn field(self) -> &'static str {
        self.descriptor().field
    }

    fn descriptor(self) -> DuplicateSchedulerRecordIdentityDescriptor {
        match self {
            Self::SchedulerEntry => DuplicateSchedulerRecordIdentityDescriptor {
                label: "duplicate-scheduler-entry",
                field: "app_game.child_ux_scheduler.scheduler_entry_id",
            },
            Self::SourceEntry => DuplicateSchedulerRecordIdentityDescriptor {
                label: "duplicate-source-entry",
                field: "app_game.child_ux_scheduler.source_entry_id",
            },
        }
    }

    fn value(self, record: &NotificationLocalOutboxSchedulerRecord) -> &str {
        match self {
            Self::SchedulerEntry => record.scheduler_entry_id.as_str(),
            Self::SourceEntry => record.source_entry_id.as_str(),
        }
    }
}

pub(super) fn assert_duplicate_identity_error(
    error: &str,
    identity: DuplicateSchedulerRecordIdentity,
    record: &NotificationLocalOutboxSchedulerRecord,
) {
    assert_eq!(
        error,
        format!(
            "invalid eventing value for {}: {}",
            identity.field(),
            identity.value(record)
        )
    );
}
