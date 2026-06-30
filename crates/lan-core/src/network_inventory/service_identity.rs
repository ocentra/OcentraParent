pub mod http;
pub mod probe;
pub mod snmp;
pub mod targets;
pub mod wsd;

use std::env;
use std::thread;
use std::time::{Duration, Instant};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{LanPairingDeviceReachability, LanPairingDeviceRef};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanServiceIdentityProbeEvidence, LanServiceIdentityProbeEvidenceKind,
};
use serde::{Deserialize, Serialize};

use super::LanNetworkInventoryDevice;
use self::probe::probe_service_identity;
use self::targets::service_identity_probe_targets;

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

impl LanServiceIdentityProbeObservation {
    fn is_meaningful(&self) -> bool {
        self.status_code.is_some()
            || self.snmp_sys_descr.is_some()
            || self.snmp_sys_name.is_some()
            || self.title.is_some()
            || self.server_header.is_some()
            || self.banner.is_some()
            || self.redirect_location.is_some()
            || self.certificate_subject.is_some()
            || !self.descriptor_links.is_empty()
            || self.wsd_endpoint_address.is_some()
            || self.wsd_types.is_some()
    }
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
    let trusted_mac_address = trusted_device
        .mac_address
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let trusted_ip_address = trusted_device
        .ip_address
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());

    let network_mac_address = network_mac_address.trim();
    if let Some(trusted_mac_address) =
        trusted_mac_address.filter(|_| !network_mac_address.is_empty())
    {
        return trusted_mac_address.eq_ignore_ascii_case(network_mac_address.trim());
    }

    trusted_ip_address
        .map(|trusted_ip_address| {
            trusted_ip_address.eq_ignore_ascii_case(network_ip_address.trim())
        })
        .unwrap_or(false)
}

pub fn enrich_service_identity_probes(
    devices: &mut [LanNetworkInventoryDevice],
    probe_suppression_devices: &[LanPairingDeviceRef],
    selected_interface: Option<&str>,
    allowed_snmp_response_observer: AllowedSnmpResponseObserver<'_>,
) {
    let settings = runtime_service_identity_probe_settings();
    let targets = service_identity_probe_targets();
    if targets.is_empty() {
        return;
    }
    let Some(selected_interface) = selected_interface
        .map(str::trim)
        .filter(|value| !value.is_empty())
    else {
        return;
    };
    let deadline = Instant::now() + Duration::from_millis(SERVICE_IDENTITY_PROBE_SCAN_BUDGET_MS);

    let candidates = devices
        .iter()
        .enumerate()
        .filter_map(|(index, device)| {
            should_probe_service_identity(device, probe_suppression_devices, selected_interface)
                .then_some((index, device.ip_address.clone(), device.device_id.clone()))
        })
        .collect::<Vec<_>>();

    for batch in candidates.chunks(SERVICE_IDENTITY_PROBE_MAX_CONCURRENCY) {
        if Instant::now() >= deadline {
            break;
        }
        let mut probe_results = Vec::new();
        thread::scope(|scope| {
            let mut handles = Vec::new();
            for (index, ip_address, device_id) in batch {
                let targets = targets.clone();
                let ip_address = ip_address.clone();
                let device_id = device_id.clone();
                handles.push(scope.spawn(move || {
                    (
                        *index,
                        probe_service_identity(
                            &ip_address,
                            Some(device_id.as_str()),
                            &targets,
                            settings,
                            deadline,
                            allowed_snmp_response_observer,
                        ),
                    )
                }));
            }

            for handle in handles {
                if let Ok(result) = handle.join() {
                    probe_results.push(result);
                }
            }
        });

        for (index, probe_match) in probe_results {
            if let Some(probe_match) = probe_match {
                if let Some(device) = devices.get_mut(index) {
                    apply_service_identity_probe(device, probe_match);
                }
            }
        }
    }
}

pub fn should_probe_service_identity(
    device: &LanNetworkInventoryDevice,
    probe_suppression_devices: &[LanPairingDeviceRef],
    selected_interface: &str,
) -> bool {
    if !device_is_on_selected_interface(device, selected_interface) {
        return false;
    }
    if device.platform == constants::lan_pairing::PLATFORM_ROUTER
        || device.agent_status.is_some()
        || probe_suppression_devices
            .iter()
            .any(|probe_suppression_device| {
                trusted_device_matches_network_identity(
                    probe_suppression_device,
                    &device.mac_address,
                    &device.ip_address,
                )
            })
    {
        return false;
    }

    matches!(
        device.reachability,
        LanPairingDeviceReachability::Online | LanPairingDeviceReachability::Stale
    )
}

pub fn device_is_on_selected_interface(
    device: &LanNetworkInventoryDevice,
    selected_interface: &str,
) -> bool {
    device
        .network_interface
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|device_interface| device_interface.eq_ignore_ascii_case(selected_interface))
        .unwrap_or(false)
}

pub fn apply_service_identity_probe(
    device: &mut LanNetworkInventoryDevice,
    probe_match: LanServiceIdentityProbeObservation,
) {
    let observed_allowed_snmp_response = probe_match.observed_allowed_snmp_response();
    let selected_interface = device.network_interface.clone();
    device.agent_status =
        Some(constants::lan_pairing::SERVICE_IDENTITY_PROBE_AGENT_STATUS.to_string());
    let incoming = probe_match.into_evidence_with_selected_interface(selected_interface);
    device.service_identity_probe_evidence = merge_service_identity_probe_evidence(
        std::mem::take(&mut device.service_identity_probe_evidence),
        incoming,
    );
    if observed_allowed_snmp_response
        && !device
            .scan_sources
            .iter()
            .any(|source| source == constants::lan_pairing::LAN_SCAN_SOURCE_ALLOWED_SNMP_RESPONSE)
    {
        device
            .scan_sources
            .push(constants::lan_pairing::LAN_SCAN_SOURCE_ALLOWED_SNMP_RESPONSE.to_string());
    }
}

impl LanServiceIdentityProbeObservation {
    pub fn observed_allowed_snmp_response(&self) -> bool {
        self.snmp_sys_descr.is_some() || self.snmp_sys_name.is_some()
    }

    pub fn into_evidence(self) -> Vec<LanServiceIdentityProbeEvidence> {
        self.into_evidence_with_selected_interface(None)
    }

    pub fn into_evidence_with_selected_interface(
        self,
        selected_interface: Option<String>,
    ) -> Vec<LanServiceIdentityProbeEvidence> {
        let mut evidence = Vec::new();
        if let Some(status_code) = self.status_code {
            evidence.push(LanServiceIdentityProbeEvidence {
                evidence_kind: LanServiceIdentityProbeEvidenceKind::HttpStatus,
                value: status_code.to_string(),
                selected_interface: selected_interface.clone(),
            });
        }
        push_probe_evidence(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::HtmlTitle,
            self.title,
            selected_interface.clone(),
        );
        push_probe_evidence(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::ServerHeader,
            self.server_header,
            selected_interface.clone(),
        );
        push_probe_evidence(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::Banner,
            self.banner,
            selected_interface.clone(),
        );
        push_probe_evidence(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::RedirectLocation,
            self.redirect_location,
            selected_interface.clone(),
        );
        push_probe_evidence(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::CertificateSubject,
            self.certificate_subject,
            selected_interface.clone(),
        );
        for descriptor_link in self.descriptor_links {
            push_probe_evidence(
                &mut evidence,
                LanServiceIdentityProbeEvidenceKind::DescriptorLink,
                Some(descriptor_link),
                selected_interface.clone(),
            );
        }
        push_probe_evidence(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::WsdEndpointAddress,
            self.wsd_endpoint_address,
            selected_interface.clone(),
        );
        push_probe_evidence(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::WsdTypes,
            self.wsd_types,
            selected_interface.clone(),
        );
        push_probe_evidence(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::SnmpSysDescr,
            self.snmp_sys_descr,
            selected_interface.clone(),
        );
        push_probe_evidence(
            &mut evidence,
            LanServiceIdentityProbeEvidenceKind::SnmpSysName,
            self.snmp_sys_name,
            selected_interface,
        );
        evidence
    }
}

pub fn push_probe_evidence(
    evidence: &mut Vec<LanServiceIdentityProbeEvidence>,
    evidence_kind: LanServiceIdentityProbeEvidenceKind,
    value: Option<String>,
    selected_interface: Option<String>,
) {
    let Some(value) = value else {
        return;
    };
    if evidence
        .iter()
        .any(|item| item.evidence_kind == evidence_kind && item.value == value)
    {
        return;
    }
    evidence.push(LanServiceIdentityProbeEvidence {
        evidence_kind,
        value,
        selected_interface,
    });
}

pub fn merge_service_identity_probe_evidence(
    existing: Vec<LanServiceIdentityProbeEvidence>,
    incoming: Vec<LanServiceIdentityProbeEvidence>,
) -> Vec<LanServiceIdentityProbeEvidence> {
    let mut merged: Vec<LanServiceIdentityProbeEvidence> =
        Vec::with_capacity(existing.len().saturating_add(incoming.len()));
    for evidence in existing.into_iter().chain(incoming) {
        if let Some(existing) = merged.iter_mut().find(|item| {
            item.evidence_kind == evidence.evidence_kind && item.value == evidence.value
        }) {
            if existing.selected_interface.is_none() {
                existing.selected_interface = evidence.selected_interface.clone();
            }
            continue;
        }
        merged.push(evidence);
    }
    merged
}

pub fn runtime_service_identity_probe_settings() -> ServiceIdentityProbeSettings {
    ServiceIdentityProbeSettings {
        allow_wsd_identity_query: env_flag_enabled(
            constants::lan_pairing::LAN_ALLOW_WSD_IDENTITY_QUERY_ENV,
        ),
        allow_snmp_identity_query: env_flag_enabled(
            constants::lan_pairing::LAN_ALLOW_SNMP_IDENTITY_QUERY_ENV,
        ),
        allow_os_fingerprint: false,
    }
}

pub fn env_flag_enabled(name: &str) -> bool {
    matches!(
        env::var(name),
        Ok(value)
            if matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
    )
}
