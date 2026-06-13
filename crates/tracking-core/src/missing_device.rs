use ocentra_parent_agent_protocol::{
    constants, TrackingMissingDeviceEvaluationId, TrackingMissingDeviceState,
};

use crate::{evaluate_tracking_device_status, TrackingDeviceStatusInput};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingLastKnownVisibilityState {
    LastKnownOnly,
    NotLimited,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingMissingDeviceDecision {
    pub evaluation_id: TrackingMissingDeviceEvaluationId,
    pub missing_device_state: TrackingMissingDeviceState,
    pub parent_visibility_state: TrackingLastKnownVisibilityState,
}

pub fn evaluate_missing_device_mode(
    input: TrackingDeviceStatusInput,
) -> TrackingMissingDeviceDecision {
    let status = evaluate_tracking_device_status(input);
    let missing =
        status.device_status == constants::tracking_runtime::DEVICE_STATUS_OFFLINE_LAST_KNOWN_ONLY;

    TrackingMissingDeviceDecision {
        evaluation_id: TrackingMissingDeviceEvaluationId::parse(
            constants::tracking_runtime::DEFAULT_MISSING_DEVICE_EVALUATION_ID,
        )
        .expect(constants::tracking_runtime::DEFAULT_MISSING_DEVICE_EVALUATION_ID),
        missing_device_state: TrackingMissingDeviceState::parse(if missing {
            constants::tracking_runtime::MISSING_DEVICE_STATE_LAST_KNOWN_ONLY
        } else {
            constants::tracking_runtime::MISSING_DEVICE_STATE_NOT_MISSING
        })
        .expect(constants::tracking_runtime::MISSING_DEVICE_STATE_NOT_MISSING),
        parent_visibility_state: if missing {
            TrackingLastKnownVisibilityState::LastKnownOnly
        } else {
            TrackingLastKnownVisibilityState::NotLimited
        },
    }
}
