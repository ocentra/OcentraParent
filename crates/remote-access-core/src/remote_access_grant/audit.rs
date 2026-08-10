use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};

use super::{
    replay_identity::{encode_component, transition_key},
    RemoteAccessGrantAuditMilestone, RemoteAccessGrantAuditOutcome,
    REMOTE_ACCESS_GRANT_AUDIT_EVENT_TYPE, REMOTE_ACCESS_GRANT_AUDIT_SCHEMA_VERSION,
};

impl DomainEvent for RemoteAccessGrantAuditMilestone {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(REMOTE_ACCESS_GRANT_AUDIT_EVENT_TYPE)?,
            SchemaVersion::new(REMOTE_ACCESS_GRANT_AUDIT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(format!("remote-access-grant:{}", self.grant_id))
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(format!(
            "remote-access-grant:{}:{}:{}:{}:{}",
            encode_component(&self.grant_id),
            encode_component(&self.audit_ref),
            encode_component(&self.attempt_ref),
            encode_component(transition_key(self.transition)),
            encode_component(outcome_key(self.outcome)),
        ))
    }
}

fn outcome_key(outcome: RemoteAccessGrantAuditOutcome) -> &'static str {
    match outcome {
        RemoteAccessGrantAuditOutcome::Accepted => "accepted",
        RemoteAccessGrantAuditOutcome::Denied => "denied",
    }
}
