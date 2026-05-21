use ocentra_parent_agent_protocol::{constants, ActivityNetworkProtocol, ActivityNetworkTcpState};

// ---------------------------------------------------------------------------
// netstat_parser — all branches (Windows only)
// ---------------------------------------------------------------------------

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
fn netstat_parser_maps_udp_row_to_observation() {
    let process_names = std::collections::BTreeMap::from([(
        1234u32,
        constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string(),
    )]);
    let row = constants::test_network::NETSTAT_UDP_ROW;
    let observations = super::network_capture_netstat::netstat_observations(row, &process_names);

    assert_eq!(observations.len(), 1);
    assert_eq!(observations[0].protocol, Some(ActivityNetworkProtocol::Udp));
    assert_eq!(observations[0].tcp_state, None);
    assert_eq!(
        observations[0].local_ip,
        Some(constants::test_network::LOOPBACK_IP.to_string())
    );
    assert_eq!(observations[0].local_port, Some(5353));
    assert_eq!(observations[0].destination_ip, None);
    assert_eq!(observations[0].destination_port, None);
    assert_eq!(observations[0].pid, Some(1234));
    assert_eq!(
        observations[0].process_name,
        Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string())
    );
}

#[cfg(windows)]
#[test]
fn netstat_parser_strips_unspecified_remote_ip_from_listening_row() {
    let process_names = std::collections::BTreeMap::new();
    let row = constants::test_network::NETSTAT_TCP_LISTENING_ROW;
    let observations = super::network_capture_netstat::netstat_observations(row, &process_names);

    assert_eq!(observations.len(), 1);
    assert_eq!(observations[0].destination_ip, None);
    assert_eq!(observations[0].destination_port, None);
    assert_eq!(
        observations[0].tcp_state,
        Some(ActivityNetworkTcpState::Listen)
    );
}

#[cfg(windows)]
#[test]
fn netstat_parser_maps_all_tcp_states() {
    let process_names = std::collections::BTreeMap::new();
    let cases: &[(&str, ActivityNetworkTcpState)] = &[
        (
            constants::activity_capture::NETSTAT_STATE_CLOSED,
            ActivityNetworkTcpState::Closed,
        ),
        (
            constants::activity_capture::NETSTAT_STATE_LISTENING,
            ActivityNetworkTcpState::Listen,
        ),
        (
            constants::activity_capture::NETSTAT_STATE_SYN_SENT,
            ActivityNetworkTcpState::SynSent,
        ),
        (
            constants::activity_capture::NETSTAT_STATE_SYN_RECEIVED,
            ActivityNetworkTcpState::SynReceived,
        ),
        (
            constants::activity_capture::NETSTAT_STATE_ESTABLISHED,
            ActivityNetworkTcpState::Established,
        ),
        (
            constants::activity_capture::NETSTAT_STATE_FIN_WAIT_1,
            ActivityNetworkTcpState::FinWait1,
        ),
        (
            constants::activity_capture::NETSTAT_STATE_FIN_WAIT_2,
            ActivityNetworkTcpState::FinWait2,
        ),
        (
            constants::activity_capture::NETSTAT_STATE_CLOSE_WAIT,
            ActivityNetworkTcpState::CloseWait,
        ),
        (
            constants::activity_capture::NETSTAT_STATE_CLOSING,
            ActivityNetworkTcpState::Closing,
        ),
        (
            constants::activity_capture::NETSTAT_STATE_LAST_ACK,
            ActivityNetworkTcpState::LastAck,
        ),
        (
            constants::activity_capture::NETSTAT_STATE_TIME_WAIT,
            ActivityNetworkTcpState::TimeWait,
        ),
        (
            constants::activity_capture::NETSTAT_STATE_DELETE_TCB,
            ActivityNetworkTcpState::DeleteTcb,
        ),
        (
            constants::test_network::TEST_BOGUS_STATE,
            ActivityNetworkTcpState::Unknown,
        ),
    ];

    for (netstat_state, expected) in cases {
        let mut row = constants::test_network::NETSTAT_TCP_STATE_ROW_PREFIX.to_string();
        row.push_str(netstat_state);
        row.push_str(constants::test_network::NETSTAT_TCP_STATE_ROW_SUFFIX);
        let observations =
            super::network_capture_netstat::netstat_observations(&row, &process_names);
        assert_eq!(observations.len(), 1);
        assert_eq!(observations[0].tcp_state, Some(expected.clone()));
    }
}

#[cfg(windows)]
#[test]
fn netstat_parser_skips_header_and_blank_lines() {
    let process_names = std::collections::BTreeMap::new();
    let output = constants::test_network::NETSTAT_MULTILINE_OUTPUT;
    let observations = super::network_capture_netstat::netstat_observations(output, &process_names);

    assert_eq!(observations.len(), 1);
}

#[cfg(windows)]
#[test]
fn netstat_parser_orders_multiple_rows_by_local_then_destination() {
    let process_names = std::collections::BTreeMap::new();
    let output = constants::test_network::NETSTAT_ORDERING_OUTPUT;
    let observations = super::network_capture_netstat::netstat_observations(output, &process_names);

    assert_eq!(observations.len(), 2);
    assert_eq!(observations[0].local_port, Some(1000));
    assert_eq!(observations[1].local_port, Some(9000));
}
