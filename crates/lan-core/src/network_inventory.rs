pub mod active_refresh;
pub mod api;
pub mod helpers;
pub mod linux_neighbors;
pub mod macos_neighbors;
pub mod mdns_dns_sd;
pub mod name_evidence;
pub mod neighbor_support;
pub mod passive_discovery;
pub mod service_identity;
pub mod ssdp_upnp;
pub mod windows_neighbors;

use std::collections::HashMap;

use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingDeviceRef,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidence;
use serde::{Deserialize, Serialize};

use self::helpers::{normalized_lookup_key, trimmed_non_empty};
use self::service_identity::trusted_device_matches_network_identity;
use crate::network_inventory_hardware::LocalNetworkIdentity;

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
    let mut merged: Vec<LanNeighborObservation> = Vec::new();
    for observation in observations {
        if let Some(existing) = merged.iter_mut().find(|candidate| {
            candidate
                .mac_address
                .eq_ignore_ascii_case(&observation.mac_address)
        }) {
            merge_neighbor_observation(existing, observation);
        } else {
            merged.push(observation);
        }
    }
    merged
}

fn merge_neighbor_observation(
    existing: &mut LanNeighborObservation,
    incoming: LanNeighborObservation,
) {
    let replace_primary_identity = should_replace_primary_observation(existing, &incoming);
    if replace_primary_identity {
        existing.ip_address = incoming.ip_address.clone();
        if incoming.network_interface.is_some() {
            existing.network_interface = incoming.network_interface.clone();
        }
        if incoming.hostname.is_some() {
            existing.hostname = incoming.hostname.clone();
        }
    } else {
        if existing.network_interface.is_none() {
            existing.network_interface = incoming.network_interface.clone();
        }
        if existing.hostname.is_none() {
            existing.hostname = incoming.hostname.clone();
        }
    }
    if reachability_rank(&incoming.reachability) > reachability_rank(&existing.reachability) {
        existing.reachability = incoming.reachability.clone();
    }
    merge_observed_at(&mut existing.observed_at, &incoming.observed_at);
    for scan_source in incoming.scan_sources {
        push_unique_scan_source(&mut existing.scan_sources, &scan_source);
    }
}

fn should_replace_primary_observation(
    existing: &LanNeighborObservation,
    incoming: &LanNeighborObservation,
) -> bool {
    let existing_is_private_ipv4 = parse_private_ipv4(&existing.ip_address).is_some();
    let incoming_is_private_ipv4 = parse_private_ipv4(&incoming.ip_address).is_some();
    if !existing_is_private_ipv4 && incoming_is_private_ipv4 {
        return true;
    }
    if existing.ip_address.is_empty() {
        return true;
    }
    if existing
        .ip_address
        .eq_ignore_ascii_case(&incoming.ip_address)
    {
        return false;
    }
    existing_is_private_ipv4
        && incoming_is_private_ipv4
        && reachability_rank(&incoming.reachability) > reachability_rank(&existing.reachability)
}

fn parse_private_ipv4(value: &str) -> Option<std::net::Ipv4Addr> {
    let ip = value.parse::<std::net::Ipv4Addr>().ok()?;
    ip.is_private().then_some(ip)
}

fn reachability_rank(reachability: &LanPairingDeviceReachability) -> u8 {
    match reachability {
        LanPairingDeviceReachability::Online => 3,
        LanPairingDeviceReachability::Stale => 2,
        LanPairingDeviceReachability::Offline => 1,
    }
}

fn push_unique_scan_source(scan_sources: &mut Vec<String>, value: &str) {
    if scan_sources.iter().any(|existing| existing == value) {
        return;
    }
    scan_sources.push(value.to_string());
}

fn merge_observed_at(existing: &mut String, incoming: &str) {
    if incoming.is_empty() {
        return;
    }
    if existing.is_empty() || incoming < existing.as_str() {
        *existing = incoming.to_string();
    }
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

pub type LanNetworkInventoryDeviceIdentifier = String;
pub type LanNetworkInventoryDisplayLabel = String;

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
    pub device_id: LanNetworkInventoryDeviceIdentifier,
    pub label: LanNetworkInventoryDisplayLabel,
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
