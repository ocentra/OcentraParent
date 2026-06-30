use super::*;

pub(super) fn network_flow_read_model_from_payload_impl(
    payload: &LogFields,
) -> Result<ActivityNetworkFlowReadModel, String> {
    let generated_at = required_string_field(payload, constants::field::GENERATED_AT)?;
    let custody = required_string_field(payload, constants::field::CUSTODY)?;
    let limit = required_u64_field(payload, constants::field::LIMIT)?;
    let reported_rows = required_u64_field(payload, constants::field::RETURNED)?;
    let capability_status = required_string_field(payload, constants::field::CAPABILITY_STATUS)?;
    let latest_event_id = optional_string_field(payload, constants::field::LATEST_EVENT_ID);
    let latest_observed_at = optional_string_field(payload, constants::field::LATEST_OBSERVED_AT);
    let tombstone_rows = required_u64_field(payload, NETWORK_FLOW_READ_MODEL_FIELD_TOMBSTONE_ROWS)?;
    let latest_tombstone_event_id = optional_string_field(
        payload,
        NETWORK_FLOW_READ_MODEL_FIELD_LATEST_TOMBSTONE_EVENT_ID,
    );
    let latest_tombstone_observed_at = optional_string_field(
        payload,
        NETWORK_FLOW_READ_MODEL_FIELD_LATEST_TOMBSTONE_OBSERVED_AT,
    );
    let visible_rows = u64::from(reported_rows > 0);

    Ok(ActivityNetworkFlowReadModel {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        generated_at,
        custody,
        limit,
        returned: visible_rows,
        active_rows: visible_rows,
        tombstone_rows,
        exportable_rows: visible_rows,
        capability_status: capability_status.clone(),
        latest_event_id: latest_event_id.clone(),
        latest_observed_at: latest_observed_at.clone(),
        latest_tombstone_event_id,
        latest_tombstone_observed_at,
        deleted_evidence_reference_ids: list_field(
            payload,
            NETWORK_FLOW_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS,
        ),
        rows: network_flow_rows(
            payload,
            visible_rows,
            &capability_status,
            latest_event_id,
            latest_observed_at,
        )?,
    })
}

fn network_flow_rows(
    payload: &LogFields,
    visible_rows: u64,
    capability_status: &str,
    latest_event_id: Option<String>,
    latest_observed_at: Option<String>,
) -> Result<Vec<ActivityNetworkFlowObservation>, String> {
    if visible_rows == 0 {
        return Ok(Vec::new());
    }
    Ok(vec![ActivityNetworkFlowObservation {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        event_id: required_latest_network_value(
            latest_event_id,
            constants::field::LATEST_EVENT_ID,
        )?,
        observed_at: required_latest_network_value(
            latest_observed_at,
            constants::field::LATEST_OBSERVED_AT,
        )?,
        observer: required_string_field(payload, constants::field::OBSERVER)?,
        capability_status: capability_status.to_string(),
        adapter_id: required_string_field(payload, constants::field::ADAPTER_ID)?,
        protocol: optional_string_field(payload, constants::field::NETWORK_PROTOCOL),
        tcp_state: optional_string_field(payload, constants::field::TCP_STATE),
        local_endpoint: ActivityNetworkEndpoint {
            ip: optional_string_field(payload, constants::field::LOCAL_IP),
            port: optional_u16_field(payload, constants::field::LOCAL_PORT),
        },
        destination_endpoint: ActivityNetworkEndpoint {
            ip: optional_string_field(payload, constants::field::DESTINATION_IP),
            port: optional_u16_field(payload, constants::field::DESTINATION_PORT),
        },
        destination_domain: optional_string_field(payload, constants::field::DESTINATION_DOMAIN),
        domain_attribution_status: required_string_field(
            payload,
            constants::field::DOMAIN_ATTRIBUTION_STATUS,
        )?,
        process_attribution_status: required_string_field(
            payload,
            constants::field::PROCESS_ATTRIBUTION_STATUS,
        )?,
        process_id: optional_u64_field(payload, constants::field::PROCESS_ID),
        process_name: optional_string_field(payload, constants::field::PROCESS_NAME),
        counters: ActivityNetworkFlowCounters {
            connection_count: required_u64_field(payload, constants::field::CONNECTION_COUNT)?,
            bytes_sent: optional_u64_field(payload, constants::field::BYTES_SENT),
            bytes_received: optional_u64_field(payload, constants::field::BYTES_RECEIVED),
            first_seen_at: optional_string_field(payload, constants::field::FIRST_SEEN_AT),
            last_seen_at: optional_string_field(payload, constants::field::LAST_SEEN_AT),
        },
        evidence: network_flow_digest(payload)
            .map(|digest| digest.evidence)
            .unwrap_or_default(),
    }])
}

fn required_latest_network_value(
    value: Option<String>,
    field_name: &str,
) -> Result<String, String> {
    value.ok_or_else(|| format!("agent-service network flow payload missing {field_name}"))
}
