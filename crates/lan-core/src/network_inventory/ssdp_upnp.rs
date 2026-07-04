pub mod http;

use std::collections::HashSet;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream, UdpSocket};
use std::time::{Duration, Instant};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanServiceIdentityProbeEvidence, LanServiceIdentityProbeEvidenceKind,
};

use self::http::text::{extract_xml_text, parse_udn, short_ssdp_label};
use self::http::{
    io_error, is_infrastructure_device, mx_seconds_for_timeout, normalize_search_target,
    normalized_header_value, parse_allowed_http_location, parse_device_type,
    parse_http_status_code, split_http_headers,
};
use super::LanNetworkInventoryDevice;

pub const SSDP_MAX_RESPONSE_BYTES: usize = 16 * 1024;
pub const SSDP_MAX_DESCRIPTION_BYTES: usize = 64 * 1024;
pub const SSDP_MAX_DESCRIPTION_TEXT_BYTES: usize = 1_024;
pub const SSDP_MAX_ATTEMPTS: usize = 3;
pub const SSDP_MAX_MX_SECONDS: u8 = 5;
pub const SSDP_DISCOVERY_TIMEOUT_MS: u64 = 350;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SsdpDiscoveryResponse {
    pub location: String,
    pub search_target: String,
    pub usn: String,
    pub udn: Option<String>,
    pub device_type: Option<String>,
    pub infrastructure: bool,
    pub enrollable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SsdpDeviceDescription {
    pub friendly_name: String,
    pub manufacturer: Option<String>,
    pub model_name: Option<String>,
    pub device_type: Option<String>,
    pub udn: Option<String>,
    pub description_url: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SsdpDiscoveryRecord {
    pub response: SsdpDiscoveryResponse,
    pub description: Option<SsdpDeviceDescription>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SsdpDiscoveryError {
    MissingLocation,
    MissingSearchTarget,
    MissingUsn,
    MalformedResponse,
    UnsupportedLocationScheme,
    ExternalLocation,
    InvalidDescription,
    ResponseTooLarge,
    Timeout,
    Io(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AllowedHttpLocation {
    pub addr: SocketAddr,
    pub path: String,
}

pub fn build_msearch_request(search_target: &str, host: SocketAddr, mx_seconds: u8) -> Vec<u8> {
    let st = normalize_search_target(search_target);
    let mx = mx_seconds.clamp(1, SSDP_MAX_MX_SECONDS);
    format!(
        "M-SEARCH * HTTP/1.1\r\nHOST: {host}\r\nMAN: \"ssdp:discover\"\r\nMX: {mx}\r\nST: {st}\r\nUSER-AGENT: ocentra-parent/lan-core\r\nCONNECTION: close\r\n\r\n"
    )
    .into_bytes()
}

pub fn discover_ssdp_upnp_devices(
    search_target: &str,
    target: SocketAddr,
    response_timeout: Duration,
    attempts: usize,
    description_timeout: Duration,
) -> Result<Vec<SsdpDiscoveryRecord>, SsdpDiscoveryError> {
    let socket = UdpSocket::bind(match target {
        SocketAddr::V4(_) => SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0),
        SocketAddr::V6(_) => SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0),
    })
    .map_err(|error| io_error(&error))?;
    socket
        .set_read_timeout(Some(response_timeout))
        .map_err(|error| io_error(&error))?;
    let request = build_msearch_request(
        search_target,
        target,
        mx_seconds_for_timeout(response_timeout),
    );
    let retry_count = attempts.clamp(1, SSDP_MAX_ATTEMPTS);
    let mut results = Vec::new();
    let mut seen = HashSet::new();

    for _ in 0..retry_count {
        socket
            .send_to(&request, target)
            .map_err(|error| io_error(&error))?;
        let deadline = Instant::now() + response_timeout;
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                break;
            }
            socket
                .set_read_timeout(Some(remaining))
                .map_err(|error| io_error(&error))?;
            let mut buffer = vec![0_u8; SSDP_MAX_RESPONSE_BYTES];
            match socket.recv_from(&mut buffer) {
                Ok((size, _)) => {
                    buffer.truncate(size);
                    let response = match parse_ssdp_response(&buffer) {
                        Ok(response) => response,
                        Err(SsdpDiscoveryError::MalformedResponse) => continue,
                        Err(SsdpDiscoveryError::MissingLocation) => continue,
                        Err(SsdpDiscoveryError::MissingSearchTarget) => continue,
                        Err(SsdpDiscoveryError::MissingUsn) => continue,
                        Err(error) => return Err(error),
                    };
                    let key = response.dedup_key();
                    if !seen.insert(key) {
                        continue;
                    }
                    let description = if response.description_fetch_allowed() {
                        fetch_ssdp_description(&response.location, description_timeout).ok()
                    } else {
                        None
                    };
                    results.push(SsdpDiscoveryRecord {
                        response,
                        description,
                    });
                }
                Err(error)
                    if error.kind() == std::io::ErrorKind::WouldBlock
                        || error.kind() == std::io::ErrorKind::TimedOut =>
                {
                    break;
                }
                Err(error) => return Err(io_error(&error)),
            }
        }
    }

    Ok(results)
}

pub fn enrich_ssdp_upnp_devices(
    devices: &mut Vec<LanNetworkInventoryDevice>,
    selected_interface: Option<&str>,
) {
    let Ok(records) = discover_ssdp_upnp_records() else {
        return;
    };

    for record in records {
        let Some(mut device) = ssdp_network_inventory_device(&record, selected_interface) else {
            continue;
        };
        let Some(existing) = devices.iter_mut().find(|existing| {
            existing.device_id == device.device_id
                || existing.ip_address.eq_ignore_ascii_case(&device.ip_address)
        }) else {
            devices.push(device);
            continue;
        };
        merge_ssdp_network_inventory_device(existing, &mut device);
    }
}

pub fn enrich_ssdp_upnp_devices_for_target(
    devices: &mut Vec<LanNetworkInventoryDevice>,
    target: SocketAddr,
) {
    let Ok(records) = discover_ssdp_upnp_records_at(target) else {
        return;
    };

    for record in records {
        let Some(mut device) = ssdp_network_inventory_device(&record, None) else {
            continue;
        };
        let Some(existing) = devices.iter_mut().find(|existing| {
            existing.device_id == device.device_id
                || existing.ip_address.eq_ignore_ascii_case(&device.ip_address)
        }) else {
            devices.push(device);
            continue;
        };
        merge_ssdp_network_inventory_device(existing, &mut device);
    }
}

pub fn parse_ssdp_response(response: &[u8]) -> Result<SsdpDiscoveryResponse, SsdpDiscoveryError> {
    if response.len() > SSDP_MAX_RESPONSE_BYTES {
        return Err(SsdpDiscoveryError::ResponseTooLarge);
    }
    let (status_line, headers, _body) = split_http_headers(response)?;
    if parse_http_status_code(status_line)? != 200 {
        return Err(SsdpDiscoveryError::MalformedResponse);
    }
    let location =
        normalized_header_value(&headers, "location").ok_or(SsdpDiscoveryError::MissingLocation)?;
    let search_target =
        normalized_header_value(&headers, "st").ok_or(SsdpDiscoveryError::MissingSearchTarget)?;
    let usn = normalized_header_value(&headers, "usn").ok_or(SsdpDiscoveryError::MissingUsn)?;
    let udn = parse_udn(&usn);
    let device_type = parse_device_type(&search_target).or_else(|| parse_device_type(&usn));
    let infrastructure = is_infrastructure_device(device_type.as_deref(), &search_target, &usn);
    Ok(SsdpDiscoveryResponse {
        location,
        search_target,
        usn,
        udn,
        device_type,
        infrastructure,
        enrollable: !infrastructure,
    })
}

pub fn fetch_ssdp_description(
    location: &str,
    timeout: Duration,
) -> Result<SsdpDeviceDescription, SsdpDiscoveryError> {
    let allowed_location = parse_allowed_http_location(location)?;
    let mut stream = TcpStream::connect_timeout(&allowed_location.addr, timeout)
        .map_err(|error| io_error(&error))?;
    stream
        .set_read_timeout(Some(timeout))
        .map_err(|error| io_error(&error))?;
    stream
        .set_write_timeout(Some(timeout))
        .map_err(|error| io_error(&error))?;
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nAccept: application/xml,text/xml\r\n\r\n",
        allowed_location.path, allowed_location.addr
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|error| io_error(&error))?;
    let response = read_http_response(&mut stream)?;
    parse_device_description_response(&response, location)
}

pub fn read_http_response(stream: &mut TcpStream) -> Result<Vec<u8>, SsdpDiscoveryError> {
    let mut response = Vec::new();
    let mut chunk = [0_u8; 1024];
    loop {
        match stream.read(&mut chunk) {
            Ok(0) => break,
            Ok(read) => {
                if response.len().saturating_add(read) > SSDP_MAX_DESCRIPTION_BYTES {
                    return Err(SsdpDiscoveryError::ResponseTooLarge);
                }
                response.extend_from_slice(&chunk[..read]);
            }
            Err(error)
                if error.kind() == std::io::ErrorKind::WouldBlock
                    || error.kind() == std::io::ErrorKind::TimedOut =>
            {
                return Err(SsdpDiscoveryError::Timeout);
            }
            Err(error) => return Err(io_error(&error)),
        }
    }

    if response.is_empty() {
        return Err(SsdpDiscoveryError::Timeout);
    }

    Ok(response)
}

pub fn parse_device_description_response(
    response: &[u8],
    description_url: &str,
) -> Result<SsdpDeviceDescription, SsdpDiscoveryError> {
    let (status_line, _headers, body) = split_http_headers(response)?;
    if parse_http_status_code(status_line)? != 200 {
        return Err(SsdpDiscoveryError::InvalidDescription);
    }
    let xml = std::str::from_utf8(body).map_err(|_error| SsdpDiscoveryError::InvalidDescription)?;
    parse_device_description_xml(xml, description_url)
}

pub fn parse_device_description_xml(
    xml: &str,
    description_url: &str,
) -> Result<SsdpDeviceDescription, SsdpDiscoveryError> {
    if xml.len() > SSDP_MAX_DESCRIPTION_BYTES {
        return Err(SsdpDiscoveryError::ResponseTooLarge);
    }
    if xml.contains("<!DOCTYPE") || xml.contains("<!ENTITY") {
        return Err(SsdpDiscoveryError::InvalidDescription);
    }
    let bounded_text = |tag| match extract_xml_text(xml, tag) {
        Some(value) if value.len() > SSDP_MAX_DESCRIPTION_TEXT_BYTES => {
            Err(SsdpDiscoveryError::InvalidDescription)
        }
        value => Ok(value),
    };
    let friendly_name =
        bounded_text("friendlyName")?.ok_or(SsdpDiscoveryError::InvalidDescription)?;
    let device_type =
        bounded_text("deviceType")?.and_then(|value| parse_device_type(&value).or(Some(value)));
    let manufacturer = bounded_text("manufacturer")?;
    let model_name = bounded_text("modelName")?;
    let udn = bounded_text("UDN")?.and_then(|value| parse_udn(&value));
    Ok(SsdpDeviceDescription {
        friendly_name,
        manufacturer,
        model_name,
        device_type,
        udn,
        description_url: description_url.to_string(),
    })
}

pub fn discover_ssdp_upnp_records() -> Result<Vec<SsdpDiscoveryRecord>, SsdpDiscoveryError> {
    discover_ssdp_upnp_records_at(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(239, 255, 255, 250)),
        1900,
    ))
}

pub fn discover_ssdp_upnp_records_at(
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

pub fn ssdp_network_inventory_device(
    record: &SsdpDiscoveryRecord,
    selected_interface: Option<&str>,
) -> Option<LanNetworkInventoryDevice> {
    let allowed_location = parse_allowed_http_location(&record.response.location).ok()?;
    let label = record
        .description
        .as_ref()
        .map(|description| description.friendly_name.clone())
        .or_else(|| short_ssdp_label(record.response.device_type.as_deref()))
        .or_else(|| short_ssdp_label(Some(record.response.search_target.as_str())))
        .unwrap_or_else(|| record.response.usn.clone());
    let platform = if record.response.infrastructure {
        constants::lan_pairing::PLATFORM_ROUTER.to_string()
    } else {
        record
            .response
            .device_type
            .as_ref()
            .and_then(|device_type| short_ssdp_label(Some(device_type.as_str())))
            .unwrap_or_else(|| constants::lan_pairing::PLATFORM_UNKNOWN.to_string())
    };
    let device_id = record
        .response
        .udn
        .clone()
        .or_else(|| parse_udn(&record.response.usn))
        .unwrap_or_else(|| record.response.usn.clone());

    Some(LanNetworkInventoryDevice {
        device_id,
        label,
        platform,
        ip_address: allowed_location.addr.ip().to_string(),
        mac_address: String::new(),
        hostname: None,
        network_interface: selected_interface.map(str::to_string),
        observed_at: String::new(),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_SSDP_UPNP.to_string()],
        used_previous_scan_hint: false,
        service_identity_probe_evidence: ssdp_hint_evidence(record, selected_interface),
    })
}

pub fn merge_ssdp_network_inventory_device(
    existing: &mut LanNetworkInventoryDevice,
    incoming: &mut LanNetworkInventoryDevice,
) {
    if existing.label.is_empty()
        || existing
            .label
            .starts_with(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX)
    {
        existing.label = incoming.label.clone();
    }
    if existing.platform == constants::lan_pairing::PLATFORM_UNKNOWN {
        existing.platform = incoming.platform.clone();
    }
    if existing.hostname.is_none() {
        existing.hostname = incoming.hostname.take();
    }
    if existing.network_interface.is_none() {
        existing.network_interface = incoming.network_interface.take();
    }
    if existing.reachability != LanPairingDeviceReachability::Online {
        existing.reachability = LanPairingDeviceReachability::Online;
    }
    if existing.agent_status.is_none() {
        existing.agent_status = incoming.agent_status.take();
    }
    merge_service_identity_probe_evidence(
        &mut existing.service_identity_probe_evidence,
        incoming.service_identity_probe_evidence.drain(..),
    );
    for source in incoming.scan_sources.drain(..) {
        if !existing
            .scan_sources
            .iter()
            .any(|existing_source| existing_source == &source)
        {
            existing.scan_sources.push(source);
        }
    }
}

pub fn ssdp_hint_evidence(
    record: &SsdpDiscoveryRecord,
    selected_interface: Option<&str>,
) -> Vec<LanServiceIdentityProbeEvidence> {
    let mut evidence = Vec::new();
    let parsed_udn = parse_udn(&record.response.usn);
    if let Some(udn) = record.response.udn.as_deref().or(parsed_udn.as_deref()) {
        push_ssdp_hint(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::SsdpUdn,
            udn,
            selected_interface.map(str::to_string),
        );
    }
    if let Some(device_type) = record
        .description
        .as_ref()
        .and_then(|description| description.device_type.as_deref())
        .or(record.response.device_type.as_deref())
    {
        push_ssdp_hint(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::SsdpDeviceType,
            device_type,
            selected_interface.map(str::to_string),
        );
    }
    evidence
}

pub fn merge_service_identity_probe_evidence(
    existing: &mut Vec<LanServiceIdentityProbeEvidence>,
    incoming: impl Iterator<Item = LanServiceIdentityProbeEvidence>,
) {
    for record in incoming {
        if let Some(current) = existing.iter_mut().find(|entry| {
            entry.evidence_kind == record.evidence_kind
                && entry.value.eq_ignore_ascii_case(&record.value)
        }) {
            if current.selected_interface.is_none() {
                current.selected_interface = record.selected_interface.clone();
            }
            continue;
        }
        existing.push(record);
    }
}

pub fn push_ssdp_hint(
    records: &mut Vec<LanServiceIdentityProbeEvidence>,
    evidence_kind: LanServiceIdentityProbeEvidenceKind,
    value: &str,
    selected_interface: Option<String>,
) {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return;
    }
    if let Some(existing) = records.iter_mut().find(|record| {
        record.evidence_kind == evidence_kind && record.value.eq_ignore_ascii_case(trimmed)
    }) {
        if existing.selected_interface.is_none() {
            existing.selected_interface = selected_interface;
        }
        return;
    }
    records.push(LanServiceIdentityProbeEvidence {
        evidence_kind,
        value: trimmed.to_string(),
        selected_interface,
    });
}

impl SsdpDiscoveryResponse {
    pub fn dedup_key(&self) -> String {
        format!("{}|{}", self.usn, self.location)
    }

    pub fn description_fetch_allowed(&self) -> bool {
        parse_allowed_http_location(&self.location).is_ok()
    }
}
