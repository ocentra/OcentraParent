use serde::{
    de::{DeserializeOwned, Deserializer},
    Deserialize, Serialize,
};

use crate::{
    AggregateKey, CausationId, CorrelationId, EventClockInstant, EventCustody, EventId, EventType,
    EventingError, IdempotencyKey, RecordedAt, RuntimeInstanceId, RuntimeRole, SchemaVersion,
    SourceComponent, SourceService, TargetHandler,
};

mod accessors;

pub trait DomainEvent: Clone + Send + Sync + Serialize + DeserializeOwned + 'static {
    fn contract(&self) -> Result<EventContract, EventingError>;
    fn aggregate_key(&self) -> Result<AggregateKey, EventingError>;
    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError>;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EventPriority {
    Low,
    #[default]
    Normal,
    High,
    Critical,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct EventMetadata {
    pub event_id: EventId,
    pub correlation_id: CorrelationId,
    #[serde(default)]
    pub causation_id: Option<CausationId>,
    pub source: EventSource,
    pub observed_at: RecordedAt,
    pub target_handler: Option<TargetHandler>,
    #[serde(default)]
    pub priority: EventPriority,
    #[serde(default)]
    pub deadline: Option<EventClockInstant>,
}

impl EventMetadata {
    pub fn new(correlation_id: CorrelationId, source: EventSource) -> Self {
        Self {
            event_id: EventId::generated(),
            correlation_id,
            causation_id: None,
            source,
            observed_at: RecordedAt::now_utc(),
            target_handler: None,
            priority: EventPriority::Normal,
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
            causation_id: None,
            source,
            observed_at,
            target_handler,
            priority: EventPriority::Normal,
            deadline: None,
        }
    }

    pub fn with_causation_id(mut self, causation_id: CausationId) -> Self {
        self.causation_id = Some(causation_id);
        self
    }

    pub fn with_priority(mut self, priority: EventPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_deadline(mut self, deadline: EventClockInstant) -> Self {
        self.deadline = Some(deadline);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", bound(serialize = "E: Serialize"))]
pub struct EventEnvelope<E: DomainEvent> {
    contract: EventContract,
    event_id: EventId,
    correlation_id: CorrelationId,
    causation_id: Option<CausationId>,
    aggregate_key: AggregateKey,
    idempotency_key: IdempotencyKey,
    source: EventSource,
    observed_at: RecordedAt,
    target_handler: Option<TargetHandler>,
    priority: EventPriority,
    #[serde(default)]
    deadline: Option<EventClockInstant>,
    payload: E,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", bound(deserialize = "E: Deserialize<'de>"))]
struct EventEnvelopeWire<E> {
    contract: EventContract,
    event_id: EventId,
    correlation_id: CorrelationId,
    #[serde(default)]
    causation_id: Option<CausationId>,
    aggregate_key: AggregateKey,
    idempotency_key: IdempotencyKey,
    source: EventSource,
    observed_at: RecordedAt,
    target_handler: Option<TargetHandler>,
    #[serde(default)]
    priority: EventPriority,
    #[serde(default)]
    deadline: Option<EventClockInstant>,
    payload: E,
}

impl<'de, E> Deserialize<'de> for EventEnvelope<E>
where
    E: DomainEvent + DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = EventEnvelopeWire::<E>::deserialize(deserializer)?;
        validate_payload_identity(
            &wire.payload,
            &wire.contract,
            &wire.aggregate_key,
            &wire.idempotency_key,
        )
        .map_err(|error| <D::Error as serde::de::Error>::custom(error.to_string()))?;
        Ok(Self {
            contract: wire.contract,
            event_id: wire.event_id,
            correlation_id: wire.correlation_id,
            causation_id: wire.causation_id,
            aggregate_key: wire.aggregate_key,
            idempotency_key: wire.idempotency_key,
            source: wire.source,
            observed_at: wire.observed_at,
            target_handler: wire.target_handler,
            priority: wire.priority,
            deadline: wire.deadline,
            payload: wire.payload,
        })
    }
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
            causation_id: metadata.causation_id,
            aggregate_key: payload.aggregate_key()?,
            idempotency_key: payload.idempotency_key()?,
            source: metadata.source,
            observed_at: metadata.observed_at,
            target_handler: metadata.target_handler,
            priority: metadata.priority,
            deadline: metadata.deadline,
            payload,
        })
    }

    pub fn store(&self) -> Result<StoredEventEnvelope, EventingError> {
        validate_payload_identity(
            self.payload(),
            &self.contract,
            &self.aggregate_key,
            &self.idempotency_key,
        )?;
        Ok(StoredEventEnvelope {
            contract: self.contract.clone(),
            event_id: self.event_id.clone(),
            correlation_id: self.correlation_id.clone(),
            causation_id: self.causation_id.clone(),
            aggregate_key: self.aggregate_key.clone(),
            idempotency_key: self.idempotency_key.clone(),
            source: self.source.clone(),
            observed_at: self.observed_at.clone(),
            target_handler: self.target_handler.clone(),
            priority: self.priority,
            deadline: self.deadline,
            payload: StoredEventPayload::from_event(self.payload())?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StoredEventPayload {
    value: serde_json::Value,
}

impl StoredEventPayload {
    fn from_event<E>(payload: &E) -> Result<Self, EventingError>
    where
        E: Serialize,
    {
        Ok(Self {
            value: serde_json::to_value(payload)
                .map_err(|error| EventingError::payload_encode(&error))?,
        })
    }

    fn decode<E>(&self) -> Result<E, serde_json::Error>
    where
        E: DeserializeOwned,
    {
        serde_json::from_value(self.value.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredEventEnvelope {
    pub contract: EventContract,
    pub event_id: EventId,
    pub correlation_id: CorrelationId,
    #[serde(default)]
    pub causation_id: Option<CausationId>,
    pub aggregate_key: AggregateKey,
    pub idempotency_key: IdempotencyKey,
    pub source: EventSource,
    pub observed_at: RecordedAt,
    pub target_handler: Option<TargetHandler>,
    #[serde(default)]
    pub priority: EventPriority,
    #[serde(default)]
    pub deadline: Option<EventClockInstant>,
    pub payload: StoredEventPayload,
}

impl StoredEventEnvelope {
    /// Decodes the typed payload and revalidates every payload-derived envelope field.
    ///
    /// This is a structural/type boundary, not an integrity primitive. Persisted
    /// envelopes must reach this method through a journal read that has already
    /// verified its hash chain; decoding raw caller-supplied JSON does not prove
    /// the remaining transport metadata authentic.
    pub fn decode<E>(&self) -> Result<EventEnvelope<E>, EventingError>
    where
        E: DomainEvent,
    {
        let payload: E = self.payload.decode::<E>().map_err(|error| {
            EventingError::payload_decode(self.contract.event_type.clone(), &error)
        })?;
        validate_payload_identity(
            &payload,
            &self.contract,
            &self.aggregate_key,
            &self.idempotency_key,
        )?;
        Ok(EventEnvelope {
            contract: self.contract.clone(),
            event_id: self.event_id.clone(),
            correlation_id: self.correlation_id.clone(),
            causation_id: self.causation_id.clone(),
            aggregate_key: self.aggregate_key.clone(),
            idempotency_key: self.idempotency_key.clone(),
            source: self.source.clone(),
            observed_at: self.observed_at.clone(),
            target_handler: self.target_handler.clone(),
            priority: self.priority,
            deadline: self.deadline,
            payload,
        })
    }

    pub fn is_deadline_expired(&self, now: EventClockInstant) -> bool {
        self.deadline.is_some_and(|deadline| now >= deadline)
    }
}

fn validate_payload_identity<E>(
    payload: &E,
    contract: &EventContract,
    aggregate_key: &AggregateKey,
    idempotency_key: &IdempotencyKey,
) -> Result<(), EventingError>
where
    E: DomainEvent,
{
    let expected = payload.contract()?;
    if expected != *contract {
        return Err(EventingError::ContractMismatch {
            expected: expected.event_type,
            received: contract.event_type.clone(),
            expected_schema_version: expected.schema_version,
            received_schema_version: contract.schema_version,
        });
    }
    if payload.aggregate_key()? != *aggregate_key {
        return Err(EventingError::invalid_value(
            "stored_event.aggregate_key",
            "[redacted mismatch]",
        ));
    }
    if payload.idempotency_key()? != *idempotency_key {
        return Err(EventingError::invalid_value(
            "stored_event.idempotency_key",
            "[redacted mismatch]",
        ));
    }
    Ok(())
}
