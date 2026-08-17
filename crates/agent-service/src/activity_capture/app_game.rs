#[cfg(windows)]
use ocentra_parent_agent_core::activity_store_app_game::{
    live_windows_foreground_window_journal_event, live_windows_inventory_journal_events_with_limit,
    live_windows_process_and_launcher_snapshot_journal_events_from_system,
    live_windows_registry_inventory_journal_events_with_limit,
    live_windows_store_package_journal_events_with_limit,
};
use ocentra_parent_agent_core::process_capture::ProcessSnapshotSystem;
use ocentra_parent_agent_protocol::activity::ActivityEvent;
#[cfg(windows)]
use ocentra_parent_agent_protocol::constants;

use super::ActivityCaptureError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ObservedAtText<'a>(pub(crate) &'a str);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct CaptureLimit(pub(crate) usize);

#[cfg(windows)]
pub(super) fn live_process_and_launcher_events_from_system(
    observed_at: ObservedAtText<'_>,
    limit: CaptureLimit,
    system: &ProcessSnapshotSystem,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(
        live_windows_process_and_launcher_snapshot_journal_events_from_system(
            constants::activity_surface::DEFAULT_DEVICE_ID,
            std::env::consts::OS,
            observed_at.0,
            limit.0,
            system,
        )?,
    )
}

#[cfg(not(windows))]
pub(super) fn live_process_and_launcher_events_from_system(
    _observed_at: ObservedAtText<'_>,
    _limit: CaptureLimit,
    _system: &ProcessSnapshotSystem,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(Vec::new())
}

#[cfg(windows)]
pub(super) fn live_foreground_event(
    observed_at: ObservedAtText<'_>,
) -> Result<Option<ActivityEvent>, ActivityCaptureError> {
    Ok(live_windows_foreground_window_journal_event(
        constants::activity_surface::DEFAULT_DEVICE_ID,
        std::env::consts::OS,
        observed_at.0,
    )?)
}

#[cfg(not(windows))]
pub(super) fn live_foreground_event(
    _observed_at: ObservedAtText<'_>,
) -> Result<Option<ActivityEvent>, ActivityCaptureError> {
    Ok(None)
}

#[cfg(windows)]
pub(super) fn live_inventory_events(
    observed_at: ObservedAtText<'_>,
    limit: CaptureLimit,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(live_windows_inventory_journal_events_with_limit(
        constants::activity_surface::DEFAULT_DEVICE_ID,
        std::env::consts::OS,
        observed_at.0,
        limit.0,
    )?)
}

#[cfg(not(windows))]
pub(super) fn live_inventory_events(
    _observed_at: ObservedAtText<'_>,
    _limit: CaptureLimit,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(Vec::new())
}

#[cfg(windows)]
pub(super) fn live_store_package_events(
    observed_at: ObservedAtText<'_>,
    limit: CaptureLimit,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(live_windows_store_package_journal_events_with_limit(
        constants::activity_surface::DEFAULT_DEVICE_ID,
        std::env::consts::OS,
        observed_at.0,
        limit.0,
    )?)
}

#[cfg(not(windows))]
pub(super) fn live_store_package_events(
    _observed_at: ObservedAtText<'_>,
    _limit: CaptureLimit,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(Vec::new())
}

#[cfg(windows)]
pub(super) fn live_registry_inventory_events(
    observed_at: ObservedAtText<'_>,
    limit: CaptureLimit,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(live_windows_registry_inventory_journal_events_with_limit(
        constants::activity_surface::DEFAULT_DEVICE_ID,
        std::env::consts::OS,
        observed_at.0,
        limit.0,
    )?)
}

#[cfg(not(windows))]
pub(super) fn live_registry_inventory_events(
    _observed_at: ObservedAtText<'_>,
    _limit: CaptureLimit,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(Vec::new())
}
