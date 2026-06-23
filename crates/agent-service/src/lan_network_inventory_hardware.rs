use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceHardwareProfile;

use crate::lan_network_inventory_command::{
    command_json_records, command_json_single, command_stdout, normalize_mac_address, record_text,
    record_u64,
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
    command_json_single(
        constants::lan_pairing::POWERSHELL_EXE,
        &local_network_identity_args(),
    )
    .map(|record| LocalNetworkIdentity {
        ip_address: record_text(&record, constants::lan_pairing::JSON_KEY_IP_ADDRESS),
        mac_address: record_text(&record, constants::lan_pairing::JSON_KEY_MAC_ADDRESS)
            .and_then(|value| normalize_mac_address(&value)),
        network_interface: record_text(&record, constants::lan_pairing::JSON_KEY_INTERFACE_ALIAS),
    })
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

fn powershell_command_args(command: &'static str) -> [&'static str; 5] {
    [
        constants::lan_pairing::POWERSHELL_NO_PROFILE_ARG,
        constants::lan_pairing::POWERSHELL_EXECUTION_POLICY_ARG,
        constants::lan_pairing::POWERSHELL_BYPASS_ARG,
        constants::lan_pairing::POWERSHELL_COMMAND_ARG,
        command,
    ]
}
