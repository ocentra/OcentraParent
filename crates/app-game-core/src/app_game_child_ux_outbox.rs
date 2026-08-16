use ocentra_eventing::error::EventingError;

use crate::app_game_child_ux_outbox_mapping::{blocked_refs, build_record};
use crate::app_game_child_ux_outbox_types::{AppGameChildUxOutboxInput, AppGameChildUxOutboxRoute};
use crate::app_game_child_ux_outbox_validation::{is_deliverable, validate_input};

pub fn build_app_game_child_ux_outbox_route(
    input: AppGameChildUxOutboxInput,
) -> Result<AppGameChildUxOutboxRoute, EventingError> {
    validate_input(&input)?;
    if is_deliverable(input.notice.state) {
        Ok(AppGameChildUxOutboxRoute::Queued(Box::new(build_record(
            input,
        ))))
    } else {
        Ok(AppGameChildUxOutboxRoute::Blocked {
            state: input.notice.state,
            blocked_reference_ids: blocked_refs(&input.notice),
        })
    }
}
