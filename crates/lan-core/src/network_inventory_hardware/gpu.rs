use ocentra_parent_agent_protocol::constants;

use crate::network_inventory_command::{command_stdout, record_text, record_u64};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct NvidiaGpu {
    adapter_name: String,
    driver: String,
    memory: String,
}

pub(super) fn nvidia_smi_gpus() -> Vec<NvidiaGpu> {
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

pub(super) fn cpu_core_summary(record: &serde_json::Value) -> Option<String> {
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

pub(super) fn memory_summary(bytes: u64) -> String {
    let mut summary = (bytes / 1024 / 1024 / 1024).to_string();
    summary.push_str(constants::lan_pairing::MEMORY_GIB_LABEL);
    summary
}

fn memory_mib_summary(memory: &str) -> String {
    let mut summary = memory.to_string();
    summary.push_str(constants::lan_pairing::MEMORY_MIB_LABEL);
    summary
}

pub(super) fn gpu_names(
    nvidia_gpus: &[NvidiaGpu],
    windows_gpus: &[serde_json::Value],
) -> Option<String> {
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

pub(super) fn gpu_drivers(
    nvidia_gpus: &[NvidiaGpu],
    windows_gpus: &[serde_json::Value],
) -> Option<String> {
    joined_values(if nvidia_gpus.is_empty() {
        windows_gpus
            .iter()
            .filter_map(|gpu| record_text(gpu, constants::lan_pairing::JSON_KEY_DRIVER_VERSION))
            .collect()
    } else {
        nvidia_gpus.iter().map(|gpu| gpu.driver.clone()).collect()
    })
}

pub(super) fn gpu_memory(
    nvidia_gpus: &[NvidiaGpu],
    windows_gpus: &[serde_json::Value],
) -> Option<String> {
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

pub(super) fn nvidia_summary(nvidia_gpus: &[NvidiaGpu]) -> Option<String> {
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
