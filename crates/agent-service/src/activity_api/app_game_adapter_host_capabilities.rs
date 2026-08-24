use ocentra_parent_agent_protocol::app_game_adapter_execution_readiness::{
    APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE, APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED,
};
use ocentra_parent_agent_protocol::constants::v08_supported_adapter_runtime_proof as proof;
use ocentra_parent_screen_capture_adapter::linux_foreground_source::LinuxForegroundSourcePreflight;

use super::app_game_adapter_host_capabilities_paths::{
    EnvironmentName, ExecutableName, android_sdk_adb_available, executable_available,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct CapabilityState(pub(super) &'static str);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct EvidenceRefs(pub(super) Vec<String>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ProbeRefs(pub(super) Vec<String>);

#[derive(Clone, Debug)]
pub(super) struct HostCapabilitySignals {
    pub(super) android_adb: bool,
    pub(super) android_adb_path: bool,
    pub(super) android_adb_sdk: bool,
}

impl HostCapabilitySignals {
    pub(super) fn detect() -> Self {
        let android_adb_path = executable_available(ExecutableName(proof::EXE_ADB));
        let android_adb_sdk = [proof::ENV_ANDROID_HOME, proof::ENV_ANDROID_SDK_ROOT]
            .into_iter()
            .map(EnvironmentName)
            .any(android_sdk_adb_available);
        Self {
            android_adb: [android_adb_path, android_adb_sdk]
                .into_iter()
                .any(|available| available),
            android_adb_path,
            android_adb_sdk,
        }
    }

    pub(super) fn unavailable() -> Self {
        Self {
            android_adb: false,
            android_adb_path: false,
            android_adb_sdk: false,
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
        EvidenceRefs(Vec::new())
    }

    pub(super) fn linux_probe_refs(&self) -> ProbeRefs {
        ProbeRefs(Vec::new())
    }

    pub(super) fn linux_state(&self) -> CapabilityState {
        CapabilityState(APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED)
    }

    pub(super) fn linux_state_for(
        &self,
        preflight: &LinuxForegroundSourcePreflight,
    ) -> CapabilityState {
        CapabilityState(
            match preflight.source_ready() {
                ocentra_parent_screen_capture_adapter::linux_foreground_source::LinuxSourceReadiness::Ready => {
                    APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE
                }
                ocentra_parent_screen_capture_adapter::linux_foreground_source::LinuxSourceReadiness::Unavailable => {
                    APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED
                }
            },
        )
    }
}
