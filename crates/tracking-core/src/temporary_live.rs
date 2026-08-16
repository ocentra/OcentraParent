use ocentra_eventing::expect_value::ExpectValue;
use ocentra_family_identity_core::family_identity::ChildDisclosureState;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    tracking_temporary_live_session_id_from_child_device_id, TrackingChildDeviceId,
    TrackingTemporaryLiveSessionId, TrackingTemporaryLiveState,
};
use ocentra_policy_control_core::policy_authority::ParentAuthorityState;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingHighCadenceState {
    Allowed,
    Blocked,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingTemporaryLiveSessionInput {
    pub child_device_id: TrackingChildDeviceId,
    pub requested_duration_minutes: u16,
    pub elapsed_minutes: u16,
    pub parent_authority_state: ParentAuthorityState,
    pub child_disclosure_state: ChildDisclosureState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingTemporaryLiveSessionDecision {
    pub session_id: TrackingTemporaryLiveSessionId,
    pub session_state: TrackingTemporaryLiveState,
    pub high_cadence_state: TrackingHighCadenceState,
}

pub fn evaluate_temporary_live_tracking_session(
    input: TrackingTemporaryLiveSessionInput,
) -> TrackingTemporaryLiveSessionDecision {
    let TrackingTemporaryLiveSessionInput {
        child_device_id,
        requested_duration_minutes,
        elapsed_minutes,
        parent_authority_state,
        child_disclosure_state,
    } = input;

    if parent_authority_state != ParentAuthorityState::Authorized
        || child_disclosure_state != ChildDisclosureState::Disclosed
    {
        return TrackingTemporaryLiveSessionDecision {
            session_id: temporary_live_session_id(&child_device_id),
            session_state: temporary_live_state(
                constants::tracking_runtime::TEMPORARY_LIVE_STATE_AUTO_STOPPED,
            ),
            high_cadence_state: TrackingHighCadenceState::Blocked,
        };
    }

    if elapsed_minutes >= requested_duration_minutes {
        return TrackingTemporaryLiveSessionDecision {
            session_id: temporary_live_session_id(&child_device_id),
            session_state: temporary_live_state(
                constants::tracking_runtime::TEMPORARY_LIVE_STATE_EXPIRED,
            ),
            high_cadence_state: TrackingHighCadenceState::Blocked,
        };
    }

    TrackingTemporaryLiveSessionDecision {
        session_id: temporary_live_session_id(&child_device_id),
        session_state: temporary_live_state(
            constants::tracking_runtime::TEMPORARY_LIVE_STATE_ACTIVE,
        ),
        high_cadence_state: TrackingHighCadenceState::Allowed,
    }
}

fn temporary_live_session_id(
    child_device_id: &TrackingChildDeviceId,
) -> TrackingTemporaryLiveSessionId {
    tracking_temporary_live_session_id_from_child_device_id(child_device_id)
}

fn temporary_live_state(value: &'static str) -> TrackingTemporaryLiveState {
    TrackingTemporaryLiveState::parse(value)
        .expect_value("canonical tracking live-session state failed to parse")
}
