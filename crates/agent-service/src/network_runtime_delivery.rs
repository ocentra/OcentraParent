use std::sync::Arc;

use ocentra_eventing::error::EventingError;
use serde::de::DeserializeOwned;
use serde_json::Value;
use tokio::sync::OnceCell;

use ocentra_parent_agent_core::{
    network_capture::NetworkObservation,
    network_event_runtime::{
        NetworkRuntimeJournalState, NetworkRuntimeReport, NetworkRuntimeSpine,
    },
};
use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowObservation;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowReadModel;
static NETWORK_RUNTIME_SPINE: OnceCell<Arc<NetworkRuntimeSpine>> = OnceCell::const_new();

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
    pub(crate) journal_state: NetworkRuntimeJournalState,
}

#[derive(Clone, Copy)]
struct ProtocolTextRef<'a>(&'a str);

#[derive(Clone, Copy)]
struct EventNameRef<'a>(&'a str);

pub(crate) async fn deliver_network_runtime_for_read_model(
    read_model: &ActivityNetworkFlowReadModel,
) -> NetworkRuntimeServiceDeliveryReport {
    let mut delivery = NetworkRuntimeServiceDeliveryReport {
        observed_rows: read_model.rows.len(),
        journal_state: NetworkRuntimeJournalState::UnavailableManualRequired,
        ..NetworkRuntimeServiceDeliveryReport::default()
    };

    let spine = match shared_network_runtime_spine().await {
        Ok(spine) => spine,
        Err(_) => {
            delivery.failed_rows = delivery.observed_rows;
            return delivery;
        }
    };
    delivery.journal_state = spine.journal_state();

    for row in &read_model.rows {
        let observation = network_runtime_observation_from_row(row);
        match spine
            .publish_observation_chain(observation, &row.observed_at)
            .await
        {
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
        self.journal_state = report.journal_state;
        if report.manual_required() {
            self.manual_required_rows += 1;
        }
        self.enforcement_command_events += count_event_type(
            report,
            EventNameRef(constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED),
        );
    }
}

pub(crate) async fn initialize_network_runtime_spine() -> Result<(), EventingError> {
    let _ = shared_network_runtime_spine().await?;
    Ok(())
}

pub(crate) async fn shared_network_runtime_spine() -> Result<Arc<NetworkRuntimeSpine>, EventingError>
{
    if let Some(spine) = NETWORK_RUNTIME_SPINE.get() {
        return Ok(Arc::clone(spine));
    }

    let spine = Arc::new(NetworkRuntimeSpine::with_default_handlers().await?);
    if NETWORK_RUNTIME_SPINE.set(Arc::clone(&spine)).is_err() {
        if let Some(existing) = NETWORK_RUNTIME_SPINE.get() {
            return Ok(Arc::clone(existing));
        }
    }
    Ok(spine)
}

pub(crate) fn network_runtime_observation_from_row(
    row: &ActivityNetworkFlowObservation,
) -> NetworkObservation {
    let status = protocol_value::<ActivityCaptureCapabilityStatus>(ProtocolTextRef(
        row.capability_status.as_str(),
    ))
    .unwrap_or(ActivityCaptureCapabilityStatus::Unavailable);
    if status != ActivityCaptureCapabilityStatus::Available {
        return NetworkObservation::degraded(status);
    }

    let pid = row.process_id.and_then(|value| u32::try_from(value).ok());
    NetworkObservation {
        status,
        protocol: optional_protocol_value(row.protocol.as_deref().map(ProtocolTextRef)),
        local_ip: row.local_endpoint.ip.clone(),
        local_port: row.local_endpoint.port,
        destination_ip: row.destination_endpoint.ip.clone(),
        destination_port: row.destination_endpoint.port,
        destination_domain: row.destination_domain.clone(),
        tcp_state: optional_protocol_value(row.tcp_state.as_deref().map(ProtocolTextRef)),
        pid,
        process_name: row.process_name.clone(),
        associated_pid_count: usize::from(pid.is_some()),
    }
}

fn optional_protocol_value<T>(value: Option<ProtocolTextRef<'_>>) -> Option<T>
where
    T: DeserializeOwned,
{
    value.and_then(protocol_value::<T>)
}

fn protocol_value<T>(value: ProtocolTextRef<'_>) -> Option<T>
where
    T: DeserializeOwned,
{
    serde_json::from_value(Value::String(value.0.to_owned())).ok()
}

fn count_event_type(report: &NetworkRuntimeReport, event_name: EventNameRef<'_>) -> usize {
    report
        .stored_events
        .iter()
        .filter(|event| event.contract.event_type.as_str() == event_name.0)
        .count()
}
