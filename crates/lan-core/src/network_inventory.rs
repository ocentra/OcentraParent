pub mod active_refresh;
pub mod api;
pub mod helpers;
pub mod linux_neighbors;
pub mod macos_neighbors;
pub mod mdns_dns_sd;
pub mod name_evidence;
mod neighbor_merge;
pub mod neighbor_support;
pub mod passive_discovery;
pub mod service_identity;
pub mod ssdp_upnp;
pub mod windows_neighbors;

use std::{collections::HashMap, sync::atomic::AtomicBool, time::Instant};

use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingDeviceRef,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEvidenceSource, LanServiceIdentityProbeEvidence,
};
use serde::{Deserialize, Serialize};

use self::helpers::{normalized_lookup_key, trimmed_non_empty};
use self::service_identity::trusted_device_matches_network_identity;
use self::service_identity::AllowedSnmpResponseObserver;
use crate::network_inventory_hardware::LocalNetworkIdentity;

pub struct LanNetworkDiscoveryRequest<'a> {
    pub identity_hint_devices: &'a [LanPairingDeviceRef],
    pub previous_devices: &'a [LanNetworkInventoryDevice],
    pub refresh_mode: LanDiscoveryRefreshMode,
    pub active_refresh_suppression_devices: &'a [LanPairingDeviceRef],
    pub probe_suppression_devices: &'a [LanPairingDeviceRef],
    pub selected_interface_scope: Option<&'a str>,
    pub allowed_snmp_response_observer: AllowedSnmpResponseObserver<'a>,
    pub cancellation: Option<&'a AtomicBool>,
    pub deadline: Option<Instant>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LanPreviousNetworkInventory {
    by_mac: HashMap<String, LanNetworkInventoryDevice>,
    by_ip: HashMap<String, LanNetworkInventoryDevice>,
}

impl LanPreviousNetworkInventory {
    pub fn from_devices(previous_devices: &[LanNetworkInventoryDevice]) -> Self {
        let mut by_mac = HashMap::new();
        let mut by_ip = HashMap::new();
        for device in previous_devices {
            if let Some(mac_address) = normalized_lookup_key(&device.mac_address) {
                by_mac.insert(mac_address, device.clone());
            }
            if let Some(ip_address) = normalized_lookup_key(&device.ip_address) {
                by_ip.insert(ip_address, device.clone());
            }
        }
        Self { by_mac, by_ip }
    }

    pub fn find(&self, mac_address: &str, ip_address: &str) -> Option<&LanNetworkInventoryDevice> {
        normalized_lookup_key(mac_address)
            .and_then(|mac_address| self.by_mac.get(&mac_address))
            .or_else(|| {
                normalized_lookup_key(ip_address).and_then(|ip_address| self.by_ip.get(&ip_address))
            })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LanIdentityHintInventory {
    devices: Vec<LanPairingDeviceRef>,
}

impl LanIdentityHintInventory {
    pub fn from_devices(identity_hint_devices: &[LanPairingDeviceRef]) -> Self {
        Self {
            devices: identity_hint_devices.to_vec(),
        }
    }

    pub fn find(&self, mac_address: &str, ip_address: &str) -> Option<&LanPairingDeviceRef> {
        self.devices.iter().find(|identity_hint_device| {
            trusted_device_matches_network_identity(identity_hint_device, mac_address, ip_address)
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LanNeighborObservation {
    pub ip_address: String,
    pub mac_address: String,
    pub network_interface: Option<String>,
    pub hostname: Option<String>,
    pub observed_at: String,
    pub reachability: LanPairingDeviceReachability,
    pub scan_sources: Vec<String>,
}

pub fn merge_neighbor_observations_by_mac(
    observations: Vec<LanNeighborObservation>,
) -> Vec<LanNeighborObservation> {
    neighbor_merge::merge_neighbor_observations_by_mac(observations)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanDiscoveryRefreshMode {
    Passive,
    ActiveSubnetRefresh,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanDiscoveryScanPlan {
    pub refresh_mode: LanDiscoveryRefreshMode,
    pub selected_interface: Option<String>,
    pub local_ip_address: Option<String>,
    pub ipv4_cidr: Option<String>,
    pub default_gateway: Option<String>,
    #[serde(default)]
    pub dns_servers: Vec<String>,
    pub dhcp_server: Option<String>,
    pub broadcast_address: Option<String>,
    #[serde(default)]
    pub ipv6_prefixes: Vec<String>,
    pub trusted_truth_device_count: u32,
    pub previous_device_count: u32,
    pub active_ipv4_candidate_count: u32,
    pub active_ipv4_target_count: u32,
    pub prioritized_previous_target_count: u32,
    pub active_ipv4_target_timeout_ms: Option<u64>,
    pub allow_wsd_identity_query: bool,
    pub allow_snmp_identity_query: bool,
    pub allow_os_fingerprint: bool,
    #[serde(default)]
    pub suppressed_active_ipv4_targets: Vec<String>,
    #[serde(default)]
    pub targeted_arp_refresh_evidence: Vec<LanTargetedArpRefreshEvidence>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanManualInterfaceSelection {
    pub selected_interface: String,
    pub local_ip_address: String,
    pub ipv4_cidr: String,
    pub default_gateway: Option<String>,
    #[serde(default)]
    pub dns_servers: Vec<String>,
    pub dhcp_server: Option<String>,
    pub broadcast_address: Option<String>,
    #[serde(default)]
    pub ipv6_prefixes: Vec<String>,
}

impl LanManualInterfaceSelection {
    pub fn into_identity(self) -> Option<LocalNetworkIdentity> {
        let selected_interface = trimmed_non_empty(self.selected_interface)?;
        let local_ip_address = trimmed_non_empty(self.local_ip_address)?;
        let ipv4_cidr = trimmed_non_empty(self.ipv4_cidr)?;
        Some(LocalNetworkIdentity {
            ip_address: Some(local_ip_address),
            mac_address: None,
            network_interface: Some(selected_interface),
            wifi_ssid: None,
            default_gateway: self.default_gateway.and_then(trimmed_non_empty),
            ipv4_cidr: Some(ipv4_cidr),
            dns_servers: self
                .dns_servers
                .into_iter()
                .filter_map(trimmed_non_empty)
                .collect(),
            dhcp_server: self.dhcp_server.and_then(trimmed_non_empty),
            broadcast_address: self.broadcast_address.and_then(trimmed_non_empty),
            ipv6_prefixes: self
                .ipv6_prefixes
                .into_iter()
                .filter_map(trimmed_non_empty)
                .collect(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LanTargetedArpRefreshOutcome {
    Response,
    NoResponse,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanTargetedArpRefreshEvidence {
    pub target_ip_address: String,
    pub selected_interface: Option<String>,
    pub expected_mac_address: Option<String>,
    pub observed_mac_address: Option<String>,
    pub observed_at_unix_ms: u128,
    pub source: String,
    pub outcome: Option<LanTargetedArpRefreshOutcome>,
    pub strong_identity_match: bool,
    pub throttled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanNetworkInventoryDevice {
    pub device_id: String,
    pub label: String,
    pub platform: String,
    pub ip_address: String,
    pub mac_address: String,
    pub hostname: Option<String>,
    pub network_interface: Option<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub observed_at: String,
    pub reachability: LanPairingDeviceReachability,
    pub agent_status: Option<String>,
    #[serde(default)]
    pub scan_sources: Vec<String>,
    #[serde(default)]
    pub used_previous_scan_hint: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub service_identity_probe_evidence: Vec<LanServiceIdentityProbeEvidence>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanPassiveRuntimeLocalNetworkIdentity {
    pub ip_address: Option<String>,
    pub network_interface: Option<String>,
    pub wifi_ssid: Option<String>,
    pub default_gateway: Option<String>,
}

impl LanPassiveRuntimeLocalNetworkIdentity {
    fn from_local_network_identity(
        identity: Option<LocalNetworkIdentity>,
    ) -> LanPassiveRuntimeLocalNetworkIdentity {
        let Some(identity) = identity else {
            return LanPassiveRuntimeLocalNetworkIdentity::default();
        };

        LanPassiveRuntimeLocalNetworkIdentity {
            ip_address: identity.ip_address,
            network_interface: identity.network_interface,
            wifi_ssid: identity.wifi_ssid,
            default_gateway: identity.default_gateway,
        }
    }
}

pub fn passive_runtime_local_network_identity() -> LanPassiveRuntimeLocalNetworkIdentity {
    api::passive_runtime_local_network_identity()
}

pub fn discover_lan_network_devices_with_hints_refresh_mode_and_scan_and_probe_suppression_and_allowed_snmp_observer(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    probe_suppression_devices: &[LanPairingDeviceRef],
    selected_interface_scope: Option<&str>,
    allowed_snmp_response_observer: AllowedSnmpResponseObserver<'_>,
) -> Vec<LanNetworkInventoryDevice> {
    api::discover_lan_network_devices_with_hints_refresh_mode_and_scan_and_probe_suppression_and_allowed_snmp_observer(
        identity_hint_devices,
        previous_devices,
        refresh_mode,
        active_refresh_suppression_devices,
        probe_suppression_devices,
        selected_interface_scope,
        allowed_snmp_response_observer,
    )
}

pub fn discover_lan_network_devices_with_hints_refresh_mode_and_scan_and_probe_suppression_and_allowed_snmp_observer_with_cancellation(
    request: &LanNetworkDiscoveryRequest<'_>,
) -> Vec<LanNetworkInventoryDevice> {
    api::cancellation::discover_lan_network_devices_with_cancellation(request)
}

pub fn plan_lan_discovery_scan_with_active_refresh_suppression(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
) -> LanDiscoveryScanPlan {
    api::plan_lan_discovery_scan_with_active_refresh_suppression(
        identity_hint_devices,
        previous_devices,
        refresh_mode,
        active_refresh_suppression_devices,
    )
}

pub fn plan_lan_discovery_scan_until(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    deadline: Instant,
    cancellation: &AtomicBool,
) -> LanDiscoveryScanPlan {
    api::cancellation::plan_lan_discovery_scan_until(
        identity_hint_devices,
        previous_devices,
        refresh_mode,
        active_refresh_suppression_devices,
        deadline,
        cancellation,
    )
}

pub fn targeted_arp_refresh_evidence_for_scan(
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
) -> Vec<LanTargetedArpRefreshEvidence> {
    api::targeted_arp_refresh::targeted_arp_refresh_evidence_for_scan(
        previous_devices,
        refresh_mode,
        active_refresh_suppression_devices,
    )
}

pub fn targeted_arp_refresh_evidence_for_scan_plan_until(
    scan_plan: &LanDiscoveryScanPlan,
    previous_devices: &[LanNetworkInventoryDevice],
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    cancellation: Option<&AtomicBool>,
    outer_deadline: Option<Instant>,
) -> Vec<LanTargetedArpRefreshEvidence> {
    api::targeted_arp_refresh::targeted_arp_refresh_evidence_for_scan_plan_until(
        scan_plan,
        previous_devices,
        active_refresh_suppression_devices,
        cancellation,
        outer_deadline,
    )
}

pub fn local_agent_device_ref(local_device_id: String, platform: String) -> LanPairingDeviceRef {
    api::local_agent_device_ref(local_device_id, platform)
}

pub fn discovery_evidence_sources_for_network_device(
    device: &LanNetworkInventoryDevice,
) -> Vec<LanDiscoveryEvidenceSource> {
    api::discovery_evidence_sources_for_network_device(device)
}
