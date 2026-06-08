use crate::{
    browser_runtime_social_provider_receipt_status_topology_manifest,
    request_browser_runtime_social_provider_receipt_status_for_input, BrowserRuntimeInput,
};
use ocentra_eventing::EventTopologyStatus;
use ocentra_parent_agent_protocol::constants;

#[tokio::test]
async fn browser_runtime_social_provider_receipt_event_subscriber_returns_manual_required_boundary()
{
    let report = request_browser_runtime_social_provider_receipt_status_for_input(
        BrowserRuntimeInput::dry_run_action_handoff_fixture(),
    )
    .await
    .unwrap();

    assert_eq!(report.dead_letters.len(), 0);
    assert_eq!(report.request_report.publish_report.handled_count, 1);
    assert_eq!(
        report
            .stored_events
            .first()
            .unwrap()
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
}

#[tokio::test]
async fn browser_runtime_social_provider_receipt_event_subscriber_keeps_manual_rows_manual_required(
) {
    let report = request_browser_runtime_social_provider_receipt_status_for_input(
        BrowserRuntimeInput::manual_required_fixture(),
    )
    .await
    .unwrap();

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
}

#[test]
fn browser_runtime_social_provider_receipt_topology_covers_named_event_and_subscriber() {
    let manifest = browser_runtime_social_provider_receipt_status_topology_manifest().unwrap();
    assert_eq!(manifest.unready_entries().len(), 0);
    assert_eq!(manifest.entries().len(), 1);

    let entry = manifest.entries().first().unwrap();
    assert_eq!(entry.status, EventTopologyStatus::Covered);
    assert_eq!(
        entry.contract.event_type.as_str(),
        constants::browser::EVENT_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_REQUESTED
    );
    assert_eq!(
        entry.publishers.first().unwrap().as_str(),
        constants::browser::RUNTIME_COMPONENT_BROWSER_SPINE
    );
    let subscriber = entry.subscribers.first().unwrap();
    assert_eq!(
        subscriber.subscriber_id.as_str(),
        constants::browser::SUBSCRIBER_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS
    );
    assert_eq!(
        subscriber.target_handler.as_str(),
        constants::browser::TARGET_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS
    );
}
