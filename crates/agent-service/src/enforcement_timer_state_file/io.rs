use std::io::{ErrorKind, Write};

use atomicwrites::{AllowOverwrite, AtomicFile};
use ocentra_parent_agent_core::enforcement_timer_state::active_timer_state_is_consistent;
use ocentra_parent_agent_protocol::enforcement::EnforcementActiveTimerState;

use crate::enforcement_timer_state_path::EnforcementTimerStatePath;

use super::EnforcementTimerStateFileError;

pub(crate) async fn read_active_timer_state(
    path: &EnforcementTimerStatePath,
) -> Result<Option<EnforcementActiveTimerState>, EnforcementTimerStateFileError> {
    let path = path.clone();
    tokio::task::spawn_blocking(move || read_active_timer_state_sync(&path))
        .await
        .map_err(activity_capture_store_error)?
}

pub(crate) async fn remove_active_timer_state(
    path: &EnforcementTimerStatePath,
) -> Result<(), EnforcementTimerStateFileError> {
    let path = path.clone();
    tokio::task::spawn_blocking(move || remove_active_timer_state_sync(&path))
        .await
        .map_err(activity_capture_store_error)?
}

pub(crate) async fn write_active_timer_state(
    path: &EnforcementTimerStatePath,
    state: &EnforcementActiveTimerState,
) -> Result<(), EnforcementTimerStateFileError> {
    let path = path.clone();
    let state = state.clone();
    tokio::task::spawn_blocking(move || write_active_timer_state_sync(&path, &state))
        .await
        .map_err(activity_capture_store_error)?
}

fn read_active_timer_state_sync(
    path: &EnforcementTimerStatePath,
) -> Result<Option<EnforcementActiveTimerState>, EnforcementTimerStateFileError> {
    match std::fs::read_to_string(&path.0) {
        Ok(text) => match serde_json::from_str(&text) {
            Ok(state) if active_timer_state_is_consistent(&state) => Ok(Some(state)),
            Ok(_) | Err(_) => Ok(None),
        },
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(error) if error.kind() == ErrorKind::InvalidData => Ok(None),
        Err(_) => Err(EnforcementTimerStateFileError::ActiveTimerStateRequired),
    }
}

fn write_active_timer_state_sync(
    path: &EnforcementTimerStatePath,
    state: &EnforcementActiveTimerState,
) -> Result<(), EnforcementTimerStateFileError> {
    if let Some(parent) = path.parent_dir() {
        parent.create_all().map_err(activity_capture_store_error)?;
    }
    let text = serde_json::to_string_pretty(state).map_err(agent_event_serializes_error)?;
    AtomicFile::new(&path.0, AllowOverwrite)
        .write(|file| {
            file.write_all(text.as_bytes())?;
            file.sync_all()
        })
        .map_err(activity_capture_store_error)
}

fn remove_active_timer_state_sync(
    path: &EnforcementTimerStatePath,
) -> Result<(), EnforcementTimerStateFileError> {
    match std::fs::remove_file(&path.0) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(_) => Err(EnforcementTimerStateFileError::Store),
    }
}

fn activity_capture_store_error(_: impl std::fmt::Debug) -> EnforcementTimerStateFileError {
    EnforcementTimerStateFileError::Store
}

fn agent_event_serializes_error(_: serde_json::Error) -> EnforcementTimerStateFileError {
    EnforcementTimerStateFileError::Serialize
}
