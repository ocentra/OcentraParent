#[path = "boundary/constants.rs"]
mod constants;
#[path = "boundary/fallback_entries.rs"]
mod fallback_entries;
#[path = "boundary/labels.rs"]
mod labels;
#[path = "boundary/preflight_entries.rs"]
mod preflight_entries;
#[path = "boundary/rows.rs"]
mod rows;
#[path = "boundary/status_entries.rs"]
mod status_entries;
#[path = "boundary/status_models.rs"]
mod status_models;
#[path = "boundary/surface.rs"]
mod surface;

use ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel;
use ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationStatusReadModels;
use ocentra_parent_agent_protocol::AppGameNotificationReadinessReadModel;
use ocentra_parent_agent_protocol::AppGameNotificationReadinessRow;

pub(super) fn app_game_notification_readiness_from_service_model(
    model: AppGameServiceReadModel,
) -> AppGameNotificationReadinessReadModel {
    rows::app_game_notification_readiness_from_service_model(model)
}

pub(super) fn notification_status_read_models<T: ToString>(
    rows: &[AppGameNotificationReadinessRow],
    generated_at: T,
) -> AppGameNotificationStatusReadModels {
    status_models::notification_status_read_models(rows, generated_at)
}
