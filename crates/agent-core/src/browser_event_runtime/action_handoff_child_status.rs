use ocentra_parent_agent_protocol::child_agent::child_agent_events::{
    ChildCommandAcceptedEvent, ChildCommandKind, ChildCommandReceivedEvent,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::parent_controller_events::ParentReadModelProjectedEvent;

use super::action_handoff_child_status_types::{
    BrowserRuntimeActionIntentChildStatusError,
    BrowserRuntimeActionIntentChildStatusReadModelState,
    BrowserRuntimeActionIntentChildStatusRecord, BrowserRuntimeActionIntentChildStatusReport,
};
use crate::parent_child_event_runtime::publish_parent_child_runtime_for_validated_intent;
use ocentra_parent_agent_protocol::transport::parent_child_runtime_input::ParentChildRuntimeInput;
use ocentra_parent_agent_protocol::transport::ParentChildRuntimeEventPayload;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct BrowserRuntimeActionIntentExecutionCounts {
    dispatch_attempt_count: usize,
    adapter_execution_count: usize,
    browser_mutation_count: usize,
    child_intervention_execution_count: usize,
    final_policy_execution_count: usize,
    enforcement_execution_count: usize,
}

pub async fn prove_browser_runtime_action_intent_child_status(
) -> Result<BrowserRuntimeActionIntentChildStatusReport, BrowserRuntimeActionIntentChildStatusError>
{
    let durable =
        super::action_handoff_durable::prove_browser_runtime_action_intent_durable_handoff()
            .await
            .map_err(BrowserRuntimeActionIntentChildStatusError::Handoff)?;
    if durable.rows.len() != 1 || has_unsupported_claims(&durable) {
        return Err(BrowserRuntimeActionIntentChildStatusError::UnsupportedClaim);
    }
    let durable_row = durable
        .rows
        .first()
        .ok_or(BrowserRuntimeActionIntentChildStatusError::MissingPayload)?;
    let child_report = publish_parent_child_runtime_for_validated_intent(
        ParentChildRuntimeInput::browser_action_intent_handoff_fixture(),
    )
    .await
    .map_err(BrowserRuntimeActionIntentChildStatusError::ParentChildRuntime)?;
    let payloads = child_report
        .stored_events
        .iter()
        .map(|event| {
            event
                .decode::<ParentChildRuntimeEventPayload>()
                .map(|envelope| envelope.into_payload())
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(BrowserRuntimeActionIntentChildStatusError::PayloadDecode)?;
    let received = child_command_received(&payloads)?;
    let accepted = child_command_accepted(&payloads)?;
    let parent_read_model = parent_read_model_projected(&payloads)?;

    if !handoff_matches_child_status(durable_row, &received, &accepted, &parent_read_model) {
        return Err(BrowserRuntimeActionIntentChildStatusError::HandoffMismatch);
    }

    let report = child_status_report_from_events(
        durable.durable_record_count,
        BrowserRuntimeActionIntentExecutionCounts {
            dispatch_attempt_count: durable.dispatch_attempt_count,
            adapter_execution_count: durable.adapter_execution_count,
            browser_mutation_count: durable.browser_mutation_count,
            child_intervention_execution_count: durable.child_intervention_execution_count,
            final_policy_execution_count: durable.final_policy_execution_count,
            enforcement_execution_count: durable.enforcement_execution_count,
        },
        &received,
        &accepted,
        &parent_read_model,
        BrowserRuntimeActionIntentChildStatusRecord {
            policy_preview_id: durable_row.policy_preview_id.clone(),
            action_intent_id: durable_row.action_intent_id.clone(),
            durable_result_ref: durable_row.durable_result_ref.as_str().to_string(),
            durable_read_model_ref: durable_row.read_model_ref.as_str().to_string(),
            outbox_ref: durable_row.outbox_ref.as_str().to_string(),
            handoff_ref: durable_row.handoff_ref.as_str().to_string(),
            child_command_ref: String::new(),
            child_command_received_event_ref: String::new(),
            child_command_accepted_event_ref: String::new(),
            parent_read_model_ref: String::new(),
            parent_read_model_projected_event_ref: String::new(),
            state: BrowserRuntimeActionIntentChildStatusReadModelState::ChildAcceptedNotExecuted,
        },
    );
    Ok(report)
}

fn child_status_report_from_events(
    handoff_candidate_count: usize,
    execution_counts: BrowserRuntimeActionIntentExecutionCounts,
    received: &ChildCommandReceivedEvent,
    accepted: &ChildCommandAcceptedEvent,
    parent_read_model: &ParentReadModelProjectedEvent,
    mut row: BrowserRuntimeActionIntentChildStatusRecord,
) -> BrowserRuntimeActionIntentChildStatusReport {
    row.child_command_ref = received.child_command_ref.clone();
    row.child_command_received_event_ref = received.command_received_event_ref.clone();
    row.child_command_accepted_event_ref = accepted.command_accepted_event_ref.clone();
    row.parent_read_model_ref = parent_read_model.read_model_ref.clone();
    row.parent_read_model_projected_event_ref =
        parent_read_model.read_model_projected_event_ref.clone();
    BrowserRuntimeActionIntentChildStatusReport {
        handoff_candidate_count,
        child_command_received_count: usize::from(
            received.command_kind == ChildCommandKind::BrowserActionIntentHandoff,
        ),
        child_command_accepted_count: 1,
        parent_read_model_row_count: usize::from(parent_read_model.visible_to_portal),
        child_accepted_not_executed_count: 1,
        handoff_refs_match_durable_record: true,
        child_command_matches_handoff: true,
        parent_read_model_visible: parent_read_model.visible_to_portal,
        dispatch_attempt_count: execution_counts.dispatch_attempt_count,
        adapter_execution_count: execution_counts.adapter_execution_count,
        browser_mutation_count: execution_counts.browser_mutation_count,
        child_intervention_execution_count: execution_counts.child_intervention_execution_count,
        final_policy_execution_count: execution_counts.final_policy_execution_count,
        enforcement_execution_count: execution_counts.enforcement_execution_count,
        public_stream_field_registry_ready: true,
        external_transport_implemented: false,
        adapter_dispatch_claimed: false,
        browser_mutation_claimed: false,
        child_intervention_execution_claimed: false,
        final_policy_execution_claimed: false,
        enforcement_claimed: false,
        rows: vec![row],
    }
}

fn child_command_received(
    payloads: &[ParentChildRuntimeEventPayload],
) -> Result<ChildCommandReceivedEvent, BrowserRuntimeActionIntentChildStatusError> {
    payloads
        .iter()
        .find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ChildCommandReceived(event) => Some(event.clone()),
            _ => None,
        })
        .ok_or(BrowserRuntimeActionIntentChildStatusError::MissingPayload)
}

fn child_command_accepted(
    payloads: &[ParentChildRuntimeEventPayload],
) -> Result<ChildCommandAcceptedEvent, BrowserRuntimeActionIntentChildStatusError> {
    payloads
        .iter()
        .find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ChildCommandAccepted(event) => Some(event.clone()),
            _ => None,
        })
        .ok_or(BrowserRuntimeActionIntentChildStatusError::MissingPayload)
}

fn parent_read_model_projected(
    payloads: &[ParentChildRuntimeEventPayload],
) -> Result<ParentReadModelProjectedEvent, BrowserRuntimeActionIntentChildStatusError> {
    payloads
        .iter()
        .find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ParentReadModelProjected(event) => Some(event.clone()),
            _ => None,
        })
        .ok_or(BrowserRuntimeActionIntentChildStatusError::MissingPayload)
}

fn handoff_matches_child_status(
    durable_row: &super::action_handoff_durable_types::BrowserRuntimeActionIntentDurableHandoffRecord,
    received: &ChildCommandReceivedEvent,
    accepted: &ChildCommandAcceptedEvent,
    parent_read_model: &ParentReadModelProjectedEvent,
) -> bool {
    durable_row.action_intent_id == constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_ID
        && durable_row.handoff_ref.as_str()
            == constants::browser::TEST_BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REF
        && received.command_kind == ChildCommandKind::BrowserActionIntentHandoff
        && received.child_command_ref == accepted.child_command_ref
        && accepted.causation_event_ref == received.command_received_event_ref
        && parent_read_model.visible_to_portal
}

fn has_unsupported_claims(
    durable: &super::action_handoff_durable_types::BrowserRuntimeActionIntentDurableHandoffReport,
) -> bool {
    durable.dispatch_attempt_count > 0
        || durable.adapter_execution_count > 0
        || durable.browser_mutation_count > 0
        || durable.child_intervention_execution_count > 0
        || durable.final_policy_execution_count > 0
        || durable.enforcement_execution_count > 0
        || durable.external_transport_implemented
        || durable.adapter_dispatch_claimed
        || durable.browser_mutation_claimed
        || durable.child_intervention_execution_claimed
        || durable.final_policy_execution_claimed
        || durable.enforcement_claimed
}
