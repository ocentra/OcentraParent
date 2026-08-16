use ocentra_eventing::envelope::DomainEvent;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_family_identity_core::family_identity::ChildDisclosureState;
use ocentra_policy_control_core::policy_authority::ParentAuthorityState;
use ocentra_remote_access_core::remote_access_session::{
    resolve_remote_access_session_request, RemoteAccessAggregateId, RemoteAccessAuthorizationId,
    RemoteAccessInputAuthorityState, RemoteAccessInputBridgeState, RemoteAccessRelayState,
    RemoteAccessReplayState, RemoteAccessSessionAuthorizationState, RemoteAccessSessionRequest,
    RemoteAccessSessionRequestedEvent, RemoteAccessViewStreamState,
};

const REMOTE_ACCESS_AGGREGATE_ID: &str = "remote-access-contract-default";
const REMOTE_ACCESS_SESSION_ID: &str = "remote-access-session-contract-default";

#[test]
fn remote_access_session_request_and_resolution_keep_contract_surface_stable() {
    let request_event = RemoteAccessSessionRequestedEvent {
        aggregate_id: RemoteAccessAggregateId::parse(REMOTE_ACCESS_AGGREGATE_ID)
            .expect("aggregate id"),
        session_id:
            ocentra_remote_access_core::remote_access_session::RemoteAccessSessionId::parse(
                REMOTE_ACCESS_SESSION_ID,
            )
            .expect("session id"),
        request: RemoteAccessSessionRequest {
            parent_authority_state: ParentAuthorityState::Authorized,
            child_disclosure_state: ChildDisclosureState::Disclosed,
            relay_state: RemoteAccessRelayState::Available,
            replay_state: RemoteAccessReplayState::Fresh,
            input_authority_state: RemoteAccessInputAuthorityState::InputAllowed,
            requested_minutes: 15,
            maximum_minutes: 60,
        },
    };
    let resolved_event = resolve_remote_access_session_request(&request_event);

    assert_eq!(
        request_event
            .contract()
            .expect_value("request contract")
            .event_type
            .as_str(),
        "remote-access.session.requested"
    );
    assert_eq!(
        resolved_event
            .contract()
            .expect_value("resolved contract")
            .event_type
            .as_str(),
        "remote-access.authorization.resolved"
    );
    assert_eq!(
        resolved_event.authorization_id,
        RemoteAccessAuthorizationId::parse(
            "remote-access-authorization:remote-access-session-contract-default"
        )
        .expect("authorization id")
    );
    assert_eq!(
        resolved_event.decision.authorization_state,
        RemoteAccessSessionAuthorizationState::Allowed
    );
    assert_eq!(
        resolved_event.effect_plan.view_stream_state,
        RemoteAccessViewStreamState::Start
    );
    assert_eq!(
        resolved_event.effect_plan.input_bridge_state,
        RemoteAccessInputBridgeState::Start
    );

    let request_json = serde_json::to_value(&request_event).expect("serialize request event");
    let resolved_json = serde_json::to_value(&resolved_event).expect("serialize resolved event");
    let round_trip_request: RemoteAccessSessionRequestedEvent =
        serde_json::from_value(request_json).expect("deserialize request event");
    let round_trip_resolved =
        serde_json::from_value(resolved_json).expect("deserialize resolved event");

    assert_eq!(round_trip_request, request_event);
    assert_eq!(round_trip_resolved, resolved_event);
}
