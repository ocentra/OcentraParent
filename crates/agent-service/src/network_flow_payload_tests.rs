use ocentra_parent_agent_protocol::{
    constants, ActivityEvidenceKind, ActivityEvidenceRef, ActivityNetworkEndpoint,
    ActivityNetworkFlowCounters, ActivityNetworkFlowObservation, ActivityNetworkFlowReadModel,
    LogFieldValue, NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE, NETWORK_FLOW_SCHEMA_VERSION,
};

use super::activity_network_flow_payload::network_flow_read_model_payload;

#[test]
fn network_flow_payload_contains_contract_shaped_digest_json() {
    let read_model = ActivityNetworkFlowReadModel {
        schema_version: NETWORK_FLOW_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        custody: NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: 1,
        capability_status: constants::activity_capture::CAPABILITY_STATUS_AVAILABLE.to_string(),
        rows: vec![observation()],
    };

    let payload = network_flow_read_model_payload(&read_model);
    let digest_json = payload
        .get(constants::field::ACTIVITY_DIGEST)
        .and_then(|value| match value {
            LogFieldValue::String(text) => Some(text),
            _ => None,
        })
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let digest: ocentra_parent_agent_protocol::ActivityNetworkFlowDigest =
        serde_json::from_str(digest_json).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        digest.top_destinations[0].label,
        constants::activity_store::TEST_NETWORK_DOMAIN
    );
    assert_eq!(digest.unusual_indicators.len(), 0);
}

fn observation() -> ActivityNetworkFlowObservation {
    ActivityNetworkFlowObservation {
        schema_version: NETWORK_FLOW_SCHEMA_VERSION,
        event_id: constants::activity_store::TEST_NETWORK_EVENT_ID.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        observer: constants::activity_observer::WINDOWS_NETWORK.to_string(),
        capability_status: constants::activity_capture::CAPABILITY_STATUS_AVAILABLE.to_string(),
        adapter_id: constants::activity_capture::NETWORK_ADAPTER_ID.to_string(),
        protocol: Some(constants::activity_capture::NETWORK_PROTOCOL_TCP.to_string()),
        tcp_state: Some(constants::activity_capture::TCP_STATE_ESTABLISHED.to_string()),
        local_endpoint: ActivityNetworkEndpoint {
            ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
            port: Some(constants::activity_store::TEST_NETWORK_LOCAL_PORT),
        },
        destination_endpoint: ActivityNetworkEndpoint {
            ip: Some(constants::activity_store::TEST_NETWORK_DESTINATION_IP.to_string()),
            port: Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT),
        },
        destination_domain: Some(constants::activity_store::TEST_NETWORK_DOMAIN.to_string()),
        domain_attribution_status:
            constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_DOMAIN_OBSERVED.to_string(),
        process_attribution_status:
            constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_ATTRIBUTED.to_string(),
        process_id: Some(4242),
        process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
        counters: ActivityNetworkFlowCounters {
            connection_count: 1,
            bytes_sent: None,
            bytes_received: None,
            first_seen_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
            last_seen_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
        },
        evidence: vec![ActivityEvidenceRef {
            evidence_id: constants::activity_store::TEST_NETWORK_EVENT_ID.to_string(),
            kind: ActivityEvidenceKind::JournalEntry,
            digest: None,
            uri: None,
        }],
    }
}
