use ocentra_parent_agent_protocol::constants;

#[test]
fn collect_network_snapshot_returns_empty_when_limit_is_zero() {
    let observations = super::collect_network_snapshot(0);
    assert!(observations.is_empty());
}

#[cfg(windows)]
#[test]
fn collect_network_snapshot_observes_current_process_socket() {
    let listener = std::net::TcpListener::bind(constants::test_network::LOOPBACK_ANY_PORT)
        .expect(constants::error::LOCALHOST_BIND_SUCCEEDS);
    let local_port = listener
        .local_addr()
        .expect(constants::error::NETWORK_CAPTURE_OBSERVES_SOCKET)
        .port();
    let current_pid = std::process::id();

    let observations = super::collect_network_snapshot(usize::MAX);

    assert!(observations
        .iter()
        .any(|o| o.pid == Some(current_pid) && o.local_port == Some(local_port)));
}

#[cfg(windows)]
#[test]
fn collect_network_snapshot_with_limit_one_returns_at_most_one_observation() {
    let _listener = std::net::TcpListener::bind(constants::test_network::LOOPBACK_ANY_PORT)
        .expect(constants::error::LOCALHOST_BIND_SUCCEEDS);

    let observations = super::collect_network_snapshot(1);
    assert!(observations.len() <= 1);
}
