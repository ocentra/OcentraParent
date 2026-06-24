use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpStream};
use std::process::{Command, Stdio};
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::Duration;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingDeviceRef,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource;
use ocentra_parent_agent_protocol::logging::AgentLogSnapshot;
use serde::{Deserialize, Serialize};

use crate::network_inventory_command::{
    command_json_records, command_stdout, normalize_mac_address, record_text, value_text,
};
use crate::network_inventory_hardware::{
    local_hardware_profile, local_network_identity, LocalNetworkIdentity,
};

static NETBIOS_CACHE_WARMED_IPS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
static NEIGHBOR_HOSTNAME_CACHE: OnceLock<Mutex<HashMap<String, LanNeighborIdentityCacheEntry>>> =
    OnceLock::new();

const SERVICE_IDENTITY_PROBE_CONNECT_TIMEOUT_MS: u64 = 250;
const SERVICE_IDENTITY_PROBE_READ_TIMEOUT_MS: u64 = 250;
const SERVICE_IDENTITY_PROBE_MAX_CONCURRENCY: usize = 4;
const SERVICE_IDENTITY_PROBE_MAX_RESPONSE_BYTES: usize = 32 * 1024;

#[derive(Clone, Debug, PartialEq, Eq)]
struct LanNeighborIdentityCacheEntry {
    hostname: String,
    platform: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LanServiceIdentityProbeMatch {
    hostname: Option<String>,
    platform: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct LanPreviousNetworkInventory {
    by_mac: HashMap<String, LanNetworkInventoryDevice>,
    by_ip: HashMap<String, LanNetworkInventoryDevice>,
}

impl LanPreviousNetworkInventory {
    fn from_devices(previous_devices: &[LanNetworkInventoryDevice]) -> Self {
        let mut by_mac = HashMap::new();
        let mut by_ip = HashMap::new();
        for device in previous_devices {
            by_mac.insert(device.mac_address.to_ascii_lowercase(), device.clone());
            by_ip.insert(device.ip_address.to_ascii_lowercase(), device.clone());
        }
        Self { by_mac, by_ip }
    }

    fn find(&self, mac_address: &str, ip_address: &str) -> Option<&LanNetworkInventoryDevice> {
        self.by_mac
            .get(&mac_address.to_ascii_lowercase())
            .or_else(|| self.by_ip.get(&ip_address.to_ascii_lowercase()))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct LanIdentityHintInventory {
    devices: Vec<LanPairingDeviceRef>,
}

impl LanIdentityHintInventory {
    fn from_devices(identity_hint_devices: &[LanPairingDeviceRef]) -> Self {
        Self {
            devices: identity_hint_devices.to_vec(),
        }
    }

    fn find(&self, mac_address: &str, ip_address: &str) -> Option<&LanPairingDeviceRef> {
        self.devices.iter().find(|identity_hint_device| {
            trusted_device_matches_network_identity(identity_hint_device, mac_address, ip_address)
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LanNeighborObservation {
    ip_address: String,
    mac_address: String,
    network_interface: Option<String>,
    hostname: Option<String>,
    reachability: LanPairingDeviceReachability,
    scan_sources: Vec<String>,
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
    #[serde(default)]
    pub suppressed_active_ipv4_targets: Vec<String>,
}

pub type LanNetworkInventoryDeviceIdentifier = String;
pub type LanNetworkInventoryDisplayLabel = String;

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
    pub reachability: LanPairingDeviceReachability,
    pub agent_status: Option<String>,
    #[serde(default)]
    pub scan_sources: Vec<String>,
    #[serde(default)]
    pub used_previous_scan_hint: bool,
}

pub fn discover_lan_network_devices() -> Vec<LanNetworkInventoryDevice> {
    discover_lan_network_devices_with_hints(&[], &[])
}

pub fn discover_lan_network_devices_with_hints(
    trusted_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
) -> Vec<LanNetworkInventoryDevice> {
    discover_lan_network_devices_with_hints_and_refresh_mode(
        trusted_devices,
        previous_devices,
        LanDiscoveryRefreshMode::Passive,
    )
}

pub fn discover_lan_network_devices_with_hints_and_refresh_mode(
    trusted_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
) -> Vec<LanNetworkInventoryDevice> {
    discover_lan_network_devices_with_hints_refresh_mode_and_probe_suppression(
        trusted_devices,
        previous_devices,
        refresh_mode,
        trusted_devices,
    )
}

pub fn discover_lan_network_devices_with_hints_refresh_mode_and_probe_suppression(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
    probe_suppression_devices: &[LanPairingDeviceRef],
) -> Vec<LanNetworkInventoryDevice> {
    discover_lan_network_devices_with_hints_refresh_mode_and_scan_and_probe_suppression(
        identity_hint_devices,
        previous_devices,
        refresh_mode,
        identity_hint_devices,
        probe_suppression_devices,
    )
}

pub fn discover_lan_network_devices_with_hints_refresh_mode_and_scan_and_probe_suppression(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    probe_suppression_devices: &[LanPairingDeviceRef],
) -> Vec<LanNetworkInventoryDevice> {
    if refresh_mode == LanDiscoveryRefreshMode::ActiveSubnetRefresh {
        stimulate_bounded_ipv4_neighbors(active_refresh_suppression_devices, previous_devices);
    }
    if cfg!(target_os = "windows") {
        windows_lan_neighbors(
            identity_hint_devices,
            previous_devices,
            probe_suppression_devices,
        )
    } else if cfg!(target_os = "linux") {
        linux_lan_neighbors(
            identity_hint_devices,
            previous_devices,
            probe_suppression_devices,
        )
    } else {
        Vec::new()
    }
}

pub fn plan_lan_discovery_scan(
    trusted_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
) -> LanDiscoveryScanPlan {
    plan_lan_discovery_scan_with_active_refresh_suppression(
        trusted_devices,
        previous_devices,
        refresh_mode,
        trusted_devices,
    )
}

pub fn plan_lan_discovery_scan_with_active_refresh_suppression(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
) -> LanDiscoveryScanPlan {
    let identity = local_network_identity();
    scan_plan_for_identity(
        identity.as_ref(),
        identity_hint_devices,
        previous_devices,
        refresh_mode,
        active_refresh_suppression_devices,
    )
}

pub fn local_agent_device_ref(local_device_id: String, platform: String) -> LanPairingDeviceRef {
    let hardware_profile = local_hardware_profile();
    let network_identity = local_network_identity();
    let hostname = hardware_profile.hostname.clone();
    let label = hostname
        .clone()
        .unwrap_or_else(|| constants::lan_pairing::LOCAL_AGENT_LABEL.to_string());
    let mut device = LanPairingDeviceRef::new(local_device_id, None, label, platform);
    device.hostname = hostname;
    if let Some(identity) = network_identity {
        device.ip_address = identity.ip_address;
        device.mac_address = identity.mac_address;
        device.network_interface = identity.network_interface;
    }
    device.agent_status = Some(constants::lan_pairing::LOCAL_AGENT_STATUS.to_string());
    device.hardware_profile = Some(hardware_profile.into_protocol_profile());
    device
}

pub fn discovery_evidence_sources_for_network_device(
    device: &LanNetworkInventoryDevice,
) -> Vec<LanDiscoveryEvidenceSource> {
    effective_scan_sources(device)
        .into_iter()
        .filter_map(|scan_source| discovery_evidence_source_from_scan_source(&scan_source))
        .collect()
}

fn windows_lan_neighbors(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    probe_suppression_devices: &[LanPairingDeviceRef],
) -> Vec<LanNetworkInventoryDevice> {
    let netbios_names = windows_netbios_cache_names();
    let identity_hint_inventory = LanIdentityHintInventory::from_devices(identity_hint_devices);
    let previous_inventory = LanPreviousNetworkInventory::from_devices(previous_devices);
    let mut devices = command_json_records(
        constants::lan_pairing::POWERSHELL_EXE,
        &[
            constants::lan_pairing::POWERSHELL_NO_PROFILE_ARG,
            constants::lan_pairing::POWERSHELL_EXECUTION_POLICY_ARG,
            constants::lan_pairing::POWERSHELL_BYPASS_ARG,
            constants::lan_pairing::POWERSHELL_COMMAND_ARG,
            constants::lan_pairing::POWERSHELL_LAN_NEIGHBOR_COMMAND,
        ],
    )
    .into_iter()
    .filter_map(|record| {
        network_device_from_windows_neighbor(
            &record,
            &netbios_names,
            &identity_hint_inventory,
            &previous_inventory,
        )
    })
    .collect::<Vec<_>>();
    enrich_service_identity_probes(&mut devices, probe_suppression_devices);
    devices
}

fn network_device_from_windows_neighbor(
    record: &serde_json::Value,
    netbios_names: &HashMap<String, String>,
    identity_hint_inventory: &LanIdentityHintInventory,
    previous_inventory: &LanPreviousNetworkInventory,
) -> Option<LanNetworkInventoryDevice> {
    let ip_address = record_text(record, constants::lan_pairing::JSON_KEY_IP_ADDRESS)?;
    if !is_supported_neighbor_ip(&ip_address) {
        return None;
    }
    let mac_address = normalize_mac_address(&record_text(
        record,
        constants::lan_pairing::JSON_KEY_LINK_LAYER_ADDRESS,
    )?)?;
    let supports_netbios = windows_neighbor_supports_netbios(&ip_address);
    let mut platform = if likely_router_address_text(&ip_address) {
        constants::lan_pairing::PLATFORM_ROUTER
    } else {
        constants::lan_pairing::PLATFORM_UNKNOWN
    }
    .to_string();
    let mut device_id = String::from(constants::lan_pairing::NETWORK_NEIGHBOR_DEVICE_PREFIX);
    device_id.push_str(
        &mac_address
            .chars()
            .filter(|character| *character != '-')
            .collect::<String>(),
    );
    let reachability =
        reachability_from_windows_state(record.get(constants::lan_pairing::JSON_KEY_STATE));
    let dns_hostname = record_text(record, constants::lan_pairing::JSON_KEY_HOSTNAME)
        .map(|value| value.trim_end_matches('.').to_string())
        .filter(|value| !value.is_empty());
    let netbios_cache_hostname = netbios_names.get(&ip_address).cloned();
    let netbios_cache_hostname_missing = netbios_cache_hostname.is_none();
    let trusted_device = identity_hint_inventory.find(&mac_address, &ip_address);
    let trusted_hostname = trusted_device_hostname(trusted_device);
    let trusted_label = trusted_device_label(trusted_device);
    let trusted_platform = trusted_device_platform(trusted_device);
    let cached_identity = cached_neighbor_identity(&mac_address);
    let previous_device = previous_inventory.find(&mac_address, &ip_address);
    let previous_hostname = previous_device
        .and_then(|device| device.hostname.clone())
        .filter(|value| !value.is_empty());
    let previous_label = previous_inventory_label(previous_device);
    let previous_platform = previous_device
        .map(|device| device.platform.clone())
        .filter(|value| !value.is_empty());
    let cached_platform = cached_identity
        .as_ref()
        .and_then(|identity| identity.platform.clone());
    let has_reusable_identity = trusted_hostname.is_some()
        || trusted_label.is_some()
        || trusted_platform.is_some()
        || previous_hostname.is_some()
        || previous_label.is_some()
        || previous_platform.is_some();
    let direct_hostname = if dns_hostname.is_none()
        && netbios_cache_hostname.is_none()
        && cached_identity.is_none()
        && !has_reusable_identity
        && supports_netbios
    {
        direct_netbios_hostname(&ip_address, &reachability, &platform)
    } else {
        None
    };
    let direct_hostname_missing = direct_hostname.is_none();
    let used_previous_scan_hostname = dns_hostname.is_none()
        && netbios_cache_hostname.is_none()
        && trusted_hostname.is_none()
        && cached_identity.is_none()
        && previous_hostname.is_some();
    if platform == constants::lan_pairing::PLATFORM_UNKNOWN {
        if let Some(trusted_platform) = trusted_platform.clone() {
            platform = trusted_platform;
        } else if netbios_cache_hostname.is_some() || direct_hostname.is_some() {
            platform = constants::lan_pairing::PLATFORM_WINDOWS.to_string();
        } else if let Some(cached_platform) = cached_platform.clone() {
            platform = cached_platform;
        } else if let Some(previous_platform) = previous_platform.clone() {
            platform = previous_platform;
        }
    }
    let hostname = dns_hostname
        .or(netbios_cache_hostname)
        .or(trusted_hostname)
        .or_else(|| cached_identity.map(|identity| identity.hostname))
        .or(previous_hostname)
        .or(direct_hostname)
        .filter(|value| !value.is_empty());
    let used_previous_scan_label =
        hostname.is_none() && trusted_label.is_none() && previous_label.is_some();
    let used_previous_scan_platform = platform != constants::lan_pairing::PLATFORM_ROUTER
        && trusted_platform.is_none()
        && netbios_cache_hostname_missing
        && direct_hostname_missing
        && cached_platform.is_none()
        && previous_platform.is_some();
    if hostname.is_none() {
        if !has_reusable_identity && supports_netbios {
            warm_netbios_cache(&ip_address, &reachability, &platform);
        }
    } else if let Some(hostname) = hostname.as_deref() {
        remember_neighbor_identity(&mac_address, hostname, &platform);
    }
    let used_previous_scan_hint =
        used_previous_scan_hostname || used_previous_scan_label || used_previous_scan_platform;
    let label = hostname
        .clone()
        .or(trusted_label)
        .or(previous_label)
        .unwrap_or_else(|| network_neighbor_label(&ip_address));

    Some(LanNetworkInventoryDevice {
        device_id,
        label,
        platform,
        ip_address,
        mac_address,
        hostname,
        network_interface: record_text(record, constants::lan_pairing::JSON_KEY_INTERFACE_ALIAS),
        reachability,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        used_previous_scan_hint,
    })
}

fn linux_lan_neighbors(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    probe_suppression_devices: &[LanPairingDeviceRef],
) -> Vec<LanNetworkInventoryDevice> {
    let trusted_inventory = LanIdentityHintInventory::from_devices(identity_hint_devices);
    let previous_inventory = LanPreviousNetworkInventory::from_devices(previous_devices);
    let observations = merge_neighbor_observations(
        linux_ip_neigh_observations()
            .into_iter()
            .chain(linux_proc_net_arp_observations())
            .collect(),
    );
    let mut devices = observations
        .into_iter()
        .filter_map(|observation| {
            network_device_from_neighbor_observation(
                observation,
                &trusted_inventory,
                &previous_inventory,
            )
        })
        .collect::<Vec<_>>();
    enrich_service_identity_probes(&mut devices, probe_suppression_devices);
    devices
}

fn previous_inventory_label(previous_device: Option<&LanNetworkInventoryDevice>) -> Option<String> {
    previous_device
        .map(|device| device.label.clone())
        .filter(|label| {
            !label.is_empty()
                && !label.starts_with(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX)
        })
}

fn trusted_device_hostname(trusted_device: Option<&LanPairingDeviceRef>) -> Option<String> {
    trusted_device
        .and_then(|device| device.hostname.clone())
        .filter(|hostname| {
            !hostname.is_empty()
                && hostname != constants::lan_pairing::NETWORK_NEIGHBOR_UNKNOWN_HOSTNAME
        })
}

fn trusted_device_label(trusted_device: Option<&LanPairingDeviceRef>) -> Option<String> {
    trusted_device
        .map(|device| device.label.trim().to_string())
        .filter(|label| !label.is_empty())
}

fn trusted_device_platform(trusted_device: Option<&LanPairingDeviceRef>) -> Option<String> {
    trusted_device
        .map(|device| device.platform.trim().to_string())
        .filter(|platform| !platform.is_empty())
}

pub(crate) fn service_identity_probe_scan_source() -> &'static str {
    constants::lan_pairing::LAN_SCAN_SOURCE_SERVICE_IDENTITY_PROBE
}

pub(crate) fn is_confirmed_agent_status(status: Option<&str>) -> bool {
    matches!(status, Some(constants::lan_pairing::LOCAL_AGENT_STATUS))
}

pub(crate) fn is_service_identity_probe_status(status: Option<&str>) -> bool {
    matches!(
        status,
        Some(constants::lan_pairing::SERVICE_IDENTITY_PROBE_AGENT_STATUS)
    )
}

fn enrich_service_identity_probes(
    devices: &mut [LanNetworkInventoryDevice],
    probe_suppression_devices: &[LanPairingDeviceRef],
) {
    let ports = service_identity_probe_ports();
    if ports.is_empty() {
        return;
    }

    let candidates = devices
        .iter()
        .enumerate()
        .filter_map(|(index, device)| {
            should_probe_service_identity(device, probe_suppression_devices)
                .then_some((index, device.ip_address.clone()))
        })
        .collect::<Vec<_>>();

    for batch in candidates.chunks(SERVICE_IDENTITY_PROBE_MAX_CONCURRENCY) {
        let mut probe_results = Vec::new();
        thread::scope(|scope| {
            let mut handles = Vec::new();
            for (index, ip_address) in batch {
                let ports = ports.clone();
                let ip_address = ip_address.clone();
                handles.push(
                    scope.spawn(move || (*index, probe_service_identity(&ip_address, &ports))),
                );
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

fn should_probe_service_identity(
    device: &LanNetworkInventoryDevice,
    probe_suppression_devices: &[LanPairingDeviceRef],
) -> bool {
    if device.platform == constants::lan_pairing::PLATFORM_ROUTER
        || device.agent_status.is_some()
        || probe_suppression_devices
            .iter()
            .any(|probe_suppression_device| same_network_device(probe_suppression_device, device))
    {
        return false;
    }

    matches!(
        device.reachability,
        LanPairingDeviceReachability::Online | LanPairingDeviceReachability::Stale
    )
}

fn same_network_device(
    trusted_device: &LanPairingDeviceRef,
    network_device: &LanNetworkInventoryDevice,
) -> bool {
    trusted_device_matches_network_identity(
        trusted_device,
        &network_device.mac_address,
        &network_device.ip_address,
    )
}

fn trusted_device_matches_network_identity(
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

    if let Some(trusted_mac_address) = trusted_mac_address {
        return trusted_mac_address.eq_ignore_ascii_case(network_mac_address.trim());
    }

    trusted_ip_address
        .map(|trusted_ip_address| {
            trusted_ip_address.eq_ignore_ascii_case(network_ip_address.trim())
        })
        .unwrap_or(false)
}

fn apply_service_identity_probe(
    device: &mut LanNetworkInventoryDevice,
    probe_match: LanServiceIdentityProbeMatch,
) {
    device.agent_status =
        Some(constants::lan_pairing::SERVICE_IDENTITY_PROBE_AGENT_STATUS.to_string());
    if device.hostname.is_none() {
        device.hostname = probe_match.hostname.clone();
    }
    if device.platform == constants::lan_pairing::PLATFORM_UNKNOWN {
        if let Some(platform) = probe_match.platform {
            device.platform = platform;
        }
    }
    if let Some(hostname) = device.hostname.as_ref() {
        if device
            .label
            .starts_with(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX)
        {
            device.label = hostname.clone();
        }
    }
}

fn probe_service_identity(ip_address: &str, ports: &[u16]) -> Option<LanServiceIdentityProbeMatch> {
    for port in ports {
        if let Some(probe_match) = probe_service_identity_on_port(ip_address, *port) {
            return Some(probe_match);
        }
    }
    None
}

fn probe_service_identity_on_port(
    ip_address: &str,
    port: u16,
) -> Option<LanServiceIdentityProbeMatch> {
    let endpoint = SocketAddr::new(ip_address.parse::<Ipv4Addr>().ok()?.into(), port);
    let timeout = Duration::from_millis(SERVICE_IDENTITY_PROBE_CONNECT_TIMEOUT_MS);
    let mut stream = TcpStream::connect_timeout(&endpoint, timeout).ok()?;
    let read_timeout = Some(Duration::from_millis(
        SERVICE_IDENTITY_PROBE_READ_TIMEOUT_MS,
    ));
    let _ = stream.set_read_timeout(read_timeout);
    let _ = stream.set_write_timeout(read_timeout);
    write_probe_request(&mut stream, &endpoint).ok()?;
    let _ = stream.shutdown(Shutdown::Write);
    parse_probe_match(&read_probe_response(&mut stream)?)
}

fn write_probe_request(stream: &mut TcpStream, endpoint: &SocketAddr) -> std::io::Result<()> {
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nAccept: application/json\r\nConnection: close\r\n\r\n",
        constants::endpoint::HEALTH,
        endpoint
    );
    stream.write_all(request.as_bytes())
}

fn read_probe_response(stream: &mut TcpStream) -> Option<Vec<u8>> {
    let mut response = Vec::new();
    let mut chunk = [0_u8; 1024];

    loop {
        match stream.read(&mut chunk) {
            Ok(0) => break,
            Ok(read) => {
                if response.len().saturating_add(read) > SERVICE_IDENTITY_PROBE_MAX_RESPONSE_BYTES {
                    return None;
                }
                response.extend_from_slice(&chunk[..read]);
            }
            Err(error)
                if error.kind() == std::io::ErrorKind::WouldBlock
                    || error.kind() == std::io::ErrorKind::TimedOut =>
            {
                break;
            }
            Err(_) => return None,
        }
    }

    (!response.is_empty()).then_some(response)
}

fn parse_probe_match(response: &[u8]) -> Option<LanServiceIdentityProbeMatch> {
    let (status_line, body) = http_status_and_body(response)?;
    if !(status_line.starts_with("HTTP/1.1 200") || status_line.starts_with("HTTP/1.0 200")) {
        return None;
    }
    let snapshot = serde_json::from_slice::<AgentLogSnapshot>(body).ok()?;
    Some(LanServiceIdentityProbeMatch {
        hostname: sanitize_probe_identity(&snapshot.agent.hostname, 64),
        platform: sanitize_probe_platform(&snapshot.agent.platform),
    })
}

fn http_status_and_body(response: &[u8]) -> Option<(&str, &[u8])> {
    let header_end = response
        .windows(4)
        .position(|window| window == b"\r\n\r\n")?;
    let header_bytes = &response[..header_end];
    let status_line_bytes = header_bytes.split(|byte| *byte == b'\n').next()?;
    let status_line = std::str::from_utf8(status_line_bytes)
        .ok()?
        .trim_end_matches('\r');
    Some((status_line, &response[(header_end + 4)..]))
}

fn sanitize_probe_identity(value: &str, max_length: usize) -> Option<String> {
    let sanitized = value
        .chars()
        .filter(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.' | ' ')
        })
        .take(max_length)
        .collect::<String>()
        .trim()
        .to_string();
    if sanitized.is_empty() || sanitized == constants::value::UNKNOWN_HOST {
        None
    } else {
        Some(sanitized)
    }
}

fn sanitize_probe_platform(value: &str) -> Option<String> {
    let sanitized = value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
        .take(32)
        .collect::<String>()
        .to_ascii_lowercase();
    (!sanitized.is_empty()).then_some(sanitized)
}

fn service_identity_probe_ports() -> Vec<u16> {
    let mut ports = Vec::new();
    push_unique_port(&mut ports, default_agent_port());
    if let Some(configured_port) = configured_agent_port() {
        push_unique_port(&mut ports, configured_port);
    }
    ports
}

fn configured_agent_port() -> Option<u16> {
    env::var(constants::env_var::AGENT_ADDR)
        .ok()
        .and_then(|value| value.parse::<SocketAddr>().ok())
        .map(|socket| socket.port())
}

fn default_agent_port() -> u16 {
    constants::bind::DEFAULT_AGENT_ADDR
        .parse::<SocketAddr>()
        .ok()
        .map(|socket| socket.port())
        .unwrap_or(4477)
}

fn push_unique_port(ports: &mut Vec<u16>, port: u16) {
    if !ports.contains(&port) {
        ports.push(port);
    }
}

fn stimulate_bounded_ipv4_neighbors(
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
) {
    let Some(identity) = local_network_identity() else {
        return;
    };
    let targets = bounded_active_ipv4_targets(
        identity.ip_address.as_deref(),
        identity.ipv4_cidr.as_deref(),
        identity.default_gateway.as_deref(),
        active_refresh_suppression_devices,
        previous_devices,
    );
    if targets.is_empty() {
        return;
    }
    for batch in targets.chunks(constants::lan_pairing::LAN_ACTIVE_IPV4_SWEEP_MAX_CONCURRENCY) {
        thread::scope(|scope| {
            let mut handles = Vec::new();
            for ip_address in batch {
                let ip_address = ip_address.clone();
                handles.push(scope.spawn(move || stimulate_ipv4_neighbor(&ip_address)));
            }
            for handle in handles {
                let _ = handle.join();
            }
        });
    }
}

fn stimulate_ipv4_neighbor(ip_address: &str) {
    let mut command = Command::new(constants::lan_pairing::PING_EXE);
    command.stdout(Stdio::null()).stderr(Stdio::null());
    if cfg!(target_os = "windows") {
        command.args([
            constants::lan_pairing::PING_WINDOWS_COUNT_ARG,
            "1",
            constants::lan_pairing::PING_WINDOWS_TIMEOUT_ARG,
            "200",
            ip_address,
        ]);
    } else if cfg!(target_os = "linux") {
        command.args([
            constants::lan_pairing::PING_LINUX_COUNT_ARG,
            "1",
            constants::lan_pairing::PING_LINUX_TIMEOUT_ARG,
            "1",
            ip_address,
        ]);
    } else {
        return;
    }
    let _ = command.status();
}

fn scan_plan_for_identity(
    identity: Option<&LocalNetworkIdentity>,
    _identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
) -> LanDiscoveryScanPlan {
    let selected_interface = identity.and_then(|identity| identity.network_interface.clone());
    let local_ip_address = identity.and_then(|identity| identity.ip_address.clone());
    let ipv4_cidr = identity.and_then(|identity| identity.ipv4_cidr.clone());
    let default_gateway = identity.and_then(|identity| identity.default_gateway.clone());
    let dns_servers = identity
        .map(|identity| identity.dns_servers.clone())
        .unwrap_or_default();
    let dhcp_server = identity.and_then(|identity| identity.dhcp_server.clone());
    let broadcast_address = identity.and_then(|identity| identity.broadcast_address.clone());
    let ipv6_prefixes = identity
        .map(|identity| identity.ipv6_prefixes.clone())
        .unwrap_or_default();

    let (
        active_ipv4_candidate_count,
        active_ipv4_target_count,
        prioritized_previous_target_count,
        active_ipv4_target_timeout_ms,
        suppressed_active_ipv4_targets,
    ) = if refresh_mode == LanDiscoveryRefreshMode::ActiveSubnetRefresh {
        let candidate_targets = bounded_active_ipv4_candidate_targets(
            local_ip_address.as_deref(),
            ipv4_cidr.as_deref(),
        );
        let suppressed_targets = suppressed_active_ipv4_targets(
            default_gateway.as_deref(),
            active_refresh_suppression_devices,
            previous_devices,
        );
        let prioritized_targets = prioritized_active_ipv4_targets(
            &candidate_targets,
            &suppressed_targets,
            previous_devices,
        );
        let mut suppressed_active_ipv4_targets = suppressed_targets.into_iter().collect::<Vec<_>>();
        suppressed_active_ipv4_targets.sort();
        (
            saturating_u32(candidate_targets.len()),
            saturating_u32(
                bounded_active_ipv4_targets(
                    local_ip_address.as_deref(),
                    ipv4_cidr.as_deref(),
                    default_gateway.as_deref(),
                    active_refresh_suppression_devices,
                    previous_devices,
                )
                .len(),
            ),
            saturating_u32(prioritized_targets.len()),
            active_ipv4_target_timeout_ms(),
            suppressed_active_ipv4_targets,
        )
    } else {
        (0, 0, 0, None, Vec::new())
    };

    LanDiscoveryScanPlan {
        refresh_mode,
        selected_interface,
        local_ip_address,
        ipv4_cidr,
        default_gateway,
        dns_servers,
        dhcp_server,
        broadcast_address,
        ipv6_prefixes,
        trusted_truth_device_count: saturating_u32(active_refresh_suppression_devices.len()),
        previous_device_count: saturating_u32(previous_devices.len()),
        active_ipv4_candidate_count,
        active_ipv4_target_count,
        prioritized_previous_target_count,
        active_ipv4_target_timeout_ms,
        suppressed_active_ipv4_targets,
    }
}

fn active_ipv4_target_timeout_ms() -> Option<u64> {
    if cfg!(target_os = "windows") {
        Some(200)
    } else if cfg!(target_os = "linux") {
        Some(1000)
    } else {
        None
    }
}

fn saturating_u32(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}

fn bounded_active_ipv4_targets(
    ip_address: Option<&str>,
    ipv4_cidr: Option<&str>,
    default_gateway: Option<&str>,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
) -> Vec<String> {
    let candidate_targets = bounded_active_ipv4_candidate_targets(ip_address, ipv4_cidr);
    if candidate_targets.is_empty() {
        return candidate_targets;
    }
    let suppressed_targets = suppressed_active_ipv4_targets(
        default_gateway,
        active_refresh_suppression_devices,
        previous_devices,
    );
    let mut prioritized_targets =
        prioritized_active_ipv4_targets(&candidate_targets, &suppressed_targets, previous_devices);
    let prioritized_index = prioritized_targets
        .iter()
        .map(|target| target.to_ascii_lowercase())
        .collect::<HashSet<_>>();
    for target in candidate_targets {
        let normalized_target = target.to_ascii_lowercase();
        if suppressed_targets.contains(&normalized_target)
            || prioritized_index.contains(&normalized_target)
        {
            continue;
        }
        prioritized_targets.push(target);
    }
    prioritized_targets
}

fn bounded_active_ipv4_candidate_targets(
    ip_address: Option<&str>,
    ipv4_cidr: Option<&str>,
) -> Vec<String> {
    let Some((host_ip, prefix_length)) = parse_ipv4_cidr(ip_address, ipv4_cidr) else {
        return Vec::new();
    };
    let effective_prefix_length = if prefix_length < 24 {
        24
    } else {
        prefix_length
    };
    if effective_prefix_length >= 31 {
        return Vec::new();
    }
    let host_bits = 32_u32.saturating_sub(u32::from(effective_prefix_length));
    let mask = u32::MAX << host_bits;
    let network = u32::from(host_ip) & mask;
    let broadcast = network | !mask;
    let mut targets = Vec::new();
    for raw_ip in (network.saturating_add(1))..broadcast {
        if raw_ip == u32::from(host_ip) {
            continue;
        }
        targets.push(Ipv4Addr::from(raw_ip).to_string());
        if targets.len() >= constants::lan_pairing::LAN_ACTIVE_IPV4_SWEEP_MAX_HOSTS as usize {
            break;
        }
    }
    targets
}

fn suppressed_active_ipv4_targets(
    default_gateway: Option<&str>,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
) -> HashSet<String> {
    let current_observations = current_active_refresh_ipv4_observations();
    suppressed_active_ipv4_targets_for_current_observations(
        default_gateway,
        active_refresh_suppression_devices,
        previous_devices,
        &current_observations,
    )
}

fn suppressed_active_ipv4_targets_for_current_observations(
    default_gateway: Option<&str>,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    current_observations: &HashMap<String, String>,
) -> HashSet<String> {
    let mut targets = HashSet::new();
    for truth_device in active_refresh_suppression_devices {
        if let Some(ip_address) = normalized_household_ipv4(truth_device.ip_address.as_deref()) {
            if !active_refresh_target_is_safely_confirmed(
                default_gateway,
                &ip_address,
                truth_device.mac_address.as_deref(),
                current_observations,
            ) {
                continue;
            }
            targets.insert(ip_address);
        }
    }
    for previous_device in previous_devices {
        if !previous_device_should_skip_active_refresh(previous_device, default_gateway) {
            continue;
        }
        if let Some(ip_address) =
            normalized_household_ipv4(Some(previous_device.ip_address.as_str()))
        {
            if !active_refresh_target_is_safely_confirmed(
                default_gateway,
                &ip_address,
                Some(previous_device.mac_address.as_str()),
                current_observations,
            ) {
                continue;
            }
            targets.insert(ip_address);
        }
    }
    targets
}

fn active_refresh_target_is_safely_confirmed(
    default_gateway: Option<&str>,
    ip_address: &str,
    expected_mac_address: Option<&str>,
    current_observations: &HashMap<String, String>,
) -> bool {
    if default_gateway
        .map(|gateway| gateway.eq_ignore_ascii_case(ip_address))
        .unwrap_or(false)
    {
        return true;
    }

    current_observation_confirms_ip_and_mac(current_observations, ip_address, expected_mac_address)
}

fn current_observation_confirms_ip_and_mac(
    current_observations: &HashMap<String, String>,
    ip_address: &str,
    expected_mac_address: Option<&str>,
) -> bool {
    let Some(expected_mac_address) = expected_mac_address
        .map(str::trim)
        .filter(|value| !value.is_empty())
    else {
        return false;
    };

    current_observations
        .get(&ip_address.to_ascii_lowercase())
        .map(|current_mac_address| current_mac_address.eq_ignore_ascii_case(expected_mac_address))
        .unwrap_or(false)
}

fn prioritized_active_ipv4_targets(
    candidate_targets: &[String],
    suppressed_targets: &HashSet<String>,
    previous_devices: &[LanNetworkInventoryDevice],
) -> Vec<String> {
    let candidate_index = candidate_targets
        .iter()
        .map(|target| target.to_ascii_lowercase())
        .collect::<HashSet<_>>();
    let mut prioritized = Vec::new();
    let mut prioritized_index = HashSet::new();
    for prefer_unresolved in [true, false] {
        for previous_device in previous_devices {
            if prefer_unresolved != previous_device_needs_active_refresh_priority(previous_device) {
                continue;
            }
            let ip_address = previous_device.ip_address.to_ascii_lowercase();
            if !candidate_index.contains(&ip_address)
                || suppressed_targets.contains(&ip_address)
                || !prioritized_index.insert(ip_address.clone())
            {
                continue;
            }
            prioritized.push(previous_device.ip_address.clone());
        }
    }
    prioritized
}

fn previous_device_should_skip_active_refresh(
    previous_device: &LanNetworkInventoryDevice,
    default_gateway: Option<&str>,
) -> bool {
    previous_device.platform == constants::lan_pairing::PLATFORM_ROUTER
        || default_gateway
            .map(|gateway| gateway.eq_ignore_ascii_case(previous_device.ip_address.trim()))
            .unwrap_or(false)
}

fn previous_device_needs_active_refresh_priority(
    previous_device: &LanNetworkInventoryDevice,
) -> bool {
    previous_device.used_previous_scan_hint
        || previous_device.platform == constants::lan_pairing::PLATFORM_UNKNOWN
        || previous_device.hostname.is_none()
        || previous_device
            .label
            .starts_with(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX)
}

fn parse_ipv4_cidr(ip_address: Option<&str>, ipv4_cidr: Option<&str>) -> Option<(Ipv4Addr, u8)> {
    let host_ip = ip_address?.parse::<Ipv4Addr>().ok()?;
    let (_, prefix_length) = ipv4_cidr?.split_once('/')?;
    let prefix_length = prefix_length.parse::<u8>().ok()?;
    (prefix_length <= 32).then_some((host_ip, prefix_length))
}

fn normalized_household_ipv4(ip_address: Option<&str>) -> Option<String> {
    let ip = ip_address?.trim().parse::<Ipv4Addr>().ok()?;
    is_household_unicast(ip).then_some(ip.to_string())
}

fn current_active_refresh_ipv4_observations() -> HashMap<String, String> {
    if cfg!(target_os = "windows") {
        current_windows_neighbor_ipv4_observations()
    } else if cfg!(target_os = "linux") {
        current_linux_neighbor_ipv4_observations()
    } else {
        HashMap::new()
    }
}

fn current_windows_neighbor_ipv4_observations() -> HashMap<String, String> {
    command_json_records(
        constants::lan_pairing::POWERSHELL_EXE,
        &[
            constants::lan_pairing::POWERSHELL_NO_PROFILE_ARG,
            constants::lan_pairing::POWERSHELL_EXECUTION_POLICY_ARG,
            constants::lan_pairing::POWERSHELL_BYPASS_ARG,
            constants::lan_pairing::POWERSHELL_COMMAND_ARG,
            constants::lan_pairing::POWERSHELL_LAN_NEIGHBOR_COMMAND,
        ],
    )
    .into_iter()
    .filter_map(|record| {
        let ip_address = record_text(&record, constants::lan_pairing::JSON_KEY_IP_ADDRESS)?;
        let ip_address = normalized_household_ipv4(Some(ip_address.as_str()))?;
        let mac_address = normalize_mac_address(&record_text(
            &record,
            constants::lan_pairing::JSON_KEY_LINK_LAYER_ADDRESS,
        )?)?;
        Some((
            ip_address.to_ascii_lowercase(),
            mac_address.to_ascii_lowercase(),
        ))
    })
    .collect()
}

fn current_linux_neighbor_ipv4_observations() -> HashMap<String, String> {
    merge_neighbor_observations(
        linux_ip_neigh_observations()
            .into_iter()
            .chain(linux_proc_net_arp_observations())
            .collect(),
    )
    .into_iter()
    .filter_map(|observation| {
        let ip_address = normalized_household_ipv4(Some(observation.ip_address.as_str()))?;
        Some((
            ip_address.to_ascii_lowercase(),
            observation.mac_address.to_ascii_lowercase(),
        ))
    })
    .collect()
}

fn linux_ip_neigh_observations() -> Vec<LanNeighborObservation> {
    command_json_records(
        constants::lan_pairing::IP_EXE,
        &[
            constants::lan_pairing::IP_JSON_ARG,
            constants::lan_pairing::IP_NEIGH_ARG,
        ],
    )
    .into_iter()
    .filter_map(|record| linux_ip_neigh_observation(&record))
    .collect()
}

fn linux_ip_neigh_observation(record: &serde_json::Value) -> Option<LanNeighborObservation> {
    let ip_address = record_text(record, constants::lan_pairing::JSON_KEY_DST)?;
    if !is_supported_neighbor_ip(&ip_address) {
        return None;
    }
    let mac_address = normalize_mac_address(&record_text(
        record,
        constants::lan_pairing::JSON_KEY_LLADDR,
    )?)?;
    let reachability =
        reachability_from_linux_state(record.get(constants::lan_pairing::JSON_KEY_LOWER_STATE))?;

    Some(LanNeighborObservation {
        ip_address,
        mac_address,
        network_interface: record_text(record, constants::lan_pairing::JSON_KEY_DEV),
        hostname: None,
        reachability,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string()],
    })
}

fn linux_proc_net_arp_observations() -> Vec<LanNeighborObservation> {
    fs::read_to_string(constants::lan_pairing::LINUX_PROC_NET_ARP_PATH)
        .ok()
        .map(|output| {
            output
                .lines()
                .skip(1)
                .filter_map(linux_proc_net_arp_observation)
                .collect()
        })
        .unwrap_or_default()
}

fn linux_proc_net_arp_observation(line: &str) -> Option<LanNeighborObservation> {
    let columns = line.split_whitespace().collect::<Vec<_>>();
    if columns.len() < 6 {
        return None;
    }
    let ip_address = columns[0].trim().to_string();
    if !is_supported_neighbor_ip(&ip_address) {
        return None;
    }
    let flags = parse_proc_net_arp_flags(columns[2])?;
    if flags & 0x2 == 0 {
        return None;
    }
    let mac_address = normalize_mac_address(columns[3])?;

    Some(LanNeighborObservation {
        ip_address,
        mac_address,
        network_interface: Some(columns[5].trim().to_string()).filter(|value| !value.is_empty()),
        hostname: None,
        reachability: LanPairingDeviceReachability::Stale,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string()],
    })
}

fn parse_proc_net_arp_flags(value: &str) -> Option<u32> {
    let normalized = value
        .trim()
        .trim_start_matches("0x")
        .trim_start_matches("0X");
    u32::from_str_radix(normalized, 16).ok()
}

fn merge_neighbor_observations(
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
    if should_replace_observation_ip(&existing.ip_address, &incoming.ip_address) {
        existing.ip_address = incoming.ip_address.clone();
    }
    if existing.network_interface.is_none() {
        existing.network_interface = incoming.network_interface.clone();
    }
    if existing.hostname.is_none() {
        existing.hostname = incoming.hostname.clone();
    }
    if reachability_rank(&incoming.reachability) > reachability_rank(&existing.reachability) {
        existing.reachability = incoming.reachability.clone();
    }
    for scan_source in incoming.scan_sources {
        push_unique_scan_source(&mut existing.scan_sources, &scan_source);
    }
}

fn should_replace_observation_ip(existing_ip: &str, incoming_ip: &str) -> bool {
    let existing_is_private_ipv4 = parse_private_ipv4(existing_ip).is_some();
    let incoming_is_private_ipv4 = parse_private_ipv4(incoming_ip).is_some();
    (!existing_is_private_ipv4 && incoming_is_private_ipv4) || existing_ip.is_empty()
}

fn windows_neighbor_supports_netbios(ip_address: &str) -> bool {
    ip_address
        .parse::<Ipv4Addr>()
        .map(is_household_unicast)
        .unwrap_or(false)
}

fn parse_private_ipv4(value: &str) -> Option<Ipv4Addr> {
    let ip = value.parse::<Ipv4Addr>().ok()?;
    is_household_unicast(ip).then_some(ip)
}

fn reachability_rank(reachability: &LanPairingDeviceReachability) -> u8 {
    match reachability {
        LanPairingDeviceReachability::Online => 3,
        LanPairingDeviceReachability::Stale => 2,
        LanPairingDeviceReachability::Offline => 1,
    }
}

fn network_device_from_neighbor_observation(
    observation: LanNeighborObservation,
    trusted_inventory: &LanIdentityHintInventory,
    previous_inventory: &LanPreviousNetworkInventory,
) -> Option<LanNetworkInventoryDevice> {
    let LanNeighborObservation {
        ip_address,
        mac_address,
        network_interface,
        hostname: hostname_hint,
        reachability,
        scan_sources,
    } = observation;

    let mut platform = if likely_router_address_text(&ip_address) {
        constants::lan_pairing::PLATFORM_ROUTER
    } else {
        constants::lan_pairing::PLATFORM_UNKNOWN
    }
    .to_string();
    let trusted_device = trusted_inventory.find(&mac_address, &ip_address);
    let trusted_hostname = trusted_device_hostname(trusted_device);
    let trusted_label = trusted_device_label(trusted_device);
    let trusted_platform = trusted_device_platform(trusted_device);
    let cached_identity = cached_neighbor_identity(&mac_address);
    let previous_device = previous_inventory.find(&mac_address, &ip_address);
    let previous_hostname = previous_device
        .and_then(|device| device.hostname.clone())
        .filter(|value| !value.is_empty());
    let previous_label = previous_inventory_label(previous_device);
    let previous_platform = previous_device
        .map(|device| device.platform.clone())
        .filter(|value| !value.is_empty());
    let cached_platform = cached_identity
        .as_ref()
        .and_then(|identity| identity.platform.clone());
    let used_previous_scan_hostname = hostname_hint.is_none()
        && trusted_hostname.is_none()
        && cached_identity.is_none()
        && previous_hostname.is_some();
    if platform == constants::lan_pairing::PLATFORM_UNKNOWN {
        if let Some(trusted_platform) = trusted_platform.clone() {
            platform = trusted_platform;
        } else if let Some(cached_platform) = cached_platform.clone() {
            platform = cached_platform;
        } else if let Some(previous_platform) = previous_platform.clone() {
            platform = previous_platform;
        }
    }
    let hostname = hostname_hint
        .or(trusted_hostname)
        .or_else(|| cached_identity.map(|identity| identity.hostname))
        .or(previous_hostname)
        .filter(|value| !value.is_empty());
    if let Some(hostname) = hostname.as_deref() {
        remember_neighbor_identity(&mac_address, hostname, &platform);
    }
    let used_previous_scan_label =
        hostname.is_none() && trusted_label.is_none() && previous_label.is_some();
    let used_previous_scan_platform = platform != constants::lan_pairing::PLATFORM_ROUTER
        && trusted_platform.is_none()
        && cached_platform.is_none()
        && previous_platform.is_some();
    let used_previous_scan_hint =
        used_previous_scan_hostname || used_previous_scan_label || used_previous_scan_platform;
    let label = hostname
        .clone()
        .or(trusted_label)
        .or(previous_label)
        .unwrap_or_else(|| network_neighbor_label(&ip_address));
    let mut device_id = String::from(constants::lan_pairing::NETWORK_NEIGHBOR_DEVICE_PREFIX);
    device_id.push_str(
        &mac_address
            .chars()
            .filter(|character| *character != '-')
            .collect::<String>(),
    );

    Some(LanNetworkInventoryDevice {
        device_id,
        label,
        platform,
        ip_address,
        mac_address,
        hostname,
        network_interface,
        reachability,
        agent_status: None,
        scan_sources,
        used_previous_scan_hint,
    })
}

fn push_unique_scan_source(scan_sources: &mut Vec<String>, value: &str) {
    if scan_sources.iter().any(|existing| existing == value) {
        return;
    }
    scan_sources.push(value.to_string());
}

fn effective_scan_sources(device: &LanNetworkInventoryDevice) -> Vec<String> {
    if device.scan_sources.is_empty() {
        vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()]
    } else {
        device.scan_sources.clone()
    }
}

fn discovery_evidence_source_from_scan_source(
    scan_source: &str,
) -> Option<LanDiscoveryEvidenceSource> {
    match scan_source {
        constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR => {
            Some(LanDiscoveryEvidenceSource::WindowsNeighborTable)
        }
        constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP => {
            Some(LanDiscoveryEvidenceSource::LinuxProcNetArp)
        }
        constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH => {
            Some(LanDiscoveryEvidenceSource::LinuxIpNeigh)
        }
        constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP => {
            Some(LanDiscoveryEvidenceSource::MacosArp)
        }
        _ => None,
    }
}

fn is_supported_neighbor_ip(ip_address: &str) -> bool {
    match ip_address.parse::<IpAddr>() {
        Ok(IpAddr::V4(ip)) => is_household_unicast(ip),
        Ok(IpAddr::V6(ip)) => !ip.is_loopback() && !ip.is_multicast() && !ip.is_unspecified(),
        Err(_) => false,
    }
}

fn likely_router_address_text(ip_address: &str) -> bool {
    ip_address
        .parse::<Ipv4Addr>()
        .ok()
        .map(likely_router_address)
        .unwrap_or(false)
}

fn reachability_from_linux_state(
    state: Option<&serde_json::Value>,
) -> Option<LanPairingDeviceReachability> {
    let states = linux_state_labels(state);
    if states.is_empty() {
        return None;
    }
    if states
        .iter()
        .any(|state| state == constants::lan_pairing::LINUX_NEIGHBOR_STATE_INCOMPLETE)
    {
        return None;
    }
    if states.iter().any(|state| {
        state == constants::lan_pairing::LINUX_NEIGHBOR_STATE_REACHABLE
            || state == constants::lan_pairing::LINUX_NEIGHBOR_STATE_PERMANENT
            || state == constants::lan_pairing::LINUX_NEIGHBOR_STATE_DELAY
            || state == constants::lan_pairing::LINUX_NEIGHBOR_STATE_PROBE
    }) {
        return Some(LanPairingDeviceReachability::Online);
    }
    if states
        .iter()
        .any(|state| state == constants::lan_pairing::LINUX_NEIGHBOR_STATE_STALE)
    {
        return Some(LanPairingDeviceReachability::Stale);
    }
    if states
        .iter()
        .any(|state| state == constants::lan_pairing::LINUX_NEIGHBOR_STATE_FAILED)
    {
        return Some(LanPairingDeviceReachability::Offline);
    }
    Some(LanPairingDeviceReachability::Stale)
}

fn linux_state_labels(state: Option<&serde_json::Value>) -> Vec<String> {
    match state {
        Some(serde_json::Value::Array(values)) => values
            .iter()
            .filter_map(value_text)
            .map(|value| value.to_ascii_lowercase())
            .collect(),
        Some(serde_json::Value::String(value)) => vec![value.trim().to_ascii_lowercase()],
        Some(other) => value_text(other)
            .map(|value| vec![value.to_ascii_lowercase()])
            .unwrap_or_default(),
        None => Vec::new(),
    }
}

fn remember_neighbor_identity(mac_address: &str, hostname: &str, platform: &str) {
    if hostname == constants::lan_pairing::NETWORK_NEIGHBOR_UNKNOWN_HOSTNAME {
        return;
    }
    let platform = if platform == constants::lan_pairing::PLATFORM_UNKNOWN {
        None
    } else {
        Some(platform.to_string())
    };
    let cache = NEIGHBOR_HOSTNAME_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let _ = cache.lock().map(|mut entries| {
        entries.insert(
            mac_address.to_ascii_lowercase(),
            LanNeighborIdentityCacheEntry {
                hostname: hostname.to_string(),
                platform,
            },
        )
    });
}

fn cached_neighbor_identity(mac_address: &str) -> Option<LanNeighborIdentityCacheEntry> {
    NEIGHBOR_HOSTNAME_CACHE
        .get()
        .and_then(|cache| cache.lock().ok())
        .and_then(|entries| entries.get(&mac_address.to_ascii_lowercase()).cloned())
}

fn direct_netbios_hostname(
    ip_address: &str,
    reachability: &LanPairingDeviceReachability,
    platform: &str,
) -> Option<String> {
    if reachability != &LanPairingDeviceReachability::Online
        || platform == constants::lan_pairing::PLATFORM_ROUTER
    {
        return None;
    }
    command_stdout(
        constants::lan_pairing::NBTSTAT_EXE,
        &[
            constants::lan_pairing::NBTSTAT_ADAPTER_STATUS_ARG,
            ip_address,
        ],
    )
    .and_then(|output| output.lines().find_map(netbios_adapter_status_name))
}

fn netbios_adapter_status_name(line: &str) -> Option<String> {
    let columns = line.split_whitespace().collect::<Vec<_>>();
    if columns.len() < 3 || columns[2] != constants::lan_pairing::NBTSTAT_UNIQUE_MARKER {
        return None;
    }
    if columns[1] != constants::lan_pairing::NBTSTAT_WORKSTATION_SERVICE_MARKER
        && columns[1] != constants::lan_pairing::NBTSTAT_SERVER_SERVICE_MARKER
    {
        return None;
    }
    Some(columns[0].to_string()).filter(|value| !value.is_empty())
}

fn windows_netbios_cache_names() -> HashMap<String, String> {
    command_stdout(
        constants::lan_pairing::NBTSTAT_EXE,
        &[constants::lan_pairing::NBTSTAT_CACHE_ARG],
    )
    .map(|output| {
        output
            .lines()
            .filter_map(netbios_cache_entry)
            .collect::<HashMap<_, _>>()
    })
    .unwrap_or_default()
}

fn netbios_cache_entry(line: &str) -> Option<(String, String)> {
    let columns = line.split_whitespace().collect::<Vec<_>>();
    if columns.len() < 4 || columns[2] != constants::lan_pairing::NBTSTAT_UNIQUE_MARKER {
        return None;
    }
    let ip = columns[3].parse::<Ipv4Addr>().ok()?;
    if !is_household_unicast(ip) {
        return None;
    }
    Some((columns[3].to_string(), columns[0].to_string()))
}

fn warm_netbios_cache(
    ip_address: &str,
    reachability: &LanPairingDeviceReachability,
    platform: &str,
) {
    if reachability != &LanPairingDeviceReachability::Online
        || platform == constants::lan_pairing::PLATFORM_ROUTER
    {
        return;
    }
    let cache = NETBIOS_CACHE_WARMED_IPS.get_or_init(|| Mutex::new(HashSet::new()));
    if !cache
        .lock()
        .map(|mut warmed| warmed.insert(ip_address.to_string()))
        .unwrap_or(false)
    {
        return;
    }
    let _ = Command::new(constants::lan_pairing::NBTSTAT_EXE)
        .arg(constants::lan_pairing::NBTSTAT_ADAPTER_STATUS_ARG)
        .arg(ip_address)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();
}

fn network_neighbor_label(ip_address: &str) -> String {
    let mut label = String::from(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX);
    label.push_str(ip_address);
    label
}

fn is_household_unicast(ip: Ipv4Addr) -> bool {
    ip.is_private()
        && !ip.is_broadcast()
        && !ip.is_link_local()
        && !ip.is_loopback()
        && !ip.is_multicast()
        && !ip.is_unspecified()
}

fn likely_router_address(ip: Ipv4Addr) -> bool {
    matches!(ip.octets()[3], 1 | 254)
}

fn reachability_from_windows_state(
    state: Option<&serde_json::Value>,
) -> LanPairingDeviceReachability {
    match state
        .and_then(value_text)
        .map(|value| value.to_ascii_lowercase())
    {
        Some(value)
            if value == constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE_NUMBER
                || value == constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_PERMANENT_NUMBER
                || value == constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE
                || value == constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_PERMANENT =>
        {
            LanPairingDeviceReachability::Online
        }
        Some(value)
            if value == constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_STALE_NUMBER
                || value == constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_STALE =>
        {
            LanPairingDeviceReachability::Stale
        }
        _ => LanPairingDeviceReachability::Offline,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Map, Value};

    #[test]
    fn windows_neighbor_parser_keeps_cached_hostname_when_later_rows_are_ip_only() {
        if let Some(cache) = NEIGHBOR_HOSTNAME_CACHE.get() {
            cache
                .lock()
                .unwrap_or_else(|_| {
                    panic!(
                        "{}",
                        constants::lan_pairing::TEST_NEIGHBOR_CACHE_LOCK_EXPECT
                    )
                })
                .clear();
        }
        let named = network_device_from_windows_neighbor(
            &neighbor_record(Some(constants::lan_pairing::TEST_HOSTNAME)),
            &HashMap::new(),
            &LanIdentityHintInventory::default(),
            &LanPreviousNetworkInventory::default(),
        )
        .unwrap_or_else(|| {
            panic!(
                "{}",
                constants::lan_pairing::TEST_NAMED_NEIGHBOR_ROW_PARSE_EXPECT
            )
        });

        assert_eq!(
            named.hostname,
            Some(constants::lan_pairing::TEST_HOSTNAME.to_string())
        );
        assert_eq!(named.label, constants::lan_pairing::TEST_HOSTNAME);

        let unnamed = network_device_from_windows_neighbor(
            &neighbor_record(None),
            &HashMap::new(),
            &LanIdentityHintInventory::default(),
            &LanPreviousNetworkInventory::default(),
        )
        .unwrap_or_else(|| {
            panic!(
                "{}",
                constants::lan_pairing::TEST_UNNAMED_NEIGHBOR_ROW_PARSE_EXPECT
            )
        });

        assert_eq!(
            unnamed.hostname,
            Some(constants::lan_pairing::TEST_HOSTNAME.to_string())
        );
        assert_eq!(unnamed.label, constants::lan_pairing::TEST_HOSTNAME);
    }

    #[test]
    fn windows_neighbor_parser_accepts_ipv6_rows_without_forcing_ipv4_only_logic() {
        let parsed = network_device_from_windows_neighbor(
            &neighbor_record_with_values(
                "fe80::2b4d",
                "00-11-22-33-44-66",
                constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE,
                Some("Ethernet"),
                None,
            ),
            &HashMap::new(),
            &LanIdentityHintInventory::default(),
            &LanPreviousNetworkInventory::default(),
        )
        .unwrap_or_else(|| panic!("{}", "windows ipv6 neighbor row parses"));

        assert_eq!(parsed.ip_address, "fe80::2b4d");
        assert_eq!(parsed.mac_address, "00-11-22-33-44-66");
        assert_eq!(parsed.platform, constants::lan_pairing::PLATFORM_UNKNOWN);
        assert_eq!(parsed.network_interface.as_deref(), Some("Ethernet"));
        assert_eq!(
            parsed.scan_sources,
            vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()]
        );
    }

    #[test]
    fn previous_scan_hydrates_hostname_platform_and_label_for_same_mac() {
        let test_ip = "192.168.2.88";
        let test_mac = "00-50-56-aa-bb-cc";
        let previous_inventory =
            LanPreviousNetworkInventory::from_devices(&[LanNetworkInventoryDevice {
                device_id: "lan-device-previous".to_string(),
                label: constants::lan_pairing::TEST_HOSTNAME.to_string(),
                platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
                ip_address: test_ip.to_string(),
                mac_address: test_mac.to_string(),
                hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
                network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
                reachability: LanPairingDeviceReachability::Online,
                agent_status: None,
                scan_sources: vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()
                ],
                used_previous_scan_hint: false,
            }]);

        let hydrated = network_device_from_windows_neighbor(
            &serde_json::json!({
                constants::lan_pairing::JSON_KEY_IP_ADDRESS: test_ip,
                constants::lan_pairing::JSON_KEY_LINK_LAYER_ADDRESS: test_mac,
                constants::lan_pairing::JSON_KEY_STATE: constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE,
            }),
            &HashMap::new(),
            &LanIdentityHintInventory::default(),
            &previous_inventory,
        )
        .unwrap_or_else(|| panic!("{}", "previous scan hint applies"));

        assert_eq!(
            hydrated.hostname.as_deref(),
            Some(constants::lan_pairing::TEST_HOSTNAME)
        );
        assert_eq!(hydrated.label, constants::lan_pairing::TEST_HOSTNAME);
        assert_eq!(hydrated.platform, constants::lan_pairing::PLATFORM_WINDOWS);
        assert!(hydrated.used_previous_scan_hint);
    }

    #[test]
    fn trusted_registry_hydrates_identity_before_previous_scan_history() {
        let trusted_inventory = LanIdentityHintInventory::from_devices(&[trusted_device(
            constants::lan_pairing::TEST_LAN_MAC,
            Some(constants::lan_pairing::TEST_LAN_IP),
            Some(constants::lan_pairing::TEST_HOSTNAME),
            "Family Tablet",
            constants::lan_pairing::PLATFORM_WINDOWS,
        )]);
        let previous_inventory =
            LanPreviousNetworkInventory::from_devices(&[LanNetworkInventoryDevice {
                device_id: "lan-device-previous".to_string(),
                label: "history-label".to_string(),
                platform: constants::lan_pairing::PLATFORM_ROUTER.to_string(),
                ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
                mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
                hostname: Some("history-hostname".to_string()),
                network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
                reachability: LanPairingDeviceReachability::Online,
                agent_status: None,
                scan_sources: vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()
                ],
                used_previous_scan_hint: false,
            }]);

        let hydrated = network_device_from_windows_neighbor(
            &neighbor_record(None),
            &HashMap::new(),
            &trusted_inventory,
            &previous_inventory,
        )
        .unwrap_or_else(|| panic!("{}", "trusted registry hint applies"));

        assert_eq!(
            hydrated.hostname.as_deref(),
            Some(constants::lan_pairing::TEST_HOSTNAME)
        );
        assert_eq!(hydrated.label, constants::lan_pairing::TEST_HOSTNAME);
        assert_eq!(hydrated.platform, constants::lan_pairing::PLATFORM_WINDOWS);
        assert!(!hydrated.used_previous_scan_hint);
    }

    #[test]
    fn linux_ip_neigh_parser_accepts_ipv6_neighbors() {
        let parsed = linux_ip_neigh_observation(&serde_json::json!({
            constants::lan_pairing::JSON_KEY_DST: "fe80::abcd",
            constants::lan_pairing::JSON_KEY_LLADDR: "00:11:22:33:44:77",
            constants::lan_pairing::JSON_KEY_LOWER_STATE: constants::lan_pairing::LINUX_NEIGHBOR_STATE_REACHABLE,
            constants::lan_pairing::JSON_KEY_DEV: "eth0",
        }))
        .unwrap_or_else(|| panic!("{}", "linux ipv6 neighbor row parses"));

        assert_eq!(parsed.ip_address, "fe80::abcd");
        assert_eq!(parsed.mac_address, "00-11-22-33-44-77");
        assert_eq!(parsed.network_interface.as_deref(), Some("eth0"));
        assert_eq!(parsed.reachability, LanPairingDeviceReachability::Online);
        assert_eq!(
            parsed.scan_sources,
            vec![constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string()]
        );
    }

    #[test]
    fn linux_ip_neigh_parser_rejects_incomplete_and_malformed_rows() {
        assert!(linux_ip_neigh_observation(&serde_json::json!({
            constants::lan_pairing::JSON_KEY_DST: "192.168.2.50",
            constants::lan_pairing::JSON_KEY_LLADDR: "00:11:22:33:44:55",
            constants::lan_pairing::JSON_KEY_LOWER_STATE: constants::lan_pairing::LINUX_NEIGHBOR_STATE_INCOMPLETE,
            constants::lan_pairing::JSON_KEY_DEV: "eth0",
        }))
        .is_none());

        assert!(linux_ip_neigh_observation(&serde_json::json!({
            constants::lan_pairing::JSON_KEY_DST: "192.168.2.50",
            constants::lan_pairing::JSON_KEY_LLADDR: "00:00:00:00:00:00",
            constants::lan_pairing::JSON_KEY_LOWER_STATE: constants::lan_pairing::LINUX_NEIGHBOR_STATE_REACHABLE,
            constants::lan_pairing::JSON_KEY_DEV: "eth0",
        }))
        .is_none());
    }

    #[test]
    fn duplicate_neighbor_rows_merge_sources_and_prefer_private_ipv4_identity() {
        let merged = merge_neighbor_observations(vec![
            LanNeighborObservation {
                ip_address: "fe80::abcd".to_string(),
                mac_address: "00-11-22-33-44-88".to_string(),
                network_interface: Some("eth0".to_string()),
                hostname: None,
                reachability: LanPairingDeviceReachability::Online,
                scan_sources: vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string()
                ],
            },
            LanNeighborObservation {
                ip_address: "192.168.2.80".to_string(),
                mac_address: "00-11-22-33-44-88".to_string(),
                network_interface: Some("eth0".to_string()),
                hostname: None,
                reachability: LanPairingDeviceReachability::Stale,
                scan_sources: vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string()
                ],
            },
        ]);

        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].ip_address, "192.168.2.80");
        assert_eq!(merged[0].reachability, LanPairingDeviceReachability::Online);
        assert_eq!(
            merged[0].scan_sources,
            vec![
                constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH.to_string(),
                constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP.to_string(),
            ]
        );
    }

    #[test]
    fn netbios_adapter_status_name_prefers_unique_device_service_rows() {
        assert_eq!(
            netbios_adapter_status_name(
                &[
                    constants::lan_pairing::TEST_HOSTNAME,
                    constants::lan_pairing::NBTSTAT_SERVER_SERVICE_MARKER,
                    constants::lan_pairing::NBTSTAT_UNIQUE_MARKER,
                ]
                .join(constants::lan_pairing::TEST_NETBIOS_STATUS_ROW_SEPARATOR)
            ),
            Some(constants::lan_pairing::TEST_HOSTNAME.to_string())
        );
        assert_eq!(
            netbios_adapter_status_name(
                &[
                    constants::lan_pairing::TEST_HOSTNAME,
                    constants::lan_pairing::NBTSTAT_WORKSTATION_SERVICE_MARKER,
                    constants::lan_pairing::NBTSTAT_UNIQUE_MARKER,
                ]
                .join(constants::lan_pairing::TEST_NETBIOS_STATUS_ROW_SEPARATOR)
            ),
            Some(constants::lan_pairing::TEST_HOSTNAME.to_string())
        );
        assert!(netbios_adapter_status_name(
            &[
                constants::lan_pairing::TEST_HOSTNAME,
                constants::lan_pairing::NBTSTAT_WORKSTATION_SERVICE_MARKER,
                constants::lan_pairing::NBTSTAT_GROUP_MARKER,
            ]
            .join(constants::lan_pairing::TEST_NETBIOS_STATUS_ROW_SEPARATOR)
        )
        .is_none());
    }

    #[test]
    fn service_identity_probe_upgrades_unknown_neighbor_to_presence_only_status() {
        let mut device = LanNetworkInventoryDevice {
            device_id: "lan-device-1".to_string(),
            label: format!(
                "{}{}",
                constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX,
                constants::lan_pairing::TEST_LAN_IP
            ),
            platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
            ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
            mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
            hostname: None,
            network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
            reachability: LanPairingDeviceReachability::Online,
            agent_status: None,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            used_previous_scan_hint: false,
        };

        apply_service_identity_probe(
            &mut device,
            LanServiceIdentityProbeMatch {
                hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
                platform: Some(constants::lan_pairing::PLATFORM_WINDOWS.to_string()),
            },
        );

        assert_eq!(
            device.agent_status.as_deref(),
            Some(constants::lan_pairing::SERVICE_IDENTITY_PROBE_AGENT_STATUS)
        );
        assert_eq!(
            device.hostname.as_deref(),
            Some(constants::lan_pairing::TEST_HOSTNAME)
        );
        assert_eq!(device.label, constants::lan_pairing::TEST_HOSTNAME);
        assert_eq!(device.platform, constants::lan_pairing::PLATFORM_WINDOWS);
    }

    #[test]
    fn trusted_device_suppresses_service_identity_probe() {
        let device = LanNetworkInventoryDevice {
            device_id: "lan-device-1".to_string(),
            label: constants::lan_pairing::TEST_HOSTNAME.to_string(),
            platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
            ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
            mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
            hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
            network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
            reachability: LanPairingDeviceReachability::Online,
            agent_status: None,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            used_previous_scan_hint: false,
        };
        let mut trusted_device = LanPairingDeviceRef::new(
            "trusted-child".to_string(),
            None,
            constants::lan_pairing::TEST_HOSTNAME.to_string(),
            constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
        );
        trusted_device.mac_address = Some(constants::lan_pairing::TEST_LAN_MAC.to_string());

        assert!(!should_probe_service_identity(&device, &[trusted_device]));
    }

    #[test]
    fn trusted_device_without_mac_can_still_match_by_ip() {
        let device = LanNetworkInventoryDevice {
            device_id: "lan-device-1".to_string(),
            label: constants::lan_pairing::TEST_HOSTNAME.to_string(),
            platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
            ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
            mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
            hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
            network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
            reachability: LanPairingDeviceReachability::Online,
            agent_status: None,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            used_previous_scan_hint: false,
        };
        let trusted_device = trusted_device(
            "",
            Some(constants::lan_pairing::TEST_LAN_IP),
            Some(constants::lan_pairing::TEST_HOSTNAME),
            constants::lan_pairing::TEST_HOSTNAME,
            constants::lan_pairing::PLATFORM_WINDOWS,
        );

        assert!(!should_probe_service_identity(&device, &[trusted_device]));
    }

    #[test]
    fn router_device_never_uses_service_identity_probe() {
        let device = LanNetworkInventoryDevice {
            device_id: "lan-router-1".to_string(),
            label: "Home Router".to_string(),
            platform: constants::lan_pairing::PLATFORM_ROUTER.to_string(),
            ip_address: constants::lan_pairing::TEST_ROUTER_IP.to_string(),
            mac_address: constants::lan_pairing::TEST_ROUTER_MAC.to_string(),
            hostname: Some("home-router".to_string()),
            network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
            reachability: LanPairingDeviceReachability::Online,
            agent_status: None,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            used_previous_scan_hint: false,
        };

        assert!(!should_probe_service_identity(&device, &[]));
    }

    #[test]
    fn trusted_device_mac_mismatch_does_not_suppress_probe_on_reused_ip() {
        let device = LanNetworkInventoryDevice {
            device_id: "lan-device-1".to_string(),
            label: constants::lan_pairing::TEST_HOSTNAME.to_string(),
            platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
            ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
            mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
            hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
            network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
            reachability: LanPairingDeviceReachability::Online,
            agent_status: None,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            used_previous_scan_hint: false,
        };
        let trusted_device = trusted_device(
            "AA-BB-CC-DD-EE-FF",
            Some(constants::lan_pairing::TEST_LAN_IP),
            Some(constants::lan_pairing::TEST_HOSTNAME),
            constants::lan_pairing::TEST_HOSTNAME,
            constants::lan_pairing::PLATFORM_WINDOWS,
        );

        assert!(should_probe_service_identity(&device, &[trusted_device]));
    }

    #[test]
    fn bounded_active_targets_exclude_network_broadcast_and_local_host() {
        assert_eq!(
            bounded_active_ipv4_targets(
                Some("192.168.2.42"),
                Some("192.168.2.42/30"),
                None,
                &[],
                &[],
            ),
            vec!["192.168.2.41".to_string()]
        );
    }

    #[test]
    fn bounded_active_targets_limit_large_subnet_to_local_24_window() {
        let targets =
            bounded_active_ipv4_targets(Some("10.1.2.42"), Some("10.1.2.42/16"), None, &[], &[]);

        assert_eq!(
            targets.len(),
            (constants::lan_pairing::LAN_ACTIVE_IPV4_SWEEP_MAX_HOSTS - 1) as usize
        );
        assert_eq!(targets.first().map(String::as_str), Some("10.1.2.1"));
        assert_eq!(targets.last().map(String::as_str), Some("10.1.2.254"));
        assert!(!targets.iter().any(|target| target == "10.1.1.1"));
        assert!(!targets.iter().any(|target| target == "10.1.2.42"));
    }

    #[test]
    fn bounded_active_targets_require_ipv4_identity_and_cidr() {
        assert!(
            bounded_active_ipv4_targets(None, Some("192.168.2.42/24"), None, &[], &[]).is_empty()
        );
        assert!(bounded_active_ipv4_targets(Some("192.168.2.42"), None, None, &[], &[]).is_empty());
        assert!(bounded_active_ipv4_targets(
            Some("192.168.2.42"),
            Some("192.168.2.42/31"),
            None,
            &[],
            &[],
        )
        .is_empty());
        assert!(bounded_active_ipv4_targets(
            Some("192.168.2.42"),
            Some("192.168.2.42/not-a-prefix"),
            None,
            &[],
            &[],
        )
        .is_empty());
    }

    #[test]
    fn active_refresh_always_skips_router_truth_but_not_unconfirmed_child_ip_truth() {
        let targets = bounded_active_ipv4_targets(
            Some("192.168.2.42"),
            Some("192.168.2.42/24"),
            Some("192.168.2.1"),
            &[trusted_device(
                constants::lan_pairing::TEST_LAN_MAC,
                Some("192.168.2.20"),
                Some(constants::lan_pairing::TEST_HOSTNAME),
                constants::lan_pairing::TEST_HOSTNAME,
                constants::lan_pairing::PLATFORM_WINDOWS,
            )],
            &[LanNetworkInventoryDevice {
                device_id: "lan-router-1".to_string(),
                label: "Home Router".to_string(),
                platform: constants::lan_pairing::PLATFORM_ROUTER.to_string(),
                ip_address: "192.168.2.1".to_string(),
                mac_address: constants::lan_pairing::TEST_ROUTER_MAC.to_string(),
                hostname: Some("home-router".to_string()),
                network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
                reachability: LanPairingDeviceReachability::Online,
                agent_status: None,
                scan_sources: vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()
                ],
                used_previous_scan_hint: false,
            }],
        );

        assert!(targets.iter().any(|target| target == "192.168.2.20"));
        assert!(!targets.iter().any(|target| target == "192.168.2.1"));
    }

    #[test]
    fn active_refresh_prioritizes_previous_unresolved_devices_before_unknown_space() {
        let targets = bounded_active_ipv4_targets(
            Some("192.168.2.42"),
            Some("192.168.2.42/24"),
            Some("192.168.2.1"),
            &[],
            &[LanNetworkInventoryDevice {
                device_id: "lan-device-previous".to_string(),
                label: format!(
                    "{}{}",
                    constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX,
                    "192.168.2.77"
                ),
                platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
                ip_address: "192.168.2.77".to_string(),
                mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
                hostname: None,
                network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
                reachability: LanPairingDeviceReachability::Offline,
                agent_status: None,
                scan_sources: vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()
                ],
                used_previous_scan_hint: true,
            }],
        );

        assert_eq!(targets.first().map(String::as_str), Some("192.168.2.77"));
    }

    #[test]
    fn probe_response_parser_extracts_agent_identity() {
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{{\"schemaVersion\":1,\"agent\":{{\"deviceId\":\"child-device-1\",\"hostname\":\"{hostname}\",\"platform\":\"windows\",\"serviceVersion\":\"0.1.1\"}},\"entries\":[]}}",
            hostname = constants::lan_pairing::TEST_HOSTNAME
        );

        let parsed = parse_probe_match(response.as_bytes()).unwrap_or_else(|| {
            panic!("{}", "probe response parses");
        });

        assert_eq!(
            parsed.hostname.as_deref(),
            Some(constants::lan_pairing::TEST_HOSTNAME)
        );
        assert_eq!(parsed.platform.as_deref(), Some("windows"));
    }

    #[test]
    fn active_refresh_suppression_skips_currently_confirmed_known_child_ip() {
        let suppressed_targets = suppressed_active_ipv4_targets_for_current_observations(
            Some("192.168.2.1"),
            &[trusted_device(
                constants::lan_pairing::TEST_LAN_MAC,
                Some("192.168.2.20"),
                Some(constants::lan_pairing::TEST_HOSTNAME),
                constants::lan_pairing::TEST_HOSTNAME,
                constants::lan_pairing::PLATFORM_WINDOWS,
            )],
            &[],
            &HashMap::from([(
                "192.168.2.20".to_string(),
                constants::lan_pairing::TEST_LAN_MAC.to_ascii_lowercase(),
            )]),
        );

        assert!(suppressed_targets.contains("192.168.2.20"));
    }

    #[test]
    fn active_refresh_suppression_does_not_skip_reused_ip_with_different_mac() {
        let suppressed_targets = suppressed_active_ipv4_targets_for_current_observations(
            Some("192.168.2.1"),
            &[trusted_device(
                constants::lan_pairing::TEST_LAN_MAC,
                Some("192.168.2.20"),
                Some(constants::lan_pairing::TEST_HOSTNAME),
                constants::lan_pairing::TEST_HOSTNAME,
                constants::lan_pairing::PLATFORM_WINDOWS,
            )],
            &[],
            &HashMap::from([(
                "192.168.2.20".to_string(),
                constants::lan_pairing::TEST_ROUTER_MAC.to_ascii_lowercase(),
            )]),
        );

        assert!(!suppressed_targets.contains("192.168.2.20"));
    }

    #[test]
    fn active_scan_plan_records_selected_interface_and_suppressed_targets() {
        let plan = scan_plan_for_identity(
            Some(&LocalNetworkIdentity {
                ip_address: Some("192.168.2.42".to_string()),
                mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
                network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
                default_gateway: Some("192.168.2.1".to_string()),
                ipv4_cidr: Some("192.168.2.42/24".to_string()),
                dns_servers: vec!["192.168.2.1".to_string(), "1.1.1.1".to_string()],
                dhcp_server: Some("192.168.2.1".to_string()),
                broadcast_address: Some("192.168.2.255".to_string()),
                ipv6_prefixes: vec!["2001:db8::42/64".to_string()],
            }),
            &[trusted_device(
                constants::lan_pairing::TEST_LAN_MAC,
                Some("192.168.2.20"),
                Some(constants::lan_pairing::TEST_HOSTNAME),
                constants::lan_pairing::TEST_HOSTNAME,
                constants::lan_pairing::PLATFORM_WINDOWS,
            )],
            &[LanNetworkInventoryDevice {
                device_id: "lan-router-1".to_string(),
                label: "Home Router".to_string(),
                platform: constants::lan_pairing::PLATFORM_ROUTER.to_string(),
                ip_address: "192.168.2.1".to_string(),
                mac_address: constants::lan_pairing::TEST_ROUTER_MAC.to_string(),
                hostname: Some("home-router".to_string()),
                network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
                reachability: LanPairingDeviceReachability::Online,
                agent_status: None,
                scan_sources: vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()
                ],
                used_previous_scan_hint: false,
            }],
            LanDiscoveryRefreshMode::ActiveSubnetRefresh,
            &[trusted_device(
                constants::lan_pairing::TEST_LAN_MAC,
                Some("192.168.2.20"),
                Some(constants::lan_pairing::TEST_HOSTNAME),
                constants::lan_pairing::TEST_HOSTNAME,
                constants::lan_pairing::PLATFORM_WINDOWS,
            )],
        );

        assert_eq!(
            plan.selected_interface.as_deref(),
            Some(constants::lan_pairing::TEST_NETWORK_INTERFACE)
        );
        assert_eq!(plan.default_gateway.as_deref(), Some("192.168.2.1"));
        assert_eq!(
            plan.dns_servers,
            vec!["192.168.2.1".to_string(), "1.1.1.1".to_string()]
        );
        assert_eq!(plan.dhcp_server.as_deref(), Some("192.168.2.1"));
        assert_eq!(plan.broadcast_address.as_deref(), Some("192.168.2.255"));
        assert_eq!(plan.ipv6_prefixes, vec!["2001:db8::42/64".to_string()]);
        assert_eq!(plan.trusted_truth_device_count, 1);
        assert_eq!(plan.previous_device_count, 1);
        assert_eq!(
            plan.active_ipv4_target_timeout_ms,
            active_ipv4_target_timeout_ms()
        );
        assert!(plan
            .suppressed_active_ipv4_targets
            .iter()
            .any(|target| target == "192.168.2.1"));
        assert_eq!(plan.active_ipv4_candidate_count, 253);
        assert_eq!(plan.active_ipv4_target_count, 252);
    }

    #[test]
    fn passive_scan_plan_keeps_identity_metadata_without_active_targets() {
        let plan = scan_plan_for_identity(
            Some(&LocalNetworkIdentity {
                ip_address: Some("192.168.2.42".to_string()),
                mac_address: Some(constants::lan_pairing::TEST_LAN_MAC.to_string()),
                network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
                default_gateway: Some("192.168.2.1".to_string()),
                ipv4_cidr: Some("192.168.2.42/24".to_string()),
                dns_servers: vec!["192.168.2.1".to_string()],
                dhcp_server: Some("192.168.2.1".to_string()),
                broadcast_address: Some("192.168.2.255".to_string()),
                ipv6_prefixes: vec!["2001:db8::42/64".to_string()],
            }),
            &[],
            &[],
            LanDiscoveryRefreshMode::Passive,
            &[],
        );

        assert_eq!(plan.refresh_mode, LanDiscoveryRefreshMode::Passive);
        assert_eq!(plan.active_ipv4_candidate_count, 0);
        assert_eq!(plan.active_ipv4_target_count, 0);
        assert_eq!(plan.prioritized_previous_target_count, 0);
        assert_eq!(plan.active_ipv4_target_timeout_ms, None);
        assert_eq!(plan.dns_servers, vec!["192.168.2.1".to_string()]);
        assert_eq!(plan.dhcp_server.as_deref(), Some("192.168.2.1"));
        assert_eq!(plan.broadcast_address.as_deref(), Some("192.168.2.255"));
        assert_eq!(plan.ipv6_prefixes, vec!["2001:db8::42/64".to_string()]);
        assert!(plan.suppressed_active_ipv4_targets.is_empty());
    }

    fn neighbor_record(hostname: Option<&str>) -> Value {
        neighbor_record_with_values(
            constants::lan_pairing::TEST_LAN_IP,
            constants::lan_pairing::TEST_LAN_MAC,
            constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE,
            None,
            hostname,
        )
    }

    fn neighbor_record_with_values(
        ip_address: &str,
        mac_address: &str,
        state: &str,
        interface_alias: Option<&str>,
        hostname: Option<&str>,
    ) -> Value {
        let mut record = Map::new();
        record.insert(
            constants::lan_pairing::JSON_KEY_IP_ADDRESS.to_string(),
            Value::String(ip_address.to_string()),
        );
        record.insert(
            constants::lan_pairing::JSON_KEY_LINK_LAYER_ADDRESS.to_string(),
            Value::String(mac_address.to_string()),
        );
        record.insert(
            constants::lan_pairing::JSON_KEY_STATE.to_string(),
            Value::String(state.to_string()),
        );
        if let Some(interface_alias) = interface_alias {
            record.insert(
                constants::lan_pairing::JSON_KEY_INTERFACE_ALIAS.to_string(),
                Value::String(interface_alias.to_string()),
            );
        }
        if let Some(hostname) = hostname {
            record.insert(
                constants::lan_pairing::JSON_KEY_HOSTNAME.to_string(),
                Value::String(hostname.to_string()),
            );
        }
        Value::Object(record)
    }

    fn trusted_device(
        mac_address: &str,
        ip_address: Option<&str>,
        hostname: Option<&str>,
        label: &str,
        platform: &str,
    ) -> LanPairingDeviceRef {
        let mut device = LanPairingDeviceRef::new(
            "trusted-child".to_string(),
            None,
            label.to_string(),
            platform.to_string(),
        );
        if !mac_address.is_empty() {
            device.mac_address = Some(mac_address.to_string());
        }
        device.ip_address = ip_address.map(str::to_string);
        device.hostname = hostname.map(str::to_string);
        device
    }
}
