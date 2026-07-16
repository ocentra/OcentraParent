use super::{ok, some, TestResult};
use ocentra_eventing::topology::EventTopologyStatus;
use ocentra_parent_agent_core::browser_event_runtime::social_provider_receipt::{
    browser_runtime_social_provider_receipt_status_topology_manifest,
    request_browser_runtime_social_provider_receipt_status_for_input,
};
use ocentra_parent_agent_core::browser_event_runtime::social_provider_receipt_durable::prove_browser_runtime_social_provider_receipt_durable;
use ocentra_parent_agent_core::browser_event_runtime::social_provider_receipt_durable_types::BrowserRuntimeSocialProviderReceiptDurableReadModelState;
use ocentra_parent_agent_core::browser_event_runtime::BrowserRuntimeInput;
use ocentra_parent_agent_protocol::constants;

#[tokio::test]
async fn browser_runtime_social_provider_receipt_event_subscriber_returns_manual_required_boundary(
) -> TestResult {
    let report = ok(
        request_browser_runtime_social_provider_receipt_status_for_input(
            BrowserRuntimeInput::dry_run_action_handoff_fixture(),
        )
        .await,
        "request social provider receipt status for dry run fixture",
    )?;

    assert_eq!(report.dead_letters.len(), 0);
    assert_eq!(report.request_report.publish_report.handled_count, 1);
    assert_eq!(
        some(
            report.stored_events.first(),
            "social provider receipt request event missing",
        )?
        .contract
        .event_type
        .as_str(),
        constants::browser::EVENT_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_REQUESTED
    );

    let receipt = report.request_report.response;
    assert_eq!(receipt.receipt_boundary_row_count, 1);
    assert_eq!(receipt.provider_dispatch_required_count, 1);
    assert_eq!(receipt.manual_receipt_required_count, 0);
    assert_eq!(
        receipt.provider_attempt_ref.as_deref(),
        Some(constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_ATTEMPT_REF)
    );
    assert_eq!(
        receipt.provider_receipt_proof_ref.as_deref(),
        Some(constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_PROOF_REF)
    );
    assert_eq!(
        receipt.action_intent_id.as_deref(),
        Some(constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_ID)
    );
    assert_eq!(
        receipt.receipt_boundary_state,
        constants::browser::SOCIAL_PROVIDER_RECEIPT_STATE_PROVIDER_DISPATCH_REQUIRED
    );
    assert_eq!(
        receipt.receipt_runtime_state,
        constants::browser::SOCIAL_PROVIDER_RECEIPT_RUNTIME_STATE_MANUAL_REQUIRED
    );
    assert_eq!(receipt.provider_receipt_count, 0);
    assert_eq!(receipt.provider_dispatch_count, 0);
    assert_eq!(receipt.provider_webhook_count, 0);
    assert_eq!(receipt.provider_credentials_count, 0);
    assert_eq!(receipt.parent_notification_ui_delivery_count, 0);
    assert_eq!(receipt.report_delivery_execution_count, 0);
    assert_eq!(receipt.final_policy_execution_count, 0);
    assert_eq!(receipt.connector_native_runtime_count, 0);
    assert_eq!(receipt.enforcement_execution_count, 0);
    Ok(())
}

#[tokio::test]
async fn browser_runtime_social_provider_receipt_event_subscriber_keeps_manual_rows_manual_required(
) -> TestResult {
    let report = ok(
        request_browser_runtime_social_provider_receipt_status_for_input(
            BrowserRuntimeInput::manual_required_fixture(),
        )
        .await,
        "request social provider receipt status for manual fixture",
    )?;

    let receipt = report.request_report.response;
    assert_eq!(receipt.receipt_boundary_row_count, 1);
    assert_eq!(receipt.provider_dispatch_required_count, 0);
    assert_eq!(receipt.manual_receipt_required_count, 1);
    assert_eq!(receipt.provider_attempt_ref, None);
    assert_eq!(receipt.action_intent_id, None);
    assert_eq!(
        receipt.receipt_boundary_state,
        constants::browser::SOCIAL_PROVIDER_RECEIPT_STATE_MANUAL_REQUIRED
    );
    assert_eq!(
        receipt.receipt_runtime_state,
        constants::browser::SOCIAL_PROVIDER_RECEIPT_RUNTIME_STATE_MANUAL_REQUIRED
    );
    assert_eq!(receipt.provider_receipt_count, 0);
    assert_eq!(receipt.provider_dispatch_count, 0);
    assert_eq!(receipt.final_policy_execution_count, 0);
    assert_eq!(receipt.enforcement_execution_count, 0);
    Ok(())
}

#[test]
fn browser_runtime_social_provider_receipt_topology_covers_named_event_and_subscriber() -> TestResult
{
    let manifest = ok(
        browser_runtime_social_provider_receipt_status_topology_manifest(),
        "browser runtime social provider receipt topology manifest",
    )?;
    assert_eq!(manifest.unready_entries().len(), 0);
    assert_eq!(manifest.entries().len(), 1);

    let entry = some(
        manifest.entries().first(),
        "social provider receipt topology entry missing",
    )?;
    assert_eq!(entry.status, EventTopologyStatus::Covered);
    assert_eq!(
        entry.contract.event_type.as_str(),
        constants::browser::EVENT_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_REQUESTED
    );
    assert_eq!(
        some(
            entry.publishers.first(),
            "social provider receipt publisher missing",
        )?
        .as_str(),
        constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE
    );
    let subscriber = some(
        entry.subscribers.first(),
        "social provider receipt subscriber missing",
    )?;
    assert_eq!(
        subscriber.subscriber_id.as_str(),
        constants::browser::SUBSCRIBER_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS
    );
    assert_eq!(
        subscriber.target_handler.as_str(),
        constants::browser::TARGET_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS
    );
    Ok(())
}

#[tokio::test]
async fn browser_runtime_social_provider_receipt_durable_preserves_refs_without_execution(
) -> TestResult {
    let report = ok(
        prove_browser_runtime_social_provider_receipt_durable().await,
        "prove browser runtime social provider receipt durable",
    )?;

    assert_eq!(report.request_event_count, 1);
    assert_eq!(report.durable_record_count, 1);
    assert_eq!(report.read_model_row_count, 1);
    assert_eq!(report.provider_dispatch_required_count, 1);
    assert_eq!(report.manual_receipt_required_count, 0);
    assert!(report.duplicate_request_event_rejected);
    assert!(report.row_matches_receipt_response);
    assert!(report.row_matches_request_event);

    let row = some(
        report.rows.first(),
        "social provider receipt durable row missing",
    )?;
    assert_eq!(
        row.state,
        BrowserRuntimeSocialProviderReceiptDurableReadModelState::ProviderDispatchRequiredManualReceipt
    );
    assert_eq!(
        row.request_event_type.as_str(),
        constants::browser::EVENT_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_REQUESTED
    );
    assert_eq!(
        row.action_intent_id,
        constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_ID
    );
    assert_eq!(
        row.provider_attempt_ref.as_str(),
        constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_ATTEMPT_REF
    );
    assert_eq!(
        row.provider_receipt_proof_ref.as_str(),
        constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_PROOF_REF
    );
    assert_eq!(
        row.durable_result_ref.as_str(),
        constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_DURABLE_RESULT_REF
    );
    assert_eq!(
        row.durable_store_ref.as_str(),
        constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_DURABLE_STORE_REF
    );
    assert_eq!(
        row.read_model_ref.as_str(),
        constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_READ_MODEL_REF
    );
    assert_eq!(
        row.support_status_ref.as_str(),
        constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_SUPPORT_STATUS_REF
    );
    assert_eq!(report.provider_receipt_count, 0);
    assert_eq!(report.provider_dispatch_count, 0);
    assert_eq!(report.connector_native_runtime_count, 0);
    assert_eq!(report.parent_notification_ui_delivery_count, 0);
    assert_eq!(report.report_delivery_execution_count, 0);
    assert_eq!(report.final_policy_execution_count, 0);
    assert_eq!(report.enforcement_execution_count, 0);
    assert!(!report.provider_receipt_claimed);
    assert!(!report.provider_dispatch_claimed);
    assert!(!report.connector_native_runtime_claimed);
    assert!(!report.parent_notification_ui_delivery_claimed);
    assert!(!report.report_delivery_execution_claimed);
    assert!(!report.final_policy_execution_claimed);
    assert!(!report.enforcement_claimed);
    Ok(())
}
