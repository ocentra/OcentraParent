use std::{fs, io::ErrorKind, path::Path};

use ocentra_parent_agent_core::{active_timer_state_from_outcome, EnforcementBoundaryOutcome};
use ocentra_parent_agent_protocol::{constants, EnforcementActiveTimerState};

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
        .map_err(|_| constants::enforcement::REJECTION_ACTIVE_TIMER_STATE_REQUIRED)?
}

pub(crate) async fn remove_active_timer_state(path: &Path) -> Result<(), &'static str> {
    let path = path.to_path_buf();
    tokio::task::spawn_blocking(move || remove_active_timer_state_sync(&path))
        .await
        .map_err(|_| constants::value::ACTIVITY_CAPTURE_STORE_ERROR)?
}

async fn write_active_timer_state(
    path: &Path,
    state: &EnforcementActiveTimerState,
) -> Result<(), &'static str> {
    let path = path.to_path_buf();
    let state = state.clone();
    tokio::task::spawn_blocking(move || write_active_timer_state_sync(&path, &state))
        .await
        .map_err(|_| constants::value::ACTIVITY_CAPTURE_STORE_ERROR)?
}

fn read_active_timer_state_sync(
    path: &Path,
) -> Result<Option<EnforcementActiveTimerState>, &'static str> {
    match fs::read_to_string(path) {
        Ok(text) => serde_json::from_str(&text)
            .map(Some)
            .map_err(|_| constants::enforcement::REJECTION_ACTIVE_TIMER_STATE_REQUIRED),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(_) => Err(constants::enforcement::REJECTION_ACTIVE_TIMER_STATE_REQUIRED),
    }
}

fn write_active_timer_state_sync(
    path: &Path,
    state: &EnforcementActiveTimerState,
) -> Result<(), &'static str> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|_| constants::value::ACTIVITY_CAPTURE_STORE_ERROR)?;
    }
    let text = serde_json::to_string_pretty(state)
        .map_err(|_| constants::error::AGENT_EVENT_SERIALIZES)?;
    fs::write(path, text).map_err(|_| constants::value::ACTIVITY_CAPTURE_STORE_ERROR)
}

fn remove_active_timer_state_sync(path: &Path) -> Result<(), &'static str> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(_) => Err(constants::value::ACTIVITY_CAPTURE_STORE_ERROR),
    }
}
