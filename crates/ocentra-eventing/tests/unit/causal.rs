use std::{sync::Arc, time::Duration};

use tokio::sync::{Mutex, Notify};

use super::fixtures::{
    metadata, metadata_with_event_id, subscriber, subscriber_for_event, test_event,
    test_event_for_type, TestText, OTHER_EVENT_TYPE, OTHER_SUBSCRIBER, OTHER_TARGET,
    TEST_SUBSCRIBER, TEST_TARGET,
};
use crate::bus::publisher::RootEventPublisher;
use crate::{EventBus, EventingError, ExpectValue};

fn accepts_root_publication_authority(_: &RootEventPublisher) {}

#[test]
fn raw_event_bus_is_not_a_root_publication_authority() {
    let raw_bus = EventBus::default();
    let root = EventBus::new();

    accepts_root_publication_authority(&root);
    assert_ne!(
        std::any::type_name_of_val(&raw_bus),
        std::any::type_name_of_val(&root)
    );
}

#[tokio::test]
async fn task_local_spawn_rejects_causal_publication_without_side_effects() {
    let bus = EventBus::new();
    bus.subscribe::<super::fixtures::TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        |context| async move {
            let publisher = context.publisher().clone();
            let result = tokio::spawn(async move {
                publisher
                    .publish(
                        test_event_for_type(
                            TestText("spawned-nested".to_owned()),
                            TestText(OTHER_EVENT_TYPE.to_owned()),
                        ),
                        metadata_with_event_id(
                            TestText(OTHER_TARGET.to_owned()),
                            TestText("spawned-nested-event-1".to_owned()),
                        ),
                    )
                    .await
            })
            .await
            .expect_value("spawned causal publication joins");

            assert!(matches!(
                result,
                Err(EventingError::CausalPublicationOutsideHandlerTask)
            ));
            Ok(())
        },
    )
    .await
    .expect_value("outer subscriber registers");

    let report = bus
        .publish(
            test_event(TestText("spawned-outer".to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("outer publication succeeds");

    assert_eq!(report.handled_count, 1);
    assert!(bus
        .journal()
        .await
        .iter()
        .all(|event| event.contract.event_type.as_str() != OTHER_EVENT_TYPE));
    assert!(bus.dead_letters().await.is_empty());
}

#[tokio::test]
async fn cancelling_causal_publication_leaves_no_nested_effects() {
    let bus = EventBus::new();
    let nested_started = Arc::new(Notify::new());

    let started_for_handler = Arc::clone(&nested_started);
    bus.subscribe::<super::fixtures::TestEvent, _, _>(
        subscriber_for_event(
            TestText(OTHER_SUBSCRIBER.to_owned()),
            TestText(OTHER_TARGET.to_owned()),
            TestText(OTHER_EVENT_TYPE.to_owned()),
        ),
        move |_| {
            let started = Arc::clone(&started_for_handler);
            async move {
                started.notify_one();
                std::future::pending::<Result<(), EventingError>>().await
            }
        },
    )
    .await
    .expect_value("nested subscriber registers");

    bus.subscribe::<super::fixtures::TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        |context| async move {
            context
                .publisher()
                .publish(
                    test_event_for_type(
                        TestText("cancelled-nested".to_owned()),
                        TestText(OTHER_EVENT_TYPE.to_owned()),
                    ),
                    metadata_with_event_id(
                        TestText(OTHER_TARGET.to_owned()),
                        TestText("cancelled-nested-event-1".to_owned()),
                    ),
                )
                .await?;
            Ok(())
        },
    )
    .await
    .expect_value("outer subscriber registers");

    let publication = tokio::spawn({
        let bus = bus.clone();
        async move {
            bus.publish(
                test_event(TestText("cancelled-outer".to_owned())),
                metadata_with_event_id(
                    TestText(TEST_TARGET.to_owned()),
                    TestText("cancelled-outer-event-1".to_owned()),
                ),
            )
            .await
        }
    });
    tokio::time::timeout(Duration::from_secs(1), nested_started.notified())
        .await
        .expect_value("nested handler starts before cancellation");
    publication.abort();
    let _ = publication.await;
    tokio::task::yield_now().await;

    let nested_journal_count = bus
        .journal()
        .await
        .iter()
        .filter(|event| event.contract.event_type.as_str() == OTHER_EVENT_TYPE)
        .count();
    let nested_dead_letter_count = bus
        .dead_letters()
        .await
        .iter()
        .filter(|dead_letter| dead_letter.envelope.contract.event_type.as_str() == OTHER_EVENT_TYPE)
        .count();
    let metrics = bus.metrics_snapshot().await;

    assert_eq!(nested_journal_count, 0);
    assert_eq!(nested_dead_letter_count, 0);
    assert_eq!(metrics.queue.queued_event_count, 0);
    assert_eq!(metrics.queue.in_flight_event_id_count, 0);
}

#[tokio::test]
async fn cross_bus_parent_child_publication_preserves_correlation() {
    let parent_bus = EventBus::new();
    let child_root = EventBus::new();
    let child_bus = child_root.event_bus().clone();
    let child_correlation = Arc::new(Mutex::new(None::<String>));
    let child_correlation_for_handler = Arc::clone(&child_correlation);

    child_root
        .subscribe::<super::fixtures::TestEvent, _, _>(
            subscriber_for_event(
                TestText(OTHER_SUBSCRIBER.to_owned()),
                TestText(OTHER_TARGET.to_owned()),
                TestText(OTHER_EVENT_TYPE.to_owned()),
            ),
            move |context| {
                let observed = Arc::clone(&child_correlation_for_handler);
                async move {
                    *observed.lock().await =
                        Some(context.envelope().correlation_id().as_str().to_owned());
                    Ok(())
                }
            },
        )
        .await
        .expect_value("child subscriber registers");

    let child_bus_for_handler = child_bus.clone();
    parent_bus
        .subscribe::<super::fixtures::TestEvent, _, _>(
            subscriber(
                TestText(TEST_SUBSCRIBER.to_owned()),
                TestText(TEST_TARGET.to_owned()),
            ),
            move |context| {
                let child_bus = child_bus_for_handler.clone();
                async move {
                    context
                        .publisher()
                        .publish_on(
                            &child_bus,
                            test_event_for_type(
                                TestText("child-work".to_owned()),
                                TestText(OTHER_EVENT_TYPE.to_owned()),
                            ),
                            metadata_with_event_id(
                                TestText(OTHER_TARGET.to_owned()),
                                TestText("child-work-event-1".to_owned()),
                            ),
                        )
                        .await?;
                    Ok(())
                }
            },
        )
        .await
        .expect_value("parent subscriber registers");

    parent_bus
        .publish(
            test_event(TestText("parent-work".to_owned())),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("parent-work-event-1".to_owned()),
            ),
        )
        .await
        .expect_value("parent publication succeeds");
    let parent_correlation = parent_bus
        .journal()
        .await
        .into_iter()
        .find(|event| event.contract.event_type.as_str() == super::fixtures::TEST_EVENT_TYPE)
        .expect_value("parent journal records root event")
        .correlation_id
        .as_str()
        .to_owned();

    assert_eq!(*child_correlation.lock().await, Some(parent_correlation));
    assert_eq!(child_bus.journal().await.len(), 1);
}
