use std::collections::BTreeSet;

use ocentra_eventing::{envelope::StoredEventEnvelope, ids::SourceComponent};
use ocentra_parent_agent_protocol::constants;

use super::social_provider_receipt::{
    request_browser_runtime_social_provider_receipt_status_for_input,
    BrowserRuntimeSocialProviderReceiptStatusReport,
};
use super::social_provider_receipt_durable_types::{
    BrowserRuntimeSocialProviderReceiptDurableError,
    BrowserRuntimeSocialProviderReceiptDurableReadModelState,
    BrowserRuntimeSocialProviderReceiptDurableRecord,
    BrowserRuntimeSocialProviderReceiptDurableReport,
};
use super::BrowserRuntimeInput;

pub async fn prove_browser_runtime_social_provider_receipt_durable() -> Result<
    BrowserRuntimeSocialProviderReceiptDurableReport,
    BrowserRuntimeSocialProviderReceiptDurableError,
> {
    let receipt = request_browser_runtime_social_provider_receipt_status_for_input(
        BrowserRuntimeInput::dry_run_action_handoff_fixture(),
    )
    .await?;
    if has_unsupported_claims(&receipt) {
        return Err(BrowserRuntimeSocialProviderReceiptDurableError::UnsupportedClaim);
    }
    let rows = durable_receipt_rows_from_report(&receipt)?;
    let duplicate_request_event_rejected = duplicate_request_event_rejected(&receipt);
    if !rows_match_receipt_response(&rows, &receipt)
        || !rows_match_request_events(&rows, &receipt.stored_events)
    {
        return Err(BrowserRuntimeSocialProviderReceiptDurableError::RowMismatch);
    }
    let response = &receipt.request_report.response;
    Ok(BrowserRuntimeSocialProviderReceiptDurableReport {
        request_event_count: receipt.stored_events.len(),
        durable_record_count: rows.len(),
        read_model_row_count: rows.len(),
        provider_dispatch_required_count: response.provider_dispatch_required_count,
        manual_receipt_required_count: response.manual_receipt_required_count,
        provider_receipt_count: usize::from(response.provider_receipt_count),
        provider_dispatch_count: usize::from(response.provider_dispatch_count),
        connector_native_runtime_count: usize::from(response.connector_native_runtime_count),
        parent_notification_ui_delivery_count: usize::from(
            response.parent_notification_ui_delivery_count,
        ),
        report_delivery_execution_count: usize::from(response.report_delivery_execution_count),
        final_policy_execution_count: usize::from(response.final_policy_execution_count),
        enforcement_execution_count: usize::from(response.enforcement_execution_count),
        duplicate_request_event_rejected,
        row_matches_receipt_response: true,
        row_matches_request_event: true,
        provider_receipt_claimed: false,
        provider_dispatch_claimed: false,
        connector_native_runtime_claimed: false,
        parent_notification_ui_delivery_claimed: false,
        report_delivery_execution_claimed: false,
        final_policy_execution_claimed: false,
        enforcement_claimed: false,
        rows,
    })
}

fn durable_receipt_rows_from_report(
    receipt: &BrowserRuntimeSocialProviderReceiptStatusReport,
) -> Result<
    Vec<BrowserRuntimeSocialProviderReceiptDurableRecord>,
    BrowserRuntimeSocialProviderReceiptDurableError,
> {
    let response = &receipt.request_report.response;
    if response.provider_dispatch_required_count == 0 || receipt.stored_events.is_empty() {
        return Err(BrowserRuntimeSocialProviderReceiptDurableError::EmptyReceipt);
    }
    assert_unique_request_events(&receipt.stored_events)?;
    receipt
        .stored_events
        .iter()
        .enumerate()
        .map(|(index, event)| durable_receipt_row_from_event(index, event, receipt))
        .collect()
}

fn durable_receipt_row_from_event(
    index: usize,
    event: &StoredEventEnvelope,
    receipt: &BrowserRuntimeSocialProviderReceiptStatusReport,
) -> Result<
    BrowserRuntimeSocialProviderReceiptDurableRecord,
    BrowserRuntimeSocialProviderReceiptDurableError,
> {
    let response = &receipt.request_report.response;
    let Ok(sequence) = u64::try_from(index) else {
        return Err(BrowserRuntimeSocialProviderReceiptDurableError::RowMismatch);
    };
    Ok(BrowserRuntimeSocialProviderReceiptDurableRecord {
        sequence: sequence.saturating_add(1),
        request_event_id: event.event_id.clone(),
        request_event_type: event.contract.event_type.clone(),
        correlation_id: event.correlation_id.clone(),
        state:
            BrowserRuntimeSocialProviderReceiptDurableReadModelState::ProviderDispatchRequiredManualReceipt,
        action_intent_id: response
            .action_intent_id
            .clone()
            .ok_or(BrowserRuntimeSocialProviderReceiptDurableError::MissingReceiptRef)?,
        provider_attempt_ref: source_component(
            response
                .provider_attempt_ref
                .as_deref()
                .ok_or(BrowserRuntimeSocialProviderReceiptDurableError::MissingReceiptRef)?,
        )?,
        provider_receipt_proof_ref: source_component(
            response.provider_receipt_proof_ref.as_deref().ok_or(
                BrowserRuntimeSocialProviderReceiptDurableError::MissingReceiptRef,
            )?,
        )?,
        durable_result_ref: source_component(
            constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_DURABLE_RESULT_REF,
        )?,
        durable_store_ref: source_component(
            constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_DURABLE_STORE_REF,
        )?,
        read_model_ref: source_component(
            constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_READ_MODEL_REF,
        )?,
        support_status_ref: source_component(
            constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_SUPPORT_STATUS_REF,
        )?,
        source_ref: source_component(&response.source_ref)?,
        evidence_ref: source_component(&response.evidence_ref)?,
    })
}

fn assert_unique_request_events(
    events: &[StoredEventEnvelope],
) -> Result<(), BrowserRuntimeSocialProviderReceiptDurableError> {
    let mut event_ids = BTreeSet::new();
    for event in events {
        if !event_ids.insert(event.event_id.as_str().to_string()) {
            return Err(BrowserRuntimeSocialProviderReceiptDurableError::DuplicateRequestEvent);
        }
    }
    Ok(())
}

fn duplicate_request_event_rejected(
    receipt: &BrowserRuntimeSocialProviderReceiptStatusReport,
) -> bool {
    let Some(first_event) = receipt.stored_events.first() else {
        return false;
    };
    let mut duplicated = receipt.stored_events.clone();
    duplicated.push(first_event.clone());
    let duplicate_report = BrowserRuntimeSocialProviderReceiptStatusReport {
        request_report: receipt.request_report.clone(),
        stored_events: duplicated,
        dead_letters: receipt.dead_letters.clone(),
    };
    matches!(
        durable_receipt_rows_from_report(&duplicate_report),
        Err(BrowserRuntimeSocialProviderReceiptDurableError::DuplicateRequestEvent)
    )
}

fn rows_match_receipt_response(
    rows: &[BrowserRuntimeSocialProviderReceiptDurableRecord],
    receipt: &BrowserRuntimeSocialProviderReceiptStatusReport,
) -> bool {
    let response = &receipt.request_report.response;
    rows.iter().all(|row| {
        response.action_intent_id.as_deref() == Some(row.action_intent_id.as_str())
            && response.provider_attempt_ref.as_deref() == Some(row.provider_attempt_ref.as_str())
            && response.provider_receipt_proof_ref.as_deref()
                == Some(row.provider_receipt_proof_ref.as_str())
            && response.source_ref == row.source_ref.as_str()
            && response.evidence_ref == row.evidence_ref.as_str()
            && response.receipt_boundary_state
                == constants::browser::SOCIAL_PROVIDER_RECEIPT_STATE_PROVIDER_DISPATCH_REQUIRED
            && response.receipt_runtime_state
                == constants::browser::SOCIAL_PROVIDER_RECEIPT_RUNTIME_STATE_MANUAL_REQUIRED
    })
}

fn rows_match_request_events(
    rows: &[BrowserRuntimeSocialProviderReceiptDurableRecord],
    events: &[StoredEventEnvelope],
) -> bool {
    rows.len() == events.len()
        && rows.iter().zip(events.iter()).all(|(row, event)| {
            row.request_event_id == event.event_id
                && row.request_event_type == event.contract.event_type
                && row.correlation_id == event.correlation_id
                && row.request_event_type.as_str()
                    == constants::browser::EVENT_BROWSER_SOCIAL_PROVIDER_RECEIPT_STATUS_REQUESTED
        })
}

fn has_unsupported_claims(receipt: &BrowserRuntimeSocialProviderReceiptStatusReport) -> bool {
    let response = &receipt.request_report.response;
    response.provider_receipt_count > 0
        || response.provider_dispatch_count > 0
        || response.provider_webhook_count > 0
        || response.provider_credentials_count > 0
        || response.parent_notification_ui_delivery_count > 0
        || response.report_delivery_execution_count > 0
        || response.final_policy_execution_count > 0
        || response.connector_native_runtime_count > 0
        || response.enforcement_execution_count > 0
}

fn source_component(
    value: &str,
) -> Result<SourceComponent, BrowserRuntimeSocialProviderReceiptDurableError> {
    SourceComponent::parse(value).map_err(BrowserRuntimeSocialProviderReceiptDurableError::Eventing)
}
