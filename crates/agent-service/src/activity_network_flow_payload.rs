use ocentra_parent_agent_protocol::{
    constants, ActivityNetworkFlowObservation, ActivityNetworkFlowReadModel, LogFieldValue,
    LogFields, NETWORK_FLOW_CUSTODY_PARENT_OWNED_EXPORT, NETWORK_FLOW_READ_MODEL_FIELD_ACTIVE_ROWS,
    NETWORK_FLOW_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS,
    NETWORK_FLOW_READ_MODEL_FIELD_EXPORTABLE_ROWS, NETWORK_FLOW_READ_MODEL_FIELD_EXPORT_CUSTODY,
    NETWORK_FLOW_READ_MODEL_FIELD_LATEST_TOMBSTONE_EVENT_ID,
    NETWORK_FLOW_READ_MODEL_FIELD_LATEST_TOMBSTONE_OBSERVED_AT,
    NETWORK_FLOW_READ_MODEL_FIELD_TOMBSTONE_ROWS,
};

use crate::{
    fields::fields_from_pairs, network_flow_digest::network_flow_digest,
    network_runtime_delivery::NetworkRuntimeServiceDeliveryReport,
};

type FieldPair = (&'static str, LogFieldValue);

pub fn network_flow_read_model_payload_with_runtime_delivery(
    read_model: &ActivityNetworkFlowReadModel,
    delivery: Option<&NetworkRuntimeServiceDeliveryReport>,
) -> LogFields {
    let latest = read_model.rows.first();
    let mut pairs = read_model_pairs(read_model);
    pairs.extend(runtime_delivery_pairs(delivery));
    pairs.extend(row_identity_pairs(latest));
    pairs.extend(endpoint_pairs(latest));
    pairs.extend(process_pairs(latest));
    pairs.extend(counter_pairs(latest));
    fields_from_pairs(pairs)
}

fn read_model_pairs(read_model: &ActivityNetworkFlowReadModel) -> Vec<FieldPair> {
    let separator = constants::delimiter::LIST.to_string();
    vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::CUSTODY,
            LogFieldValue::String(read_model.custody.clone()),
        ),
        (
            constants::field::LIMIT,
            LogFieldValue::Number(read_model.limit as f64),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.returned as f64),
        ),
        (
            NETWORK_FLOW_READ_MODEL_FIELD_ACTIVE_ROWS,
            LogFieldValue::Number(read_model.active_rows as f64),
        ),
        (
            NETWORK_FLOW_READ_MODEL_FIELD_TOMBSTONE_ROWS,
            LogFieldValue::Number(read_model.tombstone_rows as f64),
        ),
        (
            NETWORK_FLOW_READ_MODEL_FIELD_EXPORTABLE_ROWS,
            LogFieldValue::Number(read_model.exportable_rows as f64),
        ),
        (
            NETWORK_FLOW_READ_MODEL_FIELD_EXPORT_CUSTODY,
            LogFieldValue::String(NETWORK_FLOW_CUSTODY_PARENT_OWNED_EXPORT.to_string()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(read_model.capability_status.clone()),
        ),
        (
            constants::field::LATEST_EVENT_ID,
            optional_string(read_model.latest_event_id.as_ref()),
        ),
        (
            constants::field::LATEST_OBSERVED_AT,
            optional_string(read_model.latest_observed_at.as_ref()),
        ),
        (
            NETWORK_FLOW_READ_MODEL_FIELD_LATEST_TOMBSTONE_EVENT_ID,
            optional_string(read_model.latest_tombstone_event_id.as_ref()),
        ),
        (
            NETWORK_FLOW_READ_MODEL_FIELD_LATEST_TOMBSTONE_OBSERVED_AT,
            optional_string(read_model.latest_tombstone_observed_at.as_ref()),
        ),
        (
            NETWORK_FLOW_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS,
            LogFieldValue::String(read_model.deleted_evidence_reference_ids.join(&separator)),
        ),
        (
            constants::field::ACTIVITY_DIGEST,
            LogFieldValue::String(
                serde_json::to_string(&network_flow_digest(read_model))
                    .expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
    ]
}

fn row_identity_pairs(row: Option<&ActivityNetworkFlowObservation>) -> Vec<FieldPair> {
    vec![
        (
            constants::field::OBSERVER,
            optional_string(row.map(|value| &value.observer)),
        ),
        (
            constants::field::ADAPTER_ID,
            optional_string(row.map(|value| &value.adapter_id)),
        ),
        (
            constants::field::NETWORK_PROTOCOL,
            optional_string(row.and_then(|value| value.protocol.as_ref())),
        ),
        (
            constants::field::TCP_STATE,
            optional_string(row.and_then(|value| value.tcp_state.as_ref())),
        ),
    ]
}

fn endpoint_pairs(row: Option<&ActivityNetworkFlowObservation>) -> Vec<FieldPair> {
    vec![
        (
            constants::field::LOCAL_IP,
            optional_string(row.and_then(|value| value.local_endpoint.ip.as_ref())),
        ),
        (
            constants::field::LOCAL_PORT,
            optional_u16(row.and_then(|value| value.local_endpoint.port)),
        ),
        (
            constants::field::DESTINATION_IP,
            optional_string(row.and_then(|value| value.destination_endpoint.ip.as_ref())),
        ),
        (
            constants::field::DESTINATION_PORT,
            optional_u16(row.and_then(|value| value.destination_endpoint.port)),
        ),
        (
            constants::field::DESTINATION_DOMAIN,
            optional_string(row.and_then(|value| value.destination_domain.as_ref())),
        ),
        (
            constants::field::DOMAIN_ATTRIBUTION_STATUS,
            optional_string(row.map(|value| &value.domain_attribution_status)),
        ),
    ]
}

fn process_pairs(row: Option<&ActivityNetworkFlowObservation>) -> Vec<FieldPair> {
    vec![
        (
            constants::field::PROCESS_ATTRIBUTION_STATUS,
            optional_string(row.map(|value| &value.process_attribution_status)),
        ),
        (
            constants::field::PROCESS_ID,
            optional_u64(row.and_then(|value| value.process_id)),
        ),
        (
            constants::field::PROCESS_NAME,
            optional_string(row.and_then(|value| value.process_name.as_ref())),
        ),
    ]
}

fn counter_pairs(row: Option<&ActivityNetworkFlowObservation>) -> Vec<FieldPair> {
    vec![
        (
            constants::field::CONNECTION_COUNT,
            optional_u64(row.map(|value| value.counters.connection_count)),
        ),
        (
            constants::field::BYTES_SENT,
            optional_u64(row.and_then(|value| value.counters.bytes_sent)),
        ),
        (
            constants::field::BYTES_RECEIVED,
            optional_u64(row.and_then(|value| value.counters.bytes_received)),
        ),
        (
            constants::field::FIRST_SEEN_AT,
            optional_string(row.and_then(|value| value.counters.first_seen_at.as_ref())),
        ),
        (
            constants::field::LAST_SEEN_AT,
            optional_string(row.and_then(|value| value.counters.last_seen_at.as_ref())),
        ),
    ]
}

fn runtime_delivery_pairs(
    delivery: Option<&NetworkRuntimeServiceDeliveryReport>,
) -> Vec<FieldPair> {
    vec![
        (
            constants::field::NETWORK_RUNTIME_OBSERVED_ROWS,
            optional_usize(delivery.map(|value| value.observed_rows)),
        ),
        (
            constants::field::NETWORK_RUNTIME_DELIVERED_ROWS,
            optional_usize(delivery.map(|value| value.delivered_rows)),
        ),
        (
            constants::field::NETWORK_RUNTIME_FAILED_ROWS,
            optional_usize(delivery.map(|value| value.failed_rows)),
        ),
        (
            constants::field::NETWORK_RUNTIME_PUBLISH_REPORTS,
            optional_usize(delivery.map(|value| value.publish_reports)),
        ),
        (
            constants::field::NETWORK_RUNTIME_STORED_EVENTS,
            optional_usize(delivery.map(|value| value.stored_events)),
        ),
        (
            constants::field::NETWORK_RUNTIME_DEAD_LETTERS,
            optional_usize(delivery.map(|value| value.dead_letters)),
        ),
        (
            constants::field::NETWORK_RUNTIME_MANUAL_REQUIRED_ROWS,
            optional_usize(delivery.map(|value| value.manual_required_rows)),
        ),
        (
            constants::field::NETWORK_RUNTIME_ENFORCEMENT_COMMAND_EVENTS,
            optional_usize(delivery.map(|value| value.enforcement_command_events)),
        ),
    ]
}

fn optional_string(value: Option<&String>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.clone()),
        None => LogFieldValue::Null(()),
    }
}

fn optional_u16(value: Option<u16>) -> LogFieldValue {
    optional_u64(value.map(u64::from))
}

fn optional_u64(value: Option<u64>) -> LogFieldValue {
    match value {
        Some(number) => LogFieldValue::Number(number as f64),
        None => LogFieldValue::Null(()),
    }
}

fn optional_usize(value: Option<usize>) -> LogFieldValue {
    optional_u64(value.map(|number| number as u64))
}
