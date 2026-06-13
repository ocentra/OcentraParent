use ocentra_family_identity_core::ChildDisclosureState;
use ocentra_policy_control_core::ParentAuthorityState;
use ocentra_remote_access_core::{
    evaluate_remote_access_session, RemoteAccessAutoExpiryState, RemoteAccessInputAuthorityState,
    RemoteAccessRelayRequirementState, RemoteAccessRelayState, RemoteAccessReplayState,
    RemoteAccessSessionAuthorizationState, RemoteAccessSessionRequest,
};

#[test]
fn remote_access_requires_parent_child_relay_and_bounded_duration() {
    let decision = evaluate_remote_access_session(RemoteAccessSessionRequest {
        parent_authority_state: ParentAuthorityState::Authorized,
        child_disclosure_state: ChildDisclosureState::Disclosed,
        relay_state: RemoteAccessRelayState::Available,
        replay_state: RemoteAccessReplayState::Fresh,
        input_authority_state: RemoteAccessInputAuthorityState::ViewOnly,
        requested_minutes: 15,
        maximum_minutes: 30,
    });

    assert_eq!(
        decision.authorization_state,
        RemoteAccessSessionAuthorizationState::Allowed
    );
    assert_eq!(
        decision.auto_expiry_state,
        RemoteAccessAutoExpiryState::Required
    );
    assert_eq!(
        decision.relay_requirement_state,
        RemoteAccessRelayRequirementState::NotRequired
    );
}

#[test]
fn remote_access_rejects_unbounded_session_duration() {
    let decision = evaluate_remote_access_session(RemoteAccessSessionRequest {
        parent_authority_state: ParentAuthorityState::Authorized,
        child_disclosure_state: ChildDisclosureState::Disclosed,
        relay_state: RemoteAccessRelayState::Available,
        replay_state: RemoteAccessReplayState::Fresh,
        input_authority_state: RemoteAccessInputAuthorityState::ViewOnly,
        requested_minutes: 45,
        maximum_minutes: 30,
    });

    assert_eq!(
        decision.authorization_state,
        RemoteAccessSessionAuthorizationState::Rejected
    );
    assert_eq!(
        decision.auto_expiry_state,
        RemoteAccessAutoExpiryState::NotRequired
    );
}

#[test]
fn remote_access_rejects_when_child_disclosure_is_missing() {
    let decision = evaluate_remote_access_session(RemoteAccessSessionRequest {
        parent_authority_state: ParentAuthorityState::Authorized,
        child_disclosure_state: ChildDisclosureState::NotDisclosed,
        relay_state: RemoteAccessRelayState::Available,
        replay_state: RemoteAccessReplayState::Fresh,
        input_authority_state: RemoteAccessInputAuthorityState::ViewOnly,
        requested_minutes: 15,
        maximum_minutes: 30,
    });

    assert_eq!(
        decision.authorization_state,
        RemoteAccessSessionAuthorizationState::Rejected
    );
    assert_eq!(
        decision.auto_expiry_state,
        RemoteAccessAutoExpiryState::NotRequired
    );
}

#[test]
fn remote_access_rejects_and_requires_relay_when_relay_is_unavailable() {
    let decision = evaluate_remote_access_session(RemoteAccessSessionRequest {
        parent_authority_state: ParentAuthorityState::Authorized,
        child_disclosure_state: ChildDisclosureState::Disclosed,
        relay_state: RemoteAccessRelayState::Unavailable,
        replay_state: RemoteAccessReplayState::Fresh,
        input_authority_state: RemoteAccessInputAuthorityState::ViewOnly,
        requested_minutes: 15,
        maximum_minutes: 30,
    });

    assert_eq!(
        decision.authorization_state,
        RemoteAccessSessionAuthorizationState::Rejected
    );
    assert_eq!(
        decision.relay_requirement_state,
        RemoteAccessRelayRequirementState::Required
    );
    assert_eq!(
        decision.auto_expiry_state,
        RemoteAccessAutoExpiryState::NotRequired
    );
}

#[test]
fn remote_access_rejects_zero_duration_request() {
    let decision = evaluate_remote_access_session(RemoteAccessSessionRequest {
        parent_authority_state: ParentAuthorityState::Authorized,
        child_disclosure_state: ChildDisclosureState::Disclosed,
        relay_state: RemoteAccessRelayState::Available,
        replay_state: RemoteAccessReplayState::Fresh,
        input_authority_state: RemoteAccessInputAuthorityState::ViewOnly,
        requested_minutes: 0,
        maximum_minutes: 30,
    });

    assert_eq!(
        decision.authorization_state,
        RemoteAccessSessionAuthorizationState::Rejected
    );
    assert_eq!(
        decision.auto_expiry_state,
        RemoteAccessAutoExpiryState::NotRequired
    );
}

#[test]
fn remote_access_rejects_replayed_session_request() {
    let decision = evaluate_remote_access_session(RemoteAccessSessionRequest {
        parent_authority_state: ParentAuthorityState::Authorized,
        child_disclosure_state: ChildDisclosureState::Disclosed,
        relay_state: RemoteAccessRelayState::Available,
        replay_state: RemoteAccessReplayState::Replayed,
        input_authority_state: RemoteAccessInputAuthorityState::ViewOnly,
        requested_minutes: 15,
        maximum_minutes: 30,
    });

    assert_eq!(
        decision.authorization_state,
        RemoteAccessSessionAuthorizationState::Rejected
    );
    assert_eq!(
        decision.auto_expiry_state,
        RemoteAccessAutoExpiryState::NotRequired
    );
}

#[test]
fn remote_input_authority_does_not_bypass_session_gates() {
    let decision = evaluate_remote_access_session(RemoteAccessSessionRequest {
        parent_authority_state: ParentAuthorityState::Unauthorized,
        child_disclosure_state: ChildDisclosureState::Disclosed,
        relay_state: RemoteAccessRelayState::Available,
        replay_state: RemoteAccessReplayState::Fresh,
        input_authority_state: RemoteAccessInputAuthorityState::InputAllowed,
        requested_minutes: 15,
        maximum_minutes: 30,
    });

    assert_eq!(
        decision.authorization_state,
        RemoteAccessSessionAuthorizationState::Rejected
    );
    assert_eq!(
        decision.auto_expiry_state,
        RemoteAccessAutoExpiryState::NotRequired
    );
}
