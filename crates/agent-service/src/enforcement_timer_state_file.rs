use std::{fs, io::ErrorKind, path::Path};

use ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryOutcome;
use ocentra_parent_agent_core::enforcement_timer_state::active_timer_state_from_outcome;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::enforcement::EnforcementActiveTimerState;

pub(crate) async fn store_active_timer_state_for_outcome(
    outcome: &EnforcementBoundaryOutcome,
    path: &Path,
    stored_at: &str,
) -> Result<Option<EnforcementActiveTimerState>, &'static str> {
    match active_timer_state_from_outcome(outcome, stored_at) {
        Some(state) => {
            write_active_timer_state(path, &state).await?;
            Ok(Some(state))
        }
        None => Ok(None),
    }
}

pub(crate) async fn read_active_timer_state(
    path: &Path,
) -> Result<Option<EnforcementActiveTimerState>, &'static str> {
    let path = path.to_path_buf();
    tokio::task::spawn_blocking(move || read_active_timer_state_sync(&path))
        .await
        .map_err(active_timer_state_required_error)?
}

pub(crate) async fn remove_active_timer_state(path: &Path) -> Result<(), &'static str> {
    let path = path.to_path_buf();
    tokio::task::spawn_blocking(move || remove_active_timer_state_sync(&path))
        .await
        .map_err(activity_capture_store_error)?
}

async fn write_active_timer_state(
    path: &Path,
    state: &EnforcementActiveTimerState,
) -> Result<(), &'static str> {
    let path = path.to_path_buf();
    let state = state.clone();
    tokio::task::spawn_blocking(move || write_active_timer_state_sync(&path, &state))
        .await
        .map_err(activity_capture_store_error)?
}

fn read_active_timer_state_sync(
    path: &Path,
) -> Result<Option<EnforcementActiveTimerState>, &'static str> {
    match fs::read_to_string(path) {
        Ok(text) => serde_json::from_str(&text)
            .map(Some)
            .map_err(active_timer_state_deserializes_error),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(_) => Err(constants::enforcement::REJECTION_ACTIVE_TIMER_STATE_REQUIRED),
    }
}

fn write_active_timer_state_sync(
    path: &Path,
    state: &EnforcementActiveTimerState,
) -> Result<(), &'static str> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(activity_capture_store_error)?;
    }
    let text = serde_json::to_string_pretty(state).map_err(agent_event_serializes_error)?;
    fs::write(path, text).map_err(activity_capture_store_error)
}

fn remove_active_timer_state_sync(path: &Path) -> Result<(), &'static str> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(_) => Err(constants::value::ACTIVITY_CAPTURE_STORE_ERROR),
    }
}

fn active_timer_state_required_error(_: impl std::fmt::Debug) -> &'static str {
    constants::enforcement::REJECTION_ACTIVE_TIMER_STATE_REQUIRED
}

fn active_timer_state_deserializes_error(_: serde_json::Error) -> &'static str {
    constants::enforcement::REJECTION_ACTIVE_TIMER_STATE_REQUIRED
}

fn activity_capture_store_error(_: impl std::fmt::Debug) -> &'static str {
    constants::value::ACTIVITY_CAPTURE_STORE_ERROR
}

fn agent_event_serializes_error(_: serde_json::Error) -> &'static str {
    constants::error::AGENT_EVENT_SERIALIZES
}
