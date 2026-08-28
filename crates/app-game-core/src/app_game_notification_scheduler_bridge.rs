use std::io;

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::schema_domain_mirrors::notification::{
    NotificationLocalOutboxPayloadPreview, NotificationLocalOutboxReference,
    NotificationLocalOutboxSchedulerEntryId, NotificationLocalOutboxSchedulerRecord,
};

use crate::app_game_child_ux_scheduler::{
    build_app_game_child_ux_scheduler_route, validate_scheduler_record,
};
use crate::app_game_child_ux_scheduler_store::AppGameChildUxSchedulerProofStore;
use crate::app_game_child_ux_scheduler_types::{
    AppGameChildUxSchedulerInput, AppGameChildUxSchedulerPersistResult,
    AppGameChildUxSchedulerRoute,
};
use crate::app_game_notification_local_outbox_bridge_read_model_validation::validate_app_game_notification_local_outbox_bridge_read_model;
use crate::app_game_notification_local_outbox_bridge_types::{
    AppGameNotificationLocalOutboxBridgeReadModel, AppGameNotificationLocalOutboxBridgeRow,
    AppGameNotificationLocalOutboxBridgeStatus,
};
use crate::app_game_notification_scheduler_bridge_read_model_validation::validate_app_game_notification_scheduler_bridge_read_model;
use crate::app_game_notification_scheduler_bridge_types::{
    AppGameNotificationSchedulerBridgeOptions, AppGameNotificationSchedulerBridgeReadModel,
    AppGameNotificationSchedulerBridgeRow, AppGameNotificationSchedulerBridgeStatus,
};

const INVALID_CONTEXT_FIELD: &str = "app_game.notification_scheduler.context";
const INVALID_SOURCE_FIELD: &str = "app_game.notification_scheduler.source_bridge";
const SCHEDULER_ENTRY_PREFIX: &str = "app-game-notification-scheduler";
const SCHEDULER_DECISION_PREFIX: &str = "app-game-notification-scheduler-decision";
const SCHEDULER_ARTIFACT_PREFIX: &str = "app-game-notification-scheduler-artifact";

pub fn build_app_game_notification_scheduler_bridge(
    options: AppGameNotificationSchedulerBridgeOptions,
    source: AppGameNotificationLocalOutboxBridgeReadModel,
) -> Result<AppGameNotificationSchedulerBridgeReadModel, EventingError> {
    validate_options(&options)?;
    validate_app_game_notification_local_outbox_bridge_read_model(&source, INVALID_SOURCE_FIELD)?;
    let rows = source
        .rows
        .iter()
        .map(|row| scheduler_bridge_row(&options, row))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(AppGameNotificationSchedulerBridgeReadModel {
        schema_version: source.schema_version,
        bridge_id: options.bridge_id,
        source_bridge_id: source.bridge_id,
        generated_at: options.scheduler_now_at,
        scheduled_count: count_rows(&rows, AppGameNotificationSchedulerBridgeStatus::Scheduled),
        manual_required_count: count_rows(
            &rows,
            AppGameNotificationSchedulerBridgeStatus::ManualRequired,
        ),
        unavailable_count: count_rows(&rows, AppGameNotificationSchedulerBridgeStatus::Unavailable),
        rows,
        provider_delivery_runtime_claimed: false,
        provider_receipt_ingestion_claimed: false,
        retry_worker_runtime_claimed: false,
        quiet_hours_timer_runtime_claimed: false,
        production_durable_outbox_storage_claimed: false,
        cloud_routing_claimed: false,
        parent_notification_ui_claimed: false,
        child_delivery_claimed: false,
        adapter_dispatch_claimed: false,
    })
}

pub fn persist_app_game_notification_scheduler_bridge(
    store: &AppGameChildUxSchedulerProofStore,
    read_model: &AppGameNotificationSchedulerBridgeReadModel,
) -> io::Result<Vec<AppGameChildUxSchedulerPersistResult>> {
    validate_app_game_notification_scheduler_bridge_read_model(read_model, INVALID_SOURCE_FIELD)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))?;
    read_model
        .rows
        .iter()
        .filter_map(|row| row.scheduler_record.as_ref())
        .try_for_each(|record| {
            validate_scheduler_record(record)
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))
        })?;
    read_model
        .rows
        .iter()
        .filter_map(|row| row.scheduler_record.clone())
        .map(|record| store.persist(record))
        .collect()
}

pub fn serialize_app_game_notification_scheduler_jsonl(
    read_model: &AppGameNotificationSchedulerBridgeReadModel,
) -> Result<String, serde_json::Error> {
    validate_app_game_notification_scheduler_bridge_read_model(read_model, INVALID_SOURCE_FIELD)
        .map_err(json_error)?;
    let lines = read_model
        .rows
        .iter()
        .filter_map(|row| row.scheduler_record.as_ref())
        .map(|record| {
            validate_scheduler_record(record).map_err(json_error)?;
            serde_json::to_string(record)
        })
        .collect::<Result<Vec<_>, _>>()?;
    if lines.is_empty() {
        Ok(String::new())
    } else {
        Ok(format!("{}\n", lines.join("\n")))
    }
}

pub fn parse_app_game_notification_scheduler_jsonl(
    jsonl: &str,
) -> Result<Vec<NotificationLocalOutboxSchedulerRecord>, serde_json::Error> {
    jsonl
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let record = serde_json::from_str(line)?;
            validate_scheduler_record(&record).map_err(json_error)?;
            Ok(record)
        })
        .collect()
}

fn scheduler_bridge_row(
    options: &AppGameNotificationSchedulerBridgeOptions,
    source: &AppGameNotificationLocalOutboxBridgeRow,
) -> Result<AppGameNotificationSchedulerBridgeRow, EventingError> {
    let scheduler_bridge_record_id = format!("{}:{}", options.bridge_id, source.bridge_record_id);
    match source.status {
        AppGameNotificationLocalOutboxBridgeStatus::Linked => {
            scheduled_row(options, source, scheduler_bridge_record_id)
        }
        AppGameNotificationLocalOutboxBridgeStatus::ManualRequired => Ok(blocked_row(
            source,
            scheduler_bridge_record_id,
            AppGameNotificationSchedulerBridgeStatus::ManualRequired,
        )),
        AppGameNotificationLocalOutboxBridgeStatus::Unavailable => Ok(blocked_row(
            source,
            scheduler_bridge_record_id,
            AppGameNotificationSchedulerBridgeStatus::Unavailable,
        )),
    }
}

fn scheduled_row(
    options: &AppGameNotificationSchedulerBridgeOptions,
    source: &AppGameNotificationLocalOutboxBridgeRow,
    scheduler_bridge_record_id: String,
) -> Result<AppGameNotificationSchedulerBridgeRow, EventingError> {
    let source_record = source
        .outbox_record
        .clone()
        .ok_or_else(|| invalid(INVALID_SOURCE_FIELD, &source.bridge_record_id))?;
    let source_entry_id = source_record.entry_id.clone();
    let route = build_app_game_child_ux_scheduler_route(AppGameChildUxSchedulerInput {
        scheduler_entry_id: NotificationLocalOutboxSchedulerEntryId::from(format!(
            "{SCHEDULER_ENTRY_PREFIX}:{}:{}",
            options.bridge_id, source_entry_id
        )),
        scheduler_decision_ref: NotificationLocalOutboxReference::from(format!(
            "{SCHEDULER_DECISION_PREFIX}:{}:{}",
            options.bridge_id, source_entry_id
        )),
        scheduler_artifact_ref: NotificationLocalOutboxReference::from(format!(
            "{SCHEDULER_ARTIFACT_PREFIX}:{}:{}",
            options.bridge_id, source_entry_id
        )),
        scheduler_now_at: options.scheduler_now_at.clone(),
        scheduler_payload_preview: NotificationLocalOutboxPayloadPreview::from(
            source_record.envelope.provider_payload_preview.as_str(),
        ),
        source_record: source_record.clone(),
    })?;
    let AppGameChildUxSchedulerRoute::DueLocal(scheduler_record) = route else {
        return Err(invalid(INVALID_SOURCE_FIELD, &source.bridge_record_id));
    };
    Ok(AppGameNotificationSchedulerBridgeRow {
        scheduler_bridge_record_id,
        status: AppGameNotificationSchedulerBridgeStatus::Scheduled,
        source_bridge_record_id: source.bridge_record_id.clone(),
        source_entry_id: Some(source_entry_id),
        source_outbox_record: Some(source_record),
        scheduler_record: Some(*scheduler_record),
        blocked_reason_refs: Vec::new(),
    })
}

fn blocked_row(
    source: &AppGameNotificationLocalOutboxBridgeRow,
    scheduler_bridge_record_id: String,
    status: AppGameNotificationSchedulerBridgeStatus,
) -> AppGameNotificationSchedulerBridgeRow {
    AppGameNotificationSchedulerBridgeRow {
        scheduler_bridge_record_id,
        status,
        source_bridge_record_id: source.bridge_record_id.clone(),
        source_entry_id: None,
        source_outbox_record: None,
        scheduler_record: None,
        blocked_reason_refs: source.blocked_reason_refs.clone(),
    }
}

fn validate_options(
    options: &AppGameNotificationSchedulerBridgeOptions,
) -> Result<(), EventingError> {
    if options.bridge_id.trim().is_empty() || options.scheduler_now_at.as_str().trim().is_empty() {
        return Err(invalid(INVALID_CONTEXT_FIELD, &options.bridge_id));
    }
    Ok(())
}

fn count_rows(
    rows: &[AppGameNotificationSchedulerBridgeRow],
    status: AppGameNotificationSchedulerBridgeStatus,
) -> u64 {
    rows.iter().filter(|row| row.status == status).count() as u64
}

fn invalid(field: &'static str, value: &str) -> EventingError {
    EventingError::InvalidValue {
        field,
        value: value.to_owned(),
    }
}

fn json_error(error: EventingError) -> serde_json::Error {
    serde_json::Error::io(io::Error::new(
        io::ErrorKind::InvalidData,
        error.to_string(),
    ))
}
