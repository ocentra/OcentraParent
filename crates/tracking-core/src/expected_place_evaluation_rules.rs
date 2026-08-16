use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    TrackingCapabilityStatus, TrackingReasonCode,
};
use ocentra_parent_agent_protocol::tracking::runtime_event::TrackingParentActionRequirement;

use crate::expected_place::{TrackingExpectedPlaceEvaluation, TrackingExpectedPlaceException};

#[path = "expected_place_evaluation_exception_rules.rs"]
mod expected_place_evaluation_exception_rules;
#[path = "expected_place_evaluation_transition_rules.rs"]
mod expected_place_evaluation_transition_rules;

pub(crate) fn parent_action_requirement_for_expected_place_state(
    expected_place_state: &'static str,
) -> TrackingParentActionRequirement {
    if expected_place_state == constants::tracking_runtime::EXPECTED_PLACE_STATE_LEFT_EXPECTED_PLACE
        || expected_place_state == constants::tracking_runtime::EXPECTED_PLACE_STATE_LATE_ARRIVAL
    {
        TrackingParentActionRequirement::Required
    } else {
        TrackingParentActionRequirement::NotRequired
    }
}

pub(crate) fn expected_place_outcome_for(
    evaluation: &TrackingExpectedPlaceEvaluation,
) -> (&'static str, Vec<TrackingReasonCode>) {
    if !evaluation.schedule_enabled {
        return manual_required_outcome(
            constants::tracking_runtime::REASON_EXPECTED_PLACE_SCHEDULE_DISABLED,
        );
    }

    if capability_requires_manual_review(&evaluation.capability_status) {
        return manual_required_outcome(
            constants::tracking_runtime::REASON_FRESH_LOCATION_REQUIRED,
        );
    }

    if let Some(active_exception) = &evaluation.active_exception {
        return unknown_outcome(
            expected_place_evaluation_exception_rules::reason_code_for_expected_place_exception(
                active_exception,
            ),
        );
    }

    if !evaluation.within_expected_window {
        return unknown_outcome(constants::tracking_runtime::REASON_OUTSIDE_EXPECTED_PLACE_WINDOW);
    }

    if evaluation.late_grace_active {
        return unknown_outcome(
            constants::tracking_runtime::REASON_EXPECTED_PLACE_LATE_GRACE_ACTIVE,
        );
    }

    if evaluation.early_exit_grace_active {
        return unknown_outcome(
            constants::tracking_runtime::REASON_EXPECTED_PLACE_EARLY_EXIT_GRACE_ACTIVE,
        );
    }

    expected_place_evaluation_transition_rules::transition_outcome(&evaluation.transition_kind)
}

fn manual_required_outcome(reason: &'static str) -> (&'static str, Vec<TrackingReasonCode>) {
    (
        constants::tracking_runtime::EXPECTED_PLACE_STATE_MANUAL_REQUIRED,
        vec![reason_code(reason)],
    )
}

fn unknown_outcome(reason: &'static str) -> (&'static str, Vec<TrackingReasonCode>) {
    (
        constants::tracking_runtime::EXPECTED_PLACE_STATE_UNKNOWN,
        vec![reason_code(reason)],
    )
}

fn capability_requires_manual_review(capability_status: &TrackingCapabilityStatus) -> bool {
    matches!(
        capability_status.as_str(),
        constants::tracking_runtime::CAPABILITY_STATUS_STALE
            | constants::tracking_runtime::CAPABILITY_STATUS_LAST_KNOWN
            | constants::tracking_runtime::CAPABILITY_STATUS_OFFLINE_LAST_KNOWN_ONLY
            | constants::tracking_runtime::CAPABILITY_STATUS_PERMISSION_REQUIRED
            | constants::tracking_runtime::CAPABILITY_STATUS_BACKGROUND_PERMISSION_REQUIRED
            | constants::tracking_runtime::CAPABILITY_STATUS_APPROXIMATE_ONLY
            | constants::tracking_runtime::CAPABILITY_STATUS_MANUAL_REQUIRED
            | constants::tracking_runtime::CAPABILITY_STATUS_UNAVAILABLE
            | constants::tracking_runtime::CAPABILITY_STATUS_ADAPTER_ERROR
            | constants::tracking_runtime::CAPABILITY_STATUS_DISABLED_BY_PARENT
    )
}

fn reason_code(value: &'static str) -> TrackingReasonCode {
    TrackingReasonCode::parse(value).expect_value("tracking expected-place reason code parses")
}

pub(crate) fn protocol_exception_state_for_expected_place_exception(
    active_exception: &TrackingExpectedPlaceException,
) -> ocentra_parent_agent_protocol::tracking::runtime_event::TrackingExpectedPlaceExceptionState {
    expected_place_evaluation_exception_rules::protocol_exception_state_for_expected_place_exception(
        active_exception,
    )
}
