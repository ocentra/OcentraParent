use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};
use std::error::Error;

use super::fixtures::{
    metadata, subscriber, test_event, TestEvent, TEST_LABEL, TEST_SUBSCRIBER, TEST_TARGET,
};
use crate::bus::reports::HandlerOutcome;
use crate::bus::EventBus;
use crate::error::EventingError;
use crate::execution::HandlerExecutionPolicy;
use crate::testkit::EventRecorder;

#[tokio::test]
async fn retry_policy_retries_failed_attempt_and_reports_trace_fields()
    -> Result<(), Box<dyn Error>>
{
    let bus = EventBus::with_handler_policy(
        HandlerExecutionPolicy::new(None, 2)?,
    );
    let attempts = Arc::new(AtomicUsize::new(0));
    let attempts_clone = Arc::clone(&attempts);
    bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), move |_| {
        let attempts = Arc::clone(&attempts_clone);
        async move {
            let previous = attempts.fetch_add(1, Ordering::SeqCst);
            if previous == 0 {
                Err(EventingError::empty_value("retryable_handler_failure"))
            } else {
                Ok(())
            }
        }
    })
    .await
    ?;

    let report = bus
        .publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .await
        ?;
    let handler_report = &report.handler_reports[0];

    assert_eq!(handler_report.outcome, HandlerOutcome::Handled);
    assert_eq!(handler_report.attempts, 2);
    assert_eq!(handler_report.trace.event_id, report.event_id);
    assert_eq!(handler_report.trace.event_type, report.event_type);
    assert_eq!(
        handler_report.trace.correlation_id,
        metadata(TEST_TARGET).correlation_id
    );
    assert_eq!(handler_report.trace.target_handler.as_str(), TEST_TARGET);
    assert_eq!(attempts.load(Ordering::SeqCst), 2);
    Ok(())
}

#[tokio::test]
async fn timeout_policy_retries_then_dead_letters_final_timeout() -> Result<(), Box<dyn Error>> {
    let bus = EventBus::with_handler_policy(
        HandlerExecutionPolicy::new(Some(Duration::from_millis(5)), 2)?,
    );
    let attempts = Arc::new(AtomicUsize::new(0));
    let attempts_clone = Arc::clone(&attempts);
    bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), move |_| {
        let attempts = Arc::clone(&attempts_clone);
        async move {
            attempts.fetch_add(1, Ordering::SeqCst);
            tokio::time::sleep(Duration::from_millis(50)).await;
            Ok(())
        }
    })
    .await
    ?;

    let report = bus
        .publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .await
        ?;
    let dead_letters = bus.dead_letters().await;

    assert_eq!(report.handler_reports[0].outcome, HandlerOutcome::TimedOut);
    assert_eq!(report.handler_reports[0].attempts, 2);
    assert_eq!(report.dead_letter_count, 1);
    assert_eq!(
        match dead_letters[0].subscriber_id.as_ref() {
            Some(subscriber_id) => subscriber_id.as_str(),
            None => {
                return Err(std::io::Error::other("subscriber id missing").into());
            }
        },
        TEST_SUBSCRIBER
    );
    assert_eq!(attempts.load(Ordering::SeqCst), 2);
    Ok(())
}

#[tokio::test]
async fn event_recorder_uses_real_subscription_and_can_unsubscribe()
    -> Result<(), Box<dyn Error>>
{
    let bus = EventBus::new();
    let recorder =
        EventRecorder::<TestEvent>::attach(&bus, subscriber(TEST_SUBSCRIBER, TEST_TARGET))
            .await
            ?;

    let first_report = bus
        .publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .await
        ?;
    let recorded = recorder.recorded().await;
    assert!(recorder.unsubscribe());
    let second_report = bus
        .publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .await
        ?;

    assert_eq!(first_report.handled_count, 1);
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].payload.label, TEST_LABEL);
    assert_eq!(second_report.subscriber_count, 0);
    Ok(())
}
