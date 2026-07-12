pub mod http;

use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanServiceIdentityProbeEvidence, LanServiceIdentityProbeEvidenceKind,
};

use self::http::text::parse_udn;
use self::http::{
    is_infrastructure_device, normalized_header_value, parse_allowed_http_location,
    parse_device_type, parse_http_status_code, split_http_headers,
};
use super::LanNetworkInventoryDevice;

mod description;
mod discovery;
mod inventory;
mod merge;
mod receive;

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
    discovery::build_msearch_request(search_target, host, mx_seconds)
}

pub fn discover_ssdp_upnp_devices(
    search_target: &str,
    target: SocketAddr,
    response_timeout: Duration,
    attempts: usize,
    description_timeout: Duration,
) -> Result<Vec<SsdpDiscoveryRecord>, SsdpDiscoveryError> {
    discovery::discover_ssdp_upnp_devices(
        search_target,
        target,
        response_timeout,
        attempts,
        description_timeout,
    )
}

pub fn enrich_ssdp_upnp_devices(
    devices: &mut Vec<LanNetworkInventoryDevice>,
    selected_interface: Option<&str>,
) {
    inventory::enrich_ssdp_upnp_devices(devices, selected_interface)
}

pub fn enrich_ssdp_upnp_devices_for_target(
    devices: &mut Vec<LanNetworkInventoryDevice>,
    target: SocketAddr,
) {
    inventory::enrich_ssdp_upnp_devices_for_target(devices, target)
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
    description::fetch_ssdp_description(location, timeout)
}

pub fn read_http_response(stream: &mut TcpStream) -> Result<Vec<u8>, SsdpDiscoveryError> {
    description::read_http_response(stream)
}

pub fn parse_device_description_response(
    response: &[u8],
    description_url: &str,
) -> Result<SsdpDeviceDescription, SsdpDiscoveryError> {
    description::parse_device_description_response(response, description_url)
}

pub fn parse_device_description_xml(
    xml: &str,
    description_url: &str,
) -> Result<SsdpDeviceDescription, SsdpDiscoveryError> {
    description::parse_device_description_xml(xml, description_url)
}

pub fn discover_ssdp_upnp_records() -> Result<Vec<SsdpDiscoveryRecord>, SsdpDiscoveryError> {
    discovery::discover_ssdp_upnp_records()
}

pub fn discover_ssdp_upnp_records_at(
    target: SocketAddr,
) -> Result<Vec<SsdpDiscoveryRecord>, SsdpDiscoveryError> {
    discovery::discover_ssdp_upnp_records_at(target)
}

pub fn ssdp_network_inventory_device(
    record: &SsdpDiscoveryRecord,
    selected_interface: Option<&str>,
) -> Option<LanNetworkInventoryDevice> {
    inventory::ssdp_network_inventory_device(record, selected_interface)
}

pub fn merge_ssdp_network_inventory_device(
    existing: &mut LanNetworkInventoryDevice,
    incoming: &mut LanNetworkInventoryDevice,
) {
    merge::merge_ssdp_network_inventory_device(existing, incoming)
}

pub fn ssdp_hint_evidence(
    record: &SsdpDiscoveryRecord,
    selected_interface: Option<&str>,
) -> Vec<LanServiceIdentityProbeEvidence> {
    merge::ssdp_hint_evidence(record, selected_interface)
}

pub fn merge_service_identity_probe_evidence(
    existing: &mut Vec<LanServiceIdentityProbeEvidence>,
    incoming: impl Iterator<Item = LanServiceIdentityProbeEvidence>,
) {
    merge::merge_service_identity_probe_evidence(existing, incoming)
}

pub fn push_ssdp_hint(
    records: &mut Vec<LanServiceIdentityProbeEvidence>,
    evidence_kind: LanServiceIdentityProbeEvidenceKind,
    value: &str,
    selected_interface: Option<String>,
) {
    merge::push_ssdp_hint(records, evidence_kind, value, selected_interface)
}

impl SsdpDiscoveryResponse {
    pub fn dedup_key(&self) -> String {
        format!("{}|{}", self.usn, self.location)
    }

    pub fn description_fetch_allowed(&self) -> bool {
        parse_allowed_http_location(&self.location).is_ok()
    }
}
