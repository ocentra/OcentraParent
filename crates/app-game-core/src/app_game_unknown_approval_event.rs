use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use serde::{Deserialize, Serialize};

use crate::app_game_unknown_approval_types::{
    AppGameUnknownAdapterCapabilityState, AppGameUnknownApprovalRequest,
    AppGameUnknownParentResponse,
};

const APP_GAME_UNKNOWN_APPROVAL_EVENT_TYPE: &str = "app-game.unknown-approval.transitioned";
const APP_GAME_UNKNOWN_APPROVAL_SCHEMA_VERSION: u16 = 1;
const APP_GAME_UNKNOWN_APPROVAL_IDEMPOTENCY_SEPARATOR: &str = ":";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum AppGameUnknownApprovalTransition {
    RequestOpened {
        request: AppGameUnknownApprovalRequest,
    },
    ParentResponded {
        actor_ref: String,
        response: AppGameUnknownParentResponse,
        capability_state: AppGameUnknownAdapterCapabilityState,
        evidence_refs: Vec<String>,
        child_reason_refs: Vec<String>,
        child_status_refs: Vec<String>,
        audit_ref: String,
        override_ref: Option<String>,
        decision_expires_at_epoch_ms: Option<u64>,
    },
    RequestExpired {
        audit_ref: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameUnknownApprovalEvent {
    pub transition_id: String,
    pub request_id: String,
    pub occurred_at_epoch_ms: u64,
    pub transition: AppGameUnknownApprovalTransition,
}

impl DomainEvent for AppGameUnknownApprovalEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            unknown_approval_event_type()?,
            SchemaVersion::new(APP_GAME_UNKNOWN_APPROVAL_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(&self.request_id)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(format!(
            "{APP_GAME_UNKNOWN_APPROVAL_EVENT_TYPE}{APP_GAME_UNKNOWN_APPROVAL_IDEMPOTENCY_SEPARATOR}{}",
            self.transition_id
        ))
    }
}

pub fn unknown_approval_event_type() -> Result<EventType, EventingError> {
    EventType::parse(APP_GAME_UNKNOWN_APPROVAL_EVENT_TYPE)
}
