use std::collections::BTreeMap;

use ocentra_parent_agent_protocol::activity_capture::{
    ActivityCaptureCapabilityStatus, ActivityNetworkProtocol, ActivityNetworkTcpState,
};
use ocentra_parent_agent_protocol::constants;

use crate::network_capture::NetworkObservation;

#[path = "network_capture_netstat_state.rs"]
mod network_capture_netstat_state;

pub fn netstat_observations(
    output: &str,
    process_names: &BTreeMap<u32, String>,
) -> Vec<NetworkObservation> {
    let mut observations = output
        .lines()
        .filter_map(|line| netstat_line_observation(line, process_names))
        .collect::<Vec<_>>();
    observations.sort_by(network_observation_order);
    observations
}

fn netstat_line_observation(
    line: &str,
    process_names: &BTreeMap<u32, String>,
) -> Option<NetworkObservation> {
    let columns = line.split_whitespace().collect::<Vec<_>>();
    let protocol = columns.first()?;
    match *protocol {
        constants::activity_capture::NETSTAT_PROTOCOL_TCP => {
            tcp_observation(&columns, process_names)
        }
        constants::activity_capture::NETSTAT_PROTOCOL_UDP => {
            udp_observation(&columns, process_names)
        }
        _ => None,
    }
}

fn tcp_observation(
    columns: &[&str],
    process_names: &BTreeMap<u32, String>,
) -> Option<NetworkObservation> {
    let (local_ip, local_port) = endpoint_parts(columns.get(1)?)?;
    let (remote_ip, remote_port) = endpoint_parts(columns.get(2)?)?;
    let state = tcp_state_from_netstat(columns.get(3)?);
    let pid = columns.get(4).and_then(|value| value.parse::<u32>().ok());
    Some(NetworkObservation {
        status: ActivityCaptureCapabilityStatus::Available,
        protocol: Some(ActivityNetworkProtocol::Tcp),
        local_ip: Some(local_ip),
        local_port: Some(local_port),
        destination_ip: destination_ip(Some(remote_ip.as_str()), Some(remote_port)),
        destination_port: destination_port(Some(remote_ip.as_str()), Some(remote_port)),
        destination_domain: None,
        tcp_state: Some(state),
        pid,
        process_name: pid.and_then(|value| process_names.get(&value).cloned()),
        associated_pid_count: attributed_pid_count(pid),
    })
}

fn udp_observation(
    columns: &[&str],
    process_names: &BTreeMap<u32, String>,
) -> Option<NetworkObservation> {
    let (local_ip, local_port) = endpoint_parts(columns.get(1)?)?;
    let pid = columns.get(3).and_then(|value| value.parse::<u32>().ok());
    Some(NetworkObservation {
        status: ActivityCaptureCapabilityStatus::Available,
        protocol: Some(ActivityNetworkProtocol::Udp),
        local_ip: Some(local_ip),
        local_port: Some(local_port),
        destination_ip: None,
        destination_port: None,
        destination_domain: None,
        tcp_state: None,
        pid,
        process_name: pid.and_then(|value| process_names.get(&value).cloned()),
        associated_pid_count: attributed_pid_count(pid),
    })
}

fn endpoint_parts(endpoint: &str) -> Option<(String, u16)> {
    if endpoint == constants::activity_capture::NETSTAT_WILDCARD_ENDPOINT {
        return None;
    }
    bracketed_endpoint_parts(endpoint).or_else(|| plain_endpoint_parts(endpoint))
}

fn bracketed_endpoint_parts(endpoint: &str) -> Option<(String, u16)> {
    if !endpoint.starts_with(constants::delimiter::OPEN_BRACKET) {
        return None;
    }
    let close_index = endpoint.find(constants::delimiter::CLOSE_BRACKET)?;
    let port = endpoint
        .get(close_index + 1..)?
        .strip_prefix(constants::delimiter::COLON)?
        .parse::<u16>()
        .ok()?;
    Some((endpoint.get(1..close_index)?.to_string(), port))
}

fn plain_endpoint_parts(endpoint: &str) -> Option<(String, u16)> {
    let (ip, port) = endpoint.rsplit_once(constants::delimiter::COLON)?;
    Some((ip.to_string(), port.parse::<u16>().ok()?))
}

fn destination_ip(remote_ip: Option<&str>, remote_port: Option<u16>) -> Option<String> {
    let ip = remote_ip?;
    if remote_port == Some(0) || is_unspecified_ip(ip) {
        return None;
    }
    Some(ip.to_string())
}

fn destination_port(remote_ip: Option<&str>, remote_port: Option<u16>) -> Option<u16> {
    let ip = remote_ip?;
    if remote_port == Some(0) || is_unspecified_ip(ip) {
        return None;
    }
    remote_port
}

fn is_unspecified_ip(value: &str) -> bool {
    value == constants::activity_capture::NETSTAT_UNSPECIFIED_IPV4
        || value == constants::activity_capture::NETSTAT_UNSPECIFIED_IPV6
}

fn tcp_state_from_netstat(state: &str) -> ActivityNetworkTcpState {
    network_capture_netstat_state::tcp_state_from_netstat(state)
}

fn attributed_pid_count(pid: Option<u32>) -> usize {
    usize::from(pid.is_some())
}

fn network_observation_order(
    left: &NetworkObservation,
    right: &NetworkObservation,
) -> std::cmp::Ordering {
    left.local_ip
        .cmp(&right.local_ip)
        .then_with(|| left.local_port.cmp(&right.local_port))
        .then_with(|| left.destination_ip.cmp(&right.destination_ip))
        .then_with(|| left.destination_port.cmp(&right.destination_port))
        .then_with(|| left.pid.cmp(&right.pid))
}
