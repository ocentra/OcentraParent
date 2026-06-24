use ocentra_parent_agent_core::{
    browser_event_runtime::{
        publish_browser_runtime_chain_for_input,
        request_browser_runtime_action_intent_handoff_for_input,
        request_browser_runtime_action_intent_status_for_input,
        request_browser_runtime_social_provider_receipt_status_for_input,
        BrowserRuntimeActionIntentHandoffResponse, BrowserRuntimeActionIntentStatusResponse,
        BrowserRuntimeReport, BrowserRuntimeSocialProviderReceiptStatusResponse,
    },
    parent_child_event_runtime::publish_parent_child_runtime_for_validated_intent,
};
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyPreviewReadModel;
use ocentra_parent_agent_protocol::browser_read_model::BrowserEvidenceReadModel;
use ocentra_parent_agent_protocol::child_agent::child_agent_events::ChildCommandKind;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::parent_controller_events::{
    ParentControllerActionKind, ParentControllerSource,
};
use ocentra_parent_agent_protocol::transport::parent_child_runtime_input::ParentChildRuntimeInput;
use ocentra_parent_agent_protocol::transport::ParentChildRuntimeEventPayload;
use serde::{Deserialize, Serialize};

use crate::{
    browser_runtime_delivery::{
        browser_runtime_input_from_row, browser_runtime_input_from_row_with_policy_preview,
    },
    browser_runtime_stream_events::{stream_entries_from_report, BrowserRuntimeServiceStreamEntry},
    fields::fields_from_pairs,
};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct BrowserRuntimeServiceStreamReport {
    pub(crate) observed_rows: usize,
    pub(crate) streamed_events: usize,
    pub(crate) failed_rows: usize,
    pub(crate) exact_url_rows: usize,
    pub(crate) manual_required_rows: usize,
    pub(crate) intervention_command_events: usize,
    pub(crate) read_model_projection_events: usize,
    pub(crate) action_intent_candidates: usize,
    pub(crate) action_intent_dispatch_attempts: usize,
    pub(crate) action_intent_adapter_executions: usize,
    pub(crate) action_intent_child_intervention_executions: usize,
    pub(crate) action_intent_enforcement_executions: usize,
    pub(crate) action_intent_handoff_candidates: usize,
    pub(crate) action_intent_handoff_outbox_refs: Vec<String>,
    pub(crate) action_intent_handoff_refs: Vec<String>,
    pub(crate) action_intent_child_accepted_rows: usize,
    pub(crate) action_intent_child_command_refs: Vec<String>,
    pub(crate) action_intent_child_accepted_event_refs: Vec<String>,
    pub(crate) action_intent_parent_read_model_refs: Vec<String>,
    pub(crate) social_provider_receipt_boundary_rows: usize,
    pub(crate) social_provider_dispatch_required_rows: usize,
    pub(crate) social_provider_manual_receipt_required_rows: usize,
    pub(crate) social_provider_attempt_refs: Vec<String>,
    pub(crate) social_provider_receipt_proof_refs: Vec<String>,
    pub(crate) social_provider_durable_rows: usize,
    pub(crate) social_provider_durable_result_refs: Vec<String>,
    pub(crate) social_provider_durable_store_refs: Vec<String>,
    pub(crate) social_provider_read_model_refs: Vec<String>,
    pub(crate) social_provider_support_status_refs: Vec<String>,
    pub(crate) entries: Vec<BrowserRuntimeServiceStreamEntry>,
}

pub(crate) async fn stream_browser_runtime_event_chain_for_read_model_with_policy_preview(
    read_model: &BrowserEvidenceReadModel,
    policy_preview: Option<&PolicyPreviewReadModel>,
) -> BrowserRuntimeServiceStreamReport {
    let mut stream = BrowserRuntimeServiceStreamReport {
        observed_rows: read_model.rows.len(),
        ..BrowserRuntimeServiceStreamReport::default()
    };

    for row in &read_model.rows {
        let input = match policy_preview {
            Some(policy_preview) => browser_runtime_input_from_row_with_policy_preview(
                read_model,
                row,
                Some(policy_preview),
            ),
            None => browser_runtime_input_from_row(read_model, row),
        };
        if input.exact_url_claimed {
            stream.exact_url_rows += 1;
        } else {
            stream.manual_required_rows += 1;
        }
        if let Ok(report) =
            request_browser_runtime_action_intent_status_for_input(input.clone()).await
        {
            stream.record_action_intent_status(&report.request_report.response);
        }
        if let Ok(report) =
            request_browser_runtime_action_intent_handoff_for_input(input.clone()).await
        {
            let handoff = report.request_report.response;
            stream.record_action_intent_handoff(&handoff);
            if let Some(response) = action_intent_child_status_from_handoff(&handoff).await {
                stream.record_action_intent_child_status(&response);
            }
        }
        if let Ok(report) =
            request_browser_runtime_social_provider_receipt_status_for_input(input.clone()).await
        {
            stream.record_social_provider_receipt(&report.request_report.response);
        }
        match publish_browser_runtime_chain_for_input(input).await {
            Ok(report) => stream.record_success(&report),
            Err(_) => stream.failed_rows += 1,
        }
    }

    stream
}

pub(crate) fn browser_runtime_event_chain_stream_payload(
    report: &BrowserRuntimeServiceStreamReport,
) -> LogFields {
    let mut pairs = vec![
        (
            constants::field::BROWSER_RUNTIME_OBSERVED_ROWS,
            count_value(report.observed_rows),
        ),
        (
            constants::field::BROWSER_RUNTIME_STREAMED_EVENTS,
            count_value(report.streamed_events),
        ),
        (
            constants::field::BROWSER_RUNTIME_FAILED_ROWS,
            count_value(report.failed_rows),
        ),
        (
            constants::field::BROWSER_RUNTIME_EXACT_URL_ROWS,
            count_value(report.exact_url_rows),
        ),
        (
            constants::field::BROWSER_RUNTIME_MANUAL_REQUIRED_ROWS,
            count_value(report.manual_required_rows),
        ),
        (
            constants::field::BROWSER_RUNTIME_INTERVENTION_COMMAND_EVENTS,
            count_value(report.intervention_command_events),
        ),
        (
            constants::field::BROWSER_RUNTIME_READ_MODEL_PROJECTION_EVENTS,
            count_value(report.read_model_projection_events),
        ),
        (
            constants::field::BROWSER_RUNTIME_EVENT_CHAIN_STREAM,
            LogFieldValue::String(serialize_stream_json(&report.entries)),
        ),
    ];
    pairs.extend(action_intent_payload_fields(report));
    pairs.extend(social_provider_receipt_payload_fields(report));
    fields_from_pairs(pairs)
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct BrowserRuntimeActionIntentChildStatusResponse {
    pub(crate) accepted_row_count: usize,
    pub(crate) child_command_ref: Option<String>,
    pub(crate) child_accepted_event_ref: Option<String>,
    pub(crate) parent_read_model_ref: Option<String>,
    pub(crate) dispatch_attempt_count: u8,
    pub(crate) adapter_execution_count: u8,
    pub(crate) child_intervention_execution_count: u8,
    pub(crate) enforcement_execution_count: u8,
}

pub(crate) async fn action_intent_child_status_from_handoff(
    handoff: &BrowserRuntimeActionIntentHandoffResponse,
) -> Option<BrowserRuntimeActionIntentChildStatusResponse> {
    if !handoff_is_child_status_candidate(handoff) {
        return Some(BrowserRuntimeActionIntentChildStatusResponse::default());
    }
    let report = publish_parent_child_runtime_for_validated_intent(
        parent_child_input_from_handoff(handoff)?,
    )
    .await
    .ok()?;
    let payloads = report
        .stored_events
        .iter()
        .filter_map(|event| {
            event
                .decode::<ParentChildRuntimeEventPayload>()
                .ok()
                .map(|envelope| envelope.payload)
        })
        .collect::<Vec<_>>();
    child_status_response_from_payloads(handoff, &payloads)
}

fn handoff_is_child_status_candidate(handoff: &BrowserRuntimeActionIntentHandoffResponse) -> bool {
    handoff.candidate_count > 0
        && handoff.dispatch_attempt_count == 0
        && handoff.adapter_execution_count == 0
        && handoff.browser_mutation_count == 0
        && handoff.child_intervention_execution_count == 0
        && handoff.enforcement_execution_count == 0
}

fn parent_child_input_from_handoff(
    handoff: &BrowserRuntimeActionIntentHandoffResponse,
) -> Option<ParentChildRuntimeInput> {
    Some(ParentChildRuntimeInput {
        parent_intent_ref: handoff.action_intent_id.clone()?,
        parent_profile_ref: constants::parent_controller::TEST_PARENT_PROFILE_REF.to_string(),
        device_ref: constants::parent_controller::TEST_DEVICE_REF.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        action_kind: ParentControllerActionKind::Review,
        source: ParentControllerSource::PortalTypedIntent,
        child_command_kind: ChildCommandKind::BrowserActionIntentHandoff,
    })
}

fn child_status_response_from_payloads(
    handoff: &BrowserRuntimeActionIntentHandoffResponse,
    payloads: &[ParentChildRuntimeEventPayload],
) -> Option<BrowserRuntimeActionIntentChildStatusResponse> {
    let child_command_ref = child_command_ref(payloads)?;
    if !child_command_ref.contains(handoff.action_intent_id.as_deref()?) {
        return None;
    }
    Some(BrowserRuntimeActionIntentChildStatusResponse {
        accepted_row_count: 1,
        child_command_ref: Some(child_command_ref),
        child_accepted_event_ref: child_accepted_event_ref(payloads),
        parent_read_model_ref: parent_read_model_ref(payloads),
        ..BrowserRuntimeActionIntentChildStatusResponse::default()
    })
}

fn child_command_ref(payloads: &[ParentChildRuntimeEventPayload]) -> Option<String> {
    payloads.iter().find_map(|payload| match payload {
        ParentChildRuntimeEventPayload::ChildCommandReceived(event)
            if event.command_kind == ChildCommandKind::BrowserActionIntentHandoff =>
        {
            Some(event.child_command_ref.clone())
        }
        _ => None,
    })
}

fn child_accepted_event_ref(payloads: &[ParentChildRuntimeEventPayload]) -> Option<String> {
    payloads.iter().find_map(|payload| match payload {
        ParentChildRuntimeEventPayload::ChildCommandAccepted(event) => {
            Some(event.command_accepted_event_ref.clone())
        }
        _ => None,
    })
}

fn parent_read_model_ref(payloads: &[ParentChildRuntimeEventPayload]) -> Option<String> {
    payloads.iter().find_map(|payload| match payload {
        ParentChildRuntimeEventPayload::ParentReadModelProjected(event)
            if event.visible_to_portal =>
        {
            Some(event.read_model_ref.clone())
        }
        _ => None,
    })
}

fn action_intent_payload_fields(
    report: &BrowserRuntimeServiceStreamReport,
) -> Vec<(&'static str, LogFieldValue)> {
    vec![
        (
            constants::field::BROWSER_RUNTIME_ACTION_INTENT_CANDIDATES,
            count_value(report.action_intent_candidates),
        ),
        (
            constants::field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_CANDIDATES,
            count_value(report.action_intent_handoff_candidates),
        ),
        (
            constants::field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_OUTBOX_REFS,
            string_array_value(&report.action_intent_handoff_outbox_refs),
        ),
        (
            constants::field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REFS,
            string_array_value(&report.action_intent_handoff_refs),
        ),
        (
            constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_ACCEPTED_ROWS,
            count_value(report.action_intent_child_accepted_rows),
        ),
        (
            constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_COMMAND_REFS,
            string_array_value(&report.action_intent_child_command_refs),
        ),
        (
            constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_ACCEPTED_EVENT_REFS,
            string_array_value(&report.action_intent_child_accepted_event_refs),
        ),
        (
            constants::field::BROWSER_RUNTIME_ACTION_INTENT_PARENT_READ_MODEL_REFS,
            string_array_value(&report.action_intent_parent_read_model_refs),
        ),
        (
            constants::field::BROWSER_RUNTIME_ACTION_INTENT_DISPATCH_ATTEMPTS,
            count_value(report.action_intent_dispatch_attempts),
        ),
        (
            constants::field::BROWSER_RUNTIME_ACTION_INTENT_ADAPTER_EXECUTIONS,
            count_value(report.action_intent_adapter_executions),
        ),
        (
            constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_INTERVENTION_EXECUTIONS,
            count_value(report.action_intent_child_intervention_executions),
        ),
        (
            constants::field::BROWSER_RUNTIME_ACTION_INTENT_ENFORCEMENT_EXECUTIONS,
            count_value(report.action_intent_enforcement_executions),
        ),
    ]
}

fn social_provider_receipt_payload_fields(
    report: &BrowserRuntimeServiceStreamReport,
) -> Vec<(&'static str, LogFieldValue)> {
    vec![
        (
            constants::field::BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_BOUNDARY_ROWS,
            count_value(report.social_provider_receipt_boundary_rows),
        ),
        (
            constants::field::BROWSER_RUNTIME_SOCIAL_PROVIDER_DISPATCH_REQUIRED_ROWS,
            count_value(report.social_provider_dispatch_required_rows),
        ),
        (
            constants::field::BROWSER_RUNTIME_SOCIAL_PROVIDER_MANUAL_RECEIPT_REQUIRED_ROWS,
            count_value(report.social_provider_manual_receipt_required_rows),
        ),
        (
            constants::field::BROWSER_RUNTIME_SOCIAL_PROVIDER_ATTEMPT_REFS,
            string_array_value(&report.social_provider_attempt_refs),
        ),
        (
            constants::field::BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_PROOF_REFS,
            string_array_value(&report.social_provider_receipt_proof_refs),
        ),
        (
            constants::field::BROWSER_RUNTIME_SOCIAL_PROVIDER_DURABLE_ROWS,
            count_value(report.social_provider_durable_rows),
        ),
        (
            constants::field::BROWSER_RUNTIME_SOCIAL_PROVIDER_DURABLE_RESULT_REFS,
            string_array_value(&report.social_provider_durable_result_refs),
        ),
        (
            constants::field::BROWSER_RUNTIME_SOCIAL_PROVIDER_DURABLE_STORE_REFS,
            string_array_value(&report.social_provider_durable_store_refs),
        ),
        (
            constants::field::BROWSER_RUNTIME_SOCIAL_PROVIDER_READ_MODEL_REFS,
            string_array_value(&report.social_provider_read_model_refs),
        ),
        (
            constants::field::BROWSER_RUNTIME_SOCIAL_PROVIDER_SUPPORT_STATUS_REFS,
            string_array_value(&report.social_provider_support_status_refs),
        ),
    ]
}

impl BrowserRuntimeServiceStreamReport {
    fn record_success(&mut self, report: &BrowserRuntimeReport) {
        let entries = stream_entries_from_report(report);
        self.streamed_events += entries.len();
        self.intervention_command_events += entries
            .iter()
            .filter(|entry| {
                entry.runtime_event_name
                    == constants::browser::EVENT_BROWSER_INTERVENTION_COMMAND_ISSUED
            })
            .count();
        self.read_model_projection_events += entries
            .iter()
            .filter(|entry| {
                entry.runtime_event_name == constants::browser::EVENT_BROWSER_READ_MODEL_PROJECTED
            })
            .count();
        self.entries.extend(entries);
    }

    pub(crate) fn record_action_intent_status(
        &mut self,
        status: &BrowserRuntimeActionIntentStatusResponse,
    ) {
        self.action_intent_candidates += status.candidate_count;
        self.action_intent_dispatch_attempts += usize::from(status.dispatch_attempt_count);
        self.action_intent_adapter_executions += usize::from(status.adapter_execution_count);
        self.action_intent_child_intervention_executions +=
            usize::from(status.child_intervention_execution_count);
        self.action_intent_enforcement_executions +=
            usize::from(status.enforcement_execution_count);
    }

    pub(crate) fn record_action_intent_handoff(
        &mut self,
        handoff: &BrowserRuntimeActionIntentHandoffResponse,
    ) {
        self.action_intent_handoff_candidates += handoff.candidate_count;
        if let Some(outbox_ref) = &handoff.outbox_ref {
            self.action_intent_handoff_outbox_refs
                .push(outbox_ref.clone());
        }
        if let Some(handoff_ref) = &handoff.handoff_ref {
            self.action_intent_handoff_refs.push(handoff_ref.clone());
        }
        self.action_intent_dispatch_attempts += usize::from(handoff.dispatch_attempt_count);
        self.action_intent_adapter_executions += usize::from(handoff.adapter_execution_count);
        self.action_intent_child_intervention_executions +=
            usize::from(handoff.child_intervention_execution_count);
        self.action_intent_enforcement_executions +=
            usize::from(handoff.enforcement_execution_count);
    }

    pub(crate) fn record_action_intent_child_status(
        &mut self,
        status: &BrowserRuntimeActionIntentChildStatusResponse,
    ) {
        self.action_intent_child_accepted_rows += status.accepted_row_count;
        if let Some(child_command_ref) = &status.child_command_ref {
            self.action_intent_child_command_refs
                .push(child_command_ref.clone());
        }
        if let Some(child_accepted_event_ref) = &status.child_accepted_event_ref {
            self.action_intent_child_accepted_event_refs
                .push(child_accepted_event_ref.clone());
        }
        if let Some(parent_read_model_ref) = &status.parent_read_model_ref {
            self.action_intent_parent_read_model_refs
                .push(parent_read_model_ref.clone());
        }
        self.action_intent_dispatch_attempts += usize::from(status.dispatch_attempt_count);
        self.action_intent_adapter_executions += usize::from(status.adapter_execution_count);
        self.action_intent_child_intervention_executions +=
            usize::from(status.child_intervention_execution_count);
        self.action_intent_enforcement_executions +=
            usize::from(status.enforcement_execution_count);
    }

    pub(crate) fn record_social_provider_receipt(
        &mut self,
        receipt: &BrowserRuntimeSocialProviderReceiptStatusResponse,
    ) {
        self.social_provider_receipt_boundary_rows += receipt.receipt_boundary_row_count;
        self.social_provider_dispatch_required_rows += receipt.provider_dispatch_required_count;
        self.social_provider_manual_receipt_required_rows += receipt.manual_receipt_required_count;
        if let Some(provider_attempt_ref) = &receipt.provider_attempt_ref {
            self.social_provider_attempt_refs
                .push(provider_attempt_ref.clone());
        }
        if let Some(provider_receipt_proof_ref) = &receipt.provider_receipt_proof_ref {
            self.social_provider_receipt_proof_refs
                .push(provider_receipt_proof_ref.clone());
        }
        if receipt.provider_dispatch_required_count > 0 {
            self.social_provider_durable_rows += receipt.provider_dispatch_required_count;
            self.social_provider_durable_result_refs.push(
                constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_DURABLE_RESULT_REF
                    .to_string(),
            );
            self.social_provider_durable_store_refs.push(
                constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_DURABLE_STORE_REF
                    .to_string(),
            );
            self.social_provider_read_model_refs.push(
                constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_READ_MODEL_REF
                    .to_string(),
            );
            self.social_provider_support_status_refs.push(
                constants::browser::TEST_BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_SUPPORT_STATUS_REF
                    .to_string(),
            );
        }
        self.action_intent_dispatch_attempts += usize::from(receipt.provider_dispatch_count);
        self.action_intent_adapter_executions +=
            usize::from(receipt.connector_native_runtime_count);
        self.action_intent_child_intervention_executions +=
            usize::from(receipt.parent_notification_ui_delivery_count);
        self.action_intent_enforcement_executions +=
            usize::from(receipt.enforcement_execution_count);
    }
}

fn count_value(value: usize) -> LogFieldValue {
    LogFieldValue::Number(value as f64)
}

fn string_array_value(values: &[String]) -> LogFieldValue {
    LogFieldValue::String(serialize_stream_json(values))
}

fn serialize_stream_json<T>(value: &T) -> String
where
    T: Serialize + ?Sized,
{
    serde_json::to_string(value)
        .unwrap_or_else(|error| panic!("{}: {error:?}", constants::error::AGENT_EVENT_SERIALIZES))
}
