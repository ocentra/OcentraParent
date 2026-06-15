use std::sync::Arc;

use crate::ids::EventNamespace;
use crate::journal::policy::{JournalPolicy, JournalSelector};

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
async fn bus_journal_policy_honors_before_after_and_selected_journaling(
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    assert_before_dispatch_selected_type().await?;
    assert_after_dispatch_selected_namespace().await?;
    assert_before_and_after_dispatch_allowlist().await?;
    Ok(())
}

async fn assert_before_dispatch_selected_type(
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let before_log = shared_log();
    let before_bus = bus_with_recording_journal(
        JournalPolicy::before_dispatch(JournalSelector::EventTypes(vec![event_type(
            TEST_EVENT_TYPE,
        )?])),
        Arc::clone(&before_log),
    );
    subscribe_log_handler(&before_bus, Arc::clone(&before_log)).await?;
    before_bus
        .publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .await?;
    assert_eq!(
        snapshot(&before_log),
        vec![
            format!("journal:{TEST_EVENT_TYPE}"),
            String::from("handler"),
        ]
    );
    Ok(())
}

async fn assert_after_dispatch_selected_namespace(
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let after_log = shared_log();
    let after_bus = bus_with_recording_journal(
        JournalPolicy::after_dispatch(JournalSelector::Namespaces(vec![EventNamespace::parse(
            "eventing",
        )?])),
        Arc::clone(&after_log),
    );
    subscribe_log_handler(&after_bus, Arc::clone(&after_log)).await?;
    after_bus
        .publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .await?;
    assert_eq!(
        snapshot(&after_log),
        vec![
            String::from("handler"),
            format!("journal:{TEST_EVENT_TYPE}"),
        ]
    );
    Ok(())
}

async fn assert_before_and_after_dispatch_allowlist(
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let both_log = shared_log();
    let both_bus = bus_with_recording_journal(
        JournalPolicy::before_and_after_dispatch(JournalSelector::ContractAllowlist(vec![
            event_type(TEST_EVENT_TYPE)?,
        ])),
        Arc::clone(&both_log),
    );
    subscribe_log_handler(&both_bus, Arc::clone(&both_log)).await?;
    both_bus
        .publish(
            test_event_for_type(TEST_LABEL, OTHER_EVENT_TYPE),
            metadata(TEST_TARGET),
        )
        .await
        ?;
    both_bus
        .publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .await
        ?;
    assert_eq!(
        snapshot(&both_log),
        vec![
            format!("journal:{TEST_EVENT_TYPE}"),
            String::from("handler"),
            format!("journal:{TEST_EVENT_TYPE}"),
        ]
    );
    Ok(())
}
