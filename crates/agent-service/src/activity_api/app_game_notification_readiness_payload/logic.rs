#[path = "boundary/rows.rs"]
mod rows;

use ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel;
use ocentra_parent_agent_protocol::AppGameNotificationReadinessReadModel;

pub(super) fn app_game_notification_readiness_from_service_model(
    model: AppGameServiceReadModel,
) -> AppGameNotificationReadinessReadModel {
    rows::app_game_notification_readiness_from_service_model(model)
}
