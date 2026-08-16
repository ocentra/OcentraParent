use ocentra_parent_agent_protocol::app_game::*;

#[path = "app_game_sessionization_foreground.rs"]
mod app_game_sessionization_foreground;
#[path = "app_game_sessionization_lifecycle.rs"]
mod app_game_sessionization_lifecycle;
#[path = "app_game_sessionization_observation.rs"]
mod app_game_sessionization_observation;

use crate::activity_store_app_game_observation::AppGameObservation;
use crate::activity_store_app_game_rows::AppGameStoreRow;

const SESSION_STALE_TIMEOUT_MS: u64 = 60_000;

pub fn session_summaries_from_rows(
    rows: Vec<AppGameStoreRow>,
    limit: u64,
) -> Vec<AppGameSessionSummary> {
    let mut observations = rows
        .into_iter()
        .map(AppGameObservation::from_row)
        .collect::<Vec<_>>();
    observations.sort_by(|left, right| {
        left.observed_at
            .cmp(&right.observed_at)
            .then_with(|| left.process_identity.cmp(&right.process_identity))
            .then_with(|| left.kind.cmp(&right.kind))
    });

    let mut state = SessionizationState::default();
    for observation in observations {
        state.apply_observation(&observation);
    }

    let mut summaries = state.into_summaries();
    summaries.sort_by(|left, right| {
        right
            .last_observed_at
            .cmp(&left.last_observed_at)
            .then_with(|| right.session_id.cmp(&left.session_id))
    });
    summaries.truncate(limit as usize);
    summaries
}

#[derive(Default)]
struct SessionizationState {
    active_sessions: Vec<SessionState>,
    completed_sessions: Vec<AppGameSessionSummary>,
    focused_process: Option<ForegroundFocus>,
}

struct SessionState {
    summary: AppGameSessionSummary,
    started_at_ms: i64,
    last_observed_at_ms: i64,
}

struct ForegroundFocus {
    process_identity: String,
    started_at: String,
    started_at_ms: i64,
}

impl SessionizationState {
    fn apply_observation(&mut self, observation: &AppGameObservation) {
        app_game_sessionization_observation::apply_observation(self, observation);
    }

    fn into_summaries(mut self) -> Vec<AppGameSessionSummary> {
        app_game_sessionization_lifecycle::into_summaries(&mut self)
    }
}
