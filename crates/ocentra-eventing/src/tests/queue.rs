use std::{
    error::Error,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex as StdMutex},
    time::Duration,
};

use tokio::sync::{Mutex, Notify};

use super::fixtures::{
    metadata, metadata_with_event_id, subscriber, subscriber_for_event, test_event,
    test_event_for_type_with_aggregate_and_idempotency, test_event_with_idempotency, TestEvent,
    OTHER_EVENT_TYPE, OTHER_SUBSCRIBER, OTHER_TARGET, TEST_LABEL, TEST_SUBSCRIBER, TEST_TARGET,
};
use crate::bus::reports::DeadLetterReason;
use crate::bus::DispatchMode;
use crate::bus::EventBus;
use crate::envelope::DomainEvent;
use crate::error::EventingError;
use crate::journal::{EventJournal, JournalAppend, policy::{JournalPolicy, JournalSelector}};
use crate::queue::policy::{EventQueuePolicy, QueueDisposition};
use crate::envelope::StoredEventEnvelope;

type TestError = Box<dyn Error + Send + Sync>;
type TestResult = Result<(), TestError>;

fn result_context<T, E: std::fmt::Display>(
    result: Result<T, E>,
    context: &str,
) -> Result<T, TestError> {
    result.map_err(|err| std::io::Error::other(format!("{context}: {err}")).into())
}

#[tokio::test]
async fn no_subscriber_queue_drains_after_subscriber_registers() -> TestResult {
    let policy = result_context(EventQueuePolicy::no_subscriber_queue(2), "queue policy is valid")?;
    let bus = EventBus::with_queue_policy(policy);
    let queued_report = result_context(
        bus.publish(test_event(TEST_LABEL), metadata(TEST_TARGET)).await,
        "no-subscriber publish queues",
    )?;
    let handled = Arc::new(Mutex::new(Vec::new()));
    let handled_clone = Arc::clone(&handled);
    let subscription = result_context(
        bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), move |context| {
            let handled = Arc::clone(&handled_clone);
            async move {
                handled.lock().await.push(context.payload().label.clone());
                Ok(())
            }
        })
        .await,
        "subscriber registers and drains queue",
    )?;
    let empty_drain = result_context(
        bus.drain_queued(DispatchMode::Sequential).await,
        "queue is already drained",
    )?;

    assert_eq!(
        queued_report.queue_report.disposition,
        QueueDisposition::QueuedNoSubscriber
    );
    assert_eq!(queued_report.queue_report.queued_count, 1);
    assert_eq!(queued_report.subscriber_count, 0);
    assert_eq!(bus.journal().await.len(), 1);
    assert_eq!(
        subscription.event_type.as_str(),
        super::fixtures::TEST_EVENT_TYPE
    );
    assert_eq!(empty_drain.queued_before, 0);
    assert_eq!(handled.lock().await.as_slice(), &[TEST_LABEL.to_string()]);
    Ok(())
}

#[tokio::test]
async fn subscriber_auto_drain_only_drains_matching_event_type() -> TestResult {
    let policy = result_context(EventQueuePolicy::no_subscriber_queue(4), "queue policy is valid")?;
    let bus = EventBus::with_queue_policy(policy);
    result_context(
        bus.publish(
            test_event_with_idempotency("primary queued", "queue-scope-primary-key"),
            metadata_with_event_id(TEST_TARGET, "queue-scope-primary-event"),
        )
        .await,
        "primary event queues",
    )?;
    result_context(
        bus.publish(
            test_event_for_type_with_aggregate_and_idempotency(
                "other queued",
                "queue-scope-other-aggregate",
                OTHER_EVENT_TYPE,
                "queue-scope-other-key",
            ),
            metadata_with_event_id(OTHER_TARGET, "queue-scope-other-event"),
        )
        .await,
        "other event queues",
    )?;

    let handled_other = Arc::new(Mutex::new(Vec::new()));
    let handled_other_clone = Arc::clone(&handled_other);
    let other_subscription = result_context(
        bus.subscribe::<TestEvent, _, _>(
            subscriber_for_event(OTHER_SUBSCRIBER, OTHER_TARGET, OTHER_EVENT_TYPE),
            move |context| {
                let handled = Arc::clone(&handled_other_clone);
                async move {
                    handled.lock().await.push(context.payload().label.clone());
                    Ok(())
                }
            },
        )
        .await,
        "other subscriber drains only matching queued event",
    )?;
    let metrics_after_other = bus.metrics_snapshot().await;

    assert_eq!(other_subscription.drain_report.queued_before, 1);
    assert_eq!(other_subscription.drain_report.dispatched_count, 1);
    assert_eq!(other_subscription.drain_report.remaining_count, 0);
    assert_eq!(
        handled_other.lock().await.as_slice(),
        &["other queued".to_string()]
    );
    assert_eq!(metrics_after_other.queue.queued_event_count, 1);

    let handled_primary = Arc::new(Mutex::new(Vec::new()));
    let handled_primary_clone = Arc::clone(&handled_primary);
    result_context(
        bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), move |context| {
            let handled = Arc::clone(&handled_primary_clone);
            async move {
                handled.lock().await.push(context.payload().label.clone());
                Ok(())
            }
        })
        .await,
        "primary subscriber drains remaining primary queued event",
    )?;

    assert_eq!(
        handled_primary.lock().await.as_slice(),
        &["primary queued".to_string()]
    );
    assert_eq!(bus.metrics_snapshot().await.queue.queued_event_count, 0);
    Ok(())
}

#[tokio::test]
async fn bounded_queue_overflow_dead_letters_oldest_event_and_keeps_newest() -> TestResult {
    let policy = result_context(EventQueuePolicy::no_subscriber_queue(1), "queue policy is valid")?;
    let bus = EventBus::with_queue_policy(policy);
    result_context(
        bus.publish(
            test_event_with_idempotency("first", "queue-overflow-first"),
            metadata_with_event_id(TEST_TARGET, "queue-overflow-event-1"),
        )
        .await,
        "first event queues",
    )?;
    let report = result_context(
        bus.publish(
            test_event_with_idempotency("second", "queue-overflow-second"),
            metadata_with_event_id(TEST_TARGET, "queue-overflow-event-2"),
        )
        .await,
        "overflow drops oldest and queues newest",
    )?;
    let dead_letters = bus.dead_letters().await;
    let dead_letter_event = dead_letters[0].as_event();
    let expected_dead_letter_type =
        result_context(crate::bus::reports::dead_letter_recorded_event_type(), "dead-letter event type parses")?;

    assert_eq!(
        report.queue_report.disposition,
        QueueDisposition::DeadLetteredQueueOverflow
    );
    assert_eq!(report.dead_letter_count, 1);
    assert_eq!(report.queue_report.queued_count, 1);
    assert_eq!(dead_letters.len(), 1);
    assert_eq!(dead_letters[0].reason, DeadLetterReason::QueueOverflow);
    assert_eq!(
        dead_letters[0].envelope.event_id.as_str(),
        "queue-overflow-event-1"
    );
    assert!(dead_letters[0].subscriber_id.is_none());
    assert!(dead_letters[0].target_handler.is_none());
    assert_eq!(dead_letter_event.reason, DeadLetterReason::QueueOverflow);
    let dead_letter_contract_event_type = match dead_letter_event.contract() {
        Ok(contract) => contract.event_type,
        Err(err) => {
            return Err(std::io::Error::other(format!(
                "dead-letter event contract exists: {err}"
            ))
            .into());
        }
    };
    assert_eq!(dead_letter_contract_event_type, expected_dead_letter_type);

    let handled = Arc::new(Mutex::new(Vec::new()));
    let handled_clone = Arc::clone(&handled);
    result_context(
        bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), move |context| {
            let handled = Arc::clone(&handled_clone);
            async move {
                handled.lock().await.push(context.payload().label.clone());
                Ok(())
            }
        })
        .await,
        "subscriber drains newest queued event",
    )?;
    assert_eq!(handled.lock().await.as_slice(), &["second".to_string()]);
    Ok(())
}

#[tokio::test]
async fn queued_event_expires_before_dispatch_when_ttl_elapsed() -> TestResult {
    let policy = result_context(EventQueuePolicy::no_subscriber_queue(2), "queue policy is valid")?;
    let policy = result_context(policy.with_ttl(Duration::from_millis(5)), "ttl policy is valid")?;
    let bus = EventBus::with_queue_policy(policy);
    result_context(
        bus.publish(test_event(TEST_LABEL), metadata(TEST_TARGET)).await,
        "event queues",
    )?;
    tokio::time::sleep(Duration::from_millis(20)).await;
    result_context(
        bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), |_| async {
            Ok(())
        })
        .await,
        "subscriber registration drains expired queue",
    )?;
    let drain = result_context(bus.drain_queued(DispatchMode::Sequential).await, "queue stays empty")?;
    let dead_letters = bus.dead_letters().await;

    assert_eq!(drain.queued_before, 0);
    assert_eq!(drain.dispatched_count, 0);
    assert_eq!(drain.remaining_count, 0);
    assert_eq!(dead_letters[0].reason, DeadLetterReason::QueueExpired);
    Ok(())
}

#[tokio::test]
async fn idempotency_registry_rejects_queued_and_completed_duplicates() -> TestResult {
    let policy = result_context(EventQueuePolicy::no_subscriber_queue(2), "queue policy is valid")?
        .with_idempotency_registry();
    let bus = EventBus::with_queue_policy(policy);
    result_context(
        bus.publish(
            test_event_with_idempotency(TEST_LABEL, "idempotency-queue-key"),
            metadata_with_event_id(TEST_TARGET, "idempotency-queued-event-1"),
        )
        .await,
        "first event queues",
    )?;

    let queued_duplicate = bus
        .publish(
            test_event_with_idempotency("duplicate", "idempotency-queue-key"),
            metadata_with_event_id(TEST_TARGET, "idempotency-queued-event-2"),
        )
        .await;
    assert!(matches!(
        queued_duplicate,
        Err(EventingError::DuplicateIdempotencyKey { .. })
    ));

    result_context(
        bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), |_| async {
            Ok(())
        })
        .await,
        "subscriber drains queued event",
    )?;

    let completed_duplicate = bus
        .publish(
            test_event_with_idempotency("completed", "idempotency-queue-key"),
            metadata_with_event_id(TEST_TARGET, "idempotency-completed-event"),
        )
        .await;
    assert!(matches!(
        completed_duplicate,
        Err(EventingError::DuplicateIdempotencyKey { .. })
    ));
    Ok(())
}

#[tokio::test]
async fn in_flight_duplicate_guard_rejects_concurrent_event_id() -> TestResult {
    let bus = EventBus::new();
    let started = Arc::new(Notify::new());
    let started_clone = Arc::clone(&started);
    result_context(
        bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), move |_| {
            let started = Arc::clone(&started_clone);
            async move {
                started.notify_waiters();
                tokio::time::sleep(Duration::from_millis(50)).await;
                Ok(())
            }
        })
        .await,
        "subscriber registers",
    )?;

    let first_bus = bus.clone();
    let first = tokio::spawn(async move {
        first_bus
            .publish(
                test_event_with_idempotency(TEST_LABEL, "in-flight-idempotency-key-1"),
                metadata(TEST_TARGET),
            )
            .await
    });
    started.notified().await;
    let duplicate = bus
        .publish(
            test_event_with_idempotency("duplicate", "in-flight-idempotency-key-2"),
            metadata(TEST_TARGET),
        )
        .await;
    let first_report = result_context(first.await, "first task joins")?;
    let first_report = result_context(first_report, "first publish completes")?;

    assert!(matches!(
        duplicate,
        Err(EventingError::DuplicateEventId { .. })
    ));
    assert_eq!(first_report.handled_count, 1);
    Ok(())
}

#[tokio::test]
async fn failed_subscribe_drain_preserves_queued_event_for_retry() -> TestResult {
    let policy = result_context(EventQueuePolicy::no_subscriber_queue(2), "queue policy is valid")?;
    let journal = Arc::new(FailingJournal::fail_once_on(1));
    let bus = EventBus::with_journal_and_queue_policy(
        JournalPolicy::before_and_after_dispatch(JournalSelector::All),
        journal,
        policy,
    );
    result_context(
        bus.publish(
            test_event_with_idempotency(TEST_LABEL, "drain-preserve-idempotency"),
            metadata_with_event_id(TEST_TARGET, "drain-preserve-event-1"),
        )
        .await,
        "event queues",
    )?;

    let failed_subscribe = bus
        .subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), |_| async {
            Ok(())
        })
        .await;
    assert!(matches!(
        failed_subscribe,
        Err(EventingError::JournalIo { .. })
    ));

    let handled = Arc::new(Mutex::new(Vec::new()));
    let handled_clone = Arc::clone(&handled);
    result_context(
        bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), move |context| {
            let handled = Arc::clone(&handled_clone);
            async move {
                handled.lock().await.push(context.payload().label.clone());
                Ok(())
            }
        })
        .await,
        "retry subscriber drains preserved event",
    )?;

    assert_eq!(handled.lock().await.as_slice(), &[TEST_LABEL.to_string()]);
    Ok(())
}

#[tokio::test]
async fn after_dispatch_journal_failure_does_not_replay_handler_work() -> TestResult {
    let policy = result_context(EventQueuePolicy::no_subscriber_queue(2), "queue policy is valid")?;
    let journal = Arc::new(FailingJournal::fail_once_on(2));
    let bus = EventBus::with_journal_and_queue_policy(
        JournalPolicy::before_and_after_dispatch(JournalSelector::All),
        journal,
        policy,
    );
    result_context(
        bus.publish(
            test_event_with_idempotency(TEST_LABEL, "drain-after-dispatch-key"),
            metadata_with_event_id(TEST_TARGET, "drain-after-dispatch-event"),
        )
        .await,
        "event queues",
    )?;

    let handled = Arc::new(Mutex::new(Vec::new()));
    let handled_clone = Arc::clone(&handled);
    let failed_subscribe = bus
        .subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), move |context| {
            let handled = Arc::clone(&handled_clone);
            async move {
                handled.lock().await.push(context.payload().label.clone());
                Ok(())
            }
        })
        .await;
    assert!(matches!(
        failed_subscribe,
        Err(EventingError::JournalIo { .. })
    ));

    let retry_handled = Arc::new(Mutex::new(Vec::new()));
    let retry_handled_clone = Arc::clone(&retry_handled);
    let retry_subscription = result_context(
        bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), move |context| {
            let handled = Arc::clone(&retry_handled_clone);
            async move {
                handled.lock().await.push(context.payload().label.clone());
                Ok(())
            }
        })
        .await,
        "retry subscriber registers without replaying completed work",
    )?;

    assert_eq!(handled.lock().await.as_slice(), &[TEST_LABEL.to_string()]);
    assert!(retry_handled.lock().await.is_empty());
    assert_eq!(retry_subscription.drain_report.queued_before, 0);
    assert_eq!(retry_subscription.drain_report.dispatched_count, 0);
    Ok(())
}

struct FailingJournal {
    calls: StdMutex<usize>,
    fail_once_on: usize,
}

impl FailingJournal {
    fn fail_once_on(call: usize) -> Self {
        Self {
            calls: StdMutex::new(0),
            fail_once_on: call,
        }
    }
}

impl EventJournal for FailingJournal {
    fn append<'a>(
        &'a self,
        _envelope: &'a StoredEventEnvelope,
    ) -> Pin<Box<dyn Future<Output = Result<JournalAppend, EventingError>> + Send + 'a>> {
        Box::pin(async move {
            let call = {
                let mut calls = match self.calls.lock() {
                    Ok(calls) => calls,
                    Err(err) => {
                        return Err(EventingError::journal_io(
                            String::from("failing-journal"),
                            &err,
                        ));
                    }
                };
                *calls += 1;
                *calls
            };
            if call == self.fail_once_on {
                return Err(EventingError::JournalIo {
                    path: String::from("failing-journal"),
                    reason: String::from("intentional one-shot append failure"),
                });
            }
            Ok(JournalAppend {
                sequence: call as u64,
                previous_hash: None,
                current_hash: Some(crate::ids::JournalHash::parse(format!("journal-hash-{call}"))?),
            })
        })
    }
}
