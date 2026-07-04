use crate::expected_place::TrackingExpectedPlaceException;

pub(super) fn reason_code_for_expected_place_exception(
    active_exception: &TrackingExpectedPlaceException,
) -> &'static str {
    match active_exception {
        TrackingExpectedPlaceException::HolidayMode => {
            ocentra_parent_agent_protocol::constants::tracking_runtime::REASON_EXPECTED_PLACE_HOLIDAY_EXCEPTION_ACTIVE
        }
        TrackingExpectedPlaceException::TripException => {
            ocentra_parent_agent_protocol::constants::tracking_runtime::REASON_EXPECTED_PLACE_TRIP_EXCEPTION_ACTIVE
        }
    }
}

pub(super) fn protocol_exception_state_for_expected_place_exception(
    active_exception: &TrackingExpectedPlaceException,
) -> ocentra_parent_agent_protocol::tracking::runtime_event::TrackingExpectedPlaceExceptionState {
    match active_exception {
        TrackingExpectedPlaceException::HolidayMode => {
            ocentra_parent_agent_protocol::tracking::runtime_event::TrackingExpectedPlaceExceptionState::HolidayMode
        }
        TrackingExpectedPlaceException::TripException => {
            ocentra_parent_agent_protocol::tracking::runtime_event::TrackingExpectedPlaceExceptionState::TripException
        }
    }
}
