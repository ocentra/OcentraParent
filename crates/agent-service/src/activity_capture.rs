use std::{fs, path::Path, time::Duration};

use ocentra_parent_agent_core::journal_crypto::{JournalKey, JOURNAL_KEY_BYTES};
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::activity_query::ActivityIngestStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

use crate::{
    activity_store_path::{activity_db_path, activity_journal_key_path, activity_journal_path},
    fields::fields_from_pairs,
    time::timestamp_now,
};

#[path = "activity_capture/app_game.rs"]
mod app_game;
#[path = "activity_capture/capture_events.rs"]
pub(crate) mod capture_events;
#[path = "activity_capture/errors.rs"]
mod errors;
#[path = "activity_capture/persistence.rs"]
mod persistence;
pub(crate) type ActivityCaptureError = errors::ActivityCaptureError;

pub(crate) struct StartupActivityCaptureDisabledValue<'a>(pub(crate) Option<&'a str>);

pub(crate) struct ActivityCaptureObservedAt<'a>(pub(crate) &'a str);

pub fn spawn_startup_activity_capture() {
    if !startup_activity_capture_enabled() {
        return;
    }
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

pub(crate) fn startup_activity_capture_enabled() -> bool {
    startup_activity_capture_enabled_for_value(&StartupActivityCaptureDisabledValue(
        std::env::var(constants::env_var::ACTIVITY_CAPTURE_STARTUP_DISABLED)
            .ok()
            .as_deref(),
    ))
}

pub(crate) fn startup_activity_capture_enabled_for_value(
    value: &StartupActivityCaptureDisabledValue<'_>,
) -> bool {
    windows_activity_capture_supported() && value.0 != Some(constants::value::TRUE)
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
        activity_journal_path().as_ref(),
        activity_journal_key_path().as_ref(),
        activity_db_path().as_ref(),
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
    let observed_at: String = timestamp_now();
    record_activity_capture_to_paths_at(
        journal_path,
        key_path,
        store_path,
        process_limit,
        network_limit,
        &ActivityCaptureObservedAt(observed_at.as_str()),
    )
}

pub(crate) fn record_activity_capture_to_paths_at(
    journal_path: &Path,
    key_path: &Path,
    store_path: &Path,
    process_limit: usize,
    network_limit: usize,
    observed_at: &ActivityCaptureObservedAt<'_>,
) -> Result<ActivityIngestStatus, ActivityCaptureError> {
    let events = capture_events::activity_capture_events(
        capture_events::ObservedAtText(Box::leak(observed_at.0.to_string().into_boxed_str())),
        capture_events::CaptureLimit(process_limit),
        capture_events::CaptureLimit(network_limit),
    )?;
    record_activity_events_to_paths(journal_path, key_path, store_path, &events)
}

pub(crate) fn record_activity_events_to_paths(
    journal_path: &Path,
    key_path: &Path,
    store_path: &Path,
    events: &[ActivityEvent],
) -> Result<ActivityIngestStatus, ActivityCaptureError> {
    persistence::record_unseen_activity_events(journal_path, key_path, store_path, events)
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
