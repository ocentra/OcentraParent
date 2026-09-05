use std::path::Path;

use crate::activity_capture_network_observation::NetworkCaptureObservation;
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::activity_query::ActivityIngestStatus;

#[path = "runtime.rs"]
mod runtime;

use super::{
    app_game::{CaptureLimit, ObservedAtText},
    capture_events, ActivityCaptureError,
};

pub(crate) struct StartupActivityCaptureDisabledValue<'a>(pub(crate) Option<&'a str>);

pub(crate) struct ActivityCaptureObservedAt<'a>(pub(crate) &'a str);

pub(crate) fn startup_activity_capture_enabled_for_value(
    value: &StartupActivityCaptureDisabledValue<'_>,
) -> bool {
    runtime::startup_activity_capture_enabled_for_value(value)
}

pub(crate) fn record_activity_capture_to_paths_at_with_network(
    journal_path: &Path,
    key_path: &Path,
    store_path: &Path,
    process_limit: usize,
    network_limit: usize,
    observed_at: &ActivityCaptureObservedAt<'_>,
) -> Result<(ActivityIngestStatus, Vec<NetworkCaptureObservation>), ActivityCaptureError> {
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
    Ok(
        crate::activity_capture_persistence::record_activity_events_to_paths(
            journal_path,
            key_path,
            store_path,
            events,
        )?,
    )
}
