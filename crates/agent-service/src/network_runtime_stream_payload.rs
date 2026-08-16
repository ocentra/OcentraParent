use ocentra_parent_agent_core::network_event_runtime::{
    NetworkRuntimeJournalState, NetworkRuntimeReport,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowReadModel;
use ocentra_parent_agent_protocol::network_flow::NETWORK_FLOW_CUSTODY_PARENT_OWNED_EXPORT;
use ocentra_parent_agent_protocol::network_flow::NETWORK_FLOW_READ_MODEL_FIELD_ACTIVE_ROWS;
use ocentra_parent_agent_protocol::network_flow::NETWORK_FLOW_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS;
use ocentra_parent_agent_protocol::network_flow::NETWORK_FLOW_READ_MODEL_FIELD_EXPORTABLE_ROWS;
use ocentra_parent_agent_protocol::network_flow::NETWORK_FLOW_READ_MODEL_FIELD_EXPORT_CUSTODY;
use ocentra_parent_agent_protocol::network_flow::NETWORK_FLOW_READ_MODEL_FIELD_TOMBSTONE_ROWS;

use crate::{
    fields::fields_from_pairs,
    network_runtime_delivery::{
        network_runtime_observation_from_row, shared_network_runtime_spine,
    },
    network_runtime_stream_events::{stream_entries_from_report, NetworkRuntimeServiceStreamEntry},
};

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct NetworkRuntimeServiceStreamReport {
    pub(crate) observed_rows: usize,
    pub(crate) streamed_events: usize,
    pub(crate) failed_rows: usize,
    pub(crate) manual_required_rows: usize,
    pub(crate) enforcement_command_events: usize,
    pub(crate) journal_state: NetworkRuntimeJournalState,
    pub(crate) active_rows: usize,
    pub(crate) tombstone_rows: usize,
    pub(crate) exportable_rows: usize,
    pub(crate) deleted_evidence_reference_ids: Vec<String>,
    pub(crate) entries: Vec<NetworkRuntimeServiceStreamEntry>,
}

pub(crate) async fn stream_network_runtime_event_chain_for_read_model(
    read_model: &ActivityNetworkFlowReadModel,
) -> NetworkRuntimeServiceStreamReport {
    let mut stream = NetworkRuntimeServiceStreamReport {
        observed_rows: read_model.rows.len(),
        active_rows: read_model.active_rows as usize,
        tombstone_rows: read_model.tombstone_rows as usize,
        exportable_rows: read_model.exportable_rows as usize,
        deleted_evidence_reference_ids: read_model.deleted_evidence_reference_ids.clone(),
        journal_state: NetworkRuntimeJournalState::UnavailableManualRequired,
        ..NetworkRuntimeServiceStreamReport::default()
    };

    let spine = match shared_network_runtime_spine().await {
        Ok(spine) => spine,
        Err(_) => {
            stream.failed_rows = stream.observed_rows;
            return stream;
        }
    };
    stream.journal_state = spine.journal_state();

    for row in &read_model.rows {
        let observation = network_runtime_observation_from_row(row);
        match spine
            .publish_observation_chain(observation, &row.observed_at)
            .await
        {
            Ok(report) => stream.record_success(&report),
            Err(_) => stream.failed_rows += 1,
        }
    }

    stream
}

pub(crate) fn network_runtime_event_chain_stream_payload(
    report: &NetworkRuntimeServiceStreamReport,
) -> LogFields {
    let separator = constants::delimiter::LIST.to_string();
    fields_from_pairs(vec![
        (
            constants::field::NETWORK_RUNTIME_OBSERVED_ROWS,
            count_value(report.observed_rows),
        ),
        (
            constants::field::NETWORK_RUNTIME_STREAMED_EVENTS,
            count_value(report.streamed_events),
        ),
        (
            constants::field::NETWORK_RUNTIME_FAILED_ROWS,
            count_value(report.failed_rows),
        ),
        (
            constants::field::NETWORK_RUNTIME_MANUAL_REQUIRED_ROWS,
            count_value(report.manual_required_rows),
        ),
        (
            constants::field::NETWORK_RUNTIME_ENFORCEMENT_COMMAND_EVENTS,
            count_value(report.enforcement_command_events),
        ),
        (
            constants::field::NETWORK_RUNTIME_DURABLE_JOURNAL_STATE,
            LogFieldValue::String(report.journal_state.as_str().to_string()),
        ),
        (
            NETWORK_FLOW_READ_MODEL_FIELD_ACTIVE_ROWS,
            count_value(report.active_rows),
        ),
        (
            NETWORK_FLOW_READ_MODEL_FIELD_TOMBSTONE_ROWS,
            count_value(report.tombstone_rows),
        ),
        (
            NETWORK_FLOW_READ_MODEL_FIELD_EXPORTABLE_ROWS,
            count_value(report.exportable_rows),
        ),
        (
            NETWORK_FLOW_READ_MODEL_FIELD_EXPORT_CUSTODY,
            LogFieldValue::String(NETWORK_FLOW_CUSTODY_PARENT_OWNED_EXPORT.to_string()),
        ),
        (
            NETWORK_FLOW_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS,
            LogFieldValue::String(report.deleted_evidence_reference_ids.join(&separator)),
        ),
        (
            constants::field::NETWORK_RUNTIME_EVENT_CHAIN_STREAM,
            serialized_stream_entries(&report.entries),
        ),
    ])
}

impl NetworkRuntimeServiceStreamReport {
    fn record_success(&mut self, report: &NetworkRuntimeReport) {
        let entries = stream_entries_from_report(report);
        self.streamed_events += entries.len();
        self.enforcement_command_events += entries
            .iter()
            .filter(|entry| {
                entry.stream_type == constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED
            })
            .count();
        if report.manual_required() {
            self.manual_required_rows += 1;
        }
        self.journal_state = report.journal_state;
        self.entries.extend(entries);
    }
}

fn count_value(value: usize) -> LogFieldValue {
    LogFieldValue::Number(value as f64)
}

fn serialized_stream_entries(entries: &[NetworkRuntimeServiceStreamEntry]) -> LogFieldValue {
    LogFieldValue::String(match serde_json::to_string(entries) {
        Ok(json) => json,
        Err(_error) => constants::value::EMPTY.to_string(),
    })
}
