use serde::de::DeserializeOwned;
use serde_json::Value;

use ocentra_parent_agent_core::{
    publish_network_runtime_chain_for_observation, NetworkObservation, NetworkRuntimeReport,
};
use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, ActivityNetworkFlowObservation,
    ActivityNetworkFlowReadModel,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct NetworkRuntimeServiceDeliveryReport {
    pub(crate) observed_rows: usize,
    pub(crate) delivered_rows: usize,
    pub(crate) failed_rows: usize,
    pub(crate) publish_reports: usize,
    pub(crate) stored_events: usize,
    pub(crate) dead_letters: usize,
    pub(crate) manual_required_rows: usize,
    pub(crate) enforcement_command_events: usize,
}

pub(crate) async fn deliver_network_runtime_for_read_model(
    read_model: &ActivityNetworkFlowReadModel,
) -> NetworkRuntimeServiceDeliveryReport {
    let mut delivery = NetworkRuntimeServiceDeliveryReport {
        observed_rows: read_model.rows.len(),
        ..NetworkRuntimeServiceDeliveryReport::default()
    };

    for row in &read_model.rows {
        let observation = network_runtime_observation_from_row(row);
        match publish_network_runtime_chain_for_observation(observation, &row.observed_at).await {
            Ok(report) => delivery.record_success(&report),
            Err(_) => delivery.failed_rows += 1,
        }
    }

    delivery
}

impl NetworkRuntimeServiceDeliveryReport {
    fn record_success(&mut self, report: &NetworkRuntimeReport) {
        self.delivered_rows += 1;
        self.publish_reports += report.publish_reports.len();
        self.stored_events += report.stored_events.len();
        self.dead_letters += report.dead_letters.len();
        if report.manual_required() {
            self.manual_required_rows += 1;
        }
        self.enforcement_command_events += count_event_type(
            report,
            constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED,
        );
    }
}

pub(crate) fn network_runtime_observation_from_row(
    row: &ActivityNetworkFlowObservation,
) -> NetworkObservation {
    let status = protocol_value::<ActivityCaptureCapabilityStatus>(&row.capability_status)
        .unwrap_or(ActivityCaptureCapabilityStatus::Unavailable);
    if status != ActivityCaptureCapabilityStatus::Available {
        return NetworkObservation::degraded(status);
    }

    let pid = row.process_id.and_then(|value| u32::try_from(value).ok());
    NetworkObservation {
        status,
        protocol: optional_protocol_value(&row.protocol),
        local_ip: row.local_endpoint.ip.clone(),
        local_port: row.local_endpoint.port,
        destination_ip: row.destination_endpoint.ip.clone(),
        destination_port: row.destination_endpoint.port,
        destination_domain: row.destination_domain.clone(),
        tcp_state: optional_protocol_value(&row.tcp_state),
        pid,
        process_name: row.process_name.clone(),
        associated_pid_count: usize::from(pid.is_some()),
    }
}

fn optional_protocol_value<T>(value: &Option<String>) -> Option<T>
where
    T: DeserializeOwned,
{
    value
        .as_ref()
        .and_then(|text| protocol_value::<T>(text.as_str()))
}

fn protocol_value<T>(value: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    serde_json::from_value(Value::String(value.to_owned())).ok()
}

fn count_event_type(report: &NetworkRuntimeReport, event_type: &str) -> usize {
    report
        .stored_events
        .iter()
        .filter(|event| event.contract.event_type.as_str() == event_type)
        .count()
}
