mod application;
mod eligibility;
mod enrichment;
mod evidence;
mod settings;

pub mod http;
pub mod probe;
pub mod snmp;
pub mod targets;
pub mod wsd;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanServiceIdentityProbeEvidence, LanServiceIdentityProbeEvidenceKind,
};
use serde::{Deserialize, Serialize};

use super::LanNetworkInventoryDevice;
use std::{sync::atomic::AtomicBool, time::Instant};

pub type AllowedSnmpResponseObserver<'a> = Option<&'a (dyn Fn(&[u8]) + Send + Sync)>;
pub type HttpResponseParts<'a> = (u16, Vec<(String, String)>, &'a [u8]);

pub const SERVICE_IDENTITY_PROBE_CONNECT_TIMEOUT_MS: u64 = 250;
pub const SERVICE_IDENTITY_PROBE_READ_TIMEOUT_MS: u64 = 250;
pub const SERVICE_IDENTITY_PROBE_MAX_CONCURRENCY: usize = 4;
pub const SERVICE_IDENTITY_PROBE_MAX_RESPONSE_BYTES: usize = 32 * 1024;
pub const SERVICE_IDENTITY_PROBE_MAX_TEXT_BYTES: usize = 256;
pub const SERVICE_IDENTITY_PROBE_SCAN_BUDGET_MS: u64 = 3_000;
pub const SNMP_GET_REQUEST_TAG: u8 = 0xA0;
pub const SNMP_GET_RESPONSE_TAG: u8 = 0xA2;
pub const BER_TAG_INTEGER: u8 = 0x02;
pub const BER_TAG_OCTET_STRING: u8 = 0x04;
pub const BER_TAG_NULL: u8 = 0x05;
pub const BER_TAG_OBJECT_IDENTIFIER: u8 = 0x06;
pub const BER_TAG_SEQUENCE: u8 = 0x30;
pub const SNMP_VERSION_2C: i64 = 1;
pub const SNMP_REQUEST_ID: i64 = 1;
pub const SNMP_PUBLIC_COMMUNITY: &str = "public";
pub const SNMP_SYS_DESCR_OID: &[u32] = &[1, 3, 6, 1, 2, 1, 1, 1, 0];
pub const SNMP_SYS_NAME_OID: &[u32] = &[1, 3, 6, 1, 2, 1, 1, 5, 0];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProbeTransport {
    Http,
    Https,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProbeTarget {
    pub port: u16,
    pub transport: ProbeTransport,
    pub request_paths: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceIdentityProbeSettings {
    pub allow_wsd_identity_query: bool,
    pub allow_snmp_identity_query: bool,
    pub allow_os_fingerprint: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ServiceIdentityProbeFamily {
    HttpTcp,
    WsdIdentityQuery,
    SnmpIdentityQuery,
    OsFingerprint,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ServiceIdentityProbeDecision {
    Execute,
    OperatorSettingRequired,
    ManualGateRequired,
    RuntimeNotImplemented,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServiceIdentityProbeFamilyDecision {
    pub family: ServiceIdentityProbeFamily,
    pub decision: ServiceIdentityProbeDecision,
    pub allowed_ports: Vec<u16>,
    pub requires_discovered_host: bool,
    pub weak_evidence_only: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanServiceIdentityProbeObservation {
    pub status_code: Option<u16>,
    pub title: Option<String>,
    pub server_header: Option<String>,
    pub banner: Option<String>,
    pub redirect_location: Option<String>,
    pub certificate_subject: Option<String>,
    pub descriptor_links: Vec<String>,
    pub wsd_endpoint_address: Option<String>,
    pub wsd_types: Option<String>,
    pub snmp_sys_descr: Option<String>,
    pub snmp_sys_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AllowedSnmpResponseObservation {
    pub sys_descr: Option<String>,
    pub sys_name: Option<String>,
}

pub fn service_identity_probe_scan_source() -> &'static str {
    constants::lan_pairing::LAN_SCAN_SOURCE_SERVICE_IDENTITY_PROBE
}

pub fn is_confirmed_agent_status(status: Option<&str>) -> bool {
    matches!(status, Some(constants::lan_pairing::LOCAL_AGENT_STATUS))
}

pub fn is_service_identity_probe_status(status: Option<&str>) -> bool {
    matches!(
        status,
        Some(constants::lan_pairing::SERVICE_IDENTITY_PROBE_AGENT_STATUS)
    )
}

pub fn trusted_device_matches_network_identity(
    trusted_device: &LanPairingDeviceRef,
    network_mac_address: &str,
    network_ip_address: &str,
) -> bool {
    eligibility::trusted_device_matches_network_identity(
        trusted_device,
        network_mac_address,
        network_ip_address,
    )
}

pub fn enrich_service_identity_probes(
    devices: &mut [LanNetworkInventoryDevice],
    probe_suppression_devices: &[LanPairingDeviceRef],
    selected_interface: Option<&str>,
    allowed_snmp_response_observer: AllowedSnmpResponseObserver<'_>,
) {
    enrichment::enrich_service_identity_probes(
        devices,
        probe_suppression_devices,
        selected_interface,
        allowed_snmp_response_observer,
        None,
        None,
    )
}

pub fn enrich_service_identity_probes_with_cancellation(
    devices: &mut [LanNetworkInventoryDevice],
    probe_suppression_devices: &[LanPairingDeviceRef],
    selected_interface: Option<&str>,
    allowed_snmp_response_observer: AllowedSnmpResponseObserver<'_>,
    cancellation: Option<&AtomicBool>,
    deadline: Option<Instant>,
) {
    enrichment::enrich_service_identity_probes(
        devices,
        probe_suppression_devices,
        selected_interface,
        allowed_snmp_response_observer,
        cancellation,
        deadline,
    )
}

pub fn should_probe_service_identity(
    device: &LanNetworkInventoryDevice,
    probe_suppression_devices: &[LanPairingDeviceRef],
    selected_interface: &str,
) -> bool {
    eligibility::should_probe_service_identity(
        device,
        probe_suppression_devices,
        selected_interface,
    )
}

pub fn device_is_on_selected_interface(
    device: &LanNetworkInventoryDevice,
    selected_interface: &str,
) -> bool {
    eligibility::device_is_on_selected_interface(device, selected_interface)
}

pub fn apply_service_identity_probe(
    device: &mut LanNetworkInventoryDevice,
    probe_match: LanServiceIdentityProbeObservation,
) {
    application::apply_service_identity_probe(device, probe_match)
}

pub fn push_probe_evidence(
    evidence: &mut Vec<LanServiceIdentityProbeEvidence>,
    evidence_kind: LanServiceIdentityProbeEvidenceKind,
    value: Option<String>,
    selected_interface: Option<String>,
) {
    evidence::push_probe_evidence(evidence, evidence_kind, value, selected_interface)
}

pub fn merge_service_identity_probe_evidence(
    existing: Vec<LanServiceIdentityProbeEvidence>,
    incoming: Vec<LanServiceIdentityProbeEvidence>,
) -> Vec<LanServiceIdentityProbeEvidence> {
    evidence::merge_service_identity_probe_evidence(existing, incoming)
}

pub fn runtime_service_identity_probe_settings() -> ServiceIdentityProbeSettings {
    settings::runtime_service_identity_probe_settings()
}

pub fn env_flag_enabled(name: &str) -> bool {
    settings::env_flag_enabled(name)
}
