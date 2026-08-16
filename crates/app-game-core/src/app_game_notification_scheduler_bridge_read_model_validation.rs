use std::collections::HashSet;

use ocentra_eventing::error::EventingError;

use crate::app_game_notification_scheduler_bridge_types::{
    AppGameNotificationSchedulerBridgeReadModel, AppGameNotificationSchedulerBridgeRow,
    AppGameNotificationSchedulerBridgeStatus,
};

pub(super) fn validate_app_game_notification_scheduler_bridge_read_model(
    source: &AppGameNotificationSchedulerBridgeReadModel,
    invalid_field: &'static str,
) -> Result<(), EventingError> {
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
    let dishonest_row = source.rows.iter().any(|row| !source_row_is_honest(row));
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
    let missing_context = source.schema_version == 0
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

fn source_row_is_honest(row: &AppGameNotificationSchedulerBridgeRow) -> bool {
    if row.scheduler_bridge_record_id.trim().is_empty()
        || row.source_bridge_record_id.trim().is_empty()
    {
        return false;
    }
    match row.status {
        AppGameNotificationSchedulerBridgeStatus::Scheduled => {
            let Some(source_entry_id) = row.source_entry_id.as_ref() else {
                return false;
            };
            let Some(source_record) = row.source_outbox_record.as_ref() else {
                return false;
            };
            let Some(scheduler_record) = row.scheduler_record.as_ref() else {
                return false;
            };
            row.blocked_reason_refs.is_empty()
                && !scheduler_record
                    .scheduler_entry_id
                    .as_str()
                    .trim()
                    .is_empty()
                && source_record.entry_id == *source_entry_id
                && scheduler_record.source_entry_id == *source_entry_id
        }
        AppGameNotificationSchedulerBridgeStatus::ManualRequired
        | AppGameNotificationSchedulerBridgeStatus::Unavailable => {
            row.source_entry_id.is_none()
                && row.source_outbox_record.is_none()
                && row.scheduler_record.is_none()
                && !row.blocked_reason_refs.is_empty()
        }
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
