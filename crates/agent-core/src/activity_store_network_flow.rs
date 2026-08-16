use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::network_flow::{
    ActivityNetworkEndpoint, ActivityNetworkFlowCounters, ActivityNetworkFlowObservation,
    ActivityNetworkFlowReadModel, NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE,
};
use ocentra_parent_agent_protocol::NETWORK_FLOW_SCHEMA_VERSION;
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
    let store_rows = network_flow_rows(connection, limit)?;
    let latest = store_rows.first();
    let latest_tombstone = store_rows.iter().find(|row| is_tombstone(row));
    let latest_event_id = latest.map(|row| row.event_id.clone());
    let latest_observed_at = latest.map(|row| row.observed_at.clone());
    let latest_tombstone_event_id = latest_tombstone.map(|row| row.event_id.clone());
    let latest_tombstone_observed_at = latest_tombstone.map(|row| row.observed_at.clone());
    let tombstone_rows = store_rows.iter().filter(|row| is_tombstone(row)).count() as u64;
    let deleted_evidence_reference_ids = deleted_evidence_reference_ids(&store_rows);
    let observations: Vec<ActivityNetworkFlowObservation> = store_rows
        .into_iter()
        .filter(is_flow_observation)
        .filter(|row| !row_deleted(row, &deleted_evidence_reference_ids))
        .map(observation_from_row)
        .collect();
    let capability_status = read_model_capability_status(&observations);

    Ok(ActivityNetworkFlowReadModel {
        schema_version: NETWORK_FLOW_SCHEMA_VERSION,
        generated_at: generated_at.to_string(),
        custody: NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        limit,
        returned: observations.len() as u64,
        active_rows: observations.len() as u64,
        tombstone_rows,
        exportable_rows: observations.len() as u64,
        capability_status,
        latest_event_id,
        latest_observed_at,
        latest_tombstone_event_id,
        latest_tombstone_observed_at,
        deleted_evidence_reference_ids,
        rows: observations,
    })
}

fn is_flow_observation(row: &NetworkFlowStoreRow) -> bool {
    row.kind == constants::activity_event_kind::DOMAIN_OBSERVED
}

fn is_tombstone(row: &NetworkFlowStoreRow) -> bool {
    row.kind == constants::activity_event_kind::NETWORK_RETENTION_DELETED
}

fn row_deleted(row: &NetworkFlowStoreRow, deleted_ids: &[String]) -> bool {
    deleted_ids.iter().any(|id| id == &row.event_id)
        || row
            .evidence
            .iter()
            .any(|reference| deleted_ids.iter().any(|id| id == &reference.evidence_id))
}

fn deleted_evidence_reference_ids(rows: &[NetworkFlowStoreRow]) -> Vec<String> {
    let mut ids = Vec::new();
    for row in rows.iter().filter(|row| is_tombstone(row)) {
        for id in evidence_reference_ids(&row.fields, &row.evidence) {
            if !ids.iter().any(|existing| existing == &id) {
                ids.push(id);
            }
        }
    }
    ids
}

fn evidence_reference_ids(fields: &LogFields, evidence: &[ActivityEvidenceRef]) -> Vec<String> {
    let mut ids = string_field(fields, constants::field::EVIDENCE_REFERENCE_IDS)
        .map(|value| split_evidence_reference_ids(&value))
        .unwrap_or_default();

    for reference in evidence {
        if !ids.iter().any(|id| id == &reference.evidence_id) {
            ids.push(reference.evidence_id.clone());
        }
    }
    ids
}

fn split_evidence_reference_ids(value: &str) -> Vec<String> {
    value
        .split(constants::delimiter::LIST)
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(ToOwned::to_owned)
        .collect()
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
