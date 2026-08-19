use ocentra_parent_agent_core::browser_event_runtime::{
    publish_browser_runtime_chain_for_input,
    request_browser_runtime_action_intent_handoff_for_input,
    request_browser_runtime_action_intent_status_for_input,
    request_browser_runtime_social_provider_receipt_status_for_input,
};
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyPreviewReadModel;
use ocentra_parent_agent_protocol::browser_read_model::BrowserEvidenceReadModel;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use serde::{Deserialize, Serialize};

use crate::{
    browser_runtime_delivery::{
        browser_runtime_input_from_row, browser_runtime_input_from_row_with_policy_preview,
    },
    browser_runtime_stream_events::BrowserRuntimeServiceStreamEntry,
    fields::fields_from_pairs,
    json_contract::serialize_json_string,
};

#[derive(Clone, Debug, Default)]
struct BrowserRuntimePairs(Vec<(&'static str, LogFieldValue)>);

#[derive(Clone, Debug, Default)]
struct BrowserRuntimeStrings(Vec<String>);

pub(crate) async fn stream_browser_runtime_event_chain_for_read_model_with_policy_preview(
    read_model: &BrowserEvidenceReadModel,
    policy_preview: Option<&PolicyPreviewReadModel>,
) -> super::BrowserRuntimeServiceStreamReport {
    let mut stream = super::BrowserRuntimeServiceStreamReport {
        observed_rows: read_model.rows.len(),
        ..super::BrowserRuntimeServiceStreamReport::default()
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
        let evidence_requires_manual_review = !input.exact_url_claimed;
        if input.exact_url_claimed {
            stream.exact_url_rows += 1;
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
            if let Some(response) =
                crate::browser_runtime_stream_api::action_intent_child_status_from_handoff(&handoff)
                    .await
            {
                stream.record_action_intent_child_status(&response);
            }
        }
        if let Ok(report) =
            request_browser_runtime_social_provider_receipt_status_for_input(input.clone()).await
        {
            stream.record_social_provider_receipt(&report.request_report.response);
        }
        let publication = publish_browser_runtime_chain_for_input(input).await;
        super::browser_runtime_stream_publication::record_browser_runtime_publication(
            &mut stream,
            publication,
            evidence_requires_manual_review,
        );
    }

    stream
}
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
            LogFieldValue::String(serialize_json_string(&report.entries).0),
        ),
    ];
    pairs.extend(action_intent_payload_fields(report).0);
    pairs.extend(social_provider_receipt_payload_fields(report).0);
    fields_from_pairs(pairs)
}

fn action_intent_payload_fields(report: &BrowserRuntimeServiceStreamReport) -> BrowserRuntimePairs {
    BrowserRuntimePairs(vec![
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
            string_array_value(&BrowserRuntimeStrings(
                report.action_intent_handoff_outbox_refs.clone(),
            )),
        ),
        (
            constants::field::BROWSER_RUNTIME_ACTION_INTENT_HANDOFF_REFS,
            string_array_value(&BrowserRuntimeStrings(
                report.action_intent_handoff_refs.clone(),
            )),
        ),
        (
            constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_ACCEPTED_ROWS,
            count_value(report.action_intent_child_accepted_rows),
        ),
        (
            constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_COMMAND_REFS,
            string_array_value(&BrowserRuntimeStrings(
                report.action_intent_child_command_refs.clone(),
            )),
        ),
        (
            constants::field::BROWSER_RUNTIME_ACTION_INTENT_CHILD_ACCEPTED_EVENT_REFS,
            string_array_value(&BrowserRuntimeStrings(
                report.action_intent_child_accepted_event_refs.clone(),
            )),
        ),
        (
            constants::field::BROWSER_RUNTIME_ACTION_INTENT_PARENT_READ_MODEL_REFS,
            string_array_value(&BrowserRuntimeStrings(
                report.action_intent_parent_read_model_refs.clone(),
            )),
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
    ])
}

fn social_provider_receipt_payload_fields(
    report: &BrowserRuntimeServiceStreamReport,
) -> BrowserRuntimePairs {
    BrowserRuntimePairs(vec![
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
            string_array_value(&BrowserRuntimeStrings(
                report.social_provider_attempt_refs.clone(),
            )),
        ),
        (
            constants::field::BROWSER_RUNTIME_SOCIAL_PROVIDER_RECEIPT_PROOF_REFS,
            string_array_value(&BrowserRuntimeStrings(
                report.social_provider_receipt_proof_refs.clone(),
            )),
        ),
        (
            constants::field::BROWSER_RUNTIME_SOCIAL_PROVIDER_DURABLE_ROWS,
            count_value(report.social_provider_durable_rows),
        ),
        (
            constants::field::BROWSER_RUNTIME_SOCIAL_PROVIDER_DURABLE_RESULT_REFS,
            string_array_value(&BrowserRuntimeStrings(
                report.social_provider_durable_result_refs.clone(),
            )),
        ),
        (
            constants::field::BROWSER_RUNTIME_SOCIAL_PROVIDER_DURABLE_STORE_REFS,
            string_array_value(&BrowserRuntimeStrings(
                report.social_provider_durable_store_refs.clone(),
            )),
        ),
        (
            constants::field::BROWSER_RUNTIME_SOCIAL_PROVIDER_READ_MODEL_REFS,
            string_array_value(&BrowserRuntimeStrings(
                report.social_provider_read_model_refs.clone(),
            )),
        ),
        (
            constants::field::BROWSER_RUNTIME_SOCIAL_PROVIDER_SUPPORT_STATUS_REFS,
            string_array_value(&BrowserRuntimeStrings(
                report.social_provider_support_status_refs.clone(),
            )),
        ),
    ])
}

fn count_value(value: usize) -> LogFieldValue {
    LogFieldValue::Number(value as f64)
}

fn string_array_value(values: &BrowserRuntimeStrings) -> LogFieldValue {
    LogFieldValue::String(serialize_json_string(&values.0).0)
}
