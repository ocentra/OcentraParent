use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    TrackingReasonCode, TrackingTransitionKind,
};

use super::{reason_code, unknown_outcome};

pub(super) fn transition_outcome(
    transition_kind: &TrackingTransitionKind,
) -> (&'static str, Vec<TrackingReasonCode>) {
    if *transition_kind == constants::tracking_runtime::GEOFENCE_TRANSITION_ENTER
        || *transition_kind == constants::tracking_runtime::GEOFENCE_TRANSITION_DWELL
    {
        return (
            constants::tracking_runtime::EXPECTED_PLACE_STATE_WHERE_EXPECTED,
            vec![reason_code(
                constants::tracking_runtime::REASON_INSIDE_EXPECTED_PLACE_WINDOW,
            )],
        );
    }

    if *transition_kind == constants::tracking_runtime::GEOFENCE_TRANSITION_EXIT {
        return (
            constants::tracking_runtime::EXPECTED_PLACE_STATE_LEFT_EXPECTED_PLACE,
            vec![reason_code(
                constants::tracking_runtime::REASON_EXITED_EXPECTED_PLACE_WINDOW,
            )],
        );
    }

    if *transition_kind == constants::tracking_runtime::GEOFENCE_TRANSITION_MISSED_ARRIVAL {
        return (
            constants::tracking_runtime::EXPECTED_PLACE_STATE_LATE_ARRIVAL,
            vec![reason_code(
                constants::tracking_runtime::REASON_MISSED_EXPECTED_PLACE_ARRIVAL,
            )],
        );
    }

    unknown_outcome(constants::tracking_runtime::REASON_EXPECTED_PLACE_AMBIGUOUS)
}
