use std::net::{IpAddr, SocketAddr};

use super::super::super::SsdpDiscoveryError;
use super::authority::parse_authority;

pub(super) fn parse_allowed_http_location(
    location: &str,
) -> Result<super::super::super::AllowedHttpLocation, SsdpDiscoveryError> {
    let location = location.trim();
    let location = location
        .strip_prefix("http://")
        .ok_or(SsdpDiscoveryError::UnsupportedLocationScheme)?;
    let (authority, path) = location
        .split_once('/')
        .map(|(authority, path)| (authority, format!("/{path}")))
        .unwrap_or((location, "/".to_string()));
    let (host, port) = parse_authority(authority)?;
    let addr = resolve_allowed_host(host, port)?;
    let path = sanitize_path(&path)?;
    Ok(super::super::super::AllowedHttpLocation { addr, path })
}

pub(super) fn resolve_allowed_host(
    host: &str,
    port: u16,
) -> Result<SocketAddr, SsdpDiscoveryError> {
    let ip = host
        .parse::<IpAddr>()
        .map_err(|_error| SsdpDiscoveryError::ExternalLocation)?;
    is_allowed_private_ip(ip)
        .then_some(SocketAddr::new(ip, port))
        .ok_or(SsdpDiscoveryError::ExternalLocation)
}

pub(super) fn is_allowed_private_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => {
            ip.is_private()
                || ip.is_loopback()
                || ip.octets()[0] == 169 && ip.octets()[1] == 254
                || matches!(ip.octets(), [100, 64..=127, _, _])
        }
        IpAddr::V6(ip) => ip.is_loopback() || ip.is_unique_local(),
    }
}

pub(super) fn sanitize_path(path: &str) -> Result<String, SsdpDiscoveryError> {
    let path = path.trim();
    if path.is_empty() || !path.starts_with('/') || path.contains("..") {
        return Err(SsdpDiscoveryError::MalformedResponse);
    }
    Ok(path.to_string())
}
