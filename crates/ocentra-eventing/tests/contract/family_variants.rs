use super::support::TestText;
use ocentra_eventing::bus::subscriber::EventSubscriber;
use ocentra_eventing::bus::EventBus;
use ocentra_eventing::contract_registry::EventContractRegistry;
use ocentra_eventing::envelope::{
    DomainEvent, EventContract, EventEnvelope, EventMetadata, EventSource,
};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{
    AggregateKey, CorrelationId, EventCustody, EventType, IdempotencyKey, RecordedAt,
    RuntimeInstanceId, RuntimeRole, SchemaVersion, SourceComponent, SourceService, SubscriberId,
    TargetHandler,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

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
async fn family_subscriber_receives_typed_enum_variants_without_downcast() {
    let bus = EventBus::new();
    let received = Arc::new(Mutex::new(Vec::<TestText>::new()));

    let approved_seen = Arc::clone(&received);
    bus.subscribe::<DecisionFamilyEvent, _, _>(
        family_subscriber(
            TestText(APPROVED_SUBSCRIBER.to_owned()),
            TestText(APPROVED_EVENT_TYPE.to_owned()),
        ),
        move |context| {
            let approved_seen = Arc::clone(&approved_seen);
            async move {
                if let DecisionFamilyEvent::Approved(payload) = context.payload() {
                    record_payload(&approved_seen, payload);
                } else {
                    std::process::abort();
                }
                Ok(())
            }
        },
    )
    .await
    .expect_value("approved family subscriber registers");

    let rejected_seen = Arc::clone(&received);
    bus.subscribe::<DecisionFamilyEvent, _, _>(
        family_subscriber(
            TestText(REJECTED_SUBSCRIBER.to_owned()),
            TestText(REJECTED_EVENT_TYPE.to_owned()),
        ),
        move |context| {
            let rejected_seen = Arc::clone(&rejected_seen);
            async move {
                if let DecisionFamilyEvent::Rejected(payload) = context.payload() {
                    record_payload(&rejected_seen, payload);
                } else {
                    std::process::abort();
                }
                Ok(())
            }
        },
    )
    .await
    .expect_value("rejected family subscriber registers");

    bus.publish(approved_event(), family_metadata())
        .await
        .expect_value("approved variant publishes");
    bus.publish(rejected_event(), family_metadata())
        .await
        .expect_value("rejected variant publishes");

    assert_eq!(
        received.lock().expect_value("received lock").as_slice(),
        [
            TestText(APPROVED_LABEL.to_string()),
            TestText(REJECTED_LABEL.to_string())
        ]
    );
}

#[test]
fn family_variant_stored_decode_rejects_contract_variant_mismatch() {
    let envelope = EventEnvelope::from_event(approved_event(), family_metadata())
        .expect_value("approved envelope builds");
    let mut stored = envelope.store().expect_value("approved envelope stores");
    stored.contract = EventContract::new(
        EventType::parse(REJECTED_EVENT_TYPE).expect_value("rejected event type parses"),
        SchemaVersion::new(1).expect_value("schema version parses"),
    );

    assert!(matches!(
        stored.decode::<DecisionFamilyEvent>(),
        Err(EventingError::ContractMismatch { .. })
    ));
}

#[test]
fn family_variants_register_as_distinct_contract_descriptors() {
    let mut registry = EventContractRegistry::new();
    registry
        .register_event(&approved_event())
        .expect_value("approved family variant registers");
    registry
        .register_event(&rejected_event())
        .expect_value("rejected family variant registers");

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
}

fn approved_event() -> DecisionFamilyEvent {
    DecisionFamilyEvent::Approved(DecisionPayload {
        label: APPROVED_LABEL.to_string(),
        aggregate_key: AggregateKey::parse(FAMILY_AGGREGATE).expect_value("aggregate parses"),
        idempotency_key: IdempotencyKey::parse(APPROVED_IDEMPOTENCY)
            .expect_value("idempotency parses"),
    })
}

fn rejected_event() -> DecisionFamilyEvent {
    DecisionFamilyEvent::Rejected(DecisionPayload {
        label: REJECTED_LABEL.to_string(),
        aggregate_key: AggregateKey::parse(FAMILY_AGGREGATE).expect_value("aggregate parses"),
        idempotency_key: IdempotencyKey::parse(REJECTED_IDEMPOTENCY)
            .expect_value("idempotency parses"),
    })
}

fn decision_payload(event: &DecisionFamilyEvent) -> &DecisionPayload {
    match event {
        DecisionFamilyEvent::Approved(payload) | DecisionFamilyEvent::Rejected(payload) => payload,
    }
}

fn record_payload(received: &Arc<Mutex<Vec<TestText>>>, payload: &DecisionPayload) {
    received
        .lock()
        .expect_value("received lock")
        .push(TestText(payload.label.clone()));
}

fn family_metadata() -> EventMetadata {
    EventMetadata::from_parts(
        ocentra_eventing::ids::EventId::parse(FAMILY_EVENT_ID).expect_value("event id parses"),
        CorrelationId::parse(FAMILY_CORRELATION).expect_value("correlation parses"),
        EventSource::new(
            EventCustody::parse(FAMILY_CUSTODY).expect_value("event custody parses"),
            RuntimeRole::parse(FAMILY_RUNTIME_ROLE).expect_value("runtime role parses"),
            SourceService::parse(FAMILY_SOURCE_SERVICE).expect_value("source service parses"),
            SourceComponent::parse(FAMILY_SOURCE_COMPONENT).expect_value("source component parses"),
            RuntimeInstanceId::parse(FAMILY_INSTANCE).expect_value("runtime instance parses"),
        ),
        RecordedAt::parse(FAMILY_OBSERVED_AT).expect_value("recorded at parses"),
        Some(TargetHandler::parse(FAMILY_TARGET).expect_value("target handler parses")),
    )
}

fn family_subscriber(id: TestText, event_type: TestText) -> EventSubscriber {
    EventSubscriber::new(
        SubscriberId::parse(id.0).expect_value("subscriber id parses"),
        EventType::parse(event_type.0).expect_value("event type parses"),
        TargetHandler::parse(FAMILY_TARGET).expect_value("target handler parses"),
    )
}
