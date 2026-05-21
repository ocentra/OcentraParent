use ocentra_parent_agent_protocol::{
    constants, ActivityNetworkEndpoint, ActivityNetworkFlowCounters,
    ActivityNetworkFlowObservation, ActivityNetworkFlowReadModel, LogFieldValue, LogFields,
    NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE, NETWORK_FLOW_SCHEMA_VERSION,
};
use rusqlite::Connection;

use crate::{
    activity_store_network_flow_rows::{network_flow_rows, NetworkFlowStoreRow},
    ActivityStoreError,
};

pub(crate) fn network_flow_read_model(
    connection: &Connection,
    limit: u64,
    generated_at: &str,
) -> Result<ActivityNetworkFlowReadModel, ActivityStoreError> {
    let rows = network_flow_rows(connection, limit)?;
    let observations: Vec<ActivityNetworkFlowObservation> =
        rows.into_iter().map(observation_from_row).collect();
    let capability_status = read_model_capability_status(&observations);

    Ok(ActivityNetworkFlowReadModel {
        schema_version: NETWORK_FLOW_SCHEMA_VERSION,
        generated_at: generated_at.to_string(),
        custody: NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        limit,
        returned: observations.len() as u64,
        capability_status,
        rows: observations,
    })
}

fn observation_from_row(row: NetworkFlowStoreRow) -> ActivityNetworkFlowObservation {
    let fields = &row.fields;
    ActivityNetworkFlowObservation {
        schema_version: NETWORK_FLOW_SCHEMA_VERSION,
        event_id: row.event_id,
        observed_at: row.observed_at.clone(),
        observer: row.observer,
        capability_status: string_field(fields, constants::field::CAPABILITY_STATUS)
            .unwrap_or_else(|| {
                constants::activity_capture::CAPABILITY_STATUS_UNAVAILABLE.to_string()
            }),
        adapter_id: string_field(fields, constants::field::ADAPTER_ID)
            .unwrap_or_else(|| constants::activity_capture::NETWORK_ADAPTER_ID.to_string()),
        protocol: string_field(fields, constants::field::NETWORK_PROTOCOL),
        tcp_state: string_field(fields, constants::field::TCP_STATE),
        local_endpoint: ActivityNetworkEndpoint {
            ip: string_field(fields, constants::field::LOCAL_IP),
            port: port_field(fields, constants::field::LOCAL_PORT),
        },
        destination_endpoint: ActivityNetworkEndpoint {
            ip: string_field(fields, constants::field::DESTINATION_IP),
            port: port_field(fields, constants::field::DESTINATION_PORT),
        },
        destination_domain: string_field(fields, constants::field::DESTINATION_DOMAIN),
        domain_attribution_status: string_field(
            fields,
            constants::field::DOMAIN_ATTRIBUTION_STATUS,
        )
        .unwrap_or_else(|| {
            constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_UNAVAILABLE.to_string()
        }),
        process_attribution_status: string_field(
            fields,
            constants::field::PROCESS_ATTRIBUTION_STATUS,
        )
        .unwrap_or_else(|| {
            constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_UNKNOWN.to_string()
        }),
        process_id: number_field(fields, constants::field::PID),
        process_name: string_field(fields, constants::field::PROCESS_NAME),
        counters: ActivityNetworkFlowCounters {
            connection_count: 1,
            bytes_sent: None,
            bytes_received: None,
            first_seen_at: Some(row.observed_at.clone()),
            last_seen_at: Some(row.observed_at),
        },
        evidence: row.evidence,
    }
}

fn read_model_capability_status(observations: &[ActivityNetworkFlowObservation]) -> String {
    observations
        .first()
        .map(|observation| observation.capability_status.clone())
        .unwrap_or_else(|| {
            constants::activity_capture::CAPABILITY_STATUS_NO_NETWORK_OBSERVATIONS.to_string()
        })
}

fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}

fn number_field(fields: &LogFields, key: &str) -> Option<u64> {
    match fields.get(key) {
        Some(LogFieldValue::Number(value)) if value.is_finite() && *value >= 0.0 => {
            Some(*value as u64)
        }
        _ => None,
    }
}

fn port_field(fields: &LogFields, key: &str) -> Option<u16> {
    number_field(fields, key).and_then(|value| u16::try_from(value).ok())
}
