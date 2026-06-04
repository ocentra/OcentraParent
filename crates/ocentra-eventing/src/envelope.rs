use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    AggregateKey, CorrelationId, EventClockInstant, EventId, EventType, EventingError,
    IdempotencyKey, RecordedAt, RuntimeInstanceId, SchemaVersion, SourceComponent, SourceService,
    TargetHandler,
};

pub trait DomainEvent: Clone + Send + Sync + Serialize + DeserializeOwned + 'static {
    fn contract(&self) -> Result<EventContract, EventingError>;
    fn aggregate_key(&self) -> Result<AggregateKey, EventingError>;
    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError>;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventContract {
    pub event_type: EventType,
    pub schema_version: SchemaVersion,
}

impl EventContract {
    pub fn new(event_type: EventType, schema_version: SchemaVersion) -> Self {
        Self {
            event_type,
            schema_version,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RuntimeRole {
    ParentController,
    ChildAgent,
    Analyzer,
    PolicyEngine,
    EnforcementAdapter,
    AuditWriter,
    PortalReadModel,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EventCustody {
    LocalOnly,
    ChildDeviceJournal,
    ChildDeviceQueryStore,
    ParentDeviceCache,
    ParentOwnedExport,
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventSource {
    pub custody: EventCustody,
    pub role: RuntimeRole,
    pub service: SourceService,
    pub component: SourceComponent,
    pub instance_id: RuntimeInstanceId,
}

impl EventSource {
    pub fn new(
        custody: EventCustody,
        role: RuntimeRole,
        service: SourceService,
        component: SourceComponent,
        instance_id: RuntimeInstanceId,
    ) -> Self {
        Self {
            custody,
            role,
            service,
            component,
            instance_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventMetadata {
    pub event_id: EventId,
    pub correlation_id: CorrelationId,
    pub source: EventSource,
    pub observed_at: RecordedAt,
    pub target_handler: Option<TargetHandler>,
    #[serde(default)]
    pub deadline: Option<EventClockInstant>,
}

impl EventMetadata {
    pub fn new(correlation_id: CorrelationId, source: EventSource) -> Self {
        Self {
            event_id: EventId::generated(),
            correlation_id,
            source,
            observed_at: RecordedAt::now_utc(),
            target_handler: None,
            deadline: None,
        }
    }

    pub fn from_parts(
        event_id: EventId,
        correlation_id: CorrelationId,
        source: EventSource,
        observed_at: RecordedAt,
        target_handler: Option<TargetHandler>,
    ) -> Self {
        Self {
            event_id,
            correlation_id,
            source,
            observed_at,
            target_handler,
            deadline: None,
        }
    }

    pub fn with_deadline(mut self, deadline: EventClockInstant) -> Self {
        self.deadline = Some(deadline);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventEnvelope<E> {
    pub contract: EventContract,
    pub event_id: EventId,
    pub correlation_id: CorrelationId,
    pub aggregate_key: AggregateKey,
    pub idempotency_key: IdempotencyKey,
    pub source: EventSource,
    pub observed_at: RecordedAt,
    pub target_handler: Option<TargetHandler>,
    #[serde(default)]
    pub deadline: Option<EventClockInstant>,
    pub payload: E,
}

impl<E> EventEnvelope<E>
where
    E: DomainEvent,
{
    pub fn from_event(payload: E, metadata: EventMetadata) -> Result<Self, EventingError> {
        Ok(Self {
            contract: payload.contract()?,
            event_id: metadata.event_id,
            correlation_id: metadata.correlation_id,
            aggregate_key: payload.aggregate_key()?,
            idempotency_key: payload.idempotency_key()?,
            source: metadata.source,
            observed_at: metadata.observed_at,
            target_handler: metadata.target_handler,
            deadline: metadata.deadline,
            payload,
        })
    }

    pub fn store(&self) -> Result<StoredEventEnvelope, EventingError> {
        Ok(StoredEventEnvelope {
            contract: self.contract.clone(),
            event_id: self.event_id.clone(),
            correlation_id: self.correlation_id.clone(),
            aggregate_key: self.aggregate_key.clone(),
            idempotency_key: self.idempotency_key.clone(),
            source: self.source.clone(),
            observed_at: self.observed_at.clone(),
            target_handler: self.target_handler.clone(),
            deadline: self.deadline,
            payload: serde_json::to_value(&self.payload).map_err(EventingError::payload_encode)?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoredEventEnvelope {
    pub contract: EventContract,
    pub event_id: EventId,
    pub correlation_id: CorrelationId,
    pub aggregate_key: AggregateKey,
    pub idempotency_key: IdempotencyKey,
    pub source: EventSource,
    pub observed_at: RecordedAt,
    pub target_handler: Option<TargetHandler>,
    #[serde(default)]
    pub deadline: Option<EventClockInstant>,
    pub payload: serde_json::Value,
}

impl StoredEventEnvelope {
    pub fn decode<E>(&self) -> Result<EventEnvelope<E>, EventingError>
    where
        E: DomainEvent,
    {
        let payload: E = serde_json::from_value(self.payload.clone()).map_err(|error| {
            EventingError::payload_decode(self.contract.event_type.as_str().to_string(), error)
        })?;
        let expected = payload.contract()?;
        if expected != self.contract {
            return Err(EventingError::ContractMismatch {
                expected: expected.event_type.as_str().to_string(),
                received: self.contract.event_type.as_str().to_string(),
            });
        }
        Ok(EventEnvelope {
            contract: self.contract.clone(),
            event_id: self.event_id.clone(),
            correlation_id: self.correlation_id.clone(),
            aggregate_key: self.aggregate_key.clone(),
            idempotency_key: self.idempotency_key.clone(),
            source: self.source.clone(),
            observed_at: self.observed_at.clone(),
            target_handler: self.target_handler.clone(),
            deadline: self.deadline,
            payload,
        })
    }

    pub fn is_deadline_expired(&self, now: EventClockInstant) -> bool {
        self.deadline.is_some_and(|deadline| now >= deadline)
    }
}
