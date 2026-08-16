use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

use super::{AllowedHttpLocation, SsdpDiscoveryError, SSDP_MAX_MX_SECONDS};

pub mod text;

mod location;

pub type SplitHttpHeaders<'a> = (&'a str, HashMap<String, String>, &'a [u8]);

pub fn split_http_headers(response: &[u8]) -> Result<SplitHttpHeaders<'_>, SsdpDiscoveryError> {
    let header_end = response
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .ok_or(SsdpDiscoveryError::MalformedResponse)?;
    let header_bytes = &response[..header_end];
    let status_line_bytes = header_bytes
        .split(|byte| *byte == b'\n')
        .next()
        .ok_or(SsdpDiscoveryError::MalformedResponse)?;
    let status_line = std::str::from_utf8(status_line_bytes)
        .map_err(|_error| SsdpDiscoveryError::MalformedResponse)?
        .trim_end_matches('\r');
    let mut headers = HashMap::new();
    let header_text = std::str::from_utf8(header_bytes)
        .map_err(|_error| SsdpDiscoveryError::MalformedResponse)?;
    for line in header_text.lines().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let Some((name, value)) = line.split_once(':') else {
            return Err(SsdpDiscoveryError::MalformedResponse);
        };
        if name.trim().is_empty() {
            return Err(SsdpDiscoveryError::MalformedResponse);
        }
        headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_string());
    }
    Ok((status_line, headers, &response[(header_end + 4)..]))
}

pub fn normalized_header_value(headers: &HashMap<String, String>, name: &str) -> Option<String> {
    header_value(headers, name).and_then(|value| normalize_http_header_value(&value))
}

pub fn header_value(headers: &HashMap<String, String>, name: &str) -> Option<String> {
    headers.get(&name.to_ascii_lowercase()).cloned()
}

pub fn parse_device_type(value: &str) -> Option<String> {
    let value = value.trim();
    let value = value.split(['#', '?']).next().unwrap_or(value);
    let value = value.split('/').next_back().unwrap_or(value);
    (!value.is_empty()).then_some(value.to_string())
}

pub fn normalize_http_header_value(value: &str) -> Option<String> {
    let value = value.trim();
    (!value.is_empty()).then_some(value.to_string())
}

pub fn parse_http_status_code(status_line: &str) -> Result<u16, SsdpDiscoveryError> {
    let mut parts = status_line.split_whitespace();
    let _http_version = parts.next().ok_or(SsdpDiscoveryError::MalformedResponse)?;
    let status_code = parts.next().ok_or(SsdpDiscoveryError::MalformedResponse)?;
    status_code
        .parse::<u16>()
        .map_err(|_error| SsdpDiscoveryError::MalformedResponse)
}

pub fn is_infrastructure_device(device_type: Option<&str>, search_target: &str, usn: &str) -> bool {
    let values = [device_type, Some(search_target), Some(usn)];
    values.into_iter().flatten().any(|value| {
        let value = value.to_ascii_lowercase();
        value.contains("internetgatewaydevice")
            || value.contains("router")
            || value.contains("bridge")
            || value.contains("switch")
    })
}

pub fn parse_allowed_http_location(
    location: &str,
) -> Result<AllowedHttpLocation, SsdpDiscoveryError> {
    location::parse_allowed_http_location(location)
}

pub fn parse_authority(authority: &str) -> Result<(&str, u16), SsdpDiscoveryError> {
    location::parse_authority(authority)
}

pub fn parse_port(value: &str) -> Result<u16, SsdpDiscoveryError> {
    location::parse_port(value)
}

pub fn resolve_allowed_host(host: &str, port: u16) -> Result<SocketAddr, SsdpDiscoveryError> {
    location::resolve_allowed_host(host, port)
}

pub fn is_allowed_private_ip(ip: IpAddr) -> bool {
    location::is_allowed_private_ip(ip)
}

pub fn sanitize_path(path: &str) -> Result<String, SsdpDiscoveryError> {
    location::sanitize_path(path)
}

pub fn normalize_search_target(search_target: &str) -> String {
    let search_target = search_target.trim();
    if search_target.is_empty() {
        return "ssdp:all".to_string();
    }
    search_target.to_ascii_lowercase()
}

pub fn mx_seconds_for_timeout(timeout: Duration) -> u8 {
    let seconds = timeout.as_secs().max(1);
    u8::try_from(seconds.min(u64::from(SSDP_MAX_MX_SECONDS)))
        .ok()
        .unwrap_or(SSDP_MAX_MX_SECONDS)
}

pub fn io_error(error: &std::io::Error) -> SsdpDiscoveryError {
    match error.kind() {
        std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock => {
            SsdpDiscoveryError::Timeout
        }
        _ => SsdpDiscoveryError::Io(error.to_string()),
    }
}
