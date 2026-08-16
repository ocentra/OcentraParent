use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, IdempotencyKey};

use super::{
    storage_custody_events, StorageCustodyActionPlannedEvent, StorageCustodyDecisionRecordedEvent,
    STORAGE_CUSTODY_ACTION_PLANNED_EVENT_TYPE, STORAGE_CUSTODY_DECISION_RECORDED_EVENT_TYPE,
};

impl DomainEvent for StorageCustodyDecisionRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        storage_custody_events::storage_custody_event_contract(
            STORAGE_CUSTODY_DECISION_RECORDED_EVENT_TYPE,
        )
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        storage_custody_events::storage_custody_idempotency_key(
            STORAGE_CUSTODY_DECISION_RECORDED_EVENT_TYPE,
            &self.decision_id,
        )
    }
}

impl DomainEvent for StorageCustodyActionPlannedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        storage_custody_events::storage_custody_event_contract(
            STORAGE_CUSTODY_ACTION_PLANNED_EVENT_TYPE,
        )
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        storage_custody_events::storage_custody_idempotency_key(
            STORAGE_CUSTODY_ACTION_PLANNED_EVENT_TYPE,
            &self.action_plan_id,
        )
    }
}
