mod gpu;
pub mod linux_identity;
pub mod network_identity_support;

use self::gpu::{
    cpu_core_summary, gpu_drivers, gpu_memory, gpu_names, memory_summary, nvidia_smi_gpus,
    nvidia_summary,
};
use self::linux_identity::{
    linux_dns_servers_from_resolv_conf, preferred_linux_local_network_identity,
    preferred_windows_local_network_identity,
};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceHardwareProfile;

use crate::network_inventory_command::{
    command_json_records, command_json_single, record_text, record_u64,
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
pub struct LocalNetworkIdentity {
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub network_interface: Option<String>,
    pub wifi_ssid: Option<String>,
    pub default_gateway: Option<String>,
    pub ipv4_cidr: Option<String>,
    pub dns_servers: Vec<String>,
    pub dhcp_server: Option<String>,
    pub broadcast_address: Option<String>,
    pub ipv6_prefixes: Vec<String>,
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
    if cfg!(any(target_os = "linux", target_os = "android")) {
        let dns_servers = linux_dns_servers_from_resolv_conf();
        return preferred_linux_local_network_identity(
            &command_json_records(constants::lan_pairing::IP_EXE, &linux_route_args()),
            &command_json_records(constants::lan_pairing::IP_EXE, &linux_address_args()),
            &dns_servers,
        );
    }
    None
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
