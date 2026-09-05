use std::collections::HashSet;

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::app_game::APP_GAME_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::schema_domain_mirrors::notification::{
    NotificationLocalOutboxEntryId, NotificationLocalOutboxPayloadPreview,
    NotificationLocalOutboxRecord, NotificationLocalOutboxReference,
    NotificationLocalOutboxSchedulerEntryId, NotificationLocalOutboxSchedulerRecord,
};

use crate::app_game_child_ux_scheduler::{
    build_app_game_child_ux_scheduler_route, validate_scheduler_record,
};
use crate::app_game_child_ux_scheduler_types::{
    AppGameChildUxSchedulerInput, AppGameChildUxSchedulerRoute,
};
use crate::app_game_notification_scheduler_bridge_types::{
    AppGameNotificationSchedulerBridgeReadModel, AppGameNotificationSchedulerBridgeRow,
    AppGameNotificationSchedulerBridgeStatus,
};

const SOURCE_BRIDGE_RECORD_PREFIX: &str = "app-game-notification-outbox-bridge";
const SOURCE_ENTRY_PREFIX: &str = "app-game-notification-outbox";
const SCHEDULER_ENTRY_PREFIX: &str = "app-game-notification-scheduler";
const SCHEDULER_DECISION_PREFIX: &str = "app-game-notification-scheduler-decision";
const SCHEDULER_ARTIFACT_PREFIX: &str = "app-game-notification-scheduler-artifact";

pub(super) fn validate_app_game_notification_scheduler_bridge_read_model(
    source: &AppGameNotificationSchedulerBridgeReadModel,
    invalid_field: &'static str,
) -> Result<(), EventingError> {
    for scheduler_record in source
        .rows
        .iter()
        .filter_map(|row| row.scheduler_record.as_ref())
    {
        validate_scheduler_record(scheduler_record)?;
    }
    let honest_counts = source.scheduled_count
        == count_rows(source, AppGameNotificationSchedulerBridgeStatus::Scheduled)
        && source.manual_required_count
            == count_rows(
                source,
                AppGameNotificationSchedulerBridgeStatus::ManualRequired,
            )
        && source.unavailable_count
            == count_rows(
                source,
                AppGameNotificationSchedulerBridgeStatus::Unavailable,
            );
    let unsafe_claim = source.provider_delivery_runtime_claimed
        || source.provider_receipt_ingestion_claimed
        || source.retry_worker_runtime_claimed
        || source.quiet_hours_timer_runtime_claimed
        || source.production_durable_outbox_storage_claimed
        || source.cloud_routing_claimed
        || source.parent_notification_ui_claimed
        || source.child_delivery_claimed
        || source.adapter_dispatch_claimed;
    let dishonest_row = source
        .rows
        .iter()
        .any(|row| !source_row_is_honest(source, row));
    let mut identities = HashSet::new();
    let duplicate_identity = source
        .rows
        .iter()
        .any(|row| !identities.insert(row.scheduler_bridge_record_id.as_str()));
    let mut scheduled_identities = HashSet::new();
    let duplicate_scheduled_identity = source.rows.iter().any(|row| {
        row.scheduler_record.as_ref().is_some_and(|record| {
            !scheduled_identities.insert((
                record.scheduler_entry_id.as_str(),
                record.source_entry_id.as_str(),
            ))
        })
    });
    let missing_context = source.schema_version != APP_GAME_SCHEMA_VERSION
        || source.bridge_id.trim().is_empty()
        || source.source_bridge_id.trim().is_empty()
        || source.generated_at.as_str().trim().is_empty();
    if missing_context
        || !honest_counts
        || unsafe_claim
        || dishonest_row
        || duplicate_identity
        || duplicate_scheduled_identity
    {
        return Err(EventingError::InvalidValue {
            field: invalid_field,
            value: source.bridge_id.clone(),
        });
    }
    Ok(())
}

fn source_row_is_honest(
    source: &AppGameNotificationSchedulerBridgeReadModel,
    row: &AppGameNotificationSchedulerBridgeRow,
) -> bool {
    let Some(source_row_id) = source_row_id(source, row) else {
        return false;
    };
    if row.scheduler_bridge_record_id
        != format!("{}:{}", source.bridge_id, row.source_bridge_record_id)
    {
        return false;
    }
    match row.status {
        AppGameNotificationSchedulerBridgeStatus::Scheduled => {
            scheduled_row_is_honest(source, row, source_row_id)
        }
        AppGameNotificationSchedulerBridgeStatus::ManualRequired
        | AppGameNotificationSchedulerBridgeStatus::Unavailable => {
            row.source_entry_id.is_none()
                && row.source_outbox_record.is_none()
                && row.scheduler_record.is_none()
                && !row.blocked_reason_refs.is_empty()
                && row
                    .blocked_reason_refs
                    .iter()
                    .all(|reference| !reference.as_str().trim().is_empty())
        }
    }
}

fn source_row_id<'a>(
    source: &AppGameNotificationSchedulerBridgeReadModel,
    row: &'a AppGameNotificationSchedulerBridgeRow,
) -> Option<&'a str> {
    let prefix = format!("{SOURCE_BRIDGE_RECORD_PREFIX}:{}:", source.source_bridge_id);
    row.source_bridge_record_id
        .strip_prefix(&prefix)
        .filter(|row_id| !row_id.trim().is_empty())
}

fn scheduled_row_is_honest(
    source: &AppGameNotificationSchedulerBridgeReadModel,
    row: &AppGameNotificationSchedulerBridgeRow,
    source_row_id: &str,
) -> bool {
    let Some(source_entry_id) = row.source_entry_id.as_ref() else {
        return false;
    };
    let Some(source_record) = row.source_outbox_record.as_ref() else {
        return false;
    };
    let Some(scheduler_record) = row.scheduler_record.as_ref() else {
        return false;
    };
    let expected_source_entry_id = format!(
        "{SOURCE_ENTRY_PREFIX}:{}:{source_row_id}",
        source.source_bridge_id
    );
    row.blocked_reason_refs.is_empty()
        && source_entry_id.as_str() == expected_source_entry_id
        && source_record.entry_id == *source_entry_id
        && rebuilt_scheduler_record(source, source_record, source_entry_id)
            .is_some_and(|expected| expected == *scheduler_record)
}

fn rebuilt_scheduler_record(
    source: &AppGameNotificationSchedulerBridgeReadModel,
    source_record: &NotificationLocalOutboxRecord,
    source_entry_id: &NotificationLocalOutboxEntryId,
) -> Option<NotificationLocalOutboxSchedulerRecord> {
    let route = build_app_game_child_ux_scheduler_route(AppGameChildUxSchedulerInput {
        source_record: source_record.clone(),
        scheduler_entry_id: NotificationLocalOutboxSchedulerEntryId::from(format!(
            "{SCHEDULER_ENTRY_PREFIX}:{}:{}",
            source.bridge_id,
            source_entry_id.as_str()
        )),
        scheduler_decision_ref: NotificationLocalOutboxReference::from(format!(
            "{SCHEDULER_DECISION_PREFIX}:{}:{}",
            source.bridge_id,
            source_entry_id.as_str()
        )),
        scheduler_artifact_ref: NotificationLocalOutboxReference::from(format!(
            "{SCHEDULER_ARTIFACT_PREFIX}:{}:{}",
            source.bridge_id,
            source_entry_id.as_str()
        )),
        scheduler_now_at: source.generated_at.clone(),
        scheduler_payload_preview: NotificationLocalOutboxPayloadPreview::from(
            source_record.envelope.provider_payload_preview.as_str(),
        ),
    })
    .ok()?;
    match route {
        AppGameChildUxSchedulerRoute::DueLocal(record) => Some(*record),
        AppGameChildUxSchedulerRoute::Blocked { .. } => None,
    }
}

fn count_rows(
    source: &AppGameNotificationSchedulerBridgeReadModel,
    status: AppGameNotificationSchedulerBridgeStatus,
) -> u64 {
    source
        .rows
        .iter()
        .filter(|row| row.status == status)
        .count() as u64
}
