use std::collections::BTreeSet;

use super::*;

#[tokio::test]
async fn handler_dead_letters_have_distinct_idempotency_keys_per_delivery() {
    let bus = EventBus::root();
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        |_| async {
            Err(EventingError::InvalidValue {
                field: "handler",
                value: "failed".to_owned(),
            })
        },
    )
    .await
    .expect_value("first failing subscriber registers");
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(OTHER_SUBSCRIBER.to_owned()),
            TestText(OTHER_TARGET.to_owned()),
        ),
        |_| async {
            Err(EventingError::InvalidValue {
                field: "handler",
                value: "failed".to_owned(),
            })
        },
    )
    .await
    .expect_value("second failing subscriber registers");

    let mut event_metadata = metadata_with_event_id(
        TestText(TEST_TARGET.to_owned()),
        TestText("multi-handler-dead-letter".to_owned()),
    );
    event_metadata.target_handler = None;
    let report = bus
        .publish(
            test_event(TestText("multi-handler failure".to_owned())),
            event_metadata,
        )
        .await
        .expect_value("handler failures produce a publish report");
    let dead_letters = bus.dead_letters().await;

    let identities = dead_letters
        .iter()
        .map(|dead_letter| {
            (
                dead_letter
                    .subscriber_id
                    .as_ref()
                    .expect_value("handler dead letter has subscriber identity")
                    .as_str()
                    .to_owned(),
                dead_letter
                    .target_handler
                    .as_ref()
                    .expect_value("handler dead letter has target identity")
                    .as_str()
                    .to_owned(),
            )
        })
        .collect::<BTreeSet<_>>();
    let idempotency_keys = dead_letters
        .iter()
        .map(|dead_letter| {
            dead_letter
                .as_event()
                .idempotency_key()
                .expect_value("dead-letter idempotency key parses")
                .as_str()
                .to_owned()
        })
        .collect::<BTreeSet<_>>();

    assert_eq!(report.subscriber_count, 2);
    assert_eq!(report.handled_count, 0);
    assert_eq!(report.dead_letter_count, 2);
    assert_eq!(dead_letters.len(), 2);
    assert_eq!(
        identities,
        BTreeSet::from([
            (TEST_SUBSCRIBER.to_owned(), TEST_TARGET.to_owned()),
            (OTHER_SUBSCRIBER.to_owned(), OTHER_TARGET.to_owned()),
        ])
    );
    assert_eq!(
        idempotency_keys,
        BTreeSet::from([
            concat!(
                "dead-letter-multi-handler-dead-letter-handler-failed-",
                "subscriber-24:eventing-test-subscriber-target-21:eventing-test-handler"
            )
            .to_owned(),
            concat!(
                "dead-letter-multi-handler-dead-letter-handler-failed-",
                "subscriber-25:eventing-other-subscriber-target-22:eventing-other-handler"
            )
            .to_owned(),
        ])
    );
}
