use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::runtime_event::TrackingLocationObservedEvent;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingLocationValidationResultState {
    Accepted,
    Rejected,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingLocationValidationDecision {
    pub validation_state: &'static str,
    pub result_state: TrackingLocationValidationResultState,
}

pub fn validate_tracking_location_observation(
    event: &TrackingLocationObservedEvent,
) -> TrackingLocationValidationDecision {
    if !(-900_000_000..=900_000_000).contains(&event.latitude_e7) {
        return rejected(constants::tracking_runtime::LOCATION_VALIDATION_REJECTED_LATITUDE);
    }

    if !(-1_800_000_000..=1_800_000_000).contains(&event.longitude_e7) {
        return rejected(constants::tracking_runtime::LOCATION_VALIDATION_REJECTED_LONGITUDE);
    }

    if event.horizontal_accuracy_meters == 0 {
        return rejected(constants::tracking_runtime::LOCATION_VALIDATION_REJECTED_ACCURACY);
    }

    TrackingLocationValidationDecision {
        validation_state: constants::tracking_runtime::LOCATION_VALIDATION_ACCEPTED,
        result_state: TrackingLocationValidationResultState::Accepted,
    }
}

fn rejected(validation_state: &'static str) -> TrackingLocationValidationDecision {
    TrackingLocationValidationDecision {
        validation_state,
        result_state: TrackingLocationValidationResultState::Rejected,
    }
}
