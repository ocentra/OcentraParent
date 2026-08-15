use super::*;

#[tokio::test]
async fn before_dispatch_journal_is_durable_without_a_subscriber() {
    let journal = Arc::new(FailingJournal::fail_once_on(usize::MAX));
    let bus = EventBus::with_journal(
        JournalPolicy::before_dispatch(JournalSelector::All),
        Arc::<FailingJournal>::clone(&journal),
    );

    let report = bus
        .publish(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("before-dispatch journal persists without a subscriber");

    assert_eq!(report.subscriber_count, 0);
    assert_eq!(
        report.queue_report.disposition,
        QueueDisposition::Dispatched
    );
    assert_eq!(report.journal_appends.len(), 1);
    assert!(report.journal_appends[0].is_synchronized());
    assert_eq!(journal.phases(), vec![JournalDispatchPhase::BeforeDispatch]);
}
