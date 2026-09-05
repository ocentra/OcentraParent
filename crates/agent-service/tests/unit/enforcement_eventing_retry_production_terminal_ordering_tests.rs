use super::*;

#[tokio::test]
async fn production_command_retry_rejects_terminal_before_pre_action_ordering() {
    let seed = TestArtifacts::new("terminal-before-pre-action-seed");
    let command = command();
    let first =
        build_enforcement_audit_report_with_paths(command.clone(), seed.paths.clone()).await;
    assert_eq!(first.event, AgentEventName::AgentEnforcementAuditReported);
    let replay = seed
        .journal()
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("replay production ordering seed journal");
    let before = replay
        .records
        .iter()
        .find(|record| {
            record
                .envelope
                .event_id
                .as_str()
                .starts_with(constants::enforcement::JOURNAL_BEFORE_ACTION_ID_PREFIX)
        })
        .expect_value("production ordering seed before-dispatch envelope")
        .envelope
        .clone();
    let completed = replay
        .records
        .iter()
        .find(|record| {
            record.envelope.event_id.as_str() == constants::enforcement::TEST_AUDIT_EVENT_ID
        })
        .expect_value("production ordering seed after-dispatch envelope")
        .envelope
        .clone();

    let retry = TestArtifacts::new("terminal-before-pre-action-retry");
    create_dir_all(
        retry
            .eventing_path
            .parent()
            .expect_value("ordering test eventing artifact parent"),
    )
    .expect_value("create ordering test eventing artifact parent");
    retry
        .journal()
        .append_phase_idempotent_by_event_id(&completed, JournalDispatchPhase::AfterDispatch)
        .await
        .expect_value("persist ordering test terminal journal row");
    retry
        .journal()
        .append_phase_idempotent_by_event_id(&before, JournalDispatchPhase::BeforeDispatch)
        .await
        .expect_value("persist ordering test pre-action journal row");
    create_dir_all(
        retry
            .paths
            .store_path
            .parent()
            .expect_value("ordering test store artifact parent"),
    )
    .expect_value("create ordering test store artifact parent");
    std::fs::copy(&seed.paths.store_path, &retry.paths.store_path)
        .expect_value("copy real persisted ordering test report");

    let rejected = build_enforcement_audit_report_with_paths(command, retry.paths.clone()).await;
    assert_eq!(rejected.event, AgentEventName::AgentCommandRejected);
    assert_reason(
        &rejected.payload,
        constants::enforcement::REJECTION_RETRY_RECONCILIATION_REQUIRED,
    );
    let after_retry = retry
        .journal()
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("replay reversed-order production journal");
    assert_eq!(after_retry.records.len(), 2);
    assert_eq!(raw_event_entry_count(&retry.eventing_path).await, 2);
}
