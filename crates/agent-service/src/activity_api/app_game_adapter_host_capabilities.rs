use ocentra_parent_agent_protocol::app_game_adapter_execution_readiness::{
    APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE, APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED,
};
use ocentra_parent_agent_protocol::constants::v08_supported_adapter_runtime_proof as proof;

use super::app_game_adapter_host_capabilities_paths::{
    android_sdk_adb_available, executable_available, EnvironmentName, ExecutableName,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct CapabilityState(pub(super) &'static str);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct EvidenceRefs(pub(super) Vec<String>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ProbeRefs(pub(super) Vec<String>);

pub(super) struct HostCapabilitySignals {
    pub(super) android_adb: bool,
    pub(super) android_adb_path: bool,
    pub(super) android_adb_sdk: bool,
    pub(super) linux_wsl: bool,
    pub(super) linux_docker: bool,
}

impl HostCapabilitySignals {
    pub(super) fn detect() -> Self {
        let android_adb_path = executable_available(ExecutableName(proof::EXE_ADB));
        let android_adb_sdk = android_sdk_adb_available(EnvironmentName(proof::ENV_ANDROID_HOME))
            || android_sdk_adb_available(EnvironmentName(proof::ENV_ANDROID_SDK_ROOT));
        Self {
            android_adb: android_adb_path || android_adb_sdk,
            android_adb_path,
            android_adb_sdk,
            linux_wsl: executable_available(ExecutableName(proof::EXE_WSL)),
            linux_docker: executable_available(ExecutableName(proof::EXE_DOCKER)),
        }
    }

    pub(super) fn android_evidence_refs(&self) -> EvidenceRefs {
        EvidenceRefs(
            self.android_adb
                .then_some(proof::REF_ANDROID_ADB_HOST_TOOLCHAIN.to_string())
                .into_iter()
                .collect(),
        )
    }

    pub(super) fn android_probe_refs(&self) -> ProbeRefs {
        ProbeRefs(
            [
                self.android_adb_path
                    .then_some(proof::REF_ANDROID_ADB_PATH_PROBE.to_string()),
                self.android_adb_sdk
                    .then_some(proof::REF_ANDROID_ADB_SDK_PROBE.to_string()),
            ]
            .into_iter()
            .flatten()
            .collect(),
        )
    }

    pub(super) fn android_state(&self) -> CapabilityState {
        CapabilityState(
            [
                APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED,
                APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE,
            ][self.android_adb as usize],
        )
    }

    pub(super) fn linux_evidence_refs(&self) -> EvidenceRefs {
        EvidenceRefs(
            [
                self.linux_wsl
                    .then_some(proof::REF_LINUX_WSL_HOST_TOOLCHAIN.to_string()),
                self.linux_docker
                    .then_some(proof::REF_LINUX_DOCKER_HOST_TOOLCHAIN.to_string()),
            ]
            .into_iter()
            .flatten()
            .collect(),
        )
    }

    pub(super) fn linux_probe_refs(&self) -> ProbeRefs {
        ProbeRefs(
            [
                self.linux_wsl
                    .then_some(proof::REF_LINUX_WSL_PATH_PROBE.to_string()),
                self.linux_docker
                    .then_some(proof::REF_LINUX_DOCKER_PATH_PROBE.to_string()),
            ]
            .into_iter()
            .flatten()
            .collect(),
        )
    }

    pub(super) fn linux_state(&self) -> CapabilityState {
        CapabilityState(
            [
                APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED,
                APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE,
            ][(self.linux_wsl || self.linux_docker) as usize],
        )
    }
}
