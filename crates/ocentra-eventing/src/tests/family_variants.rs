use std::sync::{Arc, Mutex};
use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::bus::subscriber::EventSubscriber;
use crate::bus::EventBus;
use crate::contract_registry::EventContractRegistry;
use crate::envelope::{DomainEvent, EventContract, EventEnvelope, EventMetadata, EventSource};
use crate::error::EventingError;
use crate::ids::{
    AggregateKey, CorrelationId, EventCustody, EventType, IdempotencyKey, RecordedAt,
    RuntimeInstanceId, RuntimeRole, SchemaVersion, SourceComponent, SourceService, SubscriberId,
    TargetHandler,
};
use crate::sync::lock_unpoison;

const APPROVED_EVENT_TYPE: &str = "eventing.family.decision.approved";
const REJECTED_EVENT_TYPE: &str = "eventing.family.decision.rejected";
const APPROVED_LABEL: &str = "approved";
const REJECTED_LABEL: &str = "rejected";
const FAMILY_AGGREGATE: &str = "family-decision-aggregate";
const APPROVED_IDEMPOTENCY: &str = "family-approved-idempotency";
const REJECTED_IDEMPOTENCY: &str = "family-rejected-idempotency";
const FAMILY_CORRELATION: &str = "family-correlation";
const FAMILY_EVENT_ID: &str = "family-event-1";
const FAMILY_OBSERVED_AT: &str = "2026-06-04T02:45:00Z";
const FAMILY_SOURCE_SERVICE: &str = "family-service";
const FAMILY_SOURCE_COMPONENT: &str = "family-component";
const FAMILY_INSTANCE: &str = "family-instance";
const FAMILY_CUSTODY: &str = "local-only";
const FAMILY_RUNTIME_ROLE: &str = "agent";
const FAMILY_TARGET: &str = "family-target";
const APPROVED_SUBSCRIBER: &str = "family-approved-subscriber";
const REJECTED_SUBSCRIBER: &str = "family-rejected-subscriber";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct DecisionPayload {
    label: String,
    aggregate_key: AggregateKey,
    idempotency_key: IdempotencyKey,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "family_variant", content = "payload", rename_all = "kebab-case")]
enum DecisionFamilyEvent {
    Approved(DecisionPayload),
    Rejected(DecisionPayload),
}

impl DomainEvent for DecisionFamilyEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        let event_type = match self {
            Self::Approved(_) => APPROVED_EVENT_TYPE,
            Self::Rejected(_) => REJECTED_EVENT_TYPE,
        };
        Ok(EventContract::new(
            EventType::parse(event_type)?,
            SchemaVersion::new(1)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        Ok(decision_payload(self).aggregate_key.clone())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        Ok(decision_payload(self).idempotency_key.clone())
    }
}

#[tokio::test]
async fn family_subscriber_receives_typed_enum_variants_without_downcast()
    -> Result<(), Box<dyn Error>>
{
    let bus = EventBus::new();
    let received = Arc::new(Mutex::new(Vec::new()));

    let approved_seen = Arc::clone(&received);
    bus.subscribe::<DecisionFamilyEvent, _, _>(
        family_subscriber(APPROVED_SUBSCRIBER, APPROVED_EVENT_TYPE)?,
        move |context| {
            let approved_seen = Arc::clone(&approved_seen);
            async move {
                match context.payload() {
                    DecisionFamilyEvent::Approved(payload) => {
                        lock_unpoison(&approved_seen).push(payload.label.clone());
                    }
                    DecisionFamilyEvent::Rejected(_) => {
                        return Err(EventingError::empty_value(
                            "approved subscriber saw rejection",
                        ));
                    }
                }
                Ok(())
            }
        },
    )
    .await
    ?;

    let rejected_seen = Arc::clone(&received);
    bus.subscribe::<DecisionFamilyEvent, _, _>(
        family_subscriber(REJECTED_SUBSCRIBER, REJECTED_EVENT_TYPE)?,
        move |context| {
            let rejected_seen = Arc::clone(&rejected_seen);
            async move {
                match context.payload() {
                    DecisionFamilyEvent::Approved(_) => {
                        return Err(EventingError::empty_value(
                            "rejected subscriber saw approval",
                        ));
                    }
                    DecisionFamilyEvent::Rejected(payload) => {
                        lock_unpoison(&rejected_seen).push(payload.label.clone());
                    }
                }
                Ok(())
            }
        },
    )
    .await
    ?;

    bus.publish(approved_event()?, family_metadata()?)
        .await
        ?;
    bus.publish(rejected_event()?, family_metadata()?)
        .await
        ?;

    assert_eq!(
        lock_unpoison(&received).as_slice(),
        [APPROVED_LABEL.to_string(), REJECTED_LABEL.to_string()]
    );
    Ok(())
}

#[test]
fn family_variant_stored_decode_rejects_contract_variant_mismatch() -> Result<(), Box<dyn Error>>
{
    let envelope = EventEnvelope::from_event(approved_event()?, family_metadata()?)?;
    let mut stored = envelope.store()?;
    stored.contract = EventContract::new(
        EventType::parse(REJECTED_EVENT_TYPE)?,
        SchemaVersion::new(1)?,
    );

    assert!(matches!(
        stored.decode::<DecisionFamilyEvent>(),
        Err(EventingError::ContractMismatch { .. })
    ));
    Ok(())
}

#[test]
fn family_variants_register_as_distinct_contract_descriptors() -> Result<(), Box<dyn Error>> {
    let mut registry = EventContractRegistry::new();
    registry
        .register_event(&approved_event()?)
        ?;
    registry
        .register_event(&rejected_event()?)
        ?;

    let event_types = registry
        .descriptors()
        .map(|descriptor| descriptor.event_type().as_str().to_string())
        .collect::<Vec<_>>();
    assert_eq!(
        event_types,
        vec![
            APPROVED_EVENT_TYPE.to_string(),
            REJECTED_EVENT_TYPE.to_string()
        ]
    );
    Ok(())
}

fn approved_event() -> Result<DecisionFamilyEvent, Box<dyn Error>> {
    Ok(DecisionFamilyEvent::Approved(DecisionPayload {
        label: APPROVED_LABEL.to_string(),
        aggregate_key: AggregateKey::parse(FAMILY_AGGREGATE)?,
        idempotency_key: IdempotencyKey::parse(APPROVED_IDEMPOTENCY)?,
    }))
}

fn rejected_event() -> Result<DecisionFamilyEvent, Box<dyn Error>> {
    Ok(DecisionFamilyEvent::Rejected(DecisionPayload {
        label: REJECTED_LABEL.to_string(),
        aggregate_key: AggregateKey::parse(FAMILY_AGGREGATE)?,
        idempotency_key: IdempotencyKey::parse(REJECTED_IDEMPOTENCY)?,
    }))
}

fn decision_payload(event: &DecisionFamilyEvent) -> &DecisionPayload {
    match event {
        DecisionFamilyEvent::Approved(payload) | DecisionFamilyEvent::Rejected(payload) => payload,
    }
}

fn family_metadata() -> Result<EventMetadata, Box<dyn Error>> {
    Ok(EventMetadata::from_parts(
        crate::ids::EventId::parse(FAMILY_EVENT_ID)?,
        CorrelationId::parse(FAMILY_CORRELATION)?,
        EventSource::new(
            EventCustody::parse(FAMILY_CUSTODY)?,
            RuntimeRole::parse(FAMILY_RUNTIME_ROLE)?,
            SourceService::parse(FAMILY_SOURCE_SERVICE)?,
            SourceComponent::parse(FAMILY_SOURCE_COMPONENT)?,
            RuntimeInstanceId::parse(FAMILY_INSTANCE)?,
        ),
        RecordedAt::parse(FAMILY_OBSERVED_AT)?,
        Some(TargetHandler::parse(FAMILY_TARGET)?),
    ))
}

fn family_subscriber(id: &str, event_type: &str) -> Result<EventSubscriber, Box<dyn Error>> {
    Ok(EventSubscriber::new(
        SubscriberId::parse(id)?,
        EventType::parse(event_type)?,
        TargetHandler::parse(FAMILY_TARGET)?,
    ))
}
