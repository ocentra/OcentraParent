use ocentra_parent_agent_core::{
    network_capture::NetworkObservation, network_capture_event::network_snapshot_capture_results,
    process_capture::process_snapshot_events, window_capture_event::foreground_window_event,
};
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::constants;

use super::app_game::{CaptureLimit, ObservedAtText};

use super::{app_game, ActivityCaptureError};

#[derive(Clone, Debug)]
pub(crate) struct NetworkCaptureObservation {
    pub(crate) source_event_id: String,
    pub(crate) observed_at: String,
    pub(crate) observation: NetworkObservation,
}

#[derive(Clone, Debug)]
pub(super) struct ActivityCaptureBatch {
    pub(super) events: Vec<ActivityEvent>,
    pub(super) network_observations: Vec<NetworkCaptureObservation>,
}

pub(super) fn activity_capture_events(
    observed_at: ObservedAtText<'_>,
    process_limit: CaptureLimit,
    network_limit: CaptureLimit,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(activity_capture_batch(observed_at, process_limit, network_limit)?.events)
}

pub(super) fn activity_capture_batch(
    observed_at: ObservedAtText<'_>,
    process_limit: CaptureLimit,
    network_limit: CaptureLimit,
) -> Result<ActivityCaptureBatch, ActivityCaptureError> {
    let inventory_events = app_game::live_inventory_events(
        observed_at,
        CaptureLimit(constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT),
    )?;
    let store_package_events = app_game::live_store_package_events(
        observed_at,
        CaptureLimit(constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT),
    )?;
    let registry_inventory_events = app_game::live_registry_inventory_events(
        observed_at,
        CaptureLimit(constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT),
    )?;
    activity_capture_batch_with_inventory(
        observed_at,
        process_limit,
        network_limit,
        inventory_events,
        store_package_events,
        registry_inventory_events,
    )
}

fn activity_capture_events_with_inventory(
    observed_at: ObservedAtText<'_>,
    process_limit: CaptureLimit,
    network_limit: CaptureLimit,
    inventory_events: Vec<ActivityEvent>,
    store_package_events: Vec<ActivityEvent>,
    registry_inventory_events: Vec<ActivityEvent>,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(activity_capture_batch_with_inventory(
        observed_at,
        process_limit,
        network_limit,
        inventory_events,
        store_package_events,
        registry_inventory_events,
    )?
    .events)
}

fn activity_capture_events_with_inventory_sources(
    observed_at: ObservedAtText<'_>,
    process_limit: CaptureLimit,
    network_limit: CaptureLimit,
    inventory_events: Vec<ActivityEvent>,
    store_package_events: Vec<ActivityEvent>,
    registry_inventory_events: Vec<ActivityEvent>,
) -> Result<Vec<ActivityEvent>, ActivityCaptureError> {
    Ok(activity_capture_batch_with_inventory_sources(
        observed_at,
        process_limit,
        network_limit,
        inventory_events,
        store_package_events,
        registry_inventory_events,
    )?
    .events)
}

fn activity_capture_batch_with_inventory(
    observed_at: ObservedAtText<'_>,
    process_limit: CaptureLimit,
    network_limit: CaptureLimit,
    inventory_events: Vec<ActivityEvent>,
    store_package_events: Vec<ActivityEvent>,
    registry_inventory_events: Vec<ActivityEvent>,
) -> Result<ActivityCaptureBatch, ActivityCaptureError> {
    activity_capture_batch_with_inventory_sources(
        observed_at,
        process_limit,
        network_limit,
        inventory_events,
        store_package_events,
        registry_inventory_events,
    )
}

fn activity_capture_batch_with_inventory_sources(
    observed_at: ObservedAtText<'_>,
    process_limit: CaptureLimit,
    network_limit: CaptureLimit,
    inventory_events: Vec<ActivityEvent>,
    store_package_events: Vec<ActivityEvent>,
    registry_inventory_events: Vec<ActivityEvent>,
) -> Result<ActivityCaptureBatch, ActivityCaptureError> {
    let mut events = process_snapshot_events(observed_at.0, process_limit.0);
    events.push(foreground_window_event(observed_at.0));
    let mut network_observations = Vec::new();
    let mut network_events = Vec::new();
    for capture in network_snapshot_capture_results(observed_at.0, network_limit.0) {
        let (observation, event) = capture.into_parts();
        network_observations.push(NetworkCaptureObservation {
            source_event_id: event.event_id.clone(),
            observed_at: observed_at.0.to_string(),
            observation,
        });
        network_events.push(event);
    }
    events.extend(network_events);
    events.extend(app_game::live_process_events(observed_at, process_limit)?);
    events.extend(inventory_events);
    events.extend(store_package_events);
    events.extend(registry_inventory_events);
    events.extend(app_game::live_launcher_events(
        observed_at,
        CaptureLimit(constants::activity_capture::APP_GAME_INVENTORY_SNAPSHOT_LIMIT),
    )?);
    if let Some(event) = app_game::live_foreground_event(observed_at)? {
        events.push(event);
    }
    Ok(ActivityCaptureBatch {
        events,
        network_observations,
    })
}
