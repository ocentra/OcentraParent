#[cfg(windows)]
use std::collections::BTreeMap;

use ocentra_parent_agent_protocol::ActivityCaptureCapabilityStatus;
#[cfg(windows)]
use ocentra_parent_agent_protocol::{ActivityNetworkProtocol, ActivityNetworkTcpState};
#[cfg(windows)]
use sysinfo::{ProcessesToUpdate, System};

use crate::network_capture::NetworkObservation;

#[cfg(windows)]
fn process_names_by_pid() -> BTreeMap<u32, String> {
    let mut system = System::new();
    system.refresh_processes(ProcessesToUpdate::All, true);
    system
        .processes()
        .values()
        .map(|process| {
            (
                process.pid().as_u32(),
                process.name().to_string_lossy().into_owned(),
            )
        })
        .collect()
}

#[cfg(windows)]
pub fn platform_network_snapshot(limit: usize) -> Result<Vec<NetworkObservation>, ()> {
    use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};

    let process_names = process_names_by_pid();
    let socket_info = get_sockets_info(
        AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6,
        ProtocolFlags::TCP | ProtocolFlags::UDP,
    )
    .map_err(|_| ())?;
    let mut observations = socket_info
        .into_iter()
        .map(|socket| match socket.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp) => {
                tcp_observation(socket.associated_pids, tcp, &process_names)
            }
            ProtocolSocketInfo::Udp(udp) => {
                udp_observation(socket.associated_pids, udp, &process_names)
            }
        })
        .collect::<Vec<_>>();
    observations.sort_by(network_observation_order);
    observations.truncate(limit);
    Ok(observations)
}

#[cfg(windows)]
fn tcp_observation(
    associated_pids: Vec<u32>,
    tcp: netstat2::TcpSocketInfo,
    process_names: &BTreeMap<u32, String>,
) -> NetworkObservation {
    let destination_ip = network_destination_ip(tcp.remote_addr, tcp.remote_port);
    let destination_port = destination_ip.as_ref().map(|_| tcp.remote_port);
    let pid = associated_pids.first().copied();
    NetworkObservation {
        status: ActivityCaptureCapabilityStatus::Available,
        protocol: Some(ActivityNetworkProtocol::Tcp),
        local_ip: Some(tcp.local_addr.to_string()),
        local_port: Some(tcp.local_port),
        destination_ip,
        destination_port,
        destination_domain: None,
        tcp_state: Some(tcp_state_from_netstat(tcp.state)),
        pid,
        process_name: pid.and_then(|value| process_names.get(&value).cloned()),
        associated_pid_count: associated_pids.len(),
    }
}

#[cfg(windows)]
fn udp_observation(
    associated_pids: Vec<u32>,
    udp: netstat2::UdpSocketInfo,
    process_names: &BTreeMap<u32, String>,
) -> NetworkObservation {
    let pid = associated_pids.first().copied();
    NetworkObservation {
        status: ActivityCaptureCapabilityStatus::Available,
        protocol: Some(ActivityNetworkProtocol::Udp),
        local_ip: Some(udp.local_addr.to_string()),
        local_port: Some(udp.local_port),
        destination_ip: None,
        destination_port: None,
        destination_domain: None,
        tcp_state: None,
        pid,
        process_name: pid.and_then(|value| process_names.get(&value).cloned()),
        associated_pid_count: associated_pids.len(),
    }
}

#[cfg(windows)]
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

#[cfg(windows)]
fn network_destination_ip(remote_addr: std::net::IpAddr, remote_port: u16) -> Option<String> {
    if remote_port == 0 || ip_is_unspecified(remote_addr) {
        return None;
    }
    Some(remote_addr.to_string())
}

#[cfg(windows)]
fn ip_is_unspecified(addr: std::net::IpAddr) -> bool {
    match addr {
        std::net::IpAddr::V4(ip) => ip.is_unspecified(),
        std::net::IpAddr::V6(ip) => ip.is_unspecified(),
    }
}

#[cfg(windows)]
fn tcp_state_from_netstat(state: netstat2::TcpState) -> ActivityNetworkTcpState {
    match state {
        netstat2::TcpState::Closed => ActivityNetworkTcpState::Closed,
        netstat2::TcpState::Listen => ActivityNetworkTcpState::Listen,
        netstat2::TcpState::SynSent => ActivityNetworkTcpState::SynSent,
        netstat2::TcpState::SynReceived => ActivityNetworkTcpState::SynReceived,
        netstat2::TcpState::Established => ActivityNetworkTcpState::Established,
        netstat2::TcpState::FinWait1 => ActivityNetworkTcpState::FinWait1,
        netstat2::TcpState::FinWait2 => ActivityNetworkTcpState::FinWait2,
        netstat2::TcpState::CloseWait => ActivityNetworkTcpState::CloseWait,
        netstat2::TcpState::Closing => ActivityNetworkTcpState::Closing,
        netstat2::TcpState::LastAck => ActivityNetworkTcpState::LastAck,
        netstat2::TcpState::TimeWait => ActivityNetworkTcpState::TimeWait,
        netstat2::TcpState::DeleteTcb => ActivityNetworkTcpState::DeleteTcb,
        netstat2::TcpState::Unknown => ActivityNetworkTcpState::Unknown,
    }
}

#[cfg(not(windows))]
pub fn platform_network_snapshot(_: usize) -> Result<Vec<NetworkObservation>, ()> {
    Ok(vec![NetworkObservation::degraded(
        ActivityCaptureCapabilityStatus::Unavailable,
    )])
}
