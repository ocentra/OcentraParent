use ocentra_parent_agent_core::{
    publish_browser_runtime_chain_for_input,
    request_browser_runtime_action_intent_handoff_for_input,
    request_browser_runtime_action_intent_status_for_input,
    request_browser_runtime_social_provider_receipt_status_for_input,
    BrowserRuntimeActionIntentHandoffResponse, BrowserRuntimeActionIntentStatusResponse,
    BrowserRuntimeReport, BrowserRuntimeSocialProviderReceiptStatusResponse,
};
use ocentra_parent_agent_protocol::{
    constants, BrowserEvidenceReadModel, LogFieldValue, LogFields, PolicyPreviewReadModel,
};

use crate::{
    browser_runtime_delivery::{
        browser_runtime_input_from_row, browser_runtime_input_from_row_with_policy_preview,
    },
    browser_runtime_stream_events::{stream_entries_from_report, BrowserRuntimeServiceStreamEntry},
    fields::fields_from_pairs,
};

#[derive(Clone, Debug, Default, PartialEq)]
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
    pub(crate) social_provider_receipt_boundary_rows: usize,
    pub(crate) social_provider_dispatch_required_rows: usize,
    pub(crate) social_provider_manual_receipt_required_rows: usize,
    pub(crate) social_provider_attempt_refs: Vec<String>,
    pub(crate) social_provider_receipt_proof_refs: Vec<String>,
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
            stream.record_action_intent_handoff(&report.request_report.response);
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
    fields_from_pairs(vec![
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
        (
            constants::field::BROWSER_RUNTIME_EVENT_CHAIN_STREAM,
            LogFieldValue::String(
                serde_json::to_string(&report.entries)
                    .expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
    ])
}

impl BrowserRuntimeServiceStreamReport {
    fn record_success(&mut self, report: &BrowserRuntimeReport) {
        let entries = stream_entries_from_report(report);
        self.streamed_events += entries.len();
        self.intervention_command_events += entries
            .iter()
            .filter(|entry| {
                entry.event_type == constants::browser::EVENT_BROWSER_INTERVENTION_COMMAND_ISSUED
            })
            .count();
        self.read_model_projection_events += entries
            .iter()
            .filter(|entry| {
                entry.event_type == constants::browser::EVENT_BROWSER_READ_MODEL_PROJECTED
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
    LogFieldValue::String(
        serde_json::to_string(values).expect(constants::error::AGENT_EVENT_SERIALIZES),
    )
}
