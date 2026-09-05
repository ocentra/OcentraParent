use std::path::Path;

#[path = "app_game_notification_readiness_payload/boundary/constants.rs"]
mod constants;
#[path = "app_game_notification_readiness_payload/boundary/fallback_entries.rs"]
mod fallback_entries;
#[path = "app_game_notification_readiness_payload/boundary/labels.rs"]
mod labels;
#[path = "app_game_notification_readiness_payload/boundary/preflight_entries.rs"]
mod preflight_entries;
#[path = "app_game_notification_readiness_payload/scheduler_runtime.rs"]
mod scheduler_runtime;
#[path = "app_game_notification_readiness_payload/boundary/status_entries.rs"]
mod status_entries;
#[path = "app_game_notification_readiness_payload/boundary/status_models.rs"]
mod status_models;
#[path = "app_game_notification_readiness_payload/boundary/surface.rs"]
mod surface;

use ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel;
use ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationStatusReadModels;
use ocentra_parent_agent_protocol::constants as protocol_constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::AppGameNotificationReadinessReadModel;

use super::app_game_notification_readiness_payload::{
    app_game_notification_readiness_from_service_model, app_game_notification_readiness_payload,
};

pub(crate) struct AppGameNotificationReadinessReport {
    pub(crate) read_model: AppGameNotificationReadinessReadModel,
    pub(crate) status_read_models: AppGameNotificationStatusReadModels,
}

pub(crate) fn app_game_notification_readiness_report_from_service_model(
    model: AppGameServiceReadModel,
) -> AppGameNotificationReadinessReport {
    let read_model = app_game_notification_readiness_from_service_model(model);
    let status_read_models =
        status_models::notification_status_read_models(&read_model.rows, &read_model.generated_at);
    AppGameNotificationReadinessReport {
        read_model,
        status_read_models,
    }
}

pub(crate) fn app_game_notification_readiness_report_from_service_model_with_activity_db_path(
    model: AppGameServiceReadModel,
    activity_db_path: &Path,
) -> AppGameNotificationReadinessReport {
    let read_model = app_game_notification_readiness_from_service_model(model);
    let status_read_models = status_models::notification_status_read_models_from_activity_db_path(
        &read_model.rows,
        &read_model.generated_at,
        activity_db_path,
    );
    AppGameNotificationReadinessReport {
        read_model,
        status_read_models,
    }
}

pub(crate) fn app_game_notification_readiness_report_payload(
    report: &AppGameNotificationReadinessReport,
) -> LogFields {
    let mut payload = app_game_notification_readiness_payload(&report.read_model);
    payload.insert(
        protocol_constants::field::APP_GAME_NOTIFICATION_STATUS_READ_MODELS.to_string(),
        LogFieldValue::String(
            serde_json::to_string(&report.status_read_models).unwrap_or_default(),
        ),
    );
    payload
}
