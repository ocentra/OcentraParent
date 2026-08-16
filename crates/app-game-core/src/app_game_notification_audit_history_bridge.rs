use std::collections::HashSet;

use ocentra_eventing::error::EventingError;

use crate::app_game_notification_audit_history_bridge_types::{
    AppGameNotificationAuditHistoryEntry, AppGameNotificationAuditHistoryOptions,
    AppGameNotificationAuditHistoryReadModel, AppGameNotificationAuditHistoryStatus,
};
use crate::app_game_notification_local_outbox_bridge_read_model_validation::validate_app_game_notification_local_outbox_bridge_read_model;
use crate::app_game_notification_local_outbox_bridge_types::{
    AppGameNotificationLocalOutboxBridgeReadModel, AppGameNotificationLocalOutboxBridgeRow,
    AppGameNotificationLocalOutboxBridgeStatus,
};

const INVALID_CONTEXT_FIELD: &str = "app_game.notification_audit_history.context";
const INVALID_SOURCE_FIELD: &str = "app_game.notification_audit_history.source_bridge";
const INVALID_MODEL_FIELD: &str = "app_game.notification_audit_history.read_model";
const AUDIT_ENTRY_PREFIX: &str = "app-game-notification-audit-history";

pub fn build_app_game_notification_audit_history_bridge(
    options: AppGameNotificationAuditHistoryOptions,
    source: AppGameNotificationLocalOutboxBridgeReadModel,
) -> Result<AppGameNotificationAuditHistoryReadModel, EventingError> {
    validate_options(&options)?;
    validate_app_game_notification_local_outbox_bridge_read_model(&source, INVALID_SOURCE_FIELD)?;
    let entries = source
        .rows
        .iter()
        .map(|row| audit_entry(&options, &source, row))
        .collect::<Result<Vec<_>, _>>()?;
    let read_model = AppGameNotificationAuditHistoryReadModel {
        schema_version: source.schema_version,
        handoff_id: options.handoff_id,
        source_bridge_id: source.bridge_id,
        recorded_at: options.recorded_at,
        queued_local_count: count_entries(
            &entries,
            AppGameNotificationAuditHistoryStatus::QueuedLocal,
        ),
        manual_required_count: count_entries(
            &entries,
            AppGameNotificationAuditHistoryStatus::ManualRequired,
        ),
        unavailable_count: count_entries(
            &entries,
            AppGameNotificationAuditHistoryStatus::Unavailable,
        ),
        entries,
        provider_delivery_runtime_claimed: false,
        provider_receipt_ingestion_claimed: false,
        retry_worker_runtime_claimed: false,
        quiet_hours_timer_runtime_claimed: false,
        production_durable_history_claimed: false,
        parent_notification_history_ui_claimed: false,
        child_delivery_claimed: false,
        adapter_dispatch_claimed: false,
        cloud_routing_claimed: false,
    };
    validate_app_game_notification_audit_history_read_model(&read_model)?;
    Ok(read_model)
}

pub fn validate_app_game_notification_audit_history_read_model(
    read_model: &AppGameNotificationAuditHistoryReadModel,
) -> Result<(), EventingError> {
    let mut entry_ids = HashSet::new();
    let counts_match = read_model.queued_local_count
        == count_entries(
            &read_model.entries,
            AppGameNotificationAuditHistoryStatus::QueuedLocal,
        )
        && read_model.manual_required_count
            == count_entries(
                &read_model.entries,
                AppGameNotificationAuditHistoryStatus::ManualRequired,
            )
        && read_model.unavailable_count
            == count_entries(
                &read_model.entries,
                AppGameNotificationAuditHistoryStatus::Unavailable,
            );
    let unsafe_claim = read_model.provider_delivery_runtime_claimed
        || read_model.provider_receipt_ingestion_claimed
        || read_model.retry_worker_runtime_claimed
        || read_model.quiet_hours_timer_runtime_claimed
        || read_model.production_durable_history_claimed
        || read_model.parent_notification_history_ui_claimed
        || read_model.child_delivery_claimed
        || read_model.adapter_dispatch_claimed
        || read_model.cloud_routing_claimed;
    let dishonest_entry = read_model.entries.iter().any(|entry| {
        if entry.audit_entry_id.trim().is_empty()
            || !entry_ids.insert(entry.audit_entry_id.as_str())
            || entry.source_bridge_record_id.trim().is_empty()
            || entry.source_readiness_row_id.trim().is_empty()
            || entry.source_reason.trim().is_empty()
            || entry.audit_refs.is_empty()
            || entry.policy_refs.is_empty()
            || entry
                .audit_refs
                .iter()
                .any(|reference| reference.as_str().trim().is_empty())
            || entry
                .policy_refs
                .iter()
                .any(|reference| reference.as_str().trim().is_empty())
            || entry.provider_send_created
        {
            return true;
        }
        match entry.status {
            AppGameNotificationAuditHistoryStatus::QueuedLocal => {
                entry.source_entry_id.is_none()
                    || entry.source_outbox_state.is_none()
                    || entry.provider_channel.is_none()
                    || !entry.blocked_reason_refs.is_empty()
            }
            AppGameNotificationAuditHistoryStatus::ManualRequired
            | AppGameNotificationAuditHistoryStatus::Unavailable => {
                entry.source_entry_id.is_some()
                    || entry.source_outbox_state.is_some()
                    || entry.provider_channel.is_some()
                    || entry.blocked_reason_refs.is_empty()
            }
        }
    });
    if read_model.schema_version == 0
        || read_model.handoff_id.trim().is_empty()
        || read_model.source_bridge_id.trim().is_empty()
        || read_model.recorded_at.as_str().trim().is_empty()
        || !counts_match
        || unsafe_claim
        || dishonest_entry
    {
        return Err(invalid(INVALID_MODEL_FIELD, &read_model.handoff_id));
    }
    Ok(())
}

pub fn serialize_app_game_notification_audit_history_jsonl(
    read_model: &AppGameNotificationAuditHistoryReadModel,
) -> Result<String, serde_json::Error> {
    let lines = read_model
        .entries
        .iter()
        .map(serde_json::to_string)
        .collect::<Result<Vec<_>, _>>()?;
    if lines.is_empty() {
        Ok(String::new())
    } else {
        Ok(format!("{}\n", lines.join("\n")))
    }
}

pub fn parse_app_game_notification_audit_history_jsonl(
    jsonl: &str,
) -> Result<Vec<AppGameNotificationAuditHistoryEntry>, serde_json::Error> {
    jsonl
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(serde_json::from_str)
        .collect()
}

fn audit_entry(
    options: &AppGameNotificationAuditHistoryOptions,
    source: &AppGameNotificationLocalOutboxBridgeReadModel,
    row: &AppGameNotificationLocalOutboxBridgeRow,
) -> Result<AppGameNotificationAuditHistoryEntry, EventingError> {
    let (status, source_record) = match row.status {
        AppGameNotificationLocalOutboxBridgeStatus::Linked => (
            AppGameNotificationAuditHistoryStatus::QueuedLocal,
            Some(
                row.outbox_record
                    .as_ref()
                    .ok_or_else(|| invalid(INVALID_SOURCE_FIELD, &row.bridge_record_id))?,
            ),
        ),
        AppGameNotificationLocalOutboxBridgeStatus::ManualRequired => {
            (AppGameNotificationAuditHistoryStatus::ManualRequired, None)
        }
        AppGameNotificationLocalOutboxBridgeStatus::Unavailable => {
            (AppGameNotificationAuditHistoryStatus::Unavailable, None)
        }
    };
    Ok(AppGameNotificationAuditHistoryEntry {
        audit_entry_id: format!(
            "{AUDIT_ENTRY_PREFIX}:{}:{}",
            options.handoff_id, row.bridge_record_id
        ),
        status,
        recorded_at: options.recorded_at.clone(),
        source_bridge_record_id: row.bridge_record_id.clone(),
        source_readiness_row_id: row.source.row_id.clone(),
        source_entry_id: source_record.map(|record| record.entry_id.clone()),
        source_outbox_state: source_record.map(|record| record.state.clone()),
        provider_channel: source_record.map(|record| record.envelope.provider_channel.clone()),
        source_reason: row.source.reason.clone(),
        audit_refs: source.audit_refs.clone(),
        evidence_refs: row.source.evidence.clone(),
        policy_refs: source.policy_refs.clone(),
        blocked_reason_refs: row.blocked_reason_refs.clone(),
        provider_send_created: false,
    })
}

fn validate_options(options: &AppGameNotificationAuditHistoryOptions) -> Result<(), EventingError> {
    if options.handoff_id.trim().is_empty() || options.recorded_at.as_str().trim().is_empty() {
        return Err(invalid(INVALID_CONTEXT_FIELD, &options.handoff_id));
    }
    Ok(())
}

fn count_entries(
    entries: &[AppGameNotificationAuditHistoryEntry],
    status: AppGameNotificationAuditHistoryStatus,
) -> u64 {
    entries
        .iter()
        .filter(|entry| entry.status == status)
        .count() as u64
}

fn invalid(field: &'static str, value: &str) -> EventingError {
    EventingError::InvalidValue {
        field,
        value: value.to_owned(),
    }
}
