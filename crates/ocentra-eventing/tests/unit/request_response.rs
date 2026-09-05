use ocentra_eventing::expect_value::{ExpectErrValue, ExpectValue};
use std::{future, sync::Arc, time::Duration};

use tokio::sync::{Mutex, Notify};

use super::{
    fixtures::{metadata, metadata_with_event_id, subscriber_for_event, TestText, TEST_TARGET},
    request_response_support::{
        test_request, test_request_with_id, test_result_event, InvalidContractRequestEvent,
        TestRequestEvent, TestResponse, TestResultEvent, TestText as RequestText,
        REQUEST_EVENT_TYPE, REQUEST_ID, RESULT_EVENT_TYPE,
    },
};
use crate::{EventRecorder, EventingError, RequestCompletionOutcome, RequestId, RequestOptions};
use ocentra_eventing::request::RequestCompletionReport;

#[test]
fn request_options_reject_zero_timeout() {
    let result = RequestOptions::with_timeout(Duration::ZERO);

    assert!(matches!(
        result,
        Err(EventingError::InvalidRequestOptions { reason })
            if reason == "request timeout must be greater than zero"
    ));
}

#[test]
fn cancelled_completion_report_uses_canonical_serde_keys() {
    let report = RequestCompletionReport {
        request_id: RequestId::parse("request-cancelled-serde").expect_value("request id parses"),
        outcome: RequestCompletionOutcome::Cancelled,
    };
    let report_json = serde_json::to_value(report).expect_value("request report serializes");

    assert_eq!(
        report_json["requestId"],
        serde_json::json!("request-cancelled-serde")
    );
    assert_eq!(report_json["outcome"], serde_json::json!("cancelled"));
}

#[tokio::test]
async fn manual_clock_expires_request_at_exact_timeout_boundary() {
    let clock = crate::ManualEventClock::new();
    let bus = crate::EventBus::with_clock(clock.shared());
    let handler_started = Arc::new(Notify::new());
    let handler_started_for_subscriber = Arc::clone(&handler_started);

    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event(
            TestText("request-expiry-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
        ),
        move |_context| {
            let handler_started = Arc::clone(&handler_started_for_subscriber);
            async move {
                handler_started.notify_one();
                future::pending::<Result<(), EventingError>>().await
            }
        },
    )
    .await
    .expect_value("subscribe request handler");

    let request_id = "request-response-expiry-boundary";
    let request_bus = bus.clone();
    let request = tokio::spawn(async move {
        request_bus
            .publish_request(
                test_request_with_id(
                    RequestText("expiry-boundary".to_owned()),
                    RequestText(request_id.to_owned()),
                ),
                metadata_with_event_id(
                    TestText(TEST_TARGET.to_owned()),
                    TestText("event-expiry-boundary".to_owned()),
                ),
                RequestOptions::with_timeout(Duration::from_millis(5))
                    .expect_value("request options"),
            )
            .await
    });

    handler_started.notified().await;
    clock.advance(Duration::from_millis(5));

    let result = request.await.expect_value("request task");
    assert!(matches!(
        result,
        Err(EventingError::RequestTimedOut { request_id: actual })
            if actual.as_str() == request_id
    ));

    let metrics = bus.metrics_snapshot().await;
    assert_eq!(metrics.requests.pending_request_count, 0);
    assert_eq!(metrics.requests.timed_out_request_count, 1);
}

#[tokio::test]
async fn dropping_request_future_cancels_pending_completion_and_publish() {
    let bus = crate::EventBus::root();
    let handler_started = Arc::new(Notify::new());
    let handler_started_for_subscriber = Arc::clone(&handler_started);

    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event(
            TestText("request-cancellation-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
        ),
        move |_context| {
            let handler_started = Arc::clone(&handler_started_for_subscriber);
            async move {
                handler_started.notify_one();
                future::pending::<Result<(), EventingError>>().await
            }
        },
    )
    .await
    .expect_value("subscribe request handler");

    let request_bus = bus.clone();
    let request = tokio::spawn(async move {
        request_bus
            .publish_request(
                test_request_with_id(
                    RequestText("request-response-cancelled".to_owned()),
                    RequestText("request-response-cancelled".to_owned()),
                ),
                metadata_with_event_id(
                    TestText(TEST_TARGET.to_owned()),
                    TestText("event-cancelled".to_owned()),
                ),
                RequestOptions::with_timeout(Duration::from_secs(30))
                    .expect_value("request options"),
            )
            .await
    });

    handler_started.notified().await;
    request.abort();
    let join_error = request.await.expect_err_value("aborted request task");
    assert!(join_error.is_cancelled());

    tokio::task::yield_now().await;
    let metrics = bus.metrics_snapshot().await;
    assert_eq!(metrics.requests.pending_request_count, 0);
    assert_eq!(metrics.queue.in_flight_event_id_count, 0);
    let cancellation_reports = bus.take_request_cancellation_reports();
    assert_eq!(cancellation_reports.len(), 1);
    assert_eq!(
        cancellation_reports[0],
        RequestCompletionReport {
            request_id: RequestId::parse("request-response-cancelled")
                .expect_value("request id parses"),
            outcome: RequestCompletionOutcome::Cancelled,
        }
    );
}

#[tokio::test]
async fn dropping_causal_request_future_records_cancellation_report() {
    let parent_bus = crate::EventBus::root();
    let target_bus = crate::EventBus::root();
    let target_bus_for_parent = target_bus.event_bus().clone();
    let nested_started = Arc::new(Notify::new());
    let nested_started_for_handler = Arc::clone(&nested_started);

    target_bus
        .subscribe::<TestRequestEvent, _, _>(
            subscriber_for_event(
                TestText("causal-cancellation-target".to_owned()),
                TestText(TEST_TARGET.to_owned()),
                TestText(REQUEST_EVENT_TYPE.to_owned()),
            ),
            move |_context| {
                let nested_started = Arc::clone(&nested_started_for_handler);
                async move {
                    nested_started.notify_one();
                    future::pending::<Result<(), EventingError>>().await
                }
            },
        )
        .await
        .expect_value("causal request target subscribes");

    parent_bus
        .subscribe::<TestRequestEvent, _, _>(
            subscriber_for_event(
                TestText("causal-cancellation-parent".to_owned()),
                TestText(TEST_TARGET.to_owned()),
                TestText(REQUEST_EVENT_TYPE.to_owned()),
            ),
            move |context| {
                let target_bus = target_bus_for_parent.clone();
                async move {
                    let _ = context
                        .publisher()
                        .publish_request_on(
                            &target_bus,
                            test_request_with_id(
                                RequestText("causal-cancellation-nested".to_owned()),
                                RequestText("causal-cancellation-nested".to_owned()),
                            ),
                            metadata(TestText(TEST_TARGET.to_owned())),
                            RequestOptions::with_timeout(Duration::from_secs(30))
                                .expect_value("causal request options"),
                        )
                        .await;
                    Ok(())
                }
            },
        )
        .await
        .expect_value("causal request parent subscribes");

    let request_bus = parent_bus.clone();
    let request = tokio::spawn(async move {
        request_bus
            .publish_request(
                test_request_with_id(
                    RequestText("causal-cancellation-parent-request".to_owned()),
                    RequestText("causal-cancellation-parent-request".to_owned()),
                ),
                metadata(TestText(TEST_TARGET.to_owned())),
                RequestOptions::with_timeout(Duration::from_secs(30))
                    .expect_value("parent request options"),
            )
            .await
    });

    nested_started.notified().await;
    request.abort();
    assert!(request
        .await
        .expect_err_value("aborted parent request")
        .is_cancelled());
    tokio::task::yield_now().await;

    assert_eq!(
        target_bus.take_request_cancellation_reports(),
        vec![RequestCompletionReport {
            request_id: RequestId::parse("causal-cancellation-nested")
                .expect_value("nested request id parses"),
            outcome: RequestCompletionOutcome::Cancelled,
        }]
    );
}

#[tokio::test]
async fn causal_request_observes_shutdown_while_publish_is_in_flight() {
    let parent_bus = crate::EventBus::root();
    let target_root = crate::EventBus::root();
    let target_bus = target_root.event_bus().clone();
    let target_handler_started = Arc::new(Notify::new());
    let target_handler_started_for_subscriber = Arc::clone(&target_handler_started);

    target_root
        .subscribe::<TestRequestEvent, _, _>(
            subscriber_for_event(
                TestText("causal-shutdown-target".to_owned()),
                TestText(TEST_TARGET.to_owned()),
                TestText(REQUEST_EVENT_TYPE.to_owned()),
            ),
            move |_context| {
                let target_handler_started = Arc::clone(&target_handler_started_for_subscriber);
                async move {
                    target_handler_started.notify_one();
                    future::pending::<Result<(), EventingError>>().await
                }
            },
        )
        .await
        .expect_value("causal shutdown target subscribes");

    let observed_error = Arc::new(Mutex::new(None));
    let observed_error_for_handler = Arc::clone(&observed_error);
    let target_bus_for_parent = target_bus.clone();
    parent_bus
        .subscribe::<TestRequestEvent, _, _>(
            subscriber_for_event(
                TestText("causal-shutdown-parent".to_owned()),
                TestText(TEST_TARGET.to_owned()),
                TestText(REQUEST_EVENT_TYPE.to_owned()),
            ),
            move |context| {
                let target_bus = target_bus_for_parent.clone();
                let observed_error = Arc::clone(&observed_error_for_handler);
                async move {
                    let nested = context
                        .publisher()
                        .publish_request_on(
                            &target_bus,
                            test_request_with_id(
                                RequestText("causal-shutdown-nested".to_owned()),
                                RequestText("causal-shutdown-nested".to_owned()),
                            ),
                            metadata(TestText(TEST_TARGET.to_owned())),
                            RequestOptions::with_timeout(Duration::from_secs(30))
                                .expect_value("causal shutdown request options"),
                        )
                        .await;
                    let error = nested.expect_err_value("target shutdown cancels causal request");
                    *observed_error.lock().await = Some(error);
                    Ok(())
                }
            },
        )
        .await
        .expect_value("causal shutdown parent subscribes");

    let parent_bus_for_publication = parent_bus.clone();
    let publication = tokio::spawn(async move {
        parent_bus_for_publication
            .publish(
                test_request_with_id(
                    RequestText("causal-shutdown-parent-event".to_owned()),
                    RequestText("causal-shutdown-parent-event".to_owned()),
                ),
                metadata_with_event_id(
                    TestText(TEST_TARGET.to_owned()),
                    TestText("causal-shutdown-parent-event-id".to_owned()),
                ),
            )
            .await
    });

    target_handler_started.notified().await;
    let shutdown_report = target_root
        .shutdown(crate::ShutdownMode::Drain)
        .await
        .expect_value("target shutdown cancels in-flight causal request");
    publication
        .await
        .expect_value("parent publication task joins")
        .expect_value("parent publication completes after causal cancellation");

    let error = observed_error
        .lock()
        .await
        .take()
        .expect_value("causal cancellation error is observed");
    assert!(matches!(
        error,
        EventingError::RequestCancelled { request_id }
            if request_id.as_str() == "causal-shutdown-nested"
    ));
    assert_eq!(
        shutdown_report.cancelled_request_reports,
        vec![RequestCompletionReport {
            request_id: RequestId::parse("causal-shutdown-nested")
                .expect_value("nested request id parses"),
            outcome: RequestCompletionOutcome::Cancelled,
        }]
    );
}

const REQUEST_TERMINAL_RETENTION_PROBE_COUNT: usize = 4097;
const REQUEST_CANCELLATION_REPORT_RETENTION_PROBE_COUNT: usize = 4097;

#[tokio::test]
async fn caller_drop_cancellation_reports_retain_newest_bounded_window() {
    let bus = crate::EventBus::root();
    let handler_started = Arc::new(Notify::new());
    let handler_started_for_subscriber = Arc::clone(&handler_started);

    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event(
            TestText("request-cancellation-retention-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
        ),
        move |_context| {
            let handler_started = Arc::clone(&handler_started_for_subscriber);
            async move {
                handler_started.notify_one();
                future::pending::<Result<(), EventingError>>().await
            }
        },
    )
    .await
    .expect_value("request cancellation retention subscriber registers");

    for index in 0..REQUEST_CANCELLATION_REPORT_RETENTION_PROBE_COUNT {
        let request_id = format!("request-cancellation-retention-{index:04}");
        let event_id = format!("request-cancellation-retention-event-{index:04}");
        let request_bus = bus.clone();
        let request = tokio::spawn(async move {
            request_bus
                .publish_request(
                    test_request_with_id(RequestText(request_id.clone()), RequestText(request_id)),
                    metadata_with_event_id(TestText(TEST_TARGET.to_owned()), TestText(event_id)),
                    RequestOptions::with_timeout(Duration::from_secs(30))
                        .expect_value("request options"),
                )
                .await
        });

        handler_started.notified().await;
        request.abort();
        assert!(request
            .await
            .expect_err_value("aborted request task")
            .is_cancelled());
        tokio::task::yield_now().await;
    }

    let reports = bus.take_request_cancellation_reports();
    assert_eq!(
        reports.len(),
        REQUEST_CANCELLATION_REPORT_RETENTION_PROBE_COUNT - 1
    );
    assert_eq!(
        reports
            .first()
            .expect_value("oldest retained report")
            .request_id
            .as_str(),
        "request-cancellation-retention-0001"
    );
    assert_eq!(
        reports
            .last()
            .expect_value("newest retained report")
            .request_id
            .as_str(),
        "request-cancellation-retention-4096"
    );
    assert!(reports
        .iter()
        .all(|report| report.outcome == RequestCompletionOutcome::Cancelled));
}

#[tokio::test]
async fn publish_request_resolves_associated_response_type() {
    let bus = crate::EventBus::root();
    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event(
            TestText("request-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
        ),
        |context| async move {
            context.complete_request(TestResponse::approved()).await?;
            Ok(())
        },
    )
    .await
    .expect_value("request subscriber registers");

    let report = bus
        .publish_request(
            test_request(RequestText("resolve-associated-response".to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
            RequestOptions::with_timeout(Duration::from_millis(50))
                .expect_value("request options valid"),
        )
        .await
        .expect_value("request resolves");

    assert_eq!(report.request_id.as_str(), REQUEST_ID);
    assert_eq!(report.response.decision, "approved");
    assert_eq!(report.publish_report.handled_count, 1);
    assert!(bus.take_request_cancellation_reports().is_empty());
}

#[tokio::test]
async fn request_terminal_retention_uses_completion_order_not_request_id_sort_order() {
    let bus = crate::EventBus::root();
    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event(
            TestText("request-retention-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
        ),
        |context| async move {
            context.complete_request(TestResponse::approved()).await?;
            Ok(())
        },
    )
    .await
    .expect_value("request retention subscriber registers");

    let oldest = RequestId::parse("request-z-oldest").expect_value("oldest request id parses");
    publish_retention_probe_request(
        &bus,
        TestText("request-z-oldest".to_owned()),
        TestText("request-z-oldest".to_owned()),
        TestText("request-retention-event-oldest".to_owned()),
    )
    .await;

    let first_new = RequestId::parse("request-a-0000").expect_value("first new request id parses");
    for index in 0..(REQUEST_TERMINAL_RETENTION_PROBE_COUNT - 1) {
        let request_id = format!("request-a-{index:04}");
        let event_id = format!("request-retention-event-{index:04}");
        publish_retention_probe_request(
            &bus,
            TestText(request_id.clone()),
            TestText(request_id.clone()),
            TestText(event_id.clone()),
        )
        .await;
    }

    let retained = bus
        .publish_request(
            test_request_with_id(
                RequestText("retained-first-new".to_owned()),
                RequestText(first_new.as_str().to_owned()),
            ),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("request-retention-event-retained-probe".to_owned()),
            ),
            RequestOptions::with_timeout(Duration::from_millis(50))
                .expect_value("request options valid"),
        )
        .await;
    assert!(matches!(
        retained,
        Err(EventingError::DuplicateRequest { request_id }) if request_id == first_new
    ));

    let recycled_oldest = bus
        .publish_request(
            test_request_with_id(
                RequestText("recycled-oldest".to_owned()),
                RequestText(oldest.as_str().to_owned()),
            ),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("request-retention-event-recycled-oldest".to_owned()),
            ),
            RequestOptions::with_timeout(Duration::from_millis(50))
                .expect_value("request options valid"),
        )
        .await
        .expect_value("completion-order-evicted request id can register again");
    assert_eq!(recycled_oldest.response.decision, "approved");
    assert_eq!(
        bus.metrics_snapshot()
            .await
            .requests
            .completed_request_count,
        4096
    );
}

async fn publish_retention_probe_request(
    bus: &crate::bus::publisher::RootEventPublisher,
    label: TestText,
    request_id: TestText,
    event_id: TestText,
) {
    let label = label.0;
    let request_id = request_id.0;
    let event_id = event_id.0;
    bus.publish_request(
        test_request_with_id(
            RequestText(label.to_owned()),
            RequestText(request_id.to_owned()),
        ),
        metadata_with_event_id(
            TestText(TEST_TARGET.to_owned()),
            TestText(event_id.to_owned()),
        ),
        RequestOptions::with_timeout(Duration::from_millis(50))
            .expect_value("request options valid"),
    )
    .await
    .expect_value("request retention probe resolves");
}

#[tokio::test]
async fn invalid_response_validation_does_not_settle_request() {
    let bus = crate::EventBus::root();
    let invalid_rejected = Arc::new(Mutex::new(false));
    let invalid_rejected_clone = Arc::clone(&invalid_rejected);
    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event(
            TestText("request-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
        ),
        move |context| {
            let invalid_rejected = Arc::clone(&invalid_rejected_clone);
            async move {
                let invalid = context.complete_request(TestResponse::invalid()).await;
                *invalid_rejected.lock().await = invalid.is_err();
                context.complete_request(TestResponse::approved()).await?;
                Ok(())
            }
        },
    )
    .await
    .expect_value("request subscriber registers");

    let report = bus
        .publish_request(
            test_request(RequestText("validate-before-settle".to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
            RequestOptions::with_timeout(Duration::from_millis(50))
                .expect_value("request options valid"),
        )
        .await
        .expect_value("request resolves after valid response");

    assert!(*invalid_rejected.lock().await);
    assert_eq!(report.response.decision, "approved");
}

#[tokio::test]
async fn request_timeout_reports_late_response_without_mutating_result() {
    let bus = crate::EventBus::root();
    let attempts = Arc::new(Mutex::new(Vec::new()));
    let attempts_clone = Arc::clone(&attempts);
    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event(
            TestText("request-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
        ),
        move |context| {
            let attempts = Arc::clone(&attempts_clone);
            async move {
                tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_millis(20)).await;
                    let completion = context
                        .complete_request(TestResponse::approved())
                        .await
                        .map(|report| report.outcome);
                    attempts.lock().await.push(completion);
                });
                Ok(())
            }
        },
    )
    .await
    .expect_value("request subscriber registers");

    let result = bus
        .publish_request(
            test_request(RequestText("timeout".to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
            RequestOptions::with_timeout(Duration::from_millis(5))
                .expect_value("request options valid"),
        )
        .await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    assert!(matches!(result, Err(EventingError::RequestTimedOut { .. })));
    assert_eq!(
        attempts.lock().await.as_slice(),
        &[Err(EventingError::CausalPublicationOutsideHandlerTask)]
    );
    let metrics = bus.metrics_snapshot().await;
    assert_eq!(metrics.requests.completed_request_count, 0);
    assert_eq!(metrics.requests.timed_out_request_count, 1);
    assert_eq!(metrics.queue.queued_event_count, 0);
    assert_eq!(metrics.queue.in_flight_event_id_count, 0);
}

#[tokio::test]
async fn request_timeout_covers_slow_handler_dispatch() {
    let bus = crate::EventBus::root();
    let outcomes = Arc::new(Mutex::new(Vec::new()));
    let outcomes_clone = Arc::clone(&outcomes);
    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event(
            TestText("request-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
        ),
        move |context| {
            let outcomes = Arc::clone(&outcomes_clone);
            async move {
                tokio::time::sleep(Duration::from_millis(50)).await;
                let report = context.complete_request(TestResponse::approved()).await?;
                outcomes.lock().await.push(report.outcome);
                Ok(())
            }
        },
    )
    .await
    .expect_value("request subscriber registers");

    let result = bus
        .publish_request(
            test_request(RequestText("slow-handler-timeout".to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
            RequestOptions::with_timeout(Duration::from_millis(5))
                .expect_value("request options valid"),
        )
        .await;
    tokio::time::sleep(Duration::from_millis(75)).await;

    assert!(matches!(result, Err(EventingError::RequestTimedOut { .. })));
    assert!(outcomes.lock().await.is_empty());
    let metrics = bus.metrics_snapshot().await;
    assert_eq!(metrics.queue.in_flight_event_id_count, 0);
    assert_eq!(metrics.queue.in_flight_idempotency_key_count, 0);
    assert_eq!(metrics.requests.timed_out_request_count, 1);
}

#[tokio::test]
async fn request_timeout_aborts_never_completing_publish_and_releases_in_flight() {
    let bus = crate::EventBus::root();
    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event(
            TestText("request-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
        ),
        |_context| async move { future::pending::<Result<(), EventingError>>().await },
    )
    .await
    .expect_value("request subscriber registers");

    let result = bus
        .publish_request(
            test_request(RequestText("never-completing-handler-timeout".to_owned())),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("request-never-completing-event".to_owned()),
            ),
            RequestOptions::with_timeout(Duration::from_millis(5))
                .expect_value("request options valid"),
        )
        .await;

    assert!(matches!(result, Err(EventingError::RequestTimedOut { .. })));
    let metrics = bus.metrics_snapshot().await;
    assert_eq!(metrics.queue.in_flight_event_id_count, 0);
    assert_eq!(metrics.queue.in_flight_idempotency_key_count, 0);
    assert_eq!(metrics.requests.timed_out_request_count, 1);
}

#[tokio::test]
async fn publish_request_cancels_registry_entry_when_publish_fails() {
    let bus = crate::EventBus::root();
    let failed = bus
        .publish_request(
            InvalidContractRequestEvent::new(),
            metadata(TestText(TEST_TARGET.to_owned())),
            RequestOptions::with_timeout(Duration::from_millis(50))
                .expect_value("request options valid"),
        )
        .await;
    assert!(matches!(failed, Err(EventingError::InvalidVersion)));
    assert!(bus.take_request_cancellation_reports().is_empty());

    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event(
            TestText("request-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
        ),
        |context| async move {
            context.complete_request(TestResponse::approved()).await?;
            Ok(())
        },
    )
    .await
    .expect_value("request subscriber registers");
    let report = bus
        .publish_request(
            test_request(RequestText("retry-after-publish-failure".to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
            RequestOptions::with_timeout(Duration::from_millis(50))
                .expect_value("request options valid"),
        )
        .await
        .expect_value("request id can be reused after failed publish");

    assert_eq!(report.response.decision, "approved");
}

#[tokio::test]
async fn double_completion_is_ignored_and_reported() {
    let bus = crate::EventBus::root();
    let outcomes = Arc::new(Mutex::new(Vec::new()));
    let outcomes_clone = Arc::clone(&outcomes);
    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event(
            TestText("request-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
        ),
        move |context| {
            let outcomes = Arc::clone(&outcomes_clone);
            async move {
                let first = context.complete_request(TestResponse::approved()).await?;
                let second = context.complete_request(TestResponse::approved()).await?;
                outcomes
                    .lock()
                    .await
                    .extend([first.outcome, second.outcome]);
                Ok(())
            }
        },
    )
    .await
    .expect_value("request subscriber registers");

    let report = bus
        .publish_request(
            test_request(RequestText("double-completion".to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
            RequestOptions::with_timeout(Duration::from_millis(50))
                .expect_value("request options valid"),
        )
        .await
        .expect_value("first request completion resolves");

    assert_eq!(report.response.decision, "approved");
    assert_eq!(
        outcomes.lock().await.as_slice(),
        &[
            RequestCompletionOutcome::Completed,
            RequestCompletionOutcome::Duplicate
        ]
    );
}

#[tokio::test]
async fn in_memory_result_event_pattern_remains_separate_from_local_completion() {
    let bus = crate::EventBus::root();
    let result_owner = EventRecorder::<TestResultEvent>::attach(
        &bus,
        subscriber_for_event(
            TestText("request-result-owner".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(RESULT_EVENT_TYPE.to_owned()),
        ),
    )
    .await
    .expect_value("result owner subscribes through the real bus");
    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event(
            TestText("request-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
        ),
        move |context| async move {
            context
                .publisher()
                .publish(
                    test_result_event(),
                    metadata_with_event_id(
                        TestText(TEST_TARGET.to_owned()),
                        TestText("request-result-event-1".to_owned()),
                    ),
                )
                .await?;
            context.complete_request(TestResponse::approved()).await?;
            Ok(())
        },
    )
    .await
    .expect_value("request subscriber registers");

    let report = bus
        .publish_request(
            test_request(RequestText("durable-result-event".to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
            RequestOptions::with_timeout(Duration::from_millis(50))
                .expect_value("request options valid"),
        )
        .await
        .expect_value("request resolves");
    let journal = bus.journal().await;
    let owned_results = result_owner.recorded().await;

    assert_eq!(report.response.decision, "approved");
    assert_eq!(owned_results.len(), 1);
    assert_eq!(
        owned_results[0].contract().event_type.as_str(),
        RESULT_EVENT_TYPE
    );
    assert_eq!(journal.len(), 2);
    assert_eq!(journal[0].contract.event_type.as_str(), REQUEST_EVENT_TYPE);
    assert_eq!(journal[1].contract.event_type.as_str(), RESULT_EVENT_TYPE);
}
