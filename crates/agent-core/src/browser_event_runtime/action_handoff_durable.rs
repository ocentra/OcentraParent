use std::collections::BTreeSet;

use ocentra_eventing::{envelope::StoredEventEnvelope, ids::SourceComponent};
use ocentra_parent_agent_protocol::constants;

use super::action_handoff::request_browser_runtime_action_intent_handoff_for_input;
use super::action_handoff::BrowserRuntimeActionIntentHandoffReport;
use super::action_handoff_durable_types::{
    BrowserRuntimeActionIntentDurableHandoffError,
    BrowserRuntimeActionIntentDurableHandoffReadModelState,
    BrowserRuntimeActionIntentDurableHandoffRecord, BrowserRuntimeActionIntentDurableHandoffReport,
};
use super::BrowserRuntimeInput;

pub async fn prove_browser_runtime_action_intent_durable_handoff() -> Result<
    BrowserRuntimeActionIntentDurableHandoffReport,
    BrowserRuntimeActionIntentDurableHandoffError,
> {
    let handoff = request_browser_runtime_action_intent_handoff_for_input(
        BrowserRuntimeInput::dry_run_action_handoff_fixture(),
    )
    .await?;
    if has_unsupported_claims(&handoff) {
        return Err(BrowserRuntimeActionIntentDurableHandoffError::UnsupportedClaim);
    }
    let rows = durable_handoff_rows_from_report(&handoff)?;
    let duplicate_request_event_rejected = duplicate_request_event_rejected(&handoff);
    if !rows_match_handoff_response(&rows, &handoff)
        || !rows_match_request_events(&rows, &handoff.stored_events)
    {
        return Err(BrowserRuntimeActionIntentDurableHandoffError::RowMismatch);
    }
    Ok(BrowserRuntimeActionIntentDurableHandoffReport {
        request_event_count: handoff.stored_events.len(),
        durable_record_count: rows.len(),
        read_model_row_count: rows.len(),
        prepared_not_dispatched_count: rows.len(),
        dispatch_attempt_count: usize::from(handoff.request_report.response.dispatch_attempt_count),
        adapter_execution_count: usize::from(
            handoff.request_report.response.adapter_execution_count,
        ),
        browser_mutation_count: usize::from(handoff.request_report.response.browser_mutation_count),
        child_intervention_execution_count: usize::from(
            handoff
                .request_report
                .response
                .child_intervention_execution_count,
        ),
        final_policy_execution_count: 0,
        enforcement_execution_count: usize::from(
            handoff.request_report.response.enforcement_execution_count,
        ),
        duplicate_request_event_rejected,
        row_matches_handoff_response: true,
        row_matches_request_event: true,
        external_transport_implemented: false,
        adapter_dispatch_claimed: false,
        browser_mutation_claimed: false,
        child_intervention_execution_claimed: false,
        final_policy_execution_claimed: false,
        enforcement_claimed: false,
        rows,
    })
}

fn durable_handoff_rows_from_report(
    handoff: &BrowserRuntimeActionIntentHandoffReport,
) -> Result<
    Vec<BrowserRuntimeActionIntentDurableHandoffRecord>,
    BrowserRuntimeActionIntentDurableHandoffError,
> {
    if handoff.request_report.response.candidate_count == 0 || handoff.stored_events.is_empty() {
        return Err(BrowserRuntimeActionIntentDurableHandoffError::EmptyHandoff);
    }
    assert_unique_request_events(&handoff.stored_events)?;
    handoff
        .stored_events
        .iter()
        .enumerate()
        .map(|(index, event)| durable_handoff_row_from_event(index, event, handoff))
        .collect()
}

fn durable_handoff_row_from_event(
    index: usize,
    event: &StoredEventEnvelope,
    handoff: &BrowserRuntimeActionIntentHandoffReport,
) -> Result<
    BrowserRuntimeActionIntentDurableHandoffRecord,
    BrowserRuntimeActionIntentDurableHandoffError,
> {
    let response = &handoff.request_report.response;
    let Ok(sequence) = u64::try_from(index) else {
        return Err(BrowserRuntimeActionIntentDurableHandoffError::RowMismatch);
    };
    Ok(BrowserRuntimeActionIntentDurableHandoffRecord {
        sequence: sequence.saturating_add(1),
        request_event_id: event.event_id.clone(),
        request_event_type: event.contract.event_type.clone(),
        correlation_id: event.correlation_id.clone(),
        state: BrowserRuntimeActionIntentDurableHandoffReadModelState::PreparedNotDispatched,
        policy_preview_id: response
            .policy_preview_id
            .clone()
            .ok_or(BrowserRuntimeActionIntentDurableHandoffError::MissingHandoffRef)?,
        action_intent_id: response
            .action_intent_id
            .clone()
            .ok_or(BrowserRuntimeActionIntentDurableHandoffError::MissingHandoffRef)?,
        source_event_ref: source_component(
            response
                .source_event_ref
                .as_deref()
                .ok_or(BrowserRuntimeActionIntentDurableHandoffError::MissingHandoffRef)?,
        )?,
        durable_result_ref: source_component(
            constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_DURABLE_RESULT_REF,
        )?,
        durable_store_ref: source_component(
            constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_DURABLE_STORE_REF,
        )?,
        outbox_ref: source_component(
            response
                .outbox_ref
                .as_deref()
                .ok_or(BrowserRuntimeActionIntentDurableHandoffError::MissingHandoffRef)?,
        )?,
        handoff_ref: source_component(
            response
                .handoff_ref
                .as_deref()
                .ok_or(BrowserRuntimeActionIntentDurableHandoffError::MissingHandoffRef)?,
        )?,
        read_model_ref: source_component(
            constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_READ_MODEL_REF,
        )?,
        support_status_ref: source_component(
            constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_SUPPORT_STATUS_REF,
        )?,
        source_ref: source_component(&response.source_ref)?,
        evidence_ref: source_component(&response.evidence_ref)?,
    })
}

fn assert_unique_request_events(
    events: &[StoredEventEnvelope],
) -> Result<(), BrowserRuntimeActionIntentDurableHandoffError> {
    let mut event_ids = BTreeSet::new();
    for event in events {
        if !event_ids.insert(event.event_id.as_str().to_string()) {
            return Err(BrowserRuntimeActionIntentDurableHandoffError::DuplicateRequestEvent);
        }
    }
    Ok(())
}

fn duplicate_request_event_rejected(handoff: &BrowserRuntimeActionIntentHandoffReport) -> bool {
    let Some(first_event) = handoff.stored_events.first() else {
        return false;
    };
    let mut duplicated = handoff.stored_events.clone();
    duplicated.push(first_event.clone());
    let duplicate_report = BrowserRuntimeActionIntentHandoffReport {
        request_report: handoff.request_report.clone(),
        stored_events: duplicated,
        dead_letters: handoff.dead_letters.clone(),
    };
    matches!(
        durable_handoff_rows_from_report(&duplicate_report),
        Err(BrowserRuntimeActionIntentDurableHandoffError::DuplicateRequestEvent)
    )
}

fn rows_match_handoff_response(
    rows: &[BrowserRuntimeActionIntentDurableHandoffRecord],
    handoff: &BrowserRuntimeActionIntentHandoffReport,
) -> bool {
    let response = &handoff.request_report.response;
    rows.iter().all(|row| {
        response.policy_preview_id.as_deref() == Some(row.policy_preview_id.as_str())
            && response.action_intent_id.as_deref() == Some(row.action_intent_id.as_str())
            && response.source_event_ref.as_deref() == Some(row.source_event_ref.as_str())
            && response.outbox_ref.as_deref() == Some(row.outbox_ref.as_str())
            && response.handoff_ref.as_deref() == Some(row.handoff_ref.as_str())
            && response.source_ref == row.source_ref.as_str()
            && response.evidence_ref == row.evidence_ref.as_str()
    })
}

fn rows_match_request_events(
    rows: &[BrowserRuntimeActionIntentDurableHandoffRecord],
    events: &[StoredEventEnvelope],
) -> bool {
    rows.len() == events.len()
        && rows.iter().zip(events.iter()).all(|(row, event)| {
            row.request_event_id == event.event_id
                && row.request_event_type == event.contract.event_type
                && row.correlation_id == event.correlation_id
                && row.request_event_type.as_str()
                    == constants::browser::EVENT_BROWSER_ACTION_INTENT_HANDOFF_REQUESTED
        })
}

fn has_unsupported_claims(handoff: &BrowserRuntimeActionIntentHandoffReport) -> bool {
    let response = &handoff.request_report.response;
    response.dispatch_attempt_count > 0
        || response.adapter_execution_count > 0
        || response.browser_mutation_count > 0
        || response.child_intervention_execution_count > 0
        || response.enforcement_execution_count > 0
        || !response.dry_run_only
        || !response.policy_authority_only
}

fn source_component(
    value: &str,
) -> Result<SourceComponent, BrowserRuntimeActionIntentDurableHandoffError> {
    SourceComponent::parse(value).map_err(BrowserRuntimeActionIntentDurableHandoffError::Eventing)
}
