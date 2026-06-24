use std::{
    fs::read_to_string,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceHardwareProfile;

use crate::network_inventory_command::{
    command_json_records, command_json_single, command_stdout, normalize_mac_address, record_text,
    record_u64, value_text,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct LocalHardwareProfile {
    pub(crate) hostname: Option<String>,
    manufacturer: Option<String>,
    model: Option<String>,
    cpu_model: Option<String>,
    cpu_cores: Option<String>,
    memory_total: Option<String>,
    gpu_model: Option<String>,
    gpu_driver: Option<String>,
    gpu_memory: Option<String>,
    nvidia_smi: Option<String>,
}

impl LocalHardwareProfile {
    pub(crate) fn into_protocol_profile(self) -> LanPairingDeviceHardwareProfile {
        LanPairingDeviceHardwareProfile {
            manufacturer: self.manufacturer,
            model: self.model,
            cpu_model: self.cpu_model,
            cpu_cores: self.cpu_cores,
            memory_total: self.memory_total,
            gpu_model: self.gpu_model,
            gpu_driver: self.gpu_driver,
            gpu_memory: self.gpu_memory,
            nvidia_smi: self.nvidia_smi,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct LocalNetworkIdentity {
    pub(crate) ip_address: Option<String>,
    pub(crate) mac_address: Option<String>,
    pub(crate) network_interface: Option<String>,
    pub(crate) default_gateway: Option<String>,
    pub(crate) ipv4_cidr: Option<String>,
    pub(crate) dns_servers: Vec<String>,
    pub(crate) dhcp_server: Option<String>,
    pub(crate) broadcast_address: Option<String>,
    pub(crate) ipv6_prefixes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct NvidiaGpu {
    adapter_name: String,
    driver: String,
    memory: String,
}

pub(crate) fn local_hardware_profile() -> LocalHardwareProfile {
    let computer = command_json_single(
        constants::lan_pairing::POWERSHELL_EXE,
        &computer_system_args(),
    );
    let cpu = command_json_single(constants::lan_pairing::POWERSHELL_EXE, &cpu_args());
    let nvidia_gpus = nvidia_smi_gpus();
    let windows_gpus = command_json_records(constants::lan_pairing::POWERSHELL_EXE, &gpu_args());

    LocalHardwareProfile {
        hostname: computer
            .as_ref()
            .and_then(|record| record_text(record, constants::lan_pairing::JSON_KEY_NAME)),
        manufacturer: computer
            .as_ref()
            .and_then(|record| record_text(record, constants::lan_pairing::JSON_KEY_MANUFACTURER)),
        model: computer
            .as_ref()
            .and_then(|record| record_text(record, constants::lan_pairing::JSON_KEY_MODEL)),
        cpu_model: cpu
            .as_ref()
            .and_then(|record| record_text(record, constants::lan_pairing::JSON_KEY_NAME)),
        cpu_cores: cpu.as_ref().and_then(cpu_core_summary),
        memory_total: computer
            .as_ref()
            .and_then(|record| {
                record_u64(
                    record,
                    constants::lan_pairing::JSON_KEY_TOTAL_PHYSICAL_MEMORY,
                )
            })
            .map(memory_summary),
        gpu_model: gpu_names(&nvidia_gpus, &windows_gpus),
        gpu_driver: gpu_drivers(&nvidia_gpus, &windows_gpus),
        gpu_memory: gpu_memory(&nvidia_gpus, &windows_gpus),
        nvidia_smi: nvidia_summary(&nvidia_gpus),
    }
}

pub(crate) fn local_network_identity() -> Option<LocalNetworkIdentity> {
    if cfg!(target_os = "windows") {
        return preferred_windows_local_network_identity(&command_json_records(
            constants::lan_pairing::POWERSHELL_EXE,
            &local_network_identity_args(),
        ));
    }
    if cfg!(target_os = "linux") {
        let dns_servers = linux_dns_servers_from_resolv_conf();
        return preferred_linux_local_network_identity(
            &command_json_records(constants::lan_pairing::IP_EXE, &linux_route_args()),
            &command_json_records(constants::lan_pairing::IP_EXE, &linux_address_args()),
            &dns_servers,
        );
    }
    None
}

fn preferred_windows_local_network_identity(
    records: &[serde_json::Value],
) -> Option<LocalNetworkIdentity> {
    records
        .iter()
        .filter_map(windows_local_network_identity_candidate)
        .min_by_key(|identity| default_gateway_preference(identity.default_gateway.as_deref()))
}

fn windows_local_network_identity_candidate(
    record: &serde_json::Value,
) -> Option<LocalNetworkIdentity> {
    let interface_name = record_text(record, constants::lan_pairing::JSON_KEY_INTERFACE_ALIAS)?;
    if ignored_interface_name(&interface_name) {
        return None;
    }
    let ip_address = record_text(record, constants::lan_pairing::JSON_KEY_IP_ADDRESS)?;
    if !supported_local_ipv4_text(&ip_address) {
        return None;
    }
    let prefix_length =
        record_u64(record, constants::lan_pairing::JSON_KEY_PREFIX_LENGTH).map(|value| value as u8);
    let dns_servers = sanitized_dns_servers(record_text_values(
        record,
        constants::lan_pairing::JSON_KEY_DNS_SERVERS,
    ));
    Some(LocalNetworkIdentity {
        ip_address: Some(ip_address.clone()),
        mac_address: record_text(record, constants::lan_pairing::JSON_KEY_MAC_ADDRESS)
            .and_then(|value| normalize_mac_address(&value)),
        network_interface: Some(interface_name),
        default_gateway: record_text(record, constants::lan_pairing::JSON_KEY_DEFAULT_GATEWAY)
            .filter(|value| supported_local_ipv4_text(value)),
        ipv4_cidr: cidr_summary(&ip_address, prefix_length),
        dns_servers,
        dhcp_server: record_text(record, constants::lan_pairing::JSON_KEY_DHCP_SERVER)
            .filter(|value| supported_dns_server_text(value)),
        broadcast_address: broadcast_address_for(&ip_address, prefix_length),
        ipv6_prefixes: normalized_ipv6_prefixes(record_text_values(
            record,
            constants::lan_pairing::JSON_KEY_IPV6_PREFIXES,
        )),
    })
}

fn preferred_linux_local_network_identity(
    route_records: &[serde_json::Value],
    address_records: &[serde_json::Value],
    dns_servers: &[String],
) -> Option<LocalNetworkIdentity> {
    let default_route = linux_default_route(route_records);
    if let Some(default_route) = default_route.as_ref() {
        if let Some(identity) = address_records.iter().find_map(|record| {
            linux_local_network_identity_candidate(record, Some(default_route), dns_servers)
        }) {
            return Some(identity);
        }
    }
    address_records
        .iter()
        .find_map(|record| linux_local_network_identity_candidate(record, None, dns_servers))
}

fn linux_local_network_identity_candidate(
    record: &serde_json::Value,
    default_route: Option<&LinuxDefaultRoute>,
    dns_servers: &[String],
) -> Option<LocalNetworkIdentity> {
    let interface_name = record_text(record, constants::lan_pairing::JSON_KEY_IFNAME)?;
    if ignored_interface_name(&interface_name) {
        return None;
    }
    let (ip_address, prefix_length) = linux_ipv4_address(record)?;
    let route_matches_interface = default_route
        .map(|route| route.device == interface_name)
        .unwrap_or(true);
    if !route_matches_interface {
        return None;
    }
    Some(LocalNetworkIdentity {
        ip_address: Some(ip_address.clone()),
        mac_address: record_text(record, constants::lan_pairing::JSON_KEY_ADDRESS)
            .and_then(|value| normalize_mac_address(&value)),
        network_interface: Some(interface_name),
        default_gateway: default_route.and_then(|route| route.gateway.clone()),
        ipv4_cidr: cidr_summary(&ip_address, Some(prefix_length)),
        dns_servers: dns_servers.to_vec(),
        dhcp_server: None,
        broadcast_address: broadcast_address_for(&ip_address, Some(prefix_length)),
        ipv6_prefixes: linux_ipv6_prefixes(record),
    })
}

fn linux_ipv4_address(record: &serde_json::Value) -> Option<(String, u8)> {
    let addr_info = record
        .get(constants::lan_pairing::JSON_KEY_ADDR_INFO)?
        .as_array()?;
    addr_info.iter().find_map(|addr| {
        let family = record_text(addr, constants::lan_pairing::JSON_KEY_FAMILY)?;
        if family != "inet" {
            return None;
        }
        let scope = record_text(addr, constants::lan_pairing::JSON_KEY_SCOPE);
        let local = record_text(addr, constants::lan_pairing::JSON_KEY_LOCAL)?;
        let prefix_length = record_u64(addr, constants::lan_pairing::JSON_KEY_PREFIXLEN)
            .map(|value| value as u8)?;
        if !supported_local_ipv4_text(&local) || scope.as_deref() == Some("host") {
            return None;
        }
        Some((local, prefix_length))
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LinuxDefaultRoute {
    device: String,
    gateway: Option<String>,
}

fn linux_default_route(route_records: &[serde_json::Value]) -> Option<LinuxDefaultRoute> {
    route_records.iter().find_map(|record| {
        let device = record_text(record, constants::lan_pairing::JSON_KEY_DEV)?;
        let gateway = record_text(record, constants::lan_pairing::JSON_KEY_GATEWAY)
            .filter(|value| supported_local_ipv4_text(value));
        Some(LinuxDefaultRoute { device, gateway })
    })
}

fn linux_dns_servers_from_resolv_conf() -> Vec<String> {
    read_to_string(constants::lan_pairing::LINUX_RESOLV_CONF_PATH)
        .map(|text| linux_dns_servers_from_resolv_conf_text(&text))
        .unwrap_or_default()
}

fn linux_dns_servers_from_resolv_conf_text(text: &str) -> Vec<String> {
    let mut dns_servers = Vec::new();
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        let mut parts = line.split_whitespace();
        if parts.next() != Some("nameserver") {
            continue;
        }
        if let Some(server) = parts.next() {
            push_unique_string_if(&mut dns_servers, server, supported_dns_server_text(server));
        }
    }
    dns_servers
}

fn linux_ipv6_prefixes(record: &serde_json::Value) -> Vec<String> {
    let mut ipv6_prefixes = Vec::new();
    let Some(addr_info) = record
        .get(constants::lan_pairing::JSON_KEY_ADDR_INFO)
        .and_then(serde_json::Value::as_array)
    else {
        return ipv6_prefixes;
    };
    for addr in addr_info {
        let family = record_text(addr, constants::lan_pairing::JSON_KEY_FAMILY);
        if family.as_deref() != Some("inet6") {
            continue;
        }
        let scope = record_text(addr, constants::lan_pairing::JSON_KEY_SCOPE);
        let local = record_text(addr, constants::lan_pairing::JSON_KEY_LOCAL);
        let prefix_length = record_u64(addr, constants::lan_pairing::JSON_KEY_PREFIXLEN);
        if scope.as_deref() == Some("host") {
            continue;
        }
        if let (Some(local), Some(prefix_length)) = (local, prefix_length) {
            let prefix = format!("{local}/{prefix_length}");
            if let Some(prefix) = normalized_ipv6_prefix(&prefix) {
                push_unique_string(&mut ipv6_prefixes, prefix);
            }
        }
    }
    ipv6_prefixes
}

fn cidr_summary(ip_address: &str, prefix_length: Option<u8>) -> Option<String> {
    prefix_length.map(|prefix_length| format!("{ip_address}/{prefix_length}"))
}

fn broadcast_address_for(ip_address: &str, prefix_length: Option<u8>) -> Option<String> {
    let prefix_length = prefix_length?;
    if prefix_length > 32 {
        return None;
    }
    let ip_address = ip_address.parse::<Ipv4Addr>().ok()?;
    let mask = if prefix_length == 0 {
        0
    } else {
        u32::MAX << (32_u32.saturating_sub(u32::from(prefix_length)))
    };
    let broadcast = u32::from(ip_address) | !mask;
    Some(Ipv4Addr::from(broadcast).to_string())
}

fn supported_local_ipv4_text(value: &str) -> bool {
    value
        .parse::<Ipv4Addr>()
        .map(supported_local_ipv4)
        .unwrap_or(false)
}

fn supported_local_ipv6_text(value: &str) -> bool {
    value
        .parse::<Ipv6Addr>()
        .map(supported_local_ipv6)
        .unwrap_or(false)
}

fn supported_local_ipv4(ip_address: Ipv4Addr) -> bool {
    !ip_address.is_loopback()
        && !ip_address.is_multicast()
        && !ip_address.is_unspecified()
        && !ip_address.is_link_local()
        && ip_address != Ipv4Addr::BROADCAST
}

fn supported_local_ipv6(ip_address: Ipv6Addr) -> bool {
    !ip_address.is_loopback()
        && !ip_address.is_multicast()
        && !ip_address.is_unspecified()
        && !ip_address.is_unicast_link_local()
}

fn supported_dns_server_text(value: &str) -> bool {
    value
        .parse::<IpAddr>()
        .map(|ip_address| !ip_address.is_loopback() && !ip_address.is_unspecified())
        .unwrap_or(false)
}

fn sanitized_dns_servers(values: Vec<String>) -> Vec<String> {
    let mut dns_servers = Vec::new();
    for value in values {
        push_unique_string_if(&mut dns_servers, &value, supported_dns_server_text(&value));
    }
    dns_servers
}

fn record_text_values(record: &serde_json::Value, field_name: &str) -> Vec<String> {
    record
        .get(field_name)
        .map(value_text_values)
        .unwrap_or_default()
}

fn value_text_values(value: &serde_json::Value) -> Vec<String> {
    match value {
        serde_json::Value::Array(values) => {
            let mut texts = Vec::new();
            for value in values {
                if let Some(text) = value_text(value) {
                    push_unique_string(&mut texts, text);
                }
            }
            texts
        }
        _ => value_text(value)
            .map(|value| vec![value])
            .unwrap_or_default(),
    }
}

fn normalized_ipv6_prefixes(values: Vec<String>) -> Vec<String> {
    let mut prefixes = Vec::new();
    for value in values {
        if let Some(prefix) = normalized_ipv6_prefix(&value) {
            push_unique_string(&mut prefixes, prefix);
        }
    }
    prefixes
}

fn normalized_ipv6_prefix(value: &str) -> Option<String> {
    let (address, prefix_length) = value.trim().split_once('/')?;
    let prefix_length = prefix_length.parse::<u8>().ok()?;
    if prefix_length > 128 || !supported_local_ipv6_text(address) {
        return None;
    }
    Some(format!("{address}/{prefix_length}"))
}

fn push_unique_string(values: &mut Vec<String>, value: String) {
    if !values
        .iter()
        .any(|existing| existing.eq_ignore_ascii_case(&value))
    {
        values.push(value);
    }
}

fn push_unique_string_if(values: &mut Vec<String>, value: &str, include: bool) {
    if include {
        push_unique_string(values, value.to_string());
    }
}

fn ignored_interface_name(interface_name: &str) -> bool {
    let normalized = interface_name.trim().to_ascii_lowercase();
    normalized.is_empty()
        || normalized == "lo"
        || normalized.contains("loopback")
        || normalized.starts_with("vethernet")
        || normalized.starts_with("docker")
        || normalized.starts_with("veth")
        || normalized.starts_with("br-")
        || normalized.starts_with("virbr")
        || normalized.starts_with("vboxnet")
        || normalized.starts_with("tailscale")
        || normalized.starts_with("wg")
        || normalized.starts_with("tun")
        || normalized.starts_with("tap")
        || normalized.starts_with("zt")
        || normalized.contains("wsl")
}

fn default_gateway_preference(default_gateway: Option<&str>) -> u8 {
    if default_gateway.is_some() {
        0
    } else {
        1
    }
}

fn nvidia_smi_gpus() -> Vec<NvidiaGpu> {
    command_stdout(
        constants::lan_pairing::NVIDIA_SMI_EXE,
        &[
            constants::lan_pairing::NVIDIA_SMI_QUERY_ARG,
            constants::lan_pairing::NVIDIA_SMI_FORMAT_ARG,
        ],
    )
    .map(|output| output.lines().filter_map(nvidia_gpu_from_line).collect())
    .unwrap_or_default()
}

fn nvidia_gpu_from_line(line: &str) -> Option<NvidiaGpu> {
    let parts: Vec<&str> = line.split(',').map(str::trim).collect();
    match parts.as_slice() {
        [name, driver, memory, ..] if !name.is_empty() => Some(NvidiaGpu {
            adapter_name: (*name).to_string(),
            driver: (*driver).to_string(),
            memory: memory_mib_summary(memory),
        }),
        _ => None,
    }
}

fn cpu_core_summary(record: &serde_json::Value) -> Option<String> {
    match (
        record_u64(record, constants::lan_pairing::JSON_KEY_NUMBER_OF_CORES),
        record_u64(
            record,
            constants::lan_pairing::JSON_KEY_NUMBER_OF_LOGICAL_PROCESSORS,
        ),
    ) {
        (Some(cores), Some(logical)) => {
            let mut summary = cores.to_string();
            summary.push_str(constants::lan_pairing::CPU_CORES_LABEL);
            summary.push_str(constants::lan_pairing::CPU_LOGICAL_SEPARATOR);
            summary.push_str(&logical.to_string());
            summary.push_str(constants::lan_pairing::CPU_LOGICAL_LABEL);
            Some(summary)
        }
        (Some(cores), None) => {
            let mut summary = cores.to_string();
            summary.push_str(constants::lan_pairing::CPU_CORES_LABEL);
            Some(summary)
        }
        _ => None,
    }
}

fn memory_summary(bytes: u64) -> String {
    let mut summary = (bytes / 1024 / 1024 / 1024).to_string();
    summary.push_str(constants::lan_pairing::MEMORY_GIB_LABEL);
    summary
}

fn memory_mib_summary(memory: &str) -> String {
    let mut summary = memory.to_string();
    summary.push_str(constants::lan_pairing::MEMORY_MIB_LABEL);
    summary
}

fn gpu_names(nvidia_gpus: &[NvidiaGpu], windows_gpus: &[serde_json::Value]) -> Option<String> {
    joined_values(if nvidia_gpus.is_empty() {
        windows_gpus
            .iter()
            .filter_map(|gpu| record_text(gpu, constants::lan_pairing::JSON_KEY_NAME))
            .collect()
    } else {
        nvidia_gpus
            .iter()
            .map(|gpu| gpu.adapter_name.clone())
            .collect()
    })
}

fn gpu_drivers(nvidia_gpus: &[NvidiaGpu], windows_gpus: &[serde_json::Value]) -> Option<String> {
    joined_values(if nvidia_gpus.is_empty() {
        windows_gpus
            .iter()
            .filter_map(|gpu| record_text(gpu, constants::lan_pairing::JSON_KEY_DRIVER_VERSION))
            .collect()
    } else {
        nvidia_gpus.iter().map(|gpu| gpu.driver.clone()).collect()
    })
}

fn gpu_memory(nvidia_gpus: &[NvidiaGpu], windows_gpus: &[serde_json::Value]) -> Option<String> {
    joined_values(if nvidia_gpus.is_empty() {
        windows_gpus
            .iter()
            .filter_map(|gpu| {
                record_u64(gpu, constants::lan_pairing::JSON_KEY_ADAPTER_RAM).map(memory_summary)
            })
            .collect()
    } else {
        nvidia_gpus.iter().map(|gpu| gpu.memory.clone()).collect()
    })
}

fn nvidia_summary(nvidia_gpus: &[NvidiaGpu]) -> Option<String> {
    joined_values(
        nvidia_gpus
            .iter()
            .map(|gpu| {
                let mut summary = gpu.adapter_name.clone();
                summary.push_str(constants::lan_pairing::NVIDIA_DRIVER_SEPARATOR);
                summary.push_str(&gpu.driver);
                summary.push(' ');
                summary.push_str(&gpu.memory);
                summary.push_str(constants::lan_pairing::NVIDIA_VRAM_LABEL);
                summary
            })
            .collect(),
    )
}

fn joined_values(values: Vec<String>) -> Option<String> {
    let filtered: Vec<String> = values
        .into_iter()
        .filter(|value| !value.is_empty())
        .collect();
    if filtered.is_empty() {
        None
    } else {
        Some(filtered.join(constants::lan_pairing::HARDWARE_VALUE_SEPARATOR))
    }
}

fn computer_system_args() -> [&'static str; 5] {
    powershell_command_args(constants::lan_pairing::POWERSHELL_COMPUTER_SYSTEM_COMMAND)
}

fn cpu_args() -> [&'static str; 5] {
    powershell_command_args(constants::lan_pairing::POWERSHELL_CPU_COMMAND)
}

fn gpu_args() -> [&'static str; 5] {
    powershell_command_args(constants::lan_pairing::POWERSHELL_GPU_COMMAND)
}

fn local_network_identity_args() -> [&'static str; 5] {
    powershell_command_args(constants::lan_pairing::POWERSHELL_LOCAL_NETWORK_IDENTITY_COMMAND)
}

fn linux_route_args() -> [&'static str; 4] {
    [
        constants::lan_pairing::IP_JSON_ARG,
        constants::lan_pairing::IP_ROUTE_ARG,
        constants::lan_pairing::IP_SHOW_ARG,
        constants::lan_pairing::IP_DEFAULT_ARG,
    ]
}

fn linux_address_args() -> [&'static str; 4] {
    [
        constants::lan_pairing::IP_JSON_ARG,
        constants::lan_pairing::IP_ADDR_ARG,
        constants::lan_pairing::IP_SHOW_ARG,
        constants::lan_pairing::IP_UP_ARG,
    ]
}

fn powershell_command_args(command: &'static str) -> [&'static str; 5] {
    [
        constants::lan_pairing::POWERSHELL_NO_PROFILE_ARG,
        constants::lan_pairing::POWERSHELL_EXECUTION_POLICY_ARG,
        constants::lan_pairing::POWERSHELL_BYPASS_ARG,
        constants::lan_pairing::POWERSHELL_COMMAND_ARG,
        command,
    ]
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{
        linux_dns_servers_from_resolv_conf_text, preferred_linux_local_network_identity,
        preferred_windows_local_network_identity, LocalNetworkIdentity,
    };

    #[test]
    fn windows_prefers_default_gateway_interface_and_skips_virtual_candidates() {
        let identity = preferred_windows_local_network_identity(&[
            json!({
                "IPAddress": "172.26.32.1",
                "PrefixLength": 20,
                "InterfaceAlias": "vEthernet (WSL)",
                "MacAddress": "00-15-5d-11-22-33",
                "DefaultGateway": "172.26.32.1"
            }),
            json!({
                "IPAddress": "192.168.2.42",
                "PrefixLength": 24,
                "InterfaceAlias": "Ethernet 2",
                "MacAddress": "54-27-1e-97-c3-31",
                "DefaultGateway": "192.168.2.1",
                "DnsServers": ["192.168.2.1", "1.1.1.1"],
                "DhcpServer": "192.168.2.1",
                "Ipv6Prefixes": ["2001:db8::42/64", "fe80::42/64"]
            }),
            json!({
                "IPAddress": "192.168.2.77",
                "PrefixLength": 24,
                "InterfaceAlias": "Wi-Fi",
                "MacAddress": "aa-bb-cc-dd-ee-ff"
            }),
        ])
        .unwrap_or_else(|| unreachable!("preferred Windows identity exists"));

        assert_eq!(identity.network_interface.as_deref(), Some("Ethernet 2"));
        assert_eq!(identity.default_gateway.as_deref(), Some("192.168.2.1"));
        assert_eq!(identity.ipv4_cidr.as_deref(), Some("192.168.2.42/24"));
        assert_eq!(identity.broadcast_address.as_deref(), Some("192.168.2.255"));
        assert_eq!(
            identity.dns_servers,
            vec!["192.168.2.1".to_string(), "1.1.1.1".to_string()]
        );
        assert_eq!(identity.dhcp_server.as_deref(), Some("192.168.2.1"));
        assert_eq!(identity.ipv6_prefixes, vec!["2001:db8::42/64".to_string()]);
    }

    #[test]
    fn linux_prefers_default_route_interface_and_captures_gateway_and_cidr() {
        let identity = preferred_linux_local_network_identity(
            &[json!({
                "dst": "default",
                "gateway": "192.168.2.1",
                "dev": "wlp0s20f3"
            })],
            &[
                json!({
                    "ifname": "docker0",
                    "address": "02:42:0a:9d:00:01",
                    "addr_info": [{
                        "family": "inet",
                        "local": "10.157.0.1",
                        "prefixlen": 16,
                        "scope": "global"
                    }]
                }),
                json!({
                    "ifname": "eth0",
                    "address": "10:20:30:40:50:60",
                    "addr_info": [{
                        "family": "inet",
                        "local": "192.168.2.24",
                        "prefixlen": 24,
                        "scope": "global"
                    }]
                }),
                json!({
                    "ifname": "wlp0s20f3",
                    "address": "54:27:1e:97:c3:31",
                    "addr_info": [
                        {
                            "family": "inet",
                            "local": "192.168.2.42",
                            "prefixlen": 24,
                            "scope": "global"
                        },
                        {
                            "family": "inet6",
                            "local": "fe80::1234",
                            "prefixlen": 64,
                            "scope": "link"
                        },
                        {
                            "family": "inet6",
                            "local": "2001:db8::42",
                            "prefixlen": 64,
                            "scope": "global"
                        }
                    ]
                }),
            ],
            &[
                "192.168.2.1".to_string(),
                "2001:4860:4860::8888".to_string(),
            ],
        )
        .unwrap_or_else(|| unreachable!("preferred Linux identity exists"));

        assert_eq!(
            identity,
            LocalNetworkIdentity {
                ip_address: Some("192.168.2.42".to_string()),
                mac_address: Some("54-27-1e-97-c3-31".to_string()),
                network_interface: Some("wlp0s20f3".to_string()),
                default_gateway: Some("192.168.2.1".to_string()),
                ipv4_cidr: Some("192.168.2.42/24".to_string()),
                dns_servers: vec![
                    "192.168.2.1".to_string(),
                    "2001:4860:4860::8888".to_string(),
                ],
                dhcp_server: None,
                broadcast_address: Some("192.168.2.255".to_string()),
                ipv6_prefixes: vec!["2001:db8::42/64".to_string()],
            }
        );
    }

    #[test]
    fn linux_skips_link_local_only_and_falls_back_to_first_viable_interface() {
        let identity = preferred_linux_local_network_identity(
            &[],
            &[
                json!({
                    "ifname": "enp0s31f6",
                    "address": "10:20:30:40:50:60",
                    "addr_info": [{
                        "family": "inet",
                        "local": "169.254.10.20",
                        "prefixlen": 16,
                        "scope": "link"
                    }]
                }),
                json!({
                    "ifname": "wlp0s20f3",
                    "address": "54:27:1e:97:c3:31",
                    "addr_info": [{
                        "family": "inet",
                        "local": "192.168.2.88",
                        "prefixlen": 24,
                        "scope": "global"
                    }]
                }),
            ],
            &[],
        )
        .unwrap_or_else(|| unreachable!("fallback Linux identity exists"));

        assert_eq!(identity.network_interface.as_deref(), Some("wlp0s20f3"));
        assert_eq!(identity.default_gateway, None);
        assert_eq!(identity.ipv4_cidr.as_deref(), Some("192.168.2.88/24"));
        assert_eq!(identity.broadcast_address.as_deref(), Some("192.168.2.255"));
        assert!(identity.dns_servers.is_empty());
        assert!(identity.ipv6_prefixes.is_empty());
    }

    #[test]
    fn linux_resolv_conf_parser_filters_loopback_nameservers_and_keeps_real_servers() {
        let dns_servers = linux_dns_servers_from_resolv_conf_text(
            "\
# generated by systemd-resolved
nameserver 127.0.0.53
nameserver 192.168.2.1
search lan
nameserver 2001:4860:4860::8888
nameserver invalid
",
        );

        assert_eq!(
            dns_servers,
            vec![
                "192.168.2.1".to_string(),
                "2001:4860:4860::8888".to_string(),
            ]
        );
    }
}
