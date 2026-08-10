use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};

use super::{
    RemoteAccessGrantAuditMilestone, RemoteAccessGrantAuditOutcome, RemoteAccessGrantTransition,
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
            "remote-access-grant:{}:{}:{}",
            self.audit_ref,
            self.transition.as_str(),
            self.outcome.as_str()
        ))
    }
}

impl RemoteAccessGrantAuditOutcome {
    fn as_str(self) -> &'static str {
        ["accepted", "denied"][self as usize]
    }
}

impl RemoteAccessGrantTransition {
    fn as_str(self) -> &'static str {
        [
            "confirm-parent",
            "pair",
            "activate",
            "pause",
            "stop",
            "request-reconnect",
            "reconnect",
            "revoke",
            "remove-device",
        ][self as usize]
    }
}
