use crate::{
    browser_runtime_stream_payload::stream_browser_runtime_event_chain_for_read_model_with_policy_preview,
    browser_runtime_stream_request::request_browser_runtime_service_stream_report,
};

#[tokio::test]
async fn service_browser_runtime_stream_uses_named_event_request_boundary() -> super::TestResult {
    let read_model = super::read_model(vec![super::managed_row()]);
    let policy_preview = super::policy_preview_read_model_for_browser(&read_model)?;
    let direct_report = stream_browser_runtime_event_chain_for_read_model_with_policy_preview(
        &read_model,
        Some(&policy_preview),
    )
    .await;
    let evented_report =
        request_browser_runtime_service_stream_report(read_model, Some(policy_preview))
            .await
            .map_err(|error| std::io::Error::other(format!("{error:?}")))?;

    assert_eq!(evented_report, direct_report);
    assert_eq!(evented_report.action_intent_handoff_candidates, 1);
    assert_eq!(evented_report.action_intent_child_accepted_rows, 0);
    assert!(evented_report.action_intent_child_command_refs.is_empty());
    assert!(evented_report
        .action_intent_child_accepted_event_refs
        .is_empty());
    assert!(evented_report
        .action_intent_parent_read_model_refs
        .is_empty());
    assert_eq!(evented_report.action_intent_dispatch_attempts, 0);
    assert_eq!(evented_report.action_intent_adapter_executions, 0);
    assert_eq!(
        evented_report.action_intent_child_intervention_executions,
        0
    );
    assert_eq!(evented_report.action_intent_enforcement_executions, 0);

    Ok(())
}
