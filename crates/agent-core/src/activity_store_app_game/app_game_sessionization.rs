use ocentra_parent_agent_protocol::app_game::*;
use ocentra_parent_agent_protocol::constants;

use crate::{
    activity_store_app_game_observation::AppGameObservation,
    activity_store_app_game_rows::AppGameStoreRow,
};

use super::app_game_session_time::{add_millis, timestamp_ms};

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
        let Some(observed_at_ms) = timestamp_ms(&observation.observed_at) else {
            return;
        };

        if observation.is_foreground_observation() {
            self.apply_foreground_transition(observation, observed_at_ms);
        }

        let session_index = self.session_index_for_observation(observation, observed_at_ms);
        self.apply_observation_to_session(session_index, observation, observed_at_ms);
    }

    fn session_index_for_observation(
        &mut self,
        observation: &AppGameObservation,
        observed_at_ms: i64,
    ) -> usize {
        match self.active_session_index(&observation.process_identity) {
            Some(index)
                if self.session_gap_ms(index, observed_at_ms) > SESSION_STALE_TIMEOUT_MS =>
            {
                self.close_session(index, APP_GAME_SESSION_END_REASON_TIMEOUT_INFERRED);
                self.push_new_session(observation, observed_at_ms)
            }
            Some(index) => index,
            None => self.push_new_session(observation, observed_at_ms),
        }
    }

    fn active_session_index(&self, process_identity: &str) -> Option<usize> {
        self.active_sessions
            .iter()
            .position(|session| session.summary.primary_process_identity == process_identity)
    }

    fn session_gap_ms(&self, session_index: usize, observed_at_ms: i64) -> u64 {
        observed_at_ms
            .saturating_sub(self.active_sessions[session_index].last_observed_at_ms)
            .max(0) as u64
    }

    fn push_new_session(&mut self, observation: &AppGameObservation, observed_at_ms: i64) -> usize {
        let mut summary = observation.clone().into_summary();
        summary.session_id = self.next_session_id(&summary.primary_process_identity);
        let session = SessionState {
            summary,
            started_at_ms: observed_at_ms,
            last_observed_at_ms: observed_at_ms,
        };
        self.active_sessions.push(session);
        self.active_sessions.len() - 1
    }

    fn next_session_id(&self, process_identity: &str) -> String {
        let duplicate_count = self
            .completed_sessions
            .iter()
            .chain(self.active_sessions.iter().map(|session| &session.summary))
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

    fn apply_observation_to_session(
        &mut self,
        session_index: usize,
        observation: &AppGameObservation,
        observed_at_ms: i64,
    ) {
        let gap_ms = self.session_gap_ms(session_index, observed_at_ms);
        let session = &mut self.active_sessions[session_index];
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
            self.close_session(session_index, APP_GAME_SESSION_END_REASON_PROCESS_EXIT);
        }
    }

    fn apply_foreground_transition(
        &mut self,
        observation: &AppGameObservation,
        observed_at_ms: i64,
    ) {
        if !observation.foreground_active {
            self.close_focused_process(observed_at_ms, &observation.observed_at);
            return;
        }

        match self.focused_process.as_ref() {
            Some(focus) if focus.process_identity == observation.process_identity => {}
            Some(_) => self.close_focused_process(observed_at_ms, &observation.observed_at),
            None => {}
        }

        self.focused_process = Some(ForegroundFocus {
            process_identity: observation.process_identity.clone(),
            started_at: observation.observed_at.clone(),
            started_at_ms: observed_at_ms,
        });
        if let Some(index) = self.active_session_index(&observation.process_identity) {
            self.active_sessions[index].summary.last_foreground_at =
                Some(observation.observed_at.clone());
        }
    }

    fn close_focused_process(&mut self, ended_at_ms: i64, ended_at: &str) {
        let Some(focus) = self.focused_process.take() else {
            return;
        };
        self.apply_foreground_duration(&focus, ended_at_ms, ended_at);
    }

    fn apply_foreground_duration(
        &mut self,
        focus: &ForegroundFocus,
        ended_at_ms: i64,
        ended_at: &str,
    ) {
        let Some(index) = self.active_session_index(&focus.process_identity) else {
            return;
        };
        let duration = ended_at_ms.saturating_sub(focus.started_at_ms).max(0) as u64;
        apply_foreground_duration_to_summary(
            &mut self.active_sessions[index].summary,
            focus,
            duration,
            ended_at,
        );
    }

    fn close_session(&mut self, session_index: usize, reason: &str) {
        let mut session = self.active_sessions.remove(session_index);
        let ended_at = if reason == APP_GAME_SESSION_END_REASON_TIMEOUT_INFERRED {
            add_millis(
                &session.summary.last_observed_at,
                SESSION_STALE_TIMEOUT_MS as i64,
            )
            .unwrap_or_else(|| session.summary.last_observed_at.clone())
        } else {
            session.summary.last_observed_at.clone()
        };
        if let Some(ended_at_ms) = timestamp_ms(&ended_at) {
            update_running_duration(&mut session.summary, session.started_at_ms, ended_at_ms);
        }
        if self
            .focused_process
            .as_ref()
            .map(|focus| focus.process_identity.as_str())
            == Some(session.summary.primary_process_identity.as_str())
        {
            let focus = self.focused_process.take();
            if let (Some(focus), Some(ended_at_ms)) = (focus, timestamp_ms(&ended_at)) {
                let duration = ended_at_ms.saturating_sub(focus.started_at_ms).max(0) as u64;
                apply_foreground_duration_to_summary(
                    &mut session.summary,
                    &focus,
                    duration,
                    &ended_at,
                );
            }
        }
        session.summary.ended_at = Some(ended_at);
        session.summary.end_reason = Some(reason.to_string());
        self.completed_sessions.push(session.summary);
    }

    fn into_summaries(mut self) -> Vec<AppGameSessionSummary> {
        self.close_focused_processes_for_open_sessions();
        self.completed_sessions
            .into_iter()
            .chain(
                self.active_sessions
                    .into_iter()
                    .map(|session| session.summary),
            )
            .collect()
    }

    fn close_focused_processes_for_open_sessions(&mut self) {
        let Some(focus) = self.focused_process.take() else {
            return;
        };
        let Some(index) = self.active_session_index(&focus.process_identity) else {
            return;
        };
        let ended_at_ms = self.active_sessions[index].last_observed_at_ms;
        let ended_at = self.active_sessions[index].summary.last_observed_at.clone();
        self.apply_foreground_duration(&focus, ended_at_ms, &ended_at);
    }
}

fn apply_foreground_duration_to_summary(
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

fn update_running_duration(
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

fn is_stronger_classification(candidate: &str, current: &str) -> bool {
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
