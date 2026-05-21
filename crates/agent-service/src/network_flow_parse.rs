use ocentra_parent_agent_core::NetworkStoreRow;
use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, ActivityDomainAttributionStatus,
    ActivityNetworkEndpoint, ActivityNetworkFlowCounters, ActivityNetworkFlowObservation,
    ActivityNetworkProtocol, ActivityNetworkTcpState, ActivityObserver,
    ActivityProcessAttributionStatus, ACTIVITY_QUERY_SCHEMA_VERSION,
};

pub(crate) fn network_observations_from_rows(
    rows: &[NetworkStoreRow],
) -> Vec<ActivityNetworkFlowObservation> {
    rows.iter().map(network_observation_from_row).collect()
}

fn network_observation_from_row(row: &NetworkStoreRow) -> ActivityNetworkFlowObservation {
    let capability_status = capture_capability_status(&row.fields);
    let protocol = optional_protocol(&row.fields);
    let tcp_state = optional_tcp_state(&row.fields);
    let local_endpoint = network_endpoint(
        optional_string(&row.fields, constants::field::LOCAL_IP),
        optional_u16(&row.fields, constants::field::LOCAL_PORT),
    );
    let destination_endpoint = network_endpoint(
        optional_string(&row.fields, constants::field::DESTINATION_IP),
        optional_u16(&row.fields, constants::field::DESTINATION_PORT),
    );

    ActivityNetworkFlowObservation {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        event_id: row.event_id.clone(),
        observed_at: row.observed_at.clone(),
        observer: ActivityObserver::WindowsNetwork,
        capability_status,
        adapter_id: optional_string(&row.fields, constants::field::ADAPTER_ID)
            .unwrap_or_else(|| constants::activity_capture::NETWORK_ADAPTER_ID.to_string()),
        protocol,
        tcp_state,
        local_endpoint,
        destination_endpoint,
        destination_domain: optional_string(&row.fields, constants::field::DESTINATION_DOMAIN),
        domain_attribution_status: optional_domain_attribution_status(&row.fields),
        process_attribution_status: optional_process_attribution_status(&row.fields),
        process_id: optional_u64(&row.fields, constants::field::PID),
        process_name: optional_string(&row.fields, constants::field::PROCESS_NAME),
        counters: ActivityNetworkFlowCounters {
            connection_count: 1.0,
            bytes_sent: None,
            bytes_received: None,
            first_seen_at: Some(row.observed_at.clone()),
            last_seen_at: Some(row.observed_at.clone()),
        },
        evidence: row.evidence.clone(),
    }
}

fn capture_capability_status(
    fields: &ocentra_parent_agent_protocol::LogFields,
) -> ActivityCaptureCapabilityStatus {
    optional_string(fields, constants::field::CAPABILITY_STATUS)
        .and_then(|value| ActivityCaptureCapabilityStatus::from_protocol_str(value.as_str()))
        .unwrap_or(ActivityCaptureCapabilityStatus::NoNetworkObservations)
}

fn optional_protocol(
    fields: &ocentra_parent_agent_protocol::LogFields,
) -> Option<ActivityNetworkProtocol> {
    optional_string(fields, constants::field::NETWORK_PROTOCOL)
        .and_then(|value| ActivityNetworkProtocol::from_protocol_str(value.as_str()))
}

fn optional_tcp_state(
    fields: &ocentra_parent_agent_protocol::LogFields,
) -> Option<ActivityNetworkTcpState> {
    optional_string(fields, constants::field::TCP_STATE)
        .and_then(|value| ActivityNetworkTcpState::from_protocol_str(value.as_str()))
}

fn optional_domain_attribution_status(
    fields: &ocentra_parent_agent_protocol::LogFields,
) -> ActivityDomainAttributionStatus {
    optional_string(fields, constants::field::DOMAIN_ATTRIBUTION_STATUS)
        .and_then(|value| ActivityDomainAttributionStatus::from_protocol_str(value.as_str()))
        .unwrap_or(ActivityDomainAttributionStatus::Unavailable)
}

fn optional_process_attribution_status(
    fields: &ocentra_parent_agent_protocol::LogFields,
) -> ActivityProcessAttributionStatus {
    optional_string(fields, constants::field::PROCESS_ATTRIBUTION_STATUS)
        .and_then(|value| ActivityProcessAttributionStatus::from_protocol_str(value.as_str()))
        .unwrap_or(ActivityProcessAttributionStatus::ProcessUnknown)
}

fn network_endpoint(ip: Option<String>, port: Option<u16>) -> ActivityNetworkEndpoint {
    ActivityNetworkEndpoint { ip, port }
}

fn optional_string(fields: &ocentra_parent_agent_protocol::LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(ocentra_parent_agent_protocol::LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}

fn optional_u16(fields: &ocentra_parent_agent_protocol::LogFields, key: &str) -> Option<u16> {
    optional_number(fields, key).and_then(|value| {
        if (0.0..=u16::MAX as f64).contains(&value) && value.fract() == 0.0 {
            Some(value as u16)
        } else {
            None
        }
    })
}

fn optional_u64(fields: &ocentra_parent_agent_protocol::LogFields, key: &str) -> Option<u64> {
    optional_number(fields, key).and_then(|value| {
        if value >= 0.0 && value.fract() == 0.0 {
            Some(value as u64)
        } else {
            None
        }
    })
}

fn optional_number(fields: &ocentra_parent_agent_protocol::LogFields, key: &str) -> Option<f64> {
    match fields.get(key) {
        Some(ocentra_parent_agent_protocol::LogFieldValue::Number(value)) => Some(*value),
        _ => None,
    }
}
