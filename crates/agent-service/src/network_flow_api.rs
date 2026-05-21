use ocentra_parent_agent_core::{ActivityStore, NetworkStoreRow};
use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, ActivityNetworkCustodyState,
    ActivityNetworkFlowDigest, ActivityNetworkFlowReadModel, AgentCommandEnvelope,
    AgentEventEnvelope, AgentEventName, LogFieldValue, LogFields, LogLevel,
    ACTIVITY_QUERY_SCHEMA_VERSION,
};

use crate::{
    activity_payload::activity_store_error_payload, activity_store_path::activity_db_path,
    event_builder::build_event, fields::fields_from_pairs,
    network_flow_digest::network_flow_digest, network_flow_parse::network_observations_from_rows,
    time::timestamp_now,
};

const NETWORK_FLOW_LIMIT: u64 = constants::activity_capture::NETWORK_SNAPSHOT_LIMIT as u64;

pub async fn build_network_flow_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    match load_network_flow_report().await {
        Some(report) => build_event(
            constants::event_id::NETWORK_FLOW_REPORTED,
            &command.message_id,
            command.source,
            AgentEventName::AgentNetworkFlowReported,
            LogLevel::Info,
            network_flow_payload(&report),
            None,
        ),
        None => activity_store_error_event(
            command,
            constants::event_id::NETWORK_FLOW_REPORTED,
            AgentEventName::AgentNetworkFlowReported,
        ),
    }
}

async fn load_network_flow_report() -> Option<NetworkFlowReport> {
    let path = activity_db_path();
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(path).ok()?;
        let rows = store.recent_network_rows(NETWORK_FLOW_LIMIT).ok()?;
        Some(network_flow_report_from_rows(rows))
    })
    .await
    .ok()
    .flatten()
}

pub(crate) fn network_flow_report_from_rows(rows: Vec<NetworkStoreRow>) -> NetworkFlowReport {
    let observations = network_observations_from_rows(&rows);
    let read_model = ActivityNetworkFlowReadModel {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        generated_at: timestamp_now(),
        custody: ActivityNetworkCustodyState::ChildDeviceQueryStore,
        limit: NETWORK_FLOW_LIMIT,
        returned: observations.len() as u64,
        capability_status: observations
            .first()
            .map(|observation| observation.capability_status.clone())
            .unwrap_or(ActivityCaptureCapabilityStatus::NoNetworkObservations),
        rows: observations.clone(),
    };

    NetworkFlowReport {
        read_model,
        digest: network_flow_digest(&observations),
    }
}

pub(crate) fn network_flow_payload(report: &NetworkFlowReport) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::LIMIT,
            LogFieldValue::Number(report.read_model.limit as f64),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(report.read_model.returned as f64),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(
                report
                    .read_model
                    .capability_status
                    .as_protocol_str()
                    .to_string(),
            ),
        ),
        (
            constants::field::CUSTODY_LABEL,
            LogFieldValue::String(report.read_model.custody.as_protocol_str().to_string()),
        ),
        (
            constants::field::ACTIVITY_DIGEST,
            LogFieldValue::String(
                serde_json::to_string(&report.digest)
                    .expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
    ])
}

fn activity_store_error_event(
    command: AgentCommandEnvelope,
    event_id_suffix: &str,
    event: AgentEventName,
) -> AgentEventEnvelope {
    build_event(
        event_id_suffix,
        &command.message_id,
        command.source,
        event,
        LogLevel::Error,
        activity_store_error_payload(),
        None,
    )
}

pub(crate) struct NetworkFlowReport {
    pub(crate) read_model: ActivityNetworkFlowReadModel,
    pub(crate) digest: ActivityNetworkFlowDigest,
}
