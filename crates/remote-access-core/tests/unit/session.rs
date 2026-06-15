use ocentra_eventing::DomainEvent;
use ocentra_family_identity_core::ChildDisclosureState;
use ocentra_policy_control_core::policy_authority::ParentAuthorityState;
use ocentra_remote_access_core::{
    evaluate_remote_access_session, plan_remote_access_session_effects,
    resolve_remote_access_session_request, RemoteAccessAggregateId, RemoteAccessAutoExpiryState,
    RemoteAccessDisclosureBannerState, RemoteAccessInputAuthorityState,
    RemoteAccessInputBridgeState, RemoteAccessRelayRequirementState, RemoteAccessRelayState,
    RemoteAccessReplayState, RemoteAccessSessionAuthorizationState, RemoteAccessSessionId,
    RemoteAccessSessionRequest, RemoteAccessSessionRequestedEvent, RemoteAccessViewStreamState,
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

#[test]
fn allowed_view_only_session_starts_view_stream_without_input_bridge() {
    let plan = plan_remote_access_session_effects(RemoteAccessSessionRequest {
        parent_authority_state: ParentAuthorityState::Authorized,
        child_disclosure_state: ChildDisclosureState::Disclosed,
        relay_state: RemoteAccessRelayState::Available,
        replay_state: RemoteAccessReplayState::Fresh,
        input_authority_state: RemoteAccessInputAuthorityState::ViewOnly,
        requested_minutes: 15,
        maximum_minutes: 30,
    });

    assert_eq!(plan.view_stream_state, RemoteAccessViewStreamState::Start);
    assert_eq!(
        plan.input_bridge_state,
        RemoteAccessInputBridgeState::DoNotStart
    );
    assert_eq!(
        plan.disclosure_banner_state,
        RemoteAccessDisclosureBannerState::Show
    );
}

#[test]
fn allowed_input_session_starts_input_bridge_after_all_gates_pass() {
    let plan = plan_remote_access_session_effects(RemoteAccessSessionRequest {
        parent_authority_state: ParentAuthorityState::Authorized,
        child_disclosure_state: ChildDisclosureState::Disclosed,
        relay_state: RemoteAccessRelayState::Available,
        replay_state: RemoteAccessReplayState::Fresh,
        input_authority_state: RemoteAccessInputAuthorityState::InputAllowed,
        requested_minutes: 15,
        maximum_minutes: 30,
    });

    assert_eq!(plan.view_stream_state, RemoteAccessViewStreamState::Start);
    assert_eq!(plan.input_bridge_state, RemoteAccessInputBridgeState::Start);
}

#[test]
fn remote_access_session_request_resolves_authorization_and_effect_plan_event() {
    let request = RemoteAccessSessionRequestedEvent {
        aggregate_id: RemoteAccessAggregateId::parse("remote-access-child-default")
            .expect("remote access aggregate"),
        session_id: RemoteAccessSessionId::parse("remote-access-session-default")
            .expect("remote access session"),
        request: RemoteAccessSessionRequest {
            parent_authority_state: ParentAuthorityState::Authorized,
            child_disclosure_state: ChildDisclosureState::Disclosed,
            relay_state: RemoteAccessRelayState::Available,
            replay_state: RemoteAccessReplayState::Fresh,
            input_authority_state: RemoteAccessInputAuthorityState::InputAllowed,
            requested_minutes: 15,
            maximum_minutes: 30,
        },
    };

    let resolved = resolve_remote_access_session_request(&request);

    assert_eq!(resolved.aggregate_id, request.aggregate_id);
    assert_eq!(resolved.source_session_id, request.session_id);
    assert_eq!(
        resolved.decision.authorization_state,
        RemoteAccessSessionAuthorizationState::Allowed
    );
    assert_eq!(
        resolved.effect_plan.input_bridge_state,
        RemoteAccessInputBridgeState::Start
    );
    assert_eq!(
        request
            .contract()
            .expect("remote access request contract")
            .event_type
            .as_str(),
        "remote-access.session.requested"
    );
    assert_eq!(
        resolved
            .contract()
            .expect("remote access authorization contract")
            .event_type
            .as_str(),
        "remote-access.authorization.resolved"
    );
}
