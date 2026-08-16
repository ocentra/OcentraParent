#![forbid(unsafe_code)]

//! Remote access ownership boundary.
//!
//! This crate owns parent-approved remote access grants, relay/session
//! contracts, consent state, remote input authority, and abuse-control
//! boundaries. Live screen capture mechanics remain in screen/live-view crates.

use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use ocentra_family_identity_core::family_identity::ChildDisclosureState;
use ocentra_policy_control_core::policy_authority::ParentAuthorityState;
use serde::{Deserialize, Serialize};

pub const CRATE_NAME: &str = "ocentra-remote-access-core";
const REMOTE_ACCESS_SCHEMA_VERSION: u16 = 1;
const REMOTE_ACCESS_SESSION_REQUESTED_EVENT_TYPE: &str = "remote-access.session.requested";
const REMOTE_ACCESS_AUTHORIZATION_RESOLVED_EVENT_TYPE: &str =
    "remote-access.authorization.resolved";
const REMOTE_ACCESS_IDEMPOTENCY_SEPARATOR: &str = ":";
const REMOTE_ACCESS_AUTHORIZATION_PREFIX: &str = "remote-access-authorization:";
const ERROR_REMOTE_ACCESS_AUTHORIZATION_ID: &str = "remote access authorization id";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemoteAccessRelayState {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemoteAccessReplayState {
    #[serde(rename = "fresh")]
    Fresh,
    #[serde(rename = "replayed")]
    Replayed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemoteAccessInputAuthorityState {
    #[serde(rename = "view-only")]
    ViewOnly,
    #[serde(rename = "input-allowed")]
    InputAllowed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemoteAccessSessionAuthorizationState {
    #[serde(rename = "allowed")]
    Allowed,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemoteAccessRelayRequirementState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemoteAccessAutoExpiryState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemoteAccessViewStreamState {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "do-not-start")]
    DoNotStart,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemoteAccessInputBridgeState {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "do-not-start")]
    DoNotStart,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemoteAccessDisclosureBannerState {
    #[serde(rename = "show")]
    Show,
    #[serde(rename = "do-not-show")]
    DoNotShow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemoteAccessAuditState {
    #[serde(rename = "record")]
    Record,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteAccessSessionRequest {
    pub parent_authority_state: ParentAuthorityState,
    pub child_disclosure_state: ChildDisclosureState,
    pub relay_state: RemoteAccessRelayState,
    pub replay_state: RemoteAccessReplayState,
    pub input_authority_state: RemoteAccessInputAuthorityState,
    pub requested_minutes: u16,
    pub maximum_minutes: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteAccessSessionDecision {
    pub authorization_state: RemoteAccessSessionAuthorizationState,
    pub relay_requirement_state: RemoteAccessRelayRequirementState,
    pub auto_expiry_state: RemoteAccessAutoExpiryState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteAccessSessionEffectPlan {
    pub view_stream_state: RemoteAccessViewStreamState,
    pub input_bridge_state: RemoteAccessInputBridgeState,
    pub disclosure_banner_state: RemoteAccessDisclosureBannerState,
    pub audit_state: RemoteAccessAuditState,
}

macro_rules! remote_access_text_id {
    ($name:ident, $field:expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
                parse_remote_access_text_id($field, value).map(Self)
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = EventingError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

remote_access_text_id!(RemoteAccessSessionId, "remote_access.session_id");
remote_access_text_id!(
    RemoteAccessAuthorizationId,
    "remote_access.authorization_id"
);
remote_access_text_id!(RemoteAccessAggregateId, "remote_access.aggregate_id");

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteAccessSessionRequestedEvent {
    pub aggregate_id: RemoteAccessAggregateId,
    pub session_id: RemoteAccessSessionId,
    pub request: RemoteAccessSessionRequest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteAccessAuthorizationResolvedEvent {
    pub aggregate_id: RemoteAccessAggregateId,
    pub authorization_id: RemoteAccessAuthorizationId,
    pub source_session_id: RemoteAccessSessionId,
    pub decision: RemoteAccessSessionDecision,
    pub effect_plan: RemoteAccessSessionEffectPlan,
}

impl DomainEvent for RemoteAccessSessionRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        remote_access_event_contract(REMOTE_ACCESS_SESSION_REQUESTED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        remote_access_idempotency_key(
            REMOTE_ACCESS_SESSION_REQUESTED_EVENT_TYPE,
            self.session_id.as_str(),
        )
    }
}

impl DomainEvent for RemoteAccessAuthorizationResolvedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        remote_access_event_contract(REMOTE_ACCESS_AUTHORIZATION_RESOLVED_EVENT_TYPE)
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        remote_access_idempotency_key(
            REMOTE_ACCESS_AUTHORIZATION_RESOLVED_EVENT_TYPE,
            self.authorization_id.as_str(),
        )
    }
}

pub fn evaluate_remote_access_session(
    request: RemoteAccessSessionRequest,
) -> RemoteAccessSessionDecision {
    let allowed = remote_access_session_is_allowed(request);

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

pub fn resolve_remote_access_session_request(
    event: &RemoteAccessSessionRequestedEvent,
) -> RemoteAccessAuthorizationResolvedEvent {
    RemoteAccessAuthorizationResolvedEvent {
        aggregate_id: event.aggregate_id.clone(),
        authorization_id: RemoteAccessAuthorizationId::parse(remote_access_authorization_ref(
            &event.session_id,
        ))
        .expect_value(ERROR_REMOTE_ACCESS_AUTHORIZATION_ID),
        source_session_id: event.session_id.clone(),
        decision: evaluate_remote_access_session(event.request),
        effect_plan: plan_remote_access_session_effects(event.request),
    }
}

pub fn plan_remote_access_session_effects(
    request: RemoteAccessSessionRequest,
) -> RemoteAccessSessionEffectPlan {
    let decision = evaluate_remote_access_session(request);
    let allowed = decision.authorization_state == RemoteAccessSessionAuthorizationState::Allowed;
    let input_allowed = [
        allowed,
        request.input_authority_state == RemoteAccessInputAuthorityState::InputAllowed,
    ]
    .into_iter()
    .all(std::convert::identity);

    RemoteAccessSessionEffectPlan {
        view_stream_state: if allowed {
            RemoteAccessViewStreamState::Start
        } else {
            RemoteAccessViewStreamState::DoNotStart
        },
        input_bridge_state: if input_allowed {
            RemoteAccessInputBridgeState::Start
        } else {
            RemoteAccessInputBridgeState::DoNotStart
        },
        disclosure_banner_state: if allowed {
            RemoteAccessDisclosureBannerState::Show
        } else {
            RemoteAccessDisclosureBannerState::DoNotShow
        },
        audit_state: RemoteAccessAuditState::Record,
    }
}

fn remote_access_session_is_allowed(request: RemoteAccessSessionRequest) -> bool {
    [
        request.parent_authority_state == ParentAuthorityState::Authorized,
        request.child_disclosure_state == ChildDisclosureState::Disclosed,
        request.relay_state == RemoteAccessRelayState::Available,
        request.replay_state == RemoteAccessReplayState::Fresh,
        (1..=request.maximum_minutes).contains(&request.requested_minutes),
    ]
    .into_iter()
    .all(std::convert::identity)
}

fn remote_access_event_contract(event_type: &str) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type)?,
        SchemaVersion::new(REMOTE_ACCESS_SCHEMA_VERSION)?,
    ))
}

fn parse_remote_access_text_id(
    field: &'static str,
    value: impl Into<String>,
) -> Result<String, EventingError> {
    let value = value.into();
    (!value.trim().is_empty())
        .then_some(value)
        .ok_or(EventingError::EmptyValue { field })
}

fn remote_access_idempotency_key(
    event_type: &str,
    unique_ref: &str,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type, REMOTE_ACCESS_IDEMPOTENCY_SEPARATOR, unique_ref
    ))
}

fn remote_access_authorization_ref(session_id: &RemoteAccessSessionId) -> String {
    let mut value = String::from(REMOTE_ACCESS_AUTHORIZATION_PREFIX);
    value.push_str(session_id.as_str());
    value
}
