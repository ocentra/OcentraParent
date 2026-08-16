use ocentra_eventing::envelope::DomainEvent;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_family_identity_core::family_identity::ChildDisclosureState;
use ocentra_policy_control_core::policy_authority::ParentAuthorityState;
use ocentra_remote_access_core::remote_access_session::{
    evaluate_remote_access_session, plan_remote_access_session_effects,
    resolve_remote_access_session_request, RemoteAccessAutoExpiryState,
    RemoteAccessDisclosureBannerState, RemoteAccessInputAuthorityState,
    RemoteAccessInputBridgeState, RemoteAccessRelayRequirementState, RemoteAccessRelayState,
    RemoteAccessReplayState, RemoteAccessSessionAuthorizationState, RemoteAccessSessionRequest,
    RemoteAccessViewStreamState,
};

#[test]
fn remote_access_requires_parent_child_relay_and_bounded_duration() {
    let decision = evaluate_remote_access_session(super::allowed_request(
        RemoteAccessInputAuthorityState::ViewOnly,
    ));

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
        replay_state: RemoteAccessReplayState::Replayed,
        requested_minutes: 45,
        ..super::allowed_request(RemoteAccessInputAuthorityState::ViewOnly)
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
        child_disclosure_state: ChildDisclosureState::NotDisclosed,
        ..super::allowed_request(RemoteAccessInputAuthorityState::ViewOnly)
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
        relay_state: RemoteAccessRelayState::Unavailable,
        ..super::allowed_request(RemoteAccessInputAuthorityState::ViewOnly)
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
        requested_minutes: 0,
        ..super::allowed_request(RemoteAccessInputAuthorityState::ViewOnly)
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
        replay_state: RemoteAccessReplayState::Replayed,
        ..super::allowed_request(RemoteAccessInputAuthorityState::ViewOnly)
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
        ..super::allowed_request(RemoteAccessInputAuthorityState::InputAllowed)
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
    let plan = plan_remote_access_session_effects(super::allowed_request(
        RemoteAccessInputAuthorityState::ViewOnly,
    ));

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
    let plan = plan_remote_access_session_effects(super::allowed_request(
        RemoteAccessInputAuthorityState::InputAllowed,
    ));

    assert_eq!(plan.view_stream_state, RemoteAccessViewStreamState::Start);
    assert_eq!(plan.input_bridge_state, RemoteAccessInputBridgeState::Start);
}

#[test]
fn remote_access_session_request_resolves_authorization_and_effect_plan_event() {
    let request = super::requested_event(
        super::REMOTE_ACCESS_CHILD_AGGREGATE_ID,
        super::allowed_request(RemoteAccessInputAuthorityState::InputAllowed),
    );

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
            .expect_value("remote access request contract")
            .event_type
            .as_str(),
        super::REMOTE_ACCESS_REQUESTED_EVENT_TYPE
    );
    assert_eq!(
        resolved
            .contract()
            .expect_value("remote access authorization contract")
            .event_type
            .as_str(),
        super::REMOTE_ACCESS_RESOLVED_EVENT_TYPE
    );
}
