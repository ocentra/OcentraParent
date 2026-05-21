use super::{
    constants, ActivityNetworkEndpoint, ActivityNetworkFlowCounters,
    ActivityNetworkFlowObservation, ActivityNetworkFlowReadModel,
    NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE, NETWORK_FLOW_SCHEMA_VERSION,
};

#[test]
fn network_flow_observation_serializes_to_contract_shape() {
    let observation = ActivityNetworkFlowObservation {
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
        evidence: Vec::new(),
    };

    let serialized =
        serde_json::to_value(observation).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], NETWORK_FLOW_SCHEMA_VERSION);
    assert_eq!(
        serialized["destinationEndpoint"]["port"],
        constants::activity_store::TEST_NETWORK_DESTINATION_PORT
    );
    assert_eq!(
        serialized["destinationDomain"],
        constants::activity_store::TEST_NETWORK_DOMAIN
    );
    assert_eq!(
        serialized["counters"]["connectionCount"],
        serde_json::json!(1)
    );
}

#[test]
fn network_flow_read_model_serializes_rows_without_payload_claims() {
    let read_model = ActivityNetworkFlowReadModel {
        schema_version: NETWORK_FLOW_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        custody: NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: 0,
        capability_status: constants::activity_capture::CAPABILITY_STATUS_NO_NETWORK_OBSERVATIONS
            .to_string(),
        rows: Vec::new(),
    };

    let serialized =
        serde_json::to_value(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["custody"],
        NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE
    );
    assert_eq!(
        serialized["capabilityStatus"],
        constants::activity_capture::CAPABILITY_STATUS_NO_NETWORK_OBSERVATIONS
    );
    assert_eq!(serialized["rows"].as_array().map(Vec::len), Some(0));
}
