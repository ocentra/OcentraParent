use std::{fs, path::Path};

use ocentra_parent_agent_core::{
    foreground_window_event, network_snapshot_events, process_snapshot_events, ActivityJournal,
    ActivityStore, ActivityStoreError, JournalError, JournalKey, JOURNAL_KEY_BYTES,
};
use ocentra_parent_agent_protocol::{constants, ActivityIngestStatus, LogFieldValue};

use crate::{
    activity_store_path::{activity_db_path, activity_journal_key_path, activity_journal_path},
    fields::fields_from_pairs,
    time::timestamp_now,
};

#[derive(Debug, PartialEq, Eq)]
pub enum ActivityCaptureError {
    Store,
    Journal,
    Io,
    InvalidKeyLength,
}

impl ActivityCaptureError {
    pub fn reason(&self) -> &'static str {
        match self {
            Self::Store => constants::value::ACTIVITY_CAPTURE_STORE_ERROR,
            Self::Journal => constants::value::ACTIVITY_CAPTURE_JOURNAL_ERROR,
            Self::Io => constants::value::ACTIVITY_CAPTURE_IO_ERROR,
            Self::InvalidKeyLength => constants::value::ACTIVITY_CAPTURE_INVALID_KEY_LENGTH,
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

pub fn spawn_startup_activity_capture() {
    if windows_activity_capture_supported() {
        tokio::task::spawn_blocking(|| {
            if let Err(error) = record_activity_capture_once() {
                let _ = crate::dev_log::write_agent_info(
                    constants::dev_log_message::ACTIVITY_CAPTURE_FAILED,
                    fields_from_pairs(vec![(
                        constants::field::REASON,
                        LogFieldValue::String(error.reason().to_string()),
                    )]),
                );
            }
        });
    }
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
    let key = load_or_create_journal_key(key_path)?;
    let observed_at = timestamp_now();
    let mut events = process_snapshot_events(&observed_at, process_limit);
    events.push(foreground_window_event(&observed_at));
    events.extend(network_snapshot_events(&observed_at, network_limit));
    let mut journal = ActivityJournal::open(journal_path.to_path_buf(), key)?;
    for event in &events {
        journal.append(event)?;
    }
    let store = ActivityStore::open(store_path)?;
    Ok(store.ingest_events(&events)?)
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
