use ocentra_eventing::{error::EventingError, ids::EventId, replay::ReplayFilter};

use crate::activity_capture_network_observation::NetworkCaptureObservation;

use super::projection::RetainedEventIds;

const MAX_RECONCILED_OBSERVATIONS_PER_PASS: usize = 32;

pub(super) async fn publish_missing_observations(
    observations: Vec<NetworkCaptureObservation>,
) -> Result<(), EventingError> {
    let spine = super::shared_network_runtime_spine().await?;
    let replay = spine.replay_projection(ReplayFilter::all()).await?;
    let retained = super::projection::retained_event_ids_from_replay(&replay);
    let missing = select_missing_observations(observations, &retained)?;
    super::publish_captured_network_observations(&missing).await
}

fn select_missing_observations(
    observations: Vec<NetworkCaptureObservation>,
    retained: &RetainedEventIds,
) -> Result<Vec<NetworkCaptureObservation>, EventingError> {
    let planned = observations
        .into_iter()
        .map(|captured| {
            let source_event_id = EventId::parse(captured.source_event_id.clone())?;
            let expected =
                super::projection::expected_event_ids(&source_event_id, &captured.observation)?;
            Ok((captured, expected))
        })
        .collect::<Result<Vec<_>, EventingError>>()?;
    Ok(planned
        .into_iter()
        .filter(|(_, expected)| !retained.contains_all(expected))
        .take(MAX_RECONCILED_OBSERVATIONS_PER_PASS)
        .map(|(captured, _)| captured)
        .collect())
}
