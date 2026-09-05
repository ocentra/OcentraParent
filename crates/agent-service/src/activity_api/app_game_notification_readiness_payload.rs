#[path = "app_game_notification_readiness_payload/evidence.rs"]
mod evidence;
#[path = "app_game_notification_readiness_payload/logic.rs"]
mod logic;

use ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::AppGameNotificationReadinessReadModel;

use crate::fields::fields_from_pairs;

pub fn app_game_notification_readiness_from_service_model(
    model: AppGameServiceReadModel,
) -> AppGameNotificationReadinessReadModel {
    logic::app_game_notification_readiness_from_service_model(model)
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
