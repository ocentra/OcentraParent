use std::net::{IpAddr, SocketAddr};

use super::super::{AllowedHttpLocation, SsdpDiscoveryError};

mod access;
mod authority;

pub(super) fn parse_allowed_http_location(
    location: &str,
) -> Result<AllowedHttpLocation, SsdpDiscoveryError> {
    access::parse_allowed_http_location(location)
}

pub(super) fn parse_authority(authority: &str) -> Result<(&str, u16), SsdpDiscoveryError> {
    authority::parse_authority(authority)
}

pub(super) fn parse_port(value: &str) -> Result<u16, SsdpDiscoveryError> {
    authority::parse_port(value)
}

pub(super) fn resolve_allowed_host(
    host: &str,
    port: u16,
) -> Result<SocketAddr, SsdpDiscoveryError> {
    access::resolve_allowed_host(host, port)
}

pub(super) fn is_allowed_private_ip(ip: IpAddr) -> bool {
    access::is_allowed_private_ip(ip)
}

pub(super) fn sanitize_path(path: &str) -> Result<String, SsdpDiscoveryError> {
    access::sanitize_path(path)
}
