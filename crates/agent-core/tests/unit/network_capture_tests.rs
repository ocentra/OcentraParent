use ocentra_parent_agent_protocol::activity::{
    ActivityEventKind, ActivityObserver, ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::activity_capture::{
    ActivityCaptureCapabilityStatus, ActivityNetworkProtocol, ActivityNetworkTcpState,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

use crate::{network_observation_event, NetworkObservation};

#[cfg(windows)]
#[derive(Debug)]
struct TestError(String);

#[cfg(windows)]
impl std::fmt::Display for TestError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[cfg(windows)]
impl std::error::Error for TestError {}

#[cfg(windows)]
type TestResult = Result<(), TestError>;

#[test]
fn network_observation_event_maps_ip_only_socket_contract() {
    let event = network_observation_event(
        NetworkObservation {
            status: ActivityCaptureCapabilityStatus::Available,
            protocol: Some(ActivityNetworkProtocol::Tcp),
            local_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
            local_port: Some(4242),
            destination_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
            destination_port: Some(443),
            destination_domain: None,
            tcp_state: Some(ActivityNetworkTcpState::Established),
            pid: Some(4242),
            process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
            associated_pid_count: 1,
        },
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
    assert_eq!(
        event.fields.get(constants::field::NETWORK_PROTOCOL),
        Some(&LogFieldValue::String(
            constants::activity_capture::NETWORK_PROTOCOL_TCP.to_string()
        ))
    );
    assert_eq!(
        event
            .fields
            .get(constants::field::DOMAIN_ATTRIBUTION_STATUS),
        Some(&LogFieldValue::String(
            constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_IP_ONLY.to_string()
        ))
    );
    assert_eq!(
        event
            .fields
            .get(constants::field::PROCESS_ATTRIBUTION_STATUS),
        Some(&LogFieldValue::String(
            constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_ATTRIBUTED.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::DESTINATION_IP),
        Some(&LogFieldValue::String(
            constants::test_network::LOOPBACK_IP.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::DESTINATION_PORT),
        Some(&LogFieldValue::Number(443.0))
    );
    assert_eq!(
        event.fields.get(constants::field::TCP_STATE),
        Some(&LogFieldValue::String(
            constants::activity_capture::TCP_STATE_ESTABLISHED.to_string()
        ))
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
    assert_eq!(
        event.fields.get(constants::field::CAPABILITY_STATUS),
        Some(&LogFieldValue::String(
            constants::activity_capture::CAPABILITY_STATUS_NO_NETWORK_OBSERVATIONS.to_string()
        ))
    );
    assert_eq!(
        event
            .fields
            .get(constants::field::DOMAIN_ATTRIBUTION_STATUS),
        Some(&LogFieldValue::String(
            constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_UNAVAILABLE.to_string()
        ))
    );
    assert_eq!(
        event
            .fields
            .get(constants::field::PROCESS_ATTRIBUTION_STATUS),
        Some(&LogFieldValue::String(
            constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_UNKNOWN.to_string()
        ))
    );
    assert!(event.fields.get(constants::field::DESTINATION_IP).is_none());
}

#[cfg(windows)]
#[test]
fn netstat_parser_maps_tcp_connection_to_network_observation() {
    let process_names = std::collections::BTreeMap::from([(
        4242,
        constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string(),
    )]);
    let observations = super::network_capture_netstat::netstat_observations(
        constants::test_network::NETSTAT_TCP_ESTABLISHED_ROW,
        &process_names,
    );

    assert_eq!(observations.len(), 1);
    assert_eq!(observations[0].protocol, Some(ActivityNetworkProtocol::Tcp));
    assert_eq!(
        observations[0].tcp_state,
        Some(ActivityNetworkTcpState::Established)
    );
    assert_eq!(
        observations[0].destination_ip,
        Some(constants::test_network::LOOPBACK_IP.to_string())
    );
    assert_eq!(observations[0].destination_port, Some(443));
    assert_eq!(observations[0].pid, Some(4242));
    assert_eq!(
        observations[0].process_name,
        Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string())
    );
}

#[cfg(windows)]
#[test]
fn collect_network_snapshot_observes_current_process_socket() -> TestResult {
    let listener = ok(
        std::net::TcpListener::bind(constants::test_network::LOOPBACK_ANY_PORT),
        constants::error::LOCALHOST_BIND_SUCCEEDS,
    )?;
    let local_port = ok(
        listener.local_addr(),
        constants::error::NETWORK_CAPTURE_OBSERVES_SOCKET,
    )?
    .port();
    let current_pid = std::process::id();

    let observations = super::collect_network_snapshot(usize::MAX);

    assert!(observations.iter().any(|observation| {
        observation.pid == Some(current_pid) && observation.local_port == Some(local_port)
    }));

    Ok(())
}

#[cfg(windows)]
fn ok<T, E: core::fmt::Debug>(
    result: Result<T, E>,
    context: impl std::fmt::Display,
) -> Result<T, TestError> {
    result.map_err(|error| TestError(format!("{context}: {error:?}")))
}
