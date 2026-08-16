use ocentra_parent_agent_protocol::app_game_timer_parent_surface_read_model::AppGameTimerParentSurfaceChildUxLocalArtifactRecord;
use ocentra_parent_agent_protocol::schema_domain_mirrors::family::{
    FamilyReference, ParentActionReference, ParentDeviceReference, ParentTimestamp,
};
use ocentra_parent_agent_protocol::schema_domain_mirrors::notification::{
    NotificationLocalOutboxEntryId, NotificationLocalOutboxRecord,
    NotificationLocalOutboxReference, V3NotificationProviderChannel,
};

use crate::app_game_child_ux_types::{AppGameChildUxNotice, AppGameChildUxNoticeState};

#[derive(Clone, Debug, PartialEq)]
pub struct AppGameChildUxOutboxInput {
    pub notice: AppGameChildUxNotice,
    pub artifact: AppGameTimerParentSurfaceChildUxLocalArtifactRecord,
    pub entry_id: NotificationLocalOutboxEntryId,
    pub alert_ref: NotificationLocalOutboxReference,
    pub family: FamilyReference,
    pub device: ParentDeviceReference,
    pub parent_action: ParentActionReference,
    pub provider_channel: V3NotificationProviderChannel,
    pub observed_at: ParentTimestamp,
    pub audit_refs: Vec<NotificationLocalOutboxReference>,
    pub outbox_file_ref: NotificationLocalOutboxReference,
    pub local_data_path_ref: NotificationLocalOutboxReference,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AppGameChildUxOutboxRoute {
    Queued(Box<NotificationLocalOutboxRecord>),
    Blocked {
        state: AppGameChildUxNoticeState,
        blocked_reference_ids: Vec<String>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppGameChildUxOutboxPersistResult {
    Inserted,
    AlreadyPresent,
}
