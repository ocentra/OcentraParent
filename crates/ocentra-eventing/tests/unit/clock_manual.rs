use crate::ExpectValue;
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use tokio::{sync::Notify, task::yield_now};

use crate::{
    AggregateKey, DispatchMode, DomainEvent, EventBus, EventClock, EventContract, EventQueuePolicy,
    EventResponseContract, EventType, EventingError, HandlerExecutionPolicy, IdempotencyKey,
    ManualEventClock, RequestCompletionOutcome, RequestEvent, RequestId, RequestOptions,
    SchemaVersion,
};
use ocentra_eventing::bus::reports::dead_letter::DeadLetterReason;
use ocentra_eventing::bus::reports::handler::HandlerOutcome;

use super::fixtures::{
    metadata, subscriber, subscriber_for_event, test_event, TestText, TEST_LABEL, TEST_TARGET,
};

const CLOCK_REQUEST_EVENT_TYPE: &str = "eventing.clock.request";
const CLOCK_REQUEST_ID: &str = "eventing-clock-request";
const CLOCK_REQUEST_AGGREGATE: &str = "eventing-clock-aggregate";
const CLOCK_REQUEST_IDEMPOTENCY: &str = "eventing-clock-idempotency";

#[tokio::test]
async fn manual_clock_advances_registered_sleepers_without_wall_clock_sleep() {
    let clock = ManualEventClock::new();
    let sleeper_clock = clock.clone();
    let completed = Arc::new(AtomicUsize::new(0));
    let completed_clone = Arc::clone(&completed);
    let sleeper = tokio::spawn(async move {
        sleeper_clock.sleep(Duration::from_millis(10)).await;
        completed_clone.fetch_add(1, Ordering::SeqCst);
    });

    yield_until(|| clock.pending_sleep_count() == 1).await;
    clock.advance(Duration::from_millis(9));
    yield_now().await;
    assert_eq!(completed.load(Ordering::SeqCst), 0);

    clock.advance(Duration::from_millis(1));
    sleeper.await.expect_value("manual sleeper joins");
    assert_eq!(completed.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn manual_clock_expires_queued_ttl_without_wall_clock_sleep() {
    let clock = ManualEventClock::new();
    let policy = EventQueuePolicy::no_subscriber_queue(2)
        .expect_value("queue policy is valid")
        .with_ttl(Duration::from_millis(10))
        .expect_value("ttl policy is valid");
    let bus = EventBus::with_queue_policy_and_clock(policy, clock.shared());

    bus.publish(
        test_event(TestText(TEST_LABEL.to_owned())),
        metadata(TestText(TEST_TARGET.to_owned())),
    )
    .await
    .expect_value("event queues");
    clock.advance(Duration::from_millis(11));
    bus.subscribe::<super::fixtures::TestEvent, _, _>(
        subscriber(
            TestText("manual-clock-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        |_| async { Ok(()) },
    )
    .await
    .expect_value("subscriber registers");
    let drain = bus
        .drain_queued(DispatchMode::Sequential)
        .await
        .expect_value("queue is already drained");

    assert_eq!(drain.expired_count, 0);
    assert_eq!(drain.dispatched_count, 0);
    assert_eq!(bus.dead_letters().await.len(), 1);
}

#[tokio::test]
async fn manual_clock_dead_letters_past_deadline_without_dispatch() {
    let clock = ManualEventClock::new();
    let bus = EventBus::with_clock(clock.shared());
    let attempts = Arc::new(AtomicUsize::new(0));
    let handler_attempts = Arc::clone(&attempts);
    bus.subscribe::<super::fixtures::TestEvent, _, _>(
        subscriber(
            TestText("manual-deadline-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |_| {
            let handler_attempts = Arc::clone(&handler_attempts);
            async move {
                handler_attempts.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }
        },
    )
    .await
    .expect_value("subscriber registers");
    let deadline = clock
        .now()
        .checked_add(Duration::from_millis(5))
        .expect_value("deadline fits manual clock");
    clock.advance(Duration::from_millis(6));

    let report = bus
        .publish(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())).with_deadline(deadline),
        )
        .await
        .expect_value("deadline publish reports");
    let dead_letters = bus.dead_letters().await;

    assert_eq!(attempts.load(Ordering::SeqCst), 0);
    assert_eq!(report.subscriber_count, 0);
    assert_eq!(report.dead_letter_count, 1);
    assert_eq!(dead_letters[0].reason, DeadLetterReason::DeadlineExpired);
}

#[tokio::test]
async fn manual_clock_drives_handler_timeout_retries_without_wall_clock_sleep() {
    let clock = ManualEventClock::new();
    let bus = EventBus::with_handler_policy_and_clock(
        HandlerExecutionPolicy::new(Some(Duration::from_millis(5)), 2)
            .expect_value("handler policy is valid"),
        clock.shared(),
    );
    let attempts = Arc::new(AtomicUsize::new(0));
    let handler_clock = clock.clone();
    let handler_attempts = Arc::clone(&attempts);
    bus.subscribe::<super::fixtures::TestEvent, _, _>(
        subscriber(
            TestText("manual-timeout-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |_| {
            let handler_clock = handler_clock.clone();
            let handler_attempts = Arc::clone(&handler_attempts);
            async move {
                handler_attempts.fetch_add(1, Ordering::SeqCst);
                handler_clock.sleep(Duration::from_millis(50)).await;
                Ok(())
            }
        },
    )
    .await
    .expect_value("subscriber registers");
    let publish_bus = bus.clone();
    let publish = tokio::spawn(async move {
        publish_bus
            .publish(
                test_event(TestText(TEST_LABEL.to_owned())),
                metadata(TestText(TEST_TARGET.to_owned())),
            )
            .await
    });

    yield_until(|| attempts.load(Ordering::SeqCst) == 1 && clock.pending_sleep_count() >= 2).await;
    clock.advance(Duration::from_millis(5));
    yield_until(|| attempts.load(Ordering::SeqCst) == 2 && clock.pending_sleep_count() >= 3).await;
    clock.advance(Duration::from_millis(5));

    let report = publish
        .await
        .expect_value("publish joins")
        .expect_value("publish reports timeout");
    assert_eq!(attempts.load(Ordering::SeqCst), 2);
    assert_eq!(report.handler_reports[0].attempts, 2);
    assert_eq!(report.dead_letter_count, 1);
}

#[tokio::test]
async fn manual_clock_stops_retry_when_deadline_expires_between_attempts() {
    let clock = ManualEventClock::new();
    let bus = EventBus::with_handler_policy_and_clock(
        HandlerExecutionPolicy::new(None, 3).expect_value("handler policy is valid"),
        clock.shared(),
    );
    let attempts = Arc::new(AtomicUsize::new(0));
    let handler_clock = clock.clone();
    let handler_attempts = Arc::clone(&attempts);
    bus.subscribe::<super::fixtures::TestEvent, _, _>(
        subscriber(
            TestText("manual-retry-deadline-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |_| {
            let handler_clock = handler_clock.clone();
            let handler_attempts = Arc::clone(&handler_attempts);
            async move {
                handler_attempts.fetch_add(1, Ordering::SeqCst);
                handler_clock.sleep(Duration::from_millis(1)).await;
                Err(EventingError::EmptyValue {
                    field: "manual_clock_deadline_retry",
                })
            }
        },
    )
    .await
    .expect_value("subscriber registers");
    let deadline = clock
        .now()
        .checked_add(Duration::from_millis(1))
        .expect_value("deadline fits manual clock");
    let publish_bus = bus.clone();
    let publish = tokio::spawn(async move {
        publish_bus
            .publish(
                test_event(TestText(TEST_LABEL.to_owned())),
                metadata(TestText(TEST_TARGET.to_owned())).with_deadline(deadline),
            )
            .await
    });

    yield_until(|| clock.pending_sleep_count() >= 1).await;
    clock.advance(Duration::from_millis(1));

    let report = publish
        .await
        .expect_value("publish joins")
        .expect_value("publish reports deadline");
    assert_eq!(attempts.load(Ordering::SeqCst), 1);
    assert_eq!(
        report.handler_reports[0].outcome,
        HandlerOutcome::DeadlineExpired
    );
    assert_eq!(report.handler_reports[0].attempts, 1);
    assert_eq!(report.dead_letter_count, 1);
}

#[tokio::test]
async fn manual_clock_drives_request_timeout_and_late_completion_without_wall_clock_sleep() {
    let clock = ManualEventClock::new();
    let bus = EventBus::with_clock(clock.shared());
    let outcomes = Arc::new(Mutex::new(Vec::new()));
    let late_sleep_registered = Arc::new(Notify::new());
    let handler_clock = clock.clone();
    let handler_outcomes = Arc::clone(&outcomes);
    let handler_late_sleep_registered = Arc::clone(&late_sleep_registered);
    bus.subscribe::<ClockRequestEvent, _, _>(
        subscriber_for_event(
            TestText("manual-request-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(CLOCK_REQUEST_EVENT_TYPE.to_owned()),
        ),
        move |context| {
            let handler_clock = handler_clock.clone();
            let handler_outcomes = Arc::clone(&handler_outcomes);
            let handler_late_sleep_registered = Arc::clone(&handler_late_sleep_registered);
            async move {
                tokio::spawn(async move {
                    let sleep = handler_clock.sleep(Duration::from_millis(20));
                    handler_late_sleep_registered.notify_one();
                    sleep.await;
                    let completion = context
                        .complete_request(ClockResponse::approved())
                        .await
                        .expect_value("late completion reports");
                    handler_outcomes
                        .lock()
                        .expect_value("outcomes lock")
                        .push(completion.outcome);
                });
                Ok(())
            }
        },
    )
    .await
    .expect_value("request subscriber registers");
    let request_bus = bus.clone();
    let request = tokio::spawn(async move {
        request_bus
            .publish_request(
                ClockRequestEvent::new(),
                metadata(TestText(TEST_TARGET.to_owned())),
                RequestOptions::with_timeout(Duration::from_millis(5))
                    .expect_value("request timeout is valid"),
            )
            .await
    });

    late_sleep_registered.notified().await;
    yield_until(|| clock.pending_sleep_count() >= 2).await;
    clock.advance(Duration::from_millis(5));
    let result = request.await.expect_value("request joins");
    assert!(matches!(result, Err(EventingError::RequestTimedOut { .. })));

    clock.advance(Duration::from_millis(20));
    yield_until(|| !outcomes.lock().expect_value("outcomes lock").is_empty()).await;
    assert_eq!(
        outcomes.lock().expect_value("outcomes lock").as_slice(),
        &[RequestCompletionOutcome::Late]
    );
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct ClockRequestEvent {
    request_id: RequestId,
}

impl ClockRequestEvent {
    fn new() -> Self {
        Self {
            request_id: RequestId::parse(CLOCK_REQUEST_ID).expect_value("request id parses"),
        }
    }
}

impl DomainEvent for ClockRequestEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(CLOCK_REQUEST_EVENT_TYPE).expect_value("event type parses"),
            SchemaVersion::new(1)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(CLOCK_REQUEST_AGGREGATE)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(CLOCK_REQUEST_IDEMPOTENCY)
    }
}

impl RequestEvent for ClockRequestEvent {
    type Response = ClockResponse;

    fn request_id(&self) -> Result<RequestId, EventingError> {
        Ok(self.request_id.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct ClockResponse {
    decision: String,
}

impl ClockResponse {
    fn approved() -> Self {
        Self {
            decision: "approved".to_string(),
        }
    }
}

impl EventResponseContract for ClockResponse {}

async fn yield_until(condition: impl Fn() -> bool) {
    for _ in 0..50 {
        if condition() {
            return;
        }
        yield_now().await;
    }
    std::process::abort();
}
