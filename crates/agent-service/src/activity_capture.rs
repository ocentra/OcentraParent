use std::{fs, path::Path, time::Duration};

use ocentra_parent_agent_core::{
    ActivityJournal, ActivityStore, ActivityStoreError, AppGameLiveForegroundWindowError,
    AppGameLiveInventorySourceError, AppGameLiveProcessSnapshotError, JournalError, JournalKey,
    JOURNAL_KEY_BYTES,
};
use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityIngestStatus, LogFieldValue,
};

use crate::{
    activity_store_path::{activity_db_path, activity_journal_key_path, activity_journal_path},
    fields::fields_from_pairs,
    time::timestamp_now,
};

mod app_game;
pub(crate) mod capture_events;

#[cfg(test)]
pub(crate) mod freshness;

#[derive(Debug, PartialEq, Eq)]
pub enum ActivityCaptureError {
    Store,
    Journal,
    Io,
    InvalidKeyLength,
    AppGameRuntime,
}

impl ActivityCaptureError {
    pub fn reason(&self) -> &'static str {
        match self {
            Self::Store => constants::value::ACTIVITY_CAPTURE_STORE_ERROR,
            Self::Journal => constants::value::ACTIVITY_CAPTURE_JOURNAL_ERROR,
            Self::Io => constants::value::ACTIVITY_CAPTURE_IO_ERROR,
            Self::InvalidKeyLength => constants::value::ACTIVITY_CAPTURE_INVALID_KEY_LENGTH,
            Self::AppGameRuntime => constants::value::ACTIVITY_CAPTURE_APP_GAME_ERROR,
        }
    }
}

impl From<ActivityStoreError> for ActivityCaptureError {
    fn from(_: ActivityStoreError) -> Self {
        Self::Store
    }
}

impl From<JournalError> for ActivityCaptureError {
    fn from(_: JournalError) -> Self {
        Self::Journal
    }
}

impl From<std::io::Error> for ActivityCaptureError {
    fn from(_: std::io::Error) -> Self {
        Self::Io
    }
}

impl From<AppGameLiveProcessSnapshotError> for ActivityCaptureError {
    fn from(_: AppGameLiveProcessSnapshotError) -> Self {
        Self::AppGameRuntime
    }
}

impl From<AppGameLiveForegroundWindowError> for ActivityCaptureError {
    fn from(_: AppGameLiveForegroundWindowError) -> Self {
        Self::AppGameRuntime
    }
}

impl From<AppGameLiveInventorySourceError> for ActivityCaptureError {
    fn from(_: AppGameLiveInventorySourceError) -> Self {
        Self::AppGameRuntime
    }
}

pub fn spawn_startup_activity_capture() {
    if windows_activity_capture_supported() {
        tokio::task::spawn(async {
            loop {
                run_activity_capture_once_blocking().await;
                tokio::time::sleep(Duration::from_millis(
                    constants::activity_capture::RECURRING_CAPTURE_INTERVAL_MS,
                ))
                .await;
            }
        });
    }
}

async fn run_activity_capture_once_blocking() {
    let _ = tokio::task::spawn_blocking(|| {
        if let Err(error) = record_activity_capture_once() {
            log_activity_capture_error(&error);
        }
    })
    .await;
}

fn log_activity_capture_error(error: &ActivityCaptureError) {
    let _ = crate::dev_log::write_agent_info(
        constants::dev_log_message::ACTIVITY_CAPTURE_FAILED,
        fields_from_pairs(vec![(
            constants::field::REASON,
            LogFieldValue::String(error.reason().to_string()),
        )]),
    );
}

pub fn record_activity_capture_once() -> Result<ActivityIngestStatus, ActivityCaptureError> {
    record_activity_capture_to_paths(
        &activity_journal_path(),
        &activity_journal_key_path(),
        &activity_db_path(),
        constants::activity_capture::PROCESS_SNAPSHOT_LIMIT,
        constants::activity_capture::NETWORK_SNAPSHOT_LIMIT,
    )
}

#[cfg(windows)]
fn windows_activity_capture_supported() -> bool {
    true
}

#[cfg(not(windows))]
fn windows_activity_capture_supported() -> bool {
    false
}

pub fn record_activity_capture_to_paths(
    journal_path: &Path,
    key_path: &Path,
    store_path: &Path,
    process_limit: usize,
    network_limit: usize,
) -> Result<ActivityIngestStatus, ActivityCaptureError> {
    let observed_at = timestamp_now();
    record_activity_capture_to_paths_at(
        journal_path,
        key_path,
        store_path,
        process_limit,
        network_limit,
        &observed_at,
    )
}

pub(crate) fn record_activity_capture_to_paths_at(
    journal_path: &Path,
    key_path: &Path,
    store_path: &Path,
    process_limit: usize,
    network_limit: usize,
    observed_at: &str,
) -> Result<ActivityIngestStatus, ActivityCaptureError> {
    let events =
        capture_events::activity_capture_events(observed_at, process_limit, network_limit)?;
    record_activity_events_to_paths(journal_path, key_path, store_path, &events)
}

pub(crate) fn record_activity_events_to_paths(
    journal_path: &Path,
    key_path: &Path,
    store_path: &Path,
    events: &[ActivityEvent],
) -> Result<ActivityIngestStatus, ActivityCaptureError> {
    let key = load_or_create_journal_key(key_path)?;
    let mut journal = ActivityJournal::open(journal_path.to_path_buf(), key)?;
    let existing_line_count = journal.lines()?.len();
    for event in events {
        journal.append(event)?;
    }
    let mut appended_events = Vec::new();
    for line in journal.lines()?.into_iter().skip(existing_line_count) {
        appended_events.push(journal.decrypt_line(&line)?);
    }
    let store = ActivityStore::open(store_path)?;
    Ok(store.ingest_events(&appended_events)?)
}

fn load_or_create_journal_key(path: &Path) -> Result<JournalKey, ActivityCaptureError> {
    match fs::read(path) {
        Ok(bytes) => journal_key_from_bytes(&bytes),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let key = JournalKey::generate();
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(path, key.as_bytes())?;
            Ok(key)
        }
        Err(_) => Err(ActivityCaptureError::Io),
    }
}

fn journal_key_from_bytes(bytes: &[u8]) -> Result<JournalKey, ActivityCaptureError> {
    if bytes.len() != JOURNAL_KEY_BYTES {
        return Err(ActivityCaptureError::InvalidKeyLength);
    }
    let mut key = [0; JOURNAL_KEY_BYTES];
    key.copy_from_slice(bytes);
    Ok(JournalKey::from_bytes(key))
}
