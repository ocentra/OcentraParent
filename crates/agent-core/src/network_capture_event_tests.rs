use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, ActivityEvent, ActivityEventKind,
    ActivityNetworkProtocol, ActivityObserver, ActivitySubjectKind, LogFieldValue,
};

use super::{network_observation_event, NetworkObservation};

fn assert_string_field(event: &ActivityEvent, key: &str, value: &str) {
    assert_eq!(
        event.fields.get(key),
        Some(&LogFieldValue::String(value.to_string()))
    );
}

fn assert_number_field(event: &ActivityEvent, key: &str, value: f64) {
    assert_eq!(event.fields.get(key), Some(&LogFieldValue::Number(value)));
}

fn assert_field_absent(event: &ActivityEvent, key: &str) {
    assert!(!event.fields.contains_key(key));
}

fn ip_only_observation() -> NetworkObservation {
    NetworkObservation {
        status: ActivityCaptureCapabilityStatus::Available,
        protocol: Some(ActivityNetworkProtocol::Tcp),
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
fn network_observation_event_maps_ip_only_socket_contract() {
    let event = network_observation_event(
        ip_only_observation(),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        0,
    );

    assert_eq!(event.source.observer, ActivityObserver::WindowsNetwork);
    assert_eq!(
        event.source.source_id,
        constants::activity_capture::WINDOWS_NETWORK_SOURCE_ID
    );
    assert_eq!(event.kind, ActivityEventKind::DomainObserved);
    assert_eq!(event.subject.kind, ActivitySubjectKind::Domain);
    assert_eq!(
        event.subject.subject_id,
        constants::test_network::SUBJECT_ID
    );
    assert_string_field(
        &event,
        constants::field::NETWORK_PROTOCOL,
        constants::activity_capture::NETWORK_PROTOCOL_TCP,
    );
    assert_string_field(
        &event,
        constants::field::DOMAIN_ATTRIBUTION_STATUS,
        constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_IP_ONLY,
    );
    assert_string_field(
        &event,
        constants::field::PROCESS_ATTRIBUTION_STATUS,
        constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_ATTRIBUTED,
    );
    assert_string_field(
        &event,
        constants::field::DESTINATION_IP,
        constants::test_network::LOOPBACK_IP,
    );
    assert_number_field(&event, constants::field::DESTINATION_PORT, 443.0);
    assert_string_field(
        &event,
        constants::field::TCP_STATE,
        constants::activity_capture::TCP_STATE_ESTABLISHED,
    );
}

#[test]
fn network_observation_event_maps_domain_observed_contract() {
    let obs = NetworkObservation {
        destination_domain: Some(constants::test_network::TEST_DOMAIN.to_string()),
        destination_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
        destination_port: Some(443),
        ..ip_only_observation()
    };
    let event =
        network_observation_event(obs, constants::activity_store::TEST_FIRST_OBSERVED_AT, 0);

    assert_string_field(
        &event,
        constants::field::DOMAIN_ATTRIBUTION_STATUS,
        constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_DOMAIN_OBSERVED,
    );
    let mut expected_subject_id =
        constants::activity_capture::NETWORK_SUBJECT_ID_PREFIX.to_string();
    expected_subject_id.push_str(constants::test_network::TEST_DOMAIN);
    assert_eq!(event.subject.subject_id, expected_subject_id);
    assert_eq!(
        event.subject.display_name,
        Some(constants::test_network::TEST_DOMAIN.to_string())
    );
}

#[test]
fn network_observation_event_maps_degraded_status_contract() {
    let event = network_observation_event(
        NetworkObservation::degraded(ActivityCaptureCapabilityStatus::NoNetworkObservations),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        0,
    );

    assert_eq!(event.source.observer, ActivityObserver::WindowsNetwork);
    assert_eq!(event.subject.kind, ActivitySubjectKind::Domain);
    assert_string_field(
        &event,
        constants::field::CAPABILITY_STATUS,
        constants::activity_capture::CAPABILITY_STATUS_NO_NETWORK_OBSERVATIONS,
    );
    assert_string_field(
        &event,
        constants::field::DOMAIN_ATTRIBUTION_STATUS,
        constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_UNAVAILABLE,
    );
    assert_string_field(
        &event,
        constants::field::PROCESS_ATTRIBUTION_STATUS,
        constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_UNKNOWN,
    );
    assert_field_absent(&event, constants::field::DESTINATION_IP);
}

#[test]
fn network_observation_event_maps_adapter_error_contract() {
    let event = network_observation_event(
        NetworkObservation::degraded(ActivityCaptureCapabilityStatus::AdapterError),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        0,
    );

    assert_string_field(
        &event,
        constants::field::CAPABILITY_STATUS,
        constants::activity_capture::CAPABILITY_STATUS_ADAPTER_ERROR,
    );
    assert_field_absent(&event, constants::field::NETWORK_PROTOCOL);
    assert_field_absent(&event, constants::field::DESTINATION_IP);
}

#[test]
fn network_observation_event_maps_udp_protocol_field() {
    let obs = NetworkObservation {
        protocol: Some(ActivityNetworkProtocol::Udp),
        tcp_state: None,
        ..ip_only_observation()
    };
    let event =
        network_observation_event(obs, constants::activity_store::TEST_FIRST_OBSERVED_AT, 0);

    assert_string_field(
        &event,
        constants::field::NETWORK_PROTOCOL,
        constants::activity_capture::NETWORK_PROTOCOL_UDP,
    );
    assert_field_absent(&event, constants::field::TCP_STATE);
}

#[test]
fn network_observation_event_id_embeds_sequence_and_timestamp() {
    let observed_at = constants::activity_store::TEST_FIRST_OBSERVED_AT;
    let event = network_observation_event(ip_only_observation(), observed_at, 7);

    assert!(event
        .event_id
        .contains(constants::test_network::TEST_SEQUENCE_INDEX_STR));
    assert!(event.event_id.contains(observed_at));
    assert!(event
        .event_id
        .starts_with(constants::activity_capture::NETWORK_EVENT_ID_PREFIX));
}

#[test]
fn network_observation_event_associated_pid_count_field_is_present() {
    let event = network_observation_event(
        ip_only_observation(),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        0,
    );
    assert_number_field(&event, constants::field::ASSOCIATED_PID_COUNT, 1.0);
}
