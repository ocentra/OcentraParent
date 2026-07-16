use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::EventNamespace;
use ocentra_eventing::journal::policy::{JournalPolicy, JournalSelector};
use std::sync::Arc;

use super::{
    super::fixtures::{
        metadata, test_event, test_event_for_type, TestText, OTHER_EVENT_TYPE, TEST_EVENT_TYPE,
        TEST_LABEL, TEST_TARGET,
    },
    support::{
        bus_with_recording_journal, event_type, shared_log, snapshot, subscribe_log_handler,
    },
};

#[tokio::test]
async fn bus_journal_policy_honors_before_after_and_selected_journaling() {
    assert_before_dispatch_selected_type().await;
    assert_after_dispatch_selected_namespace().await;
    assert_before_and_after_dispatch_allowlist().await;
}

async fn assert_before_dispatch_selected_type() {
    let before_log = shared_log();
    let before_bus = bus_with_recording_journal(
        JournalPolicy::before_dispatch(JournalSelector::EventTypes(vec![event_type(TestText(
            TEST_EVENT_TYPE.to_owned(),
        ))])),
        Arc::clone(&before_log),
    );
    subscribe_log_handler(&before_bus, Arc::clone(&before_log)).await;
    before_bus
        .publish(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("before dispatch publish");
    assert_eq!(
        snapshot(&before_log),
        vec![
            format!("journal:{TEST_EVENT_TYPE}"),
            String::from("handler"),
        ]
    );
}

async fn assert_after_dispatch_selected_namespace() {
    let after_log = shared_log();
    let after_bus = bus_with_recording_journal(
        JournalPolicy::after_dispatch(JournalSelector::Namespaces(vec![EventNamespace::parse(
            "eventing",
        )
        .expect_value("namespace parses")])),
        Arc::clone(&after_log),
    );
    subscribe_log_handler(&after_bus, Arc::clone(&after_log)).await;
    after_bus
        .publish(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("after dispatch publish");
    assert_eq!(
        snapshot(&after_log),
        vec![
            String::from("handler"),
            format!("journal:{TEST_EVENT_TYPE}"),
        ]
    );
}

async fn assert_before_and_after_dispatch_allowlist() {
    let both_log = shared_log();
    let both_bus = bus_with_recording_journal(
        JournalPolicy::before_and_after_dispatch(JournalSelector::ContractAllowlist(vec![
            event_type(TestText(TEST_EVENT_TYPE.to_owned())),
        ])),
        Arc::clone(&both_log),
    );
    subscribe_log_handler(&both_bus, Arc::clone(&both_log)).await;
    both_bus
        .publish(
            test_event_for_type(
                TestText(TEST_LABEL.to_owned()),
                TestText(OTHER_EVENT_TYPE.to_owned()),
            ),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("unselected publish");
    both_bus
        .publish(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("selected publish");
    assert_eq!(
        snapshot(&both_log),
        vec![
            format!("journal:{TEST_EVENT_TYPE}"),
            String::from("handler"),
            format!("journal:{TEST_EVENT_TYPE}"),
        ]
    );
}
