use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{
    AggregateKey, EventType, IdempotencyKey, RuntimeInstanceId, SchemaVersion,
};

use super::{constants, NetworkFlowObservedEvent};

impl DomainEvent for NetworkFlowObservedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        if self.schema_version != constants::network_flow::EVENT_SCHEMA_VERSION {
            return Err(EventingError::InvalidVersion);
        }
        Ok(EventContract::new(
            EventType::parse(constants::network_flow::EVENT_NETWORK_FLOW_EVENTING_OBSERVED)?,
            SchemaVersion::new(constants::network_flow::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        let device_ref = RuntimeInstanceId::parse(self.device_ref.clone())?;
        AggregateKey::parse(format!(
            "{}{}",
            constants::network_flow::AGGREGATE_NETWORK_FLOW_PREFIX,
            device_ref.as_str()
        ))
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let flow_event_ref = IdempotencyKey::parse(self.flow_event_ref.clone())?;
        let aggregate_key = self.aggregate_key()?;
        IdempotencyKey::parse(format!(
            "{}{}-{}:{}-{}:{}",
            constants::network_flow::IDEMPOTENCY_NETWORK_RUNTIME_PREFIX,
            constants::network_flow::EVENT_NETWORK_FLOW_EVENTING_OBSERVED,
            aggregate_key.as_str().len(),
            aggregate_key.as_str(),
            flow_event_ref.as_str().len(),
            flow_event_ref.as_str()
        ))
    }
}
