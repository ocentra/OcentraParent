use super::support::{
    metadata, subscriber, test_event, TestEvent, TestText, OTHER_EVENT_TYPE, OTHER_SUBSCRIBER,
    OTHER_TARGET, TEST_LABEL, TEST_SUBSCRIBER, TEST_TARGET,
};
use ocentra_eventing::bus::EventBus;
use ocentra_eventing::envelope::{EventEnvelope, EventPriority};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{CausationId, EventNamespace, EventType, RecordedAt, SchemaVersion};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn event_bus_dispatches_typed_envelope_and_stores_serialized_boundary() {
    let bus = EventBus::new();
    let handled = Arc::new(Mutex::new(Vec::new()));
    let handled_clone = Arc::clone(&handled);
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |context| {
            let handled = Arc::clone(&handled_clone);
            async move {
                handled.lock().await.push(context.payload().label.clone());
                Ok(())
            }
        },
    )
    .await
    .expect_value("subscriber registers");

    let metadata = metadata(TestText(TEST_TARGET.to_owned()))
        .with_causation_id(
            CausationId::parse("causation-test-1").expect_value("causation id parses"),
        )
        .with_priority(EventPriority::High);
    let report = bus
        .publish(test_event(TestText(TEST_LABEL.to_owned())), metadata)
        .await
        .expect_value("publish succeeds");
    let journal = bus.journal().await;
    let decoded: EventEnvelope<TestEvent> =
        journal[0].decode().expect_value("stored envelope decodes");

    assert_eq!(report.subscriber_count, 1);
    assert_eq!(report.handled_count, 1);
    assert_eq!(handled.lock().await.as_slice(), &[TEST_LABEL.to_string()]);
    assert_eq!(decoded.payload.label, TEST_LABEL);
    assert_eq!(
        decoded
            .causation_id
            .as_ref()
            .expect_value("causation id is stored")
            .as_str(),
        "causation-test-1"
    );
    assert_eq!(decoded.priority, EventPriority::High);
    assert_eq!(journal.len(), 1);
}

#[tokio::test]
async fn target_handler_filter_prevents_wrong_handler_delivery() {
    let bus = EventBus::new();
    let handled = Arc::new(Mutex::new(0_usize));
    let handled_clone = Arc::clone(&handled);
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |_| {
            let handled = Arc::clone(&handled_clone);
            async move {
                *handled.lock().await += 1;
                Ok(())
            }
        },
    )
    .await
    .expect_value("subscriber registers");
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(OTHER_SUBSCRIBER.to_owned()),
            TestText(OTHER_TARGET.to_owned()),
        ),
        |_| async { Ok(()) },
    )
    .await
    .expect_value("second subscriber registers");

    let report = bus
        .publish(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("publish succeeds");

    assert_eq!(report.subscriber_count, 1);
    assert_eq!(*handled.lock().await, 1);
}

#[tokio::test]
async fn concurrent_dispatch_records_handler_dead_letter_without_losing_journal() {
    let bus = EventBus::new();
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        |_| async {
            Err(EventingError::InvalidValue {
                field: "handler_failure",
                value: "handler_failure".to_string(),
            })
        },
    )
    .await
    .expect_value("subscriber registers");

    let report = bus
        .publish_with_mode(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
            ocentra_eventing::bus::DispatchMode::Concurrent,
        )
        .await
        .expect_value("publish succeeds");
    let dead_letters = bus.dead_letters().await;

    assert_eq!(report.dead_letter_count, 1);
    assert_eq!(report.handled_count, 0);
    assert_eq!(bus.journal().await.len(), 1);
    assert_eq!(
        dead_letters[0]
            .target_handler
            .as_ref()
            .expect_value("handler dead letter has target")
            .as_str(),
        TEST_TARGET
    );
}

#[tokio::test]
async fn duplicate_subscriber_ids_are_rejected() {
    let bus = EventBus::new();
    let duplicate = subscriber(
        TestText(TEST_SUBSCRIBER.to_owned()),
        TestText(TEST_TARGET.to_owned()),
    );
    bus.subscribe::<TestEvent, _, _>(duplicate.clone(), |_| async { Ok(()) })
        .await
        .expect_value("first subscriber registers");

    let result = bus
        .subscribe::<TestEvent, _, _>(duplicate, |_| async { Ok(()) })
        .await;

    assert!(matches!(
        result,
        Err(EventingError::DuplicateSubscriber { .. })
    ));
}

#[test]
fn eventing_newtypes_reject_empty_values_and_zero_versions() {
    assert_eq!(
        EventType::parse(""),
        Err(EventingError::EmptyValue {
            field: "event_type"
        })
    );
    assert_eq!(
        EventType::parse(".leading"),
        Err(EventingError::InvalidValue {
            field: "event_type",
            value: ".leading".to_owned(),
        })
    );
    assert_eq!(
        EventType::parse("trailing."),
        Err(EventingError::InvalidValue {
            field: "event_type",
            value: "trailing.".to_owned(),
        })
    );
    assert_eq!(
        EventType::parse("empty..segment"),
        Err(EventingError::InvalidValue {
            field: "event_type",
            value: "empty..segment".to_owned(),
        })
    );
    assert_eq!(
        EventType::parse("eventing/slash-taxonomy/observed")
            .expect_value("slash taxonomy event type parses")
            .as_str(),
        "eventing/slash-taxonomy/observed"
    );
    assert_eq!(
        RecordedAt::parse(" "),
        Err(EventingError::EmptyValue {
            field: "recorded_at"
        })
    );
    assert_eq!(SchemaVersion::new(0), Err(EventingError::InvalidVersion));
}

#[test]
fn event_namespaces_match_dot_and_slash_event_taxonomy() {
    let slash_event =
        EventType::parse("network/transport/observed").expect_value("slash event type parses");
    let dot_event =
        EventType::parse("network.transport.observed").expect_value("dot event type parses");
    let network_namespace = EventNamespace::parse("network").expect_value("namespace parses");

    assert_eq!(
        EventNamespace::from_event_type(&slash_event)
            .expect_value("slash namespace derives")
            .as_str(),
        "network"
    );
    assert!(network_namespace.matches_event_type(&slash_event));
    assert!(network_namespace.matches_event_type(&dot_event));
}

#[test]
fn stored_decode_rejects_contract_mismatch() {
    let envelope = EventEnvelope::from_event(
        test_event(TestText(TEST_LABEL.to_owned())),
        metadata(TestText(TEST_TARGET.to_owned())),
    )
    .expect_value("envelope builds");
    let mut stored = envelope.store().expect_value("stored envelope builds");
    stored.contract.event_type =
        EventType::parse(OTHER_EVENT_TYPE).expect_value("other event parses");

    let decoded = stored.decode::<super::support::TestEvent>();

    assert!(matches!(
        decoded,
        Err(EventingError::ContractMismatch { .. })
    ));
}
