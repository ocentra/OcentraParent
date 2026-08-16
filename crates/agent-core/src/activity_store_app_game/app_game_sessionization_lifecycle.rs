use ocentra_parent_agent_protocol::app_game::*;

use crate::activity_store_app_game_observation::AppGameObservation;

use super::{
    app_game_sessionization_foreground::apply_foreground_duration_to_summary,
    app_game_sessionization_foreground::close_focused_processes_for_open_sessions,
    app_game_sessionization_foreground::is_stronger_classification,
    app_game_sessionization_foreground::update_running_duration, SessionizationState,
};

pub(super) fn apply_observation_to_session(
    state: &mut SessionizationState,
    session_index: usize,
    observation: &AppGameObservation,
    observed_at_ms: i64,
) {
    let gap_ms = observed_at_ms
        .saturating_sub(state.active_sessions[session_index].last_observed_at_ms)
        .max(0) as u64;
    let session = &mut state.active_sessions[session_index];
    let summary = &mut session.summary;
    let is_process_observation = observation.is_process_observation();
    let is_process_exit = observation.is_process_exit();
    if summary.last_observed_at != observation.observed_at {
        summary.observation_count += 1;
    }
    summary.observation_gap_ms = summary.observation_gap_ms.max(gap_ms);
    summary.last_observed_at = observation.observed_at.clone();
    session.last_observed_at_ms = observed_at_ms;
    summary.evidence_count += observation.evidence.len() as u64;
    summary.evidence.extend(observation.evidence.clone());
    if is_process_observation {
        update_running_duration(summary, session.started_at_ms, observed_at_ms);
    }
    if is_stronger_classification(
        &observation.classification_state,
        &summary.classification_state,
    ) {
        summary.classification_state = observation.classification_state.clone();
        summary.display_name = observation.display_name.clone();
        summary.confidence = observation.confidence;
    }
    if is_process_exit {
        close_session(
            state,
            session_index,
            APP_GAME_SESSION_END_REASON_PROCESS_EXIT,
        );
    }
}

pub(super) fn close_session(state: &mut SessionizationState, session_index: usize, reason: &str) {
    let mut session = state.active_sessions.remove(session_index);
    let ended_at = if reason == APP_GAME_SESSION_END_REASON_TIMEOUT_INFERRED {
        crate::activity_store_app_game::app_game_session_time::add_millis(
            &session.summary.last_observed_at,
            super::SESSION_STALE_TIMEOUT_MS as i64,
        )
        .unwrap_or_else(|| session.summary.last_observed_at.clone())
    } else {
        session.summary.last_observed_at.clone()
    };
    if let Some(ended_at_ms) =
        crate::activity_store_app_game::app_game_session_time::timestamp_ms(&ended_at)
    {
        update_running_duration(&mut session.summary, session.started_at_ms, ended_at_ms);
    }
    if state
        .focused_process
        .as_ref()
        .map(|focus| focus.process_identity.as_str())
        == Some(session.summary.primary_process_identity.as_str())
    {
        let focus = state.focused_process.take();
        if let (Some(focus), Some(ended_at_ms)) = (
            focus,
            crate::activity_store_app_game::app_game_session_time::timestamp_ms(&ended_at),
        ) {
            let duration = ended_at_ms.saturating_sub(focus.started_at_ms).max(0) as u64;
            apply_foreground_duration_to_summary(&mut session.summary, &focus, duration, &ended_at);
        }
    }
    session.summary.ended_at = Some(ended_at);
    session.summary.end_reason = Some(reason.to_string());
    state.completed_sessions.push(session.summary);
}

pub(super) fn into_summaries(state: &mut SessionizationState) -> Vec<AppGameSessionSummary> {
    close_focused_processes_for_open_sessions(state);
    state
        .completed_sessions
        .drain(..)
        .chain(
            state
                .active_sessions
                .drain(..)
                .map(|session| session.summary),
        )
        .collect()
}
