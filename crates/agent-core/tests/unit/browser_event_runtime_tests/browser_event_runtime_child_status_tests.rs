use super::{ok, some, TestResult};
use ocentra_parent_agent_core::browser_event_runtime::action_handoff_child_status::prove_browser_runtime_action_intent_child_status;
use ocentra_parent_agent_core::browser_event_runtime::action_handoff_child_status_types::BrowserRuntimeActionIntentChildStatusReadModelState;
use ocentra_parent_agent_protocol::constants;

#[tokio::test]
async fn browser_runtime_action_intent_child_status_links_durable_handoff_to_child_acceptance(
) -> TestResult {
    let report = ok(
        prove_browser_runtime_action_intent_child_status().await,
        constants::browser::ERROR_BROWSER_RUNTIME_ACTION_INTENT_DURABLE_HANDOFF,
    )?;
    let row = some(
        report.rows.first(),
        constants::browser::ERROR_BROWSER_RUNTIME_ACTION_INTENT_DURABLE_HANDOFF,
    )?;

    assert_eq!(report.handoff_candidate_count, 1);
    assert_eq!(report.child_command_received_count, 1);
    assert_eq!(report.child_command_accepted_count, 1);
    assert_eq!(report.parent_read_model_row_count, 1);
    assert_eq!(report.child_accepted_not_executed_count, 1);
    assert!(report.handoff_refs_match_durable_record);
    assert!(report.child_command_matches_handoff);
    assert!(report.parent_read_model_visible);
    assert_eq!(
        row.policy_preview_id,
        constants::browser::TEST_BROWSER_RUNTIME_POLICY_PREVIEW_ID
    );
    assert_eq!(
        row.action_intent_id,
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_ID
    );
    assert_eq!(
        row.durable_result_ref,
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_DURABLE_RESULT_REF
    );
    assert_eq!(
        row.durable_read_model_ref,
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_READ_MODEL_REF
    );
    assert_eq!(
        row.outbox_ref,
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_OUTBOX_REF
    );
    assert_eq!(
        row.handoff_ref,
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REF
    );
    assert_eq!(
        row.state,
        BrowserRuntimeActionIntentChildStatusReadModelState::ChildAcceptedNotExecuted
    );
    assert!(row
        .child_command_ref
        .contains(constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_ID));
    assert!(row
        .child_command_received_event_ref
        .contains(constants::child_agent::EVENT_COMMAND_RECEIVED));
    assert!(row
        .child_command_accepted_event_ref
        .contains(constants::child_agent::EVENT_COMMAND_ACCEPTED));
    assert!(row
        .parent_read_model_projected_event_ref
        .contains(constants::parent_controller::EVENT_READ_MODEL_PROJECTED));
    assert_eq!(report.dispatch_attempt_count, 0);
    assert_eq!(report.adapter_execution_count, 0);
    assert_eq!(report.browser_mutation_count, 0);
    assert_eq!(report.child_intervention_execution_count, 0);
    assert_eq!(report.final_policy_execution_count, 0);
    assert_eq!(report.enforcement_execution_count, 0);
    assert!(report.public_stream_field_registry_ready);
    assert!(!report.external_transport_implemented);
    assert!(!report.adapter_dispatch_claimed);
    assert!(!report.browser_mutation_claimed);
    assert!(!report.child_intervention_execution_claimed);
    assert!(!report.final_policy_execution_claimed);
    assert!(!report.enforcement_claimed);
    Ok(())
}
