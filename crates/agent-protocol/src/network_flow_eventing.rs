use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{
    AggregateKey, EventType, IdempotencyKey, RuntimeInstanceId, SchemaVersion,
};

use super::{constants, NetworkFlowObservedEvent, NetworkRuntimeEventContract};

impl DomainEvent for NetworkFlowObservedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(Self::EVENT_TYPE)?,
            SchemaVersion::new(self.schema_version)?,
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
        IdempotencyKey::parse(format!(
            "{}{}-{}",
            constants::network_flow::IDEMPOTENCY_NETWORK_RUNTIME_PREFIX,
            Self::EVENT_TYPE,
            flow_event_ref.as_str()
        ))
    }
}
