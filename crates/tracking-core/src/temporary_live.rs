use ocentra_family_identity_core::ChildDisclosureState;
use ocentra_parent_agent_protocol::{
    constants, TrackingTemporaryLiveSessionId, TrackingTemporaryLiveState,
};
use ocentra_policy_control_core::ParentAuthorityState;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackingHighCadenceState {
    Allowed,
    Blocked,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingTemporaryLiveSessionInput {
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
    if input.parent_authority_state != ParentAuthorityState::Authorized
        || input.child_disclosure_state != ChildDisclosureState::Disclosed
    {
        return TrackingTemporaryLiveSessionDecision {
            session_id: temporary_live_session_id(),
            session_state: temporary_live_state(
                constants::tracking_runtime::TEMPORARY_LIVE_STATE_AUTO_STOPPED,
            ),
            high_cadence_state: TrackingHighCadenceState::Blocked,
        };
    }

    if input.elapsed_minutes >= input.requested_duration_minutes {
        return TrackingTemporaryLiveSessionDecision {
            session_id: temporary_live_session_id(),
            session_state: temporary_live_state(
                constants::tracking_runtime::TEMPORARY_LIVE_STATE_EXPIRED,
            ),
            high_cadence_state: TrackingHighCadenceState::Blocked,
        };
    }

    TrackingTemporaryLiveSessionDecision {
        session_id: temporary_live_session_id(),
        session_state: temporary_live_state(
            constants::tracking_runtime::TEMPORARY_LIVE_STATE_ACTIVE,
        ),
        high_cadence_state: TrackingHighCadenceState::Allowed,
    }
}

fn temporary_live_session_id() -> TrackingTemporaryLiveSessionId {
    TrackingTemporaryLiveSessionId::parse(
        constants::tracking_runtime::DEFAULT_TEMPORARY_LIVE_SESSION_ID,
    )
    .expect(constants::tracking_runtime::DEFAULT_TEMPORARY_LIVE_SESSION_ID)
}

fn temporary_live_state(value: &'static str) -> TrackingTemporaryLiveState {
    TrackingTemporaryLiveState::parse(value)
        .expect(constants::tracking_runtime::TEMPORARY_LIVE_STATE_ACTIVE)
}
