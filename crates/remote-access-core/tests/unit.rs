use ocentra_eventing::expect_value::ExpectValue;
#[path = "unit/session.rs"]
mod session;

#[path = "unit/session_authorization.rs"]
mod session_authorization;

use ocentra_family_identity_core::family_identity::ChildDisclosureState;
use ocentra_policy_control_core::policy_authority::ParentAuthorityState;
use ocentra_remote_access_core::remote_access_session::{
    RemoteAccessAggregateId, RemoteAccessInputAuthorityState, RemoteAccessRelayState,
    RemoteAccessReplayState, RemoteAccessSessionId, RemoteAccessSessionRequest,
    RemoteAccessSessionRequestedEvent,
};

const REMOTE_ACCESS_CHILD_AGGREGATE_ID: &str = "remote-access-child-default";
const REMOTE_ACCESS_FAMILY_AGGREGATE_ID: &str = "remote-access-family-default";
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

fn requested_event(
    aggregate_id: &str,
    request: RemoteAccessSessionRequest,
) -> RemoteAccessSessionRequestedEvent {
    RemoteAccessSessionRequestedEvent {
        aggregate_id: RemoteAccessAggregateId::parse(aggregate_id)
            .expect_value("remote access aggregate id"),
        session_id: RemoteAccessSessionId::parse(REMOTE_ACCESS_SESSION_ID)
            .expect_value("remote access session id"),
        request,
    }
}
