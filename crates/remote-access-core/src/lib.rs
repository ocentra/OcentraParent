#![forbid(unsafe_code)]

//! Remote access ownership boundary.
//!
//! This crate owns parent-approved remote access grants, relay/session
//! contracts, consent state, remote input authority, and abuse-control
//! boundaries. Live screen capture mechanics remain in screen/live-view crates.

use ocentra_family_identity_core::ChildDisclosureState;
use ocentra_policy_control_core::ParentAuthorityState;

pub const CRATE_NAME: &str = "ocentra-remote-access-core";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteAccessRelayState {
    Available,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteAccessReplayState {
    Fresh,
    Replayed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteAccessInputAuthorityState {
    ViewOnly,
    InputAllowed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteAccessSessionAuthorizationState {
    Allowed,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteAccessRelayRequirementState {
    Required,
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteAccessAutoExpiryState {
    Required,
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RemoteAccessSessionRequest {
    pub parent_authority_state: ParentAuthorityState,
    pub child_disclosure_state: ChildDisclosureState,
    pub relay_state: RemoteAccessRelayState,
    pub replay_state: RemoteAccessReplayState,
    pub input_authority_state: RemoteAccessInputAuthorityState,
    pub requested_minutes: u16,
    pub maximum_minutes: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RemoteAccessSessionDecision {
    pub authorization_state: RemoteAccessSessionAuthorizationState,
    pub relay_requirement_state: RemoteAccessRelayRequirementState,
    pub auto_expiry_state: RemoteAccessAutoExpiryState,
}

pub fn evaluate_remote_access_session(
    request: RemoteAccessSessionRequest,
) -> RemoteAccessSessionDecision {
    let bounded_duration =
        request.requested_minutes > 0 && request.requested_minutes <= request.maximum_minutes;
    let allowed = request.parent_authority_state == ParentAuthorityState::Authorized
        && request.child_disclosure_state == ChildDisclosureState::Disclosed
        && request.relay_state == RemoteAccessRelayState::Available
        && request.replay_state == RemoteAccessReplayState::Fresh
        && bounded_duration;

    RemoteAccessSessionDecision {
        authorization_state: if allowed {
            RemoteAccessSessionAuthorizationState::Allowed
        } else {
            RemoteAccessSessionAuthorizationState::Rejected
        },
        relay_requirement_state: if request.relay_state != RemoteAccessRelayState::Available {
            RemoteAccessRelayRequirementState::Required
        } else {
            RemoteAccessRelayRequirementState::NotRequired
        },
        auto_expiry_state: if allowed {
            RemoteAccessAutoExpiryState::Required
        } else {
            RemoteAccessAutoExpiryState::NotRequired
        },
    }
}
