use crate::{EventNamespace, JournalPolicy, JournalSelector};

use super::{
    super::fixtures::{
        metadata, test_event, test_event_for_type, OTHER_EVENT_TYPE, TEST_EVENT_TYPE, TEST_LABEL,
        TEST_TARGET,
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
        JournalPolicy::before_dispatch(JournalSelector::EventTypes(vec![event_type(
            TEST_EVENT_TYPE,
        )])),
        before_log.clone(),
    );
    subscribe_log_handler(&before_bus, before_log.clone()).await;
    before_bus
        .publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .await
        .expect("before dispatch publish");
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
        .expect("namespace parses")])),
        after_log.clone(),
    );
    subscribe_log_handler(&after_bus, after_log.clone()).await;
    after_bus
        .publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .await
        .expect("after dispatch publish");
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
            event_type(TEST_EVENT_TYPE),
        ])),
        both_log.clone(),
    );
    subscribe_log_handler(&both_bus, both_log.clone()).await;
    both_bus
        .publish(
            test_event_for_type(TEST_LABEL, OTHER_EVENT_TYPE),
            metadata(TEST_TARGET),
        )
        .await
        .expect("unselected publish");
    both_bus
        .publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .await
        .expect("selected publish");
    assert_eq!(
        snapshot(&both_log),
        vec![
            format!("journal:{TEST_EVENT_TYPE}"),
            String::from("handler"),
            format!("journal:{TEST_EVENT_TYPE}"),
        ]
    );
}
