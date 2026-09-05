use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

use super::http::{io_error, mx_seconds_for_timeout, normalize_search_target};
use super::{
    SsdpDiscoveryError, SsdpDiscoveryRecord, SSDP_DISCOVERY_TIMEOUT_MS, SSDP_MAX_ATTEMPTS,
    SSDP_MAX_MX_SECONDS,
};

pub(super) fn build_msearch_request(
    search_target: &str,
    host: SocketAddr,
    mx_seconds: u8,
) -> Vec<u8> {
    let st = normalize_search_target(search_target);
    let mx = mx_seconds.clamp(1, SSDP_MAX_MX_SECONDS);
    format!(
        "M-SEARCH * HTTP/1.1\r\nHOST: {host}\r\nMAN: \"ssdp:discover\"\r\nMX: {mx}\r\nST: {st}\r\nUSER-AGENT: ocentra-parent/lan-core\r\nCONNECTION: close\r\n\r\n"
    )
    .into_bytes()
}

pub(super) fn discover_ssdp_upnp_devices(
    search_target: &str,
    target: SocketAddr,
    response_timeout: Duration,
    attempts: usize,
    description_timeout: Duration,
) -> Result<Vec<SsdpDiscoveryRecord>, SsdpDiscoveryError> {
    discover_ssdp_upnp_devices_with_cancellation(
        search_target,
        target,
        response_timeout,
        attempts,
        description_timeout,
        None,
        None,
    )
}

pub(super) fn discover_ssdp_upnp_devices_with_cancellation(
    search_target: &str,
    target: SocketAddr,
    response_timeout: Duration,
    attempts: usize,
    description_timeout: Duration,
    cancellation: Option<&AtomicBool>,
    deadline: Option<Instant>,
) -> Result<Vec<SsdpDiscoveryRecord>, SsdpDiscoveryError> {
    if cancellation.is_some_and(|value| value.load(std::sync::atomic::Ordering::Acquire)) {
        return Ok(Vec::new());
    }
    let socket = bind_ssdp_socket(target)?;
    socket
        .set_read_timeout(Some(response_timeout))
        .map_err(|error| io_error(&error))?;
    let request = build_msearch_request(
        search_target,
        target,
        mx_seconds_for_timeout(response_timeout),
    );
    super::receive::collect_ssdp_records_with_cancellation(&super::receive::SsdpCollectionRequest {
        socket: &socket,
        request: &request,
        target,
        response_timeout,
        attempts: attempts.clamp(1, SSDP_MAX_ATTEMPTS),
        description_timeout,
        cancellation,
        outer_deadline: deadline,
    })
}

pub(super) fn discover_ssdp_upnp_records() -> Result<Vec<SsdpDiscoveryRecord>, SsdpDiscoveryError> {
    discover_ssdp_upnp_records_at(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(239, 255, 255, 250)),
        1900,
    ))
}

pub(super) fn discover_ssdp_upnp_records_at(
    target: SocketAddr,
) -> Result<Vec<SsdpDiscoveryRecord>, SsdpDiscoveryError> {
    discover_ssdp_upnp_devices(
        "ssdp:all",
        target,
        Duration::from_millis(SSDP_DISCOVERY_TIMEOUT_MS),
        1,
        Duration::from_millis(SSDP_DISCOVERY_TIMEOUT_MS),
    )
}

fn bind_ssdp_socket(target: SocketAddr) -> Result<UdpSocket, SsdpDiscoveryError> {
    let bind_address = match target {
        SocketAddr::V4(_) => SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0),
        SocketAddr::V6(_) => SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0),
    };
    UdpSocket::bind(bind_address).map_err(|error| io_error(&error))
}
