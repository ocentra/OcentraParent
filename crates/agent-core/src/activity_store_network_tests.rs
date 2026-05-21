use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, ActivityNetworkProtocol, ActivityNetworkTcpState,
};

use super::{network_observation_event, ActivityStore, NetworkObservation};

#[test]
fn activity_store_returns_recent_network_rows() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);
    let observation = NetworkObservation {
        status: ActivityCaptureCapabilityStatus::Available,
        protocol: Some(ActivityNetworkProtocol::Tcp),
        local_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
        local_port: Some(4242),
        destination_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
        destination_port: Some(443),
        destination_domain: Some(constants::test_network::TEST_DOMAIN.to_string()),
        tcp_state: Some(ActivityNetworkTcpState::Established),
        pid: Some(std::process::id()),
        process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
        associated_pid_count: 1,
    };
    let event = network_observation_event(
        observation,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        0,
    );

    store
        .ingest_events(std::slice::from_ref(&event))
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let rows = store
        .recent_network_rows(constants::activity_store::DEFAULT_RECENT_LIMIT)
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].event_id, event.event_id);
    assert_eq!(
        rows[0]
            .fields
            .get(constants::field::PROCESS_NAME)
            .and_then(|value| match value {
                ocentra_parent_agent_protocol::LogFieldValue::String(value) => Some(value.as_str()),
                _ => None,
            }),
        Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME)
    );
    assert_eq!(rows[0].evidence.len(), 0);
}
