use std::io;

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::app_game::APP_GAME_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::app_game_notification_readiness::AppGameNotificationReadinessRow;
use ocentra_parent_agent_protocol::schema_domain_mirrors::notification::NotificationLocalOutboxRecord;

use crate::app_game_child_ux_outbox_store::AppGameChildUxLocalOutboxStore;
use crate::app_game_child_ux_outbox_types::AppGameChildUxOutboxPersistResult;
use crate::app_game_notification_local_outbox_bridge_mapping::bridge_row;
use crate::app_game_notification_local_outbox_bridge_types::{
    AppGameNotificationLocalOutboxBridgeOptions, AppGameNotificationLocalOutboxBridgeReadModel,
    AppGameNotificationLocalOutboxBridgeStatus,
};
use crate::app_game_notification_local_outbox_bridge_validation::{
    validate_options, validate_source,
};

pub fn build_app_game_notification_local_outbox_bridge(
    options: AppGameNotificationLocalOutboxBridgeOptions,
    source_rows: Vec<AppGameNotificationReadinessRow>,
) -> Result<AppGameNotificationLocalOutboxBridgeReadModel, EventingError> {
    validate_options(&options)?;
    let rows = source_rows
        .into_iter()
        .map(|source| {
            validate_source(&source)?;
            bridge_row(&options, source)
        })
        .collect::<Result<Vec<_>, EventingError>>()?;
    let linked_record_count = count_rows(&rows, AppGameNotificationLocalOutboxBridgeStatus::Linked);
    let manual_required_count = count_rows(
        &rows,
        AppGameNotificationLocalOutboxBridgeStatus::ManualRequired,
    );
    let unavailable_count = count_rows(
        &rows,
        AppGameNotificationLocalOutboxBridgeStatus::Unavailable,
    );
    Ok(AppGameNotificationLocalOutboxBridgeReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        bridge_id: options.bridge_id,
        generated_at: options.generated_at,
        family: options.family,
        outbox_root_ref: options.outbox_root_ref,
        rows,
        linked_record_count,
        manual_required_count,
        unavailable_count,
        provider_delivery_runtime_claimed: false,
        provider_receipt_ingestion_claimed: false,
        scheduler_runtime_claimed: false,
        cloud_routing_claimed: false,
        parent_notification_ui_claimed: false,
        child_delivery_claimed: false,
        adapter_dispatch_claimed: false,
    })
}

pub fn persist_app_game_notification_local_outbox_bridge(
    store: &AppGameChildUxLocalOutboxStore,
    read_model: &AppGameNotificationLocalOutboxBridgeReadModel,
) -> io::Result<Vec<AppGameChildUxOutboxPersistResult>> {
    read_model
        .rows
        .iter()
        .filter_map(|row| row.outbox_record.clone())
        .map(|record| store.persist(record))
        .collect()
}

pub fn serialize_app_game_notification_local_outbox_jsonl(
    read_model: &AppGameNotificationLocalOutboxBridgeReadModel,
) -> Result<String, serde_json::Error> {
    let lines = read_model
        .rows
        .iter()
        .filter_map(|row| row.outbox_record.as_ref())
        .map(serde_json::to_string)
        .collect::<Result<Vec<_>, _>>()?;
    if lines.is_empty() {
        Ok(String::new())
    } else {
        Ok(format!("{}\n", lines.join("\n")))
    }
}

pub fn parse_app_game_notification_local_outbox_jsonl(
    jsonl: &str,
) -> Result<Vec<NotificationLocalOutboxRecord>, serde_json::Error> {
    jsonl
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(serde_json::from_str)
        .collect()
}

fn count_rows(
    rows: &[crate::app_game_notification_local_outbox_bridge_types::AppGameNotificationLocalOutboxBridgeRow],
    status: AppGameNotificationLocalOutboxBridgeStatus,
) -> u64 {
    rows.iter().filter(|row| row.status == status).count() as u64
}
