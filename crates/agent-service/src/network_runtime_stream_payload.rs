use ocentra_parent_agent_core::{
    publish_network_runtime_chain_for_observation, NetworkRuntimeReport,
};
use ocentra_parent_agent_protocol::{
    constants, ActivityNetworkFlowReadModel, LogFieldValue, LogFields,
};

use crate::{
    fields::fields_from_pairs,
    network_runtime_delivery::network_runtime_observation_from_row,
    network_runtime_stream_events::{stream_entries_from_report, NetworkRuntimeServiceStreamEntry},
};

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct NetworkRuntimeServiceStreamReport {
    pub(crate) observed_rows: usize,
    pub(crate) streamed_events: usize,
    pub(crate) failed_rows: usize,
    pub(crate) manual_required_rows: usize,
    pub(crate) enforcement_command_events: usize,
    pub(crate) entries: Vec<NetworkRuntimeServiceStreamEntry>,
}

pub(crate) async fn stream_network_runtime_event_chain_for_read_model(
    read_model: &ActivityNetworkFlowReadModel,
) -> NetworkRuntimeServiceStreamReport {
    let mut stream = NetworkRuntimeServiceStreamReport {
        observed_rows: read_model.rows.len(),
        ..NetworkRuntimeServiceStreamReport::default()
    };

    for row in &read_model.rows {
        let observation = network_runtime_observation_from_row(row);
        match publish_network_runtime_chain_for_observation(observation, &row.observed_at).await {
            Ok(report) => stream.record_success(&report),
            Err(_) => stream.failed_rows += 1,
        }
    }

    stream
}

pub(crate) fn network_runtime_event_chain_stream_payload(
    report: &NetworkRuntimeServiceStreamReport,
) -> LogFields {
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
            constants::field::NETWORK_RUNTIME_EVENT_CHAIN_STREAM,
            LogFieldValue::String(
                serde_json::to_string(&report.entries)
                    .expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
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
                entry.event_type == constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED
            })
            .count();
        if report.manual_required() {
            self.manual_required_rows += 1;
        }
        self.entries.extend(entries);
    }
}

fn count_value(value: usize) -> LogFieldValue {
    LogFieldValue::Number(value as f64)
}
