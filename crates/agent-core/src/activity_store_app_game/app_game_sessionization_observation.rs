use ocentra_parent_agent_protocol::app_game::*;
use ocentra_parent_agent_protocol::constants;

use crate::activity_store_app_game_observation::AppGameObservation;

use super::{SessionState, SessionizationState, SESSION_STALE_TIMEOUT_MS};

pub(super) fn apply_observation(state: &mut SessionizationState, observation: &AppGameObservation) {
    let Some(observed_at_ms) = crate::activity_store_app_game::app_game_session_time::timestamp_ms(
        &observation.observed_at,
    ) else {
        return;
    };

    if observation.is_foreground_observation() {
        super::app_game_sessionization_foreground::apply_foreground_transition(
            state,
            observation,
            observed_at_ms,
        );
    }

    let session_index = session_index_for_observation(state, observation, observed_at_ms);
    super::app_game_sessionization_lifecycle::apply_observation_to_session(
        state,
        session_index,
        observation,
        observed_at_ms,
    );
}

fn session_index_for_observation(
    state: &mut SessionizationState,
    observation: &AppGameObservation,
    observed_at_ms: i64,
) -> usize {
    match active_session_index(state, &observation.process_identity) {
        Some(index)
            if observation.is_process_observation()
                && process_gap_ms(state, index, observed_at_ms) > SESSION_STALE_TIMEOUT_MS =>
        {
            super::app_game_sessionization_lifecycle::close_session(
                state,
                index,
                APP_GAME_SESSION_END_REASON_TIMEOUT_INFERRED,
            );
            push_new_session(state, observation, observed_at_ms)
        }
        Some(index) => index,
        None => push_new_session(state, observation, observed_at_ms),
    }
}

fn active_session_index(state: &SessionizationState, process_identity: &str) -> Option<usize> {
    state
        .active_sessions
        .iter()
        .position(|session| session.summary.primary_process_identity == process_identity)
}

fn process_gap_ms(state: &SessionizationState, session_index: usize, observed_at_ms: i64) -> u64 {
    state.active_sessions[session_index]
        .last_process_observed_at_ms
        .map(|last_observed_at_ms| observed_at_ms.saturating_sub(last_observed_at_ms).max(0) as u64)
        .unwrap_or(0)
}

fn push_new_session(
    state: &mut SessionizationState,
    observation: &AppGameObservation,
    observed_at_ms: i64,
) -> usize {
    let mut summary = observation.clone().into_summary();
    summary.session_id = next_session_id(state, &summary.primary_process_identity);
    let is_process_observation = observation.is_process_observation();
    let session = SessionState {
        summary,
        running_started_at_ms: is_process_observation.then_some(observed_at_ms),
        last_process_observed_at_ms: is_process_observation.then_some(observed_at_ms),
        last_process_observed_at: is_process_observation.then(|| observation.observed_at.clone()),
        last_observed_at_ms: observed_at_ms,
    };
    state.active_sessions.push(session);
    state.active_sessions.len() - 1
}

fn next_session_id(state: &SessionizationState, process_identity: &str) -> String {
    let duplicate_count = state
        .completed_sessions
        .iter()
        .chain(state.active_sessions.iter().map(|session| &session.summary))
        .filter(|summary| summary.primary_process_identity == process_identity)
        .count();
    let mut session_id = APP_GAME_SESSION_ID_PREFIX.to_string();
    session_id.push_str(process_identity);
    if duplicate_count > 0 {
        session_id.push(constants::delimiter::HYPHEN);
        session_id.push_str(&duplicate_count.to_string());
    }
    session_id
}
