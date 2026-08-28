use ocentra_parent_agent_protocol::app_game::*;

use crate::activity_store_app_game_observation::AppGameObservation;

use super::{ForegroundFocus, SessionizationState};

pub(super) fn apply_foreground_transition(
    state: &mut SessionizationState,
    observation: &AppGameObservation,
    observed_at_ms: i64,
) {
    if !observation.foreground_active {
        close_focused_process(state, observed_at_ms, &observation.observed_at);
        return;
    }

    match state.focused_process.as_ref() {
        Some(focus) if focus.process_identity == observation.process_identity => return,
        Some(_) => close_focused_process(state, observed_at_ms, &observation.observed_at),
        None => {}
    }

    state.focused_process = Some(ForegroundFocus {
        process_identity: observation.process_identity.clone(),
        started_at: observation.observed_at.clone(),
        started_at_ms: observed_at_ms,
    });
    if let Some(index) = active_session_index(state, &observation.process_identity) {
        state.active_sessions[index].summary.last_foreground_at =
            Some(observation.observed_at.clone());
    }
}

pub(super) fn close_focused_process(
    state: &mut SessionizationState,
    ended_at_ms: i64,
    ended_at: &str,
) {
    let Some(focus) = state.focused_process.take() else {
        return;
    };
    apply_foreground_duration(state, &focus, ended_at_ms, ended_at);
}

pub(super) fn apply_foreground_duration(
    state: &mut SessionizationState,
    focus: &ForegroundFocus,
    ended_at_ms: i64,
    ended_at: &str,
) {
    let Some(index) = active_session_index(state, &focus.process_identity) else {
        return;
    };
    let duration = ended_at_ms.saturating_sub(focus.started_at_ms).max(0) as u64;
    apply_foreground_duration_to_summary(
        &mut state.active_sessions[index].summary,
        focus,
        duration,
        ended_at,
    );
}

pub(super) fn apply_foreground_duration_to_summary(
    summary: &mut AppGameSessionSummary,
    focus: &ForegroundFocus,
    duration: u64,
    ended_at: &str,
) {
    let unclamped_foreground = summary.foreground_duration_ms.saturating_add(duration);
    summary.foreground_duration_ms = unclamped_foreground.min(summary.running_duration_ms);
    summary.background_duration_ms = summary
        .running_duration_ms
        .saturating_sub(summary.foreground_duration_ms);
    summary.last_foreground_at = Some(focus.started_at.clone());
    summary.last_background_at = Some(ended_at.to_string());
}

pub(super) fn update_running_duration(
    summary: &mut AppGameSessionSummary,
    started_at_ms: i64,
    observed_at_ms: i64,
) {
    let running_duration = observed_at_ms.saturating_sub(started_at_ms).max(0) as u64;
    summary.running_duration_ms = summary.running_duration_ms.max(running_duration);
    summary.background_duration_ms = summary
        .running_duration_ms
        .saturating_sub(summary.foreground_duration_ms);
    if summary.background_duration_ms > 0 {
        summary.last_background_at = Some(summary.last_observed_at.clone());
    }
}

pub(super) fn close_focused_processes_for_open_sessions(state: &mut SessionizationState) {
    let Some(focus) = state.focused_process.take() else {
        return;
    };
    let Some(index) = active_session_index(state, &focus.process_identity) else {
        return;
    };
    let ended_at_ms = state.active_sessions[index].last_observed_at_ms;
    let ended_at = state.active_sessions[index]
        .summary
        .last_observed_at
        .clone();
    apply_foreground_duration(state, &focus, ended_at_ms, &ended_at);
}

fn active_session_index(state: &SessionizationState, process_identity: &str) -> Option<usize> {
    state
        .active_sessions
        .iter()
        .position(|session| session.summary.primary_process_identity == process_identity)
}

pub(crate) fn is_stronger_classification(candidate: &str, current: &str) -> bool {
    classification_rank(candidate) > classification_rank(current)
}

fn classification_rank(value: &str) -> u8 {
    match value {
        APP_GAME_CLASSIFICATION_POSSIBLY_GAME => 2,
        APP_GAME_CLASSIFICATION_PERMISSION_LIMITED => 1,
        APP_GAME_CLASSIFICATION_ADAPTER_ERROR => 1,
        _ => 0,
    }
}
