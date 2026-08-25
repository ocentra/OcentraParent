#[path = "browser_runtime_stream_payload_impl.rs"]
mod browser_runtime_stream_payload_impl;
#[path = "browser_runtime_stream_publication.rs"]
mod browser_runtime_stream_publication;

use ocentra_eventing::bus::reports::handler::EventConsumerOutcome;
use ocentra_parent_agent_core::browser_event_runtime::{
    BrowserRuntimeActionIntentHandoffResponse, BrowserRuntimeActionIntentStatusResponse,
    BrowserRuntimeReport, BrowserRuntimeSocialProviderReceiptStatusResponse,
};
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyPreviewReadModel;
use ocentra_parent_agent_protocol::browser_read_model::BrowserEvidenceReadModel;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use serde::{Deserialize, Serialize};

use crate::browser_runtime_stream_api::BrowserRuntimeActionIntentChildStatusResponse;
use crate::browser_runtime_stream_events::BrowserRuntimeServiceStreamEntry;
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
    self::browser_runtime_stream_payload_impl::stream_browser_runtime_event_chain_for_read_model_with_policy_preview(
        read_model,
        policy_preview,
    )
    .await
}
pub(crate) fn browser_runtime_event_chain_stream_payload(
    report: &BrowserRuntimeServiceStreamReport,
) -> LogFields {
    let child_report = browser_runtime_stream_payload_impl::BrowserRuntimeServiceStreamReport {
        observed_rows: report.observed_rows,
        streamed_events: report.streamed_events,
        failed_rows: report.failed_rows,
        exact_url_rows: report.exact_url_rows,
        manual_required_rows: report.manual_required_rows,
        intervention_command_events: report.intervention_command_events,
        read_model_projection_events: report.read_model_projection_events,
        action_intent_candidates: report.action_intent_candidates,
        action_intent_dispatch_attempts: report.action_intent_dispatch_attempts,
        action_intent_adapter_executions: report.action_intent_adapter_executions,
        action_intent_child_intervention_executions: report
            .action_intent_child_intervention_executions,
        action_intent_enforcement_executions: report.action_intent_enforcement_executions,
        action_intent_handoff_candidates: report.action_intent_handoff_candidates,
        action_intent_handoff_outbox_refs: report.action_intent_handoff_outbox_refs.clone(),
        action_intent_handoff_refs: report.action_intent_handoff_refs.clone(),
        action_intent_child_accepted_rows: report.action_intent_child_accepted_rows,
        action_intent_child_command_refs: report.action_intent_child_command_refs.clone(),
        action_intent_child_accepted_event_refs: report
            .action_intent_child_accepted_event_refs
            .clone(),
        action_intent_parent_read_model_refs: report.action_intent_parent_read_model_refs.clone(),
        social_provider_receipt_boundary_rows: report.social_provider_receipt_boundary_rows,
        social_provider_dispatch_required_rows: report.social_provider_dispatch_required_rows,
        social_provider_manual_receipt_required_rows: report
            .social_provider_manual_receipt_required_rows,
        social_provider_attempt_refs: report.social_provider_attempt_refs.clone(),
        social_provider_receipt_proof_refs: report.social_provider_receipt_proof_refs.clone(),
        social_provider_durable_rows: report.social_provider_durable_rows,
        social_provider_durable_result_refs: report.social_provider_durable_result_refs.clone(),
        social_provider_durable_store_refs: report.social_provider_durable_store_refs.clone(),
        social_provider_read_model_refs: report.social_provider_read_model_refs.clone(),
        social_provider_support_status_refs: report.social_provider_support_status_refs.clone(),
        entries: report.entries.clone(),
    };
    browser_runtime_stream_payload_impl::browser_runtime_event_chain_stream_payload(&child_report)
}

impl From<browser_runtime_stream_payload_impl::BrowserRuntimeServiceStreamReport>
    for BrowserRuntimeServiceStreamReport
{
    fn from(
        report: browser_runtime_stream_payload_impl::BrowserRuntimeServiceStreamReport,
    ) -> Self {
        Self {
            observed_rows: report.observed_rows,
            streamed_events: report.streamed_events,
            failed_rows: report.failed_rows,
            exact_url_rows: report.exact_url_rows,
            manual_required_rows: report.manual_required_rows,
            intervention_command_events: report.intervention_command_events,
            read_model_projection_events: report.read_model_projection_events,
            action_intent_candidates: report.action_intent_candidates,
            action_intent_dispatch_attempts: report.action_intent_dispatch_attempts,
            action_intent_adapter_executions: report.action_intent_adapter_executions,
            action_intent_child_intervention_executions: report
                .action_intent_child_intervention_executions,
            action_intent_enforcement_executions: report.action_intent_enforcement_executions,
            action_intent_handoff_candidates: report.action_intent_handoff_candidates,
            action_intent_handoff_outbox_refs: report.action_intent_handoff_outbox_refs,
            action_intent_handoff_refs: report.action_intent_handoff_refs,
            action_intent_child_accepted_rows: report.action_intent_child_accepted_rows,
            action_intent_child_command_refs: report.action_intent_child_command_refs,
            action_intent_child_accepted_event_refs: report.action_intent_child_accepted_event_refs,
            action_intent_parent_read_model_refs: report.action_intent_parent_read_model_refs,
            social_provider_receipt_boundary_rows: report.social_provider_receipt_boundary_rows,
            social_provider_dispatch_required_rows: report.social_provider_dispatch_required_rows,
            social_provider_manual_receipt_required_rows: report
                .social_provider_manual_receipt_required_rows,
            social_provider_attempt_refs: report.social_provider_attempt_refs,
            social_provider_receipt_proof_refs: report.social_provider_receipt_proof_refs,
            social_provider_durable_rows: report.social_provider_durable_rows,
            social_provider_durable_result_refs: report.social_provider_durable_result_refs,
            social_provider_durable_store_refs: report.social_provider_durable_store_refs,
            social_provider_read_model_refs: report.social_provider_read_model_refs,
            social_provider_support_status_refs: report.social_provider_support_status_refs,
            entries: report.entries,
        }
    }
}

impl BrowserRuntimeServiceStreamReport {
    pub(crate) fn record_publication(&mut self, report: &BrowserRuntimeReport) -> bool {
        if report.publish_reports.is_empty()
            || report
                .publish_reports
                .iter()
                .any(|publish| publish.consumer_outcome() != EventConsumerOutcome::Handled)
        {
            self.failed_rows += 1;
            return false;
        }
        let entries = crate::browser_runtime_stream_events::stream_entries_from_report(report);
        self.streamed_events += entries.len();
        self.intervention_command_events += entries
            .iter()
            .filter(|entry| {
                entry.runtime_event_name.0
                    == constants::browser::EVENT_BROWSER_INTERVENTION_COMMAND_ISSUED
            })
            .count();
        self.read_model_projection_events += entries
            .iter()
            .filter(|entry| {
                entry.runtime_event_name.0 == constants::browser::EVENT_BROWSER_READ_MODEL_PROJECTED
            })
            .count();
        self.entries.extend(entries);
        true
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
        self.action_intent_handoff_outbox_refs
            .extend(handoff.outbox_ref.iter().cloned());
        self.action_intent_handoff_refs
            .extend(handoff.handoff_ref.iter().cloned());
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
        self.action_intent_child_command_refs
            .extend(status.child_command_ref.iter().cloned());
        self.action_intent_child_accepted_event_refs
            .extend(status.child_accepted_event_ref.iter().cloned());
        self.action_intent_parent_read_model_refs
            .extend(status.parent_read_model_ref.iter().cloned());
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
        self.social_provider_attempt_refs
            .extend(receipt.provider_attempt_ref.iter().cloned());
        self.social_provider_receipt_proof_refs
            .extend(receipt.provider_receipt_proof_ref.iter().cloned());
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
