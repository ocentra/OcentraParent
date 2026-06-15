use ocentra_eventing::envelope::DomainEvent;
use ocentra_family_identity_core::ChildDisclosureState;
use ocentra_policy_control_core::policy_authority::ParentAuthorityState;
use ocentra_remote_access_core::{
    evaluate_remote_access_session, plan_remote_access_session_effects,
    resolve_remote_access_session_request, RemoteAccessAggregateId, RemoteAccessAuditState,
    RemoteAccessAutoExpiryState, RemoteAccessDisclosureBannerState,
    RemoteAccessInputAuthorityState, RemoteAccessInputBridgeState,
    RemoteAccessRelayRequirementState, RemoteAccessRelayState, RemoteAccessReplayState,
    RemoteAccessSessionAuthorizationState, RemoteAccessSessionId, RemoteAccessSessionRequest,
    RemoteAccessSessionRequestedEvent, RemoteAccessViewStreamState,
};

const REMOTE_ACCESS_AGGREGATE_ID: &str = "remote-access-family-default";
const REMOTE_ACCESS_SESSION_ID: &str = "remote-access-session-default";
const REMOTE_ACCESS_REQUESTED_EVENT_TYPE: &str = "remote-access.session.requested";
const REMOTE_ACCESS_RESOLVED_EVENT_TYPE: &str = "remote-access.authorization.resolved";

fn allowed_request(
    input_authority_state: RemoteAccessInputAuthorityState,
) -> RemoteAccessSessionRequest {
    RemoteAccessSessionRequest {
        parent_authority_state: ParentAuthorityState::Authorized,
        child_disclosure_state: ChildDisclosureState::Disclosed,
        relay_state: RemoteAccessRelayState::Available,
        replay_state: RemoteAccessReplayState::Fresh,
        input_authority_state,
        requested_minutes: 15,
        maximum_minutes: 30,
    }
}

fn requested_event(request: RemoteAccessSessionRequest) -> RemoteAccessSessionRequestedEvent {
    RemoteAccessSessionRequestedEvent {
        aggregate_id: RemoteAccessAggregateId::parse(REMOTE_ACCESS_AGGREGATE_ID)
            .expect("remote access aggregate id"),
        session_id: RemoteAccessSessionId::parse(REMOTE_ACCESS_SESSION_ID)
            .expect("remote access session id"),
        request,
    }
}

#[test]
fn allowed_session_starts_view_input_disclosure_and_expiry() {
    let request = allowed_request(RemoteAccessInputAuthorityState::InputAllowed);

    let decision = evaluate_remote_access_session(request);
    let effects = plan_remote_access_session_effects(request);

    assert_eq!(
        decision.authorization_state,
        RemoteAccessSessionAuthorizationState::Allowed
    );
    assert_eq!(
        decision.relay_requirement_state,
        RemoteAccessRelayRequirementState::NotRequired
    );
    assert_eq!(
        decision.auto_expiry_state,
        RemoteAccessAutoExpiryState::Required
    );
    assert_eq!(
        effects.view_stream_state,
        RemoteAccessViewStreamState::Start
    );
    assert_eq!(
        effects.input_bridge_state,
        RemoteAccessInputBridgeState::Start
    );
    assert_eq!(
        effects.disclosure_banner_state,
        RemoteAccessDisclosureBannerState::Show
    );
    assert_eq!(effects.audit_state, RemoteAccessAuditState::Record);
}

#[test]
fn replayed_or_oversized_session_is_rejected_without_effects() {
    let request = RemoteAccessSessionRequest {
        replay_state: RemoteAccessReplayState::Replayed,
        requested_minutes: 60,
        maximum_minutes: 30,
        ..allowed_request(RemoteAccessInputAuthorityState::ViewOnly)
    };

    let decision = evaluate_remote_access_session(request);
    let effects = plan_remote_access_session_effects(request);

    assert_eq!(
        decision.authorization_state,
        RemoteAccessSessionAuthorizationState::Rejected
    );
    assert_eq!(
        effects.view_stream_state,
        RemoteAccessViewStreamState::DoNotStart
    );
    assert_eq!(
        effects.input_bridge_state,
        RemoteAccessInputBridgeState::DoNotStart
    );
    assert_eq!(
        effects.disclosure_banner_state,
        RemoteAccessDisclosureBannerState::DoNotShow
    );
}

#[test]
fn session_request_resolves_to_typed_authorization_event() {
    let request_event = requested_event(allowed_request(RemoteAccessInputAuthorityState::ViewOnly));

    let resolved_event = resolve_remote_access_session_request(&request_event);

    assert_eq!(resolved_event.aggregate_id, request_event.aggregate_id);
    assert_eq!(resolved_event.source_session_id, request_event.session_id);
    assert_eq!(
        request_event
            .contract()
            .expect("remote access request contract")
            .event_type
            .as_str(),
        REMOTE_ACCESS_REQUESTED_EVENT_TYPE
    );
    assert_eq!(
        resolved_event
            .contract()
            .expect("remote access resolved contract")
            .event_type
            .as_str(),
        REMOTE_ACCESS_RESOLVED_EVENT_TYPE
    );
}
