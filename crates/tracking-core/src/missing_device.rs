use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    tracking_missing_device_evaluation_id_from_child_device_id, TrackingMissingDeviceEvaluationId,
    TrackingMissingDeviceState,
};

use crate::status::{evaluate_tracking_device_status, TrackingDeviceStatusInput};

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
    let child_device_id = input.child_device_id.clone();
    let status = evaluate_tracking_device_status(input);
    let missing =
        status.device_status == constants::tracking_runtime::DEVICE_STATUS_OFFLINE_LAST_KNOWN_ONLY;
    let missing_device_state = if missing {
        constants::tracking_runtime::MISSING_DEVICE_STATE_LAST_KNOWN_ONLY
    } else {
        constants::tracking_runtime::MISSING_DEVICE_STATE_NOT_MISSING
    };

    TrackingMissingDeviceDecision {
        evaluation_id: tracking_missing_device_evaluation_id_from_child_device_id(&child_device_id),
        missing_device_state: TrackingMissingDeviceState::parse(missing_device_state)
            .expect_value("tracking missing-device contract drift"),
        parent_visibility_state: if missing {
            TrackingLastKnownVisibilityState::LastKnownOnly
        } else {
            TrackingLastKnownVisibilityState::NotLimited
        },
    }
}
