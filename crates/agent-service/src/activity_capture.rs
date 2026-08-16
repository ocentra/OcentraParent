use std::{fs, path::Path};

use ocentra_parent_agent_core::journal_crypto::{JournalKey, JOURNAL_KEY_BYTES};
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::activity_query::ActivityIngestStatus;
use ocentra_parent_agent_protocol::constants;

use crate::{
    activity_store_path::{activity_db_path, activity_journal_key_path, activity_journal_path},
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
#[path = "activity_capture/runtime.rs"]
mod runtime;
pub(crate) type ActivityCaptureError = errors::ActivityCaptureError;

use app_game::{CaptureLimit, ObservedAtText};

pub(crate) struct StartupActivityCaptureDisabledValue<'a>(pub(crate) Option<&'a str>);

pub(crate) struct ActivityCaptureObservedAt<'a>(pub(crate) &'a str);

pub(crate) fn startup_activity_capture_enabled() -> bool {
    runtime::startup_activity_capture_enabled()
}

pub(crate) fn startup_activity_capture_enabled_for_value(
    value: &StartupActivityCaptureDisabledValue<'_>,
) -> bool {
    runtime::startup_activity_capture_enabled_for_value(value)
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

pub(crate) struct CapturedActivityIngest {
    pub(crate) network_observations: Vec<capture_events::NetworkCaptureObservation>,
}

pub(crate) fn record_activity_capture_once_with_network(
) -> Result<CapturedActivityIngest, ActivityCaptureError> {
    let observed_at = timestamp_now::<String>();
    let status = record_activity_capture_to_paths_at_with_network(
        activity_journal_path().as_ref(),
        activity_journal_key_path().as_ref(),
        activity_db_path().as_ref(),
        constants::activity_capture::PROCESS_SNAPSHOT_LIMIT,
        constants::activity_capture::NETWORK_SNAPSHOT_LIMIT,
        &ActivityCaptureObservedAt(observed_at.as_str()),
    )?;
    Ok(CapturedActivityIngest {
        network_observations: status.1,
    })
}

pub(crate) fn record_activity_capture_to_paths_at(
    journal_path: &Path,
    key_path: &Path,
    store_path: &Path,
    process_limit: usize,
    network_limit: usize,
    observed_at: &ActivityCaptureObservedAt<'_>,
) -> Result<ActivityIngestStatus, ActivityCaptureError> {
    Ok(record_activity_capture_to_paths_at_with_network(
        journal_path,
        key_path,
        store_path,
        process_limit,
        network_limit,
        observed_at,
    )?
    .0)
}

fn record_activity_capture_to_paths_at_with_network(
    journal_path: &Path,
    key_path: &Path,
    store_path: &Path,
    process_limit: usize,
    network_limit: usize,
    observed_at: &ActivityCaptureObservedAt<'_>,
) -> Result<
    (
        ActivityIngestStatus,
        Vec<capture_events::NetworkCaptureObservation>,
    ),
    ActivityCaptureError,
> {
    let batch = capture_events::activity_capture_batch(
        ObservedAtText(observed_at.0),
        CaptureLimit(process_limit),
        CaptureLimit(network_limit),
    )?;
    let status =
        record_activity_events_to_paths(journal_path, key_path, store_path, &batch.events)?;
    Ok((status, batch.network_observations))
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
