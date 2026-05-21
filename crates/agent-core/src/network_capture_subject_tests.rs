use ocentra_parent_agent_protocol::{constants, ActivityCaptureCapabilityStatus};

use super::{network_observation_event, NetworkObservation};

fn ip_only_observation() -> NetworkObservation {
    NetworkObservation {
        status: ActivityCaptureCapabilityStatus::Available,
        protocol: Some(ocentra_parent_agent_protocol::ActivityNetworkProtocol::Tcp),
        local_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
        local_port: Some(4242),
        destination_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
        destination_port: Some(443),
        destination_domain: None,
        tcp_state: Some(ocentra_parent_agent_protocol::ActivityNetworkTcpState::Established),
        pid: Some(4242),
        process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
        associated_pid_count: 1,
    }
}

#[test]
fn network_observation_event_display_name_uses_ip_colon_port_when_no_domain() {
    let event = network_observation_event(
        ip_only_observation(),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        0,
    );
    assert_eq!(
        event.subject.display_name,
        Some(constants::test_network::LOOPBACK_IP_WITH_PORT.to_string())
    );
}

#[test]
fn network_observation_event_display_name_is_none_when_no_address_info() {
    let obs = NetworkObservation::degraded(ActivityCaptureCapabilityStatus::NoNetworkObservations);
    let event =
        network_observation_event(obs, constants::activity_store::TEST_FIRST_OBSERVED_AT, 0);
    assert_eq!(event.subject.display_name, None);
}
