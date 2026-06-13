use ocentra_family_identity_core::ChildDisclosureState;
use ocentra_parent_agent_protocol::{constants, TrackingTemporaryLiveState};
use ocentra_policy_control_core::ParentAuthorityState;
use ocentra_tracking_core::TrackingHighCadenceState;

#[test]
fn temporary_live_tracking_auto_stops_without_authority_or_disclosure() {
    let decision = ocentra_tracking_core::evaluate_temporary_live_tracking_session(
        ocentra_tracking_core::TrackingTemporaryLiveSessionInput {
            requested_duration_minutes: 15,
            elapsed_minutes: 1,
            parent_authority_state: ParentAuthorityState::Authorized,
            child_disclosure_state: ChildDisclosureState::NotDisclosed,
        },
    );

    assert_eq!(
        decision.session_state,
        TrackingTemporaryLiveState::parse(
            constants::tracking_runtime::TEMPORARY_LIVE_STATE_AUTO_STOPPED,
        )
        .expect(constants::tracking_runtime::TEMPORARY_LIVE_STATE_AUTO_STOPPED)
    );
    assert_eq!(
        decision.high_cadence_state,
        TrackingHighCadenceState::Blocked
    );
}

#[test]
fn temporary_live_tracking_expires_at_duration_boundary() {
    let decision = ocentra_tracking_core::evaluate_temporary_live_tracking_session(
        ocentra_tracking_core::TrackingTemporaryLiveSessionInput {
            requested_duration_minutes: 15,
            elapsed_minutes: 15,
            parent_authority_state: ParentAuthorityState::Authorized,
            child_disclosure_state: ChildDisclosureState::Disclosed,
        },
    );

    assert_eq!(
        decision.session_state,
        TrackingTemporaryLiveState::parse(constants::tracking_runtime::TEMPORARY_LIVE_STATE_EXPIRED)
            .expect(constants::tracking_runtime::TEMPORARY_LIVE_STATE_EXPIRED)
    );
    assert_eq!(
        decision.high_cadence_state,
        TrackingHighCadenceState::Blocked
    );
}
