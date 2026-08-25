use std::net::TcpStream;
use std::sync::atomic::AtomicBool;
use std::time::{Duration, Instant};

use super::{SsdpDeviceDescription, SsdpDiscoveryError};

mod fetch;
mod parse;

pub(super) fn fetch_ssdp_description(
    location: &str,
    timeout: Duration,
) -> Result<SsdpDeviceDescription, SsdpDiscoveryError> {
    fetch::fetch_ssdp_description(location, timeout)
}

pub(super) fn fetch_ssdp_description_until(
    location: &str,
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Result<SsdpDeviceDescription, SsdpDiscoveryError> {
    fetch::fetch_ssdp_description_until(location, deadline, cancellation)
}

pub(super) fn read_http_response(stream: &mut TcpStream) -> Result<Vec<u8>, SsdpDiscoveryError> {
    fetch::read_http_response(stream)
}

pub(super) fn parse_device_description_response(
    response: &[u8],
    description_url: &str,
) -> Result<SsdpDeviceDescription, SsdpDiscoveryError> {
    parse::parse_device_description_response(response, description_url)
}

pub(super) fn parse_device_description_xml(
    xml: &str,
    description_url: &str,
) -> Result<SsdpDeviceDescription, SsdpDiscoveryError> {
    parse::parse_device_description_xml(xml, description_url)
}
