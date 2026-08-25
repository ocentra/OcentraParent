use std::net::SocketAddr;

#[cfg(windows)]
use std::net::IpAddr;

use ocentra_parent_agent_protocol::constants;

use crate::browser_bridge_poll::BrowserBridgePollError;

#[cfg(windows)]
pub(super) fn verify_endpoint_owner(
    endpoint: SocketAddr,
    process_id: u32,
) -> Result<(), BrowserBridgePollError> {
    if !endpoint.ip().is_loopback() {
        return Err(BrowserBridgePollError::NonLoopbackEndpoint);
    }
    let output = std::process::Command::new(constants::activity_capture::NETSTAT_COMMAND)
        .args(constants::activity_capture::NETSTAT_ARGS)
        .output()
        .map_err(|_error| BrowserBridgePollError::ManualRequired)?;
    if !output.status.success() {
        return Err(BrowserBridgePollError::ManualRequired);
    }
    let output = String::from_utf8(output.stdout)
        .map_err(|_error| BrowserBridgePollError::ManualRequired)?;
    let owner = output
        .lines()
        .filter_map(|line| listener_owner(line, endpoint))
        .next()
        .ok_or(BrowserBridgePollError::ManualRequired)?;
    if owner == process_id {
        Ok(())
    } else {
        Err(BrowserBridgePollError::UntrustedProcess)
    }
}

#[cfg(not(windows))]
pub(super) fn verify_endpoint_owner(
    endpoint: SocketAddr,
    process_id: u32,
) -> Result<(), BrowserBridgePollError> {
    let _ = (endpoint, process_id);
    Err(BrowserBridgePollError::ManualRequired)
}

#[cfg(windows)]
fn listener_owner(line: &str, endpoint: SocketAddr) -> Option<u32> {
    let columns = line.split_whitespace().collect::<Vec<_>>();
    if columns.first().copied() != Some(constants::activity_capture::NETSTAT_PROTOCOL_TCP)
        || columns.get(3).copied() != Some(constants::activity_capture::NETSTAT_STATE_LISTENING)
    {
        return None;
    }
    if parse_socket_addr(columns.get(1)?)? != endpoint {
        return None;
    }
    columns.get(4)?.parse::<u32>().ok()
}

#[cfg(windows)]
fn parse_socket_addr(value: &str) -> Option<SocketAddr> {
    let (host, port) = value.rsplit_once(constants::delimiter::COLON)?;
    let host = host
        .strip_prefix(constants::delimiter::OPEN_BRACKET)
        .unwrap_or(host)
        .strip_suffix(constants::delimiter::CLOSE_BRACKET)
        .unwrap_or(host);
    Some(SocketAddr::new(
        host.parse::<IpAddr>().ok()?,
        port.parse::<u16>().ok()?,
    ))
}
