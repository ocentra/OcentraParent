#[path = "app_game_notification_readiness_payload/evidence.rs"]
mod evidence;
#[path = "app_game_notification_readiness_payload/logic.rs"]
mod logic;
#[path = "app_game_notification_readiness_payload/scheduler_runtime.rs"]
mod scheduler_runtime;

use ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel;
use ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationStatusReadModels;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::AppGameNotificationReadinessReadModel;

use crate::fields::fields_from_pairs;

pub(crate) struct AppGameNotificationReadinessReport {
    pub(crate) read_model: AppGameNotificationReadinessReadModel,
    pub(crate) status_read_models: AppGameNotificationStatusReadModels,
}

pub fn app_game_notification_readiness_from_service_model(
    model: AppGameServiceReadModel,
    local_outbox_runtime_claimed: bool,
) -> AppGameNotificationReadinessReadModel {
    logic::app_game_notification_readiness_from_service_model(model, local_outbox_runtime_claimed)
}

pub fn app_game_notification_readiness_payload(
    read_model: &AppGameNotificationReadinessReadModel,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::CUSTODY_LABEL,
            LogFieldValue::String(read_model.custody_label.clone()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(read_model.capability_status.clone()),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.returned as f64),
        ),
        (
            constants::field::APP_GAME_NOTIFICATION_READINESS_READ_MODEL,
            LogFieldValue::String(serde_json::to_string(read_model).unwrap_or_default()),
        ),
    ])
}

pub(crate) fn app_game_notification_readiness_report_from_service_model(
    model: AppGameServiceReadModel,
    local_outbox_runtime_claimed: bool,
) -> AppGameNotificationReadinessReport {
    let read_model =
        app_game_notification_readiness_from_service_model(model, local_outbox_runtime_claimed);
    let status_read_models =
        logic::notification_status_read_models(&read_model.rows, &read_model.generated_at);
    AppGameNotificationReadinessReport {
        read_model,
        status_read_models,
    }
}

pub(crate) fn app_game_notification_readiness_report_payload(
    report: &AppGameNotificationReadinessReport,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(report.read_model.generated_at.clone()),
        ),
        (
            constants::field::CUSTODY_LABEL,
            LogFieldValue::String(report.read_model.custody_label.clone()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(report.read_model.capability_status.clone()),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(report.read_model.returned as f64),
        ),
        (
            constants::field::APP_GAME_NOTIFICATION_READINESS_READ_MODEL,
            LogFieldValue::String(serde_json::to_string(&report.read_model).unwrap_or_default()),
        ),
        (
            constants::field::APP_GAME_NOTIFICATION_STATUS_READ_MODELS,
            LogFieldValue::String(
                serde_json::to_string(&report.status_read_models).unwrap_or_default(),
            ),
        ),
    ])
}
