use ocentra_parent_agent_core::{
    request_browser_runtime_social_provider_receipt_status_for_input, BrowserRuntimeInput,
};
use ocentra_parent_agent_protocol::constants;

use crate::browser_runtime_stream_payload::{
    stream_browser_runtime_event_chain_for_read_model_with_policy_preview,
    BrowserRuntimeServiceStreamReport,
};

#[tokio::test]
async fn service_browser_runtime_social_provider_receipt_status_records_provider_boundary() {
    let receipt = request_browser_runtime_social_provider_receipt_status_for_input(
        BrowserRuntimeInput::dry_run_action_handoff_fixture(),
    )
    .await
    .unwrap()
    .request_report
    .response;
    let mut report = BrowserRuntimeServiceStreamReport::default();
    report.record_social_provider_receipt(&receipt);

    assert_eq!(report.social_provider_receipt_boundary_rows, 1);
    assert_eq!(report.social_provider_dispatch_required_rows, 1);
    assert_eq!(report.social_provider_manual_receipt_required_rows, 0);
    assert_social_provider_durable_refs(&report);
    assert_eq!(
        report.social_provider_attempt_refs,
        vec![constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_ATTEMPT_REF.to_string()]
    );
    assert_eq!(
        report.social_provider_receipt_proof_refs,
        vec![
            constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_PROOF_REF.to_string()
        ]
    );
    assert_eq!(report.action_intent_dispatch_attempts, 0);
    assert_eq!(report.action_intent_adapter_executions, 0);
    assert_eq!(report.action_intent_child_intervention_executions, 0);
    assert_eq!(report.action_intent_enforcement_executions, 0);
}

#[tokio::test]
async fn service_browser_runtime_stream_records_store_backed_social_provider_receipt_status() {
    let read_model = super::read_model(vec![super::managed_row()]);
    let policy_preview = super::policy_preview_read_model_for_browser(&read_model);
    let report = stream_browser_runtime_event_chain_for_read_model_with_policy_preview(
        &read_model,
        Some(&policy_preview),
    )
    .await;

    assert_eq!(report.social_provider_receipt_boundary_rows, 1);
    assert_eq!(report.social_provider_dispatch_required_rows, 1);
    assert_eq!(report.social_provider_manual_receipt_required_rows, 0);
    assert_social_provider_durable_refs(&report);
    assert_eq!(
        report.social_provider_attempt_refs,
        vec![constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_ATTEMPT_REF.to_string()]
    );
    assert_eq!(
        report.social_provider_receipt_proof_refs,
        vec![
            constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_PROOF_REF.to_string()
        ]
    );
    assert_eq!(report.action_intent_dispatch_attempts, 0);
    assert_eq!(report.action_intent_adapter_executions, 0);
    assert_eq!(report.action_intent_child_intervention_executions, 0);
    assert_eq!(report.action_intent_enforcement_executions, 0);
}

#[tokio::test]
async fn service_browser_runtime_stream_keeps_manual_social_provider_receipt_rows_manual_required()
{
    let report = stream_browser_runtime_event_chain_for_read_model_with_policy_preview(
        &super::read_model(vec![super::unavailable_row()]),
        None,
    )
    .await;

    assert_eq!(report.social_provider_receipt_boundary_rows, 1);
    assert_eq!(report.social_provider_dispatch_required_rows, 0);
    assert_eq!(report.social_provider_manual_receipt_required_rows, 1);
    assert_eq!(report.social_provider_durable_rows, 0);
    assert!(report.social_provider_durable_result_refs.is_empty());
    assert!(report.social_provider_durable_store_refs.is_empty());
    assert!(report.social_provider_read_model_refs.is_empty());
    assert!(report.social_provider_support_status_refs.is_empty());
    assert!(report.social_provider_attempt_refs.is_empty());
    assert_eq!(
        report.social_provider_receipt_proof_refs,
        vec![
            constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_PROOF_REF.to_string()
        ]
    );
    assert_eq!(report.action_intent_dispatch_attempts, 0);
    assert_eq!(report.action_intent_adapter_executions, 0);
    assert_eq!(report.action_intent_child_intervention_executions, 0);
    assert_eq!(report.action_intent_enforcement_executions, 0);
}

fn assert_social_provider_durable_refs(report: &BrowserRuntimeServiceStreamReport) {
    assert_eq!(report.social_provider_durable_rows, 1);
    assert_eq!(
        report.social_provider_durable_result_refs,
        vec![
            constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_DURABLE_RESULT_REF
                .to_string()
        ]
    );
    assert_eq!(
        report.social_provider_durable_store_refs,
        vec![
            constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_DURABLE_STORE_REF
                .to_string()
        ]
    );
    assert_eq!(
        report.social_provider_read_model_refs,
        vec![
            constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_READ_MODEL_REF
                .to_string()
        ]
    );
    assert_eq!(
        report.social_provider_support_status_refs,
        vec![
            constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_SUPPORT_STATUS_REF
                .to_string()
        ]
    );
}
