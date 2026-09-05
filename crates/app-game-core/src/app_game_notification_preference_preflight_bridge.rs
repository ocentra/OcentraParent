use std::io;

use ocentra_parent_agent_protocol::schema_domain_mirrors::notification::{
    NotificationLocalOutboxReference, NotificationLocalOutboxSchedulerRecord,
};

use crate::app_game_child_ux_preference_preflight::build_app_game_child_ux_preference_preflight;
use crate::app_game_child_ux_preference_preflight_types::{
    AppGameChildUxPreferencePreflightInput, AppGameChildUxPreferencePreflightStatus,
};
use crate::app_game_child_ux_scheduler_store::AppGameChildUxSchedulerProofStore;
use crate::app_game_notification_preference_preflight_bridge_types::{
    AppGameNotificationPreferencePreflightBridgeOptions,
    AppGameNotificationPreferencePreflightBridgeReadModel,
    AppGameNotificationPreferencePreflightBridgeRow,
};
use crate::app_game_notification_scheduler_bridge_read_model_validation::validate_app_game_notification_scheduler_bridge_read_model;
use crate::app_game_notification_scheduler_bridge_types::{
    AppGameNotificationSchedulerBridgeReadModel, AppGameNotificationSchedulerBridgeRow,
    AppGameNotificationSchedulerBridgeStatus,
};

const INVALID_CONTEXT_FIELD: &str = "app_game.notification_preference_preflight.context";
const INVALID_SOURCE_FIELD: &str = "app_game.notification_preference_preflight.source_bridge";
const PREFLIGHT_ROW_PREFIX: &str = "app-game-notification-preference-preflight";
const PREFERENCE_REQUIREMENT_PREFIX: &str = "app-game-parent-preference-requirement";
const FREQUENCY_REQUIREMENT_PREFIX: &str = "app-game-notification-frequency-requirement";
const QUIET_HOURS_REQUIREMENT_PREFIX: &str = "app-game-quiet-hours-requirement";

pub fn build_app_game_notification_preference_preflight_bridge(
    store: &AppGameChildUxSchedulerProofStore,
    options: AppGameNotificationPreferencePreflightBridgeOptions,
    source: AppGameNotificationSchedulerBridgeReadModel,
) -> io::Result<AppGameNotificationPreferencePreflightBridgeReadModel> {
    validate_options(&options)?;
    validate_app_game_notification_scheduler_bridge_read_model(&source, INVALID_SOURCE_FIELD)
        .map_err(invalid_data)?;
    let persisted_records = store.records()?;
    let rows = source
        .rows
        .iter()
        .map(|row| preflight_bridge_row(&options, row, &persisted_records))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(AppGameNotificationPreferencePreflightBridgeReadModel {
        schema_version: source.schema_version,
        bridge_id: options.bridge_id,
        source_bridge_id: source.bridge_id,
        generated_at: options.generated_at,
        parent_preference_required_count: count_rows(
            &rows,
            AppGameChildUxPreferencePreflightStatus::ParentPreferenceRequired,
        ),
        manual_required_count: count_rows(
            &rows,
            AppGameChildUxPreferencePreflightStatus::ManualRequired,
        ),
        unavailable_count: count_rows(&rows, AppGameChildUxPreferencePreflightStatus::Unavailable),
        rows,
        parent_preference_mutation_runtime_claimed: false,
        parent_frequency_control_ui_claimed: false,
        quiet_hours_timer_runtime_claimed: false,
        provider_delivery_runtime_claimed: false,
        provider_receipt_ingestion_claimed: false,
        provider_credentials_claimed: false,
        retry_worker_runtime_claimed: false,
        production_durable_outbox_storage_claimed: false,
        cloud_routing_claimed: false,
        parent_notification_ui_claimed: false,
        child_delivery_claimed: false,
        adapter_dispatch_claimed: false,
        platform_enforcement_claimed: false,
    })
}

fn preflight_bridge_row(
    options: &AppGameNotificationPreferencePreflightBridgeOptions,
    source: &AppGameNotificationSchedulerBridgeRow,
    persisted_records: &[NotificationLocalOutboxSchedulerRecord],
) -> io::Result<AppGameNotificationPreferencePreflightBridgeRow> {
    let preflight_bridge_record_id = format!(
        "{}:{}",
        options.bridge_id, source.scheduler_bridge_record_id
    );
    match source.status {
        AppGameNotificationSchedulerBridgeStatus::Scheduled => scheduled_row(
            options,
            source,
            preflight_bridge_record_id,
            persisted_records,
        ),
        AppGameNotificationSchedulerBridgeStatus::ManualRequired => Ok(blocked_row(
            source,
            preflight_bridge_record_id,
            AppGameChildUxPreferencePreflightStatus::ManualRequired,
        )),
        AppGameNotificationSchedulerBridgeStatus::Unavailable => Ok(blocked_row(
            source,
            preflight_bridge_record_id,
            AppGameChildUxPreferencePreflightStatus::Unavailable,
        )),
    }
}

fn scheduled_row(
    options: &AppGameNotificationPreferencePreflightBridgeOptions,
    source: &AppGameNotificationSchedulerBridgeRow,
    preflight_bridge_record_id: String,
    persisted_records: &[NotificationLocalOutboxSchedulerRecord],
) -> io::Result<AppGameNotificationPreferencePreflightBridgeRow> {
    let source_scheduler_record = source
        .scheduler_record
        .clone()
        .ok_or_else(|| invalid_io(INVALID_SOURCE_FIELD, &source.scheduler_bridge_record_id))?;
    let source_outbox_record = source
        .source_outbox_record
        .clone()
        .ok_or_else(|| invalid_io(INVALID_SOURCE_FIELD, &source.scheduler_bridge_record_id))?;
    let scheduler_record = persisted_records
        .iter()
        .find(|record| record.scheduler_entry_id == source_scheduler_record.scheduler_entry_id)
        .cloned()
        .ok_or_else(|| invalid_io(INVALID_SOURCE_FIELD, &source.scheduler_bridge_record_id))?;
    let mut expected_persisted_record = source_scheduler_record;
    expected_persisted_record.parent_owned_artifact_written = true;
    if scheduler_record != expected_persisted_record {
        return Err(invalid_io(
            INVALID_SOURCE_FIELD,
            &source.scheduler_bridge_record_id,
        ));
    }
    let ref_suffix = format!(
        "{}:{}",
        options.bridge_id, scheduler_record.scheduler_entry_id
    );
    let preflight_row =
        build_app_game_child_ux_preference_preflight(AppGameChildUxPreferencePreflightInput {
            scheduler_record,
            source_outbox_record,
            preflight_row_id: generated_ref(PREFLIGHT_ROW_PREFIX, &ref_suffix),
            parent_preference_requirement_ref: generated_ref(
                PREFERENCE_REQUIREMENT_PREFIX,
                &ref_suffix,
            ),
            notification_frequency_requirement_ref: generated_ref(
                FREQUENCY_REQUIREMENT_PREFIX,
                &ref_suffix,
            ),
            quiet_hours_requirement_ref: generated_ref(QUIET_HOURS_REQUIREMENT_PREFIX, &ref_suffix),
        })
        .map_err(invalid_data)?;
    Ok(AppGameNotificationPreferencePreflightBridgeRow {
        preflight_bridge_record_id,
        status: preflight_row.status,
        source_scheduler_bridge_record_id: source.scheduler_bridge_record_id.clone(),
        preflight_row: Some(preflight_row),
        blocked_reason_refs: Vec::new(),
    })
}

fn blocked_row(
    source: &AppGameNotificationSchedulerBridgeRow,
    preflight_bridge_record_id: String,
    status: AppGameChildUxPreferencePreflightStatus,
) -> AppGameNotificationPreferencePreflightBridgeRow {
    AppGameNotificationPreferencePreflightBridgeRow {
        preflight_bridge_record_id,
        status,
        source_scheduler_bridge_record_id: source.scheduler_bridge_record_id.clone(),
        preflight_row: None,
        blocked_reason_refs: source.blocked_reason_refs.clone(),
    }
}

fn generated_ref(prefix: &str, suffix: &str) -> NotificationLocalOutboxReference {
    format!("{prefix}:{suffix}").into()
}

fn validate_options(
    options: &AppGameNotificationPreferencePreflightBridgeOptions,
) -> io::Result<()> {
    if options.bridge_id.trim().is_empty() || options.generated_at.as_str().trim().is_empty() {
        return Err(invalid_io(INVALID_CONTEXT_FIELD, &options.bridge_id));
    }
    Ok(())
}

fn count_rows(
    rows: &[AppGameNotificationPreferencePreflightBridgeRow],
    status: AppGameChildUxPreferencePreflightStatus,
) -> u64 {
    rows.iter().filter(|row| row.status == status).count() as u64
}

fn invalid_io(field: &'static str, value: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, format!("{field}: {value}"))
}

fn invalid_data(error: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error.to_string())
}
