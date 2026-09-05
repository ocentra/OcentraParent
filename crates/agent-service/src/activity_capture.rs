use ocentra_parent_agent_protocol::constants;

use crate::{
    activity_capture_network_observation::NetworkCaptureObservation,
    activity_store_path::{activity_db_path, activity_journal_key_path, activity_journal_path},
    time::timestamp_now,
};

#[path = "activity_capture/app_game.rs"]
mod app_game;
#[path = "activity_capture/capture_events.rs"]
pub(crate) mod capture_events;
#[path = "activity_capture/errors.rs"]
mod errors;
#[path = "activity_capture/injected.rs"]
mod injected;
pub(crate) type ActivityCaptureError = errors::ActivityCaptureError;

pub(crate) fn startup_activity_capture_enabled() -> bool {
    let disabled = std::env::var(constants::env_var::ACTIVITY_CAPTURE_STARTUP_DISABLED).ok();
    injected::startup_activity_capture_enabled_for_value(
        &injected::StartupActivityCaptureDisabledValue(disabled.as_deref()),
    )
}

pub(crate) struct CapturedActivityIngest {
    pub(crate) network_observations: Vec<NetworkCaptureObservation>,
}

pub(crate) fn record_activity_capture_once_with_network(
) -> Result<CapturedActivityIngest, ActivityCaptureError> {
    let observed_at = timestamp_now::<String>();
    let status = injected::record_activity_capture_to_paths_at_with_network(
        activity_journal_path().as_ref(),
        activity_journal_key_path().as_ref(),
        activity_db_path().as_ref(),
        constants::activity_capture::PROCESS_SNAPSHOT_LIMIT,
        constants::activity_capture::NETWORK_SNAPSHOT_LIMIT,
        &injected::ActivityCaptureObservedAt(observed_at.as_str()),
    )?;
    Ok(CapturedActivityIngest {
        network_observations: status.1,
    })
}
