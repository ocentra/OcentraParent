#[path = "../../src/activity_api/app_game_adapter_host_capabilities.rs"]
mod app_game_adapter_host_capabilities;

use ocentra_parent_agent_protocol::app_game_adapter_execution_readiness::{
    APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE, APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED,
};
use ocentra_parent_agent_protocol::constants::v08_supported_adapter_runtime_proof as proof;

use app_game_adapter_host_capabilities::HostCapabilitySignals;

#[test]
fn android_probe_refs_distinguish_path_and_sdk_visibility() {
    let signals = HostCapabilitySignals {
        android_adb: true,
        android_adb_path: true,
        android_adb_sdk: true,
        linux_wsl: false,
        linux_docker: false,
    };

    assert_eq!(
        signals.android_state(),
        APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE
    );
    assert_eq!(
        signals.android_evidence_refs(),
        vec![proof::REF_ANDROID_ADB_HOST_TOOLCHAIN]
    );
    assert_eq!(
        signals.android_probe_refs(),
        vec![
            proof::REF_ANDROID_ADB_PATH_PROBE,
            proof::REF_ANDROID_ADB_SDK_PROBE,
        ]
    );
}

#[test]
fn linux_probe_refs_keep_wsl_and_docker_separate() {
    let signals = HostCapabilitySignals {
        android_adb: false,
        android_adb_path: false,
        android_adb_sdk: false,
        linux_wsl: true,
        linux_docker: true,
    };

    assert_eq!(
        signals.linux_state(),
        APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE
    );
    assert_eq!(
        signals.linux_evidence_refs(),
        vec![
            proof::REF_LINUX_WSL_HOST_TOOLCHAIN,
            proof::REF_LINUX_DOCKER_HOST_TOOLCHAIN,
        ]
    );
    assert_eq!(
        signals.linux_probe_refs(),
        vec![
            proof::REF_LINUX_WSL_PATH_PROBE,
            proof::REF_LINUX_DOCKER_PATH_PROBE,
        ]
    );
}

#[test]
fn missing_host_tools_report_not_detected_without_probe_refs() {
    let signals = HostCapabilitySignals {
        android_adb: false,
        android_adb_path: false,
        android_adb_sdk: false,
        linux_wsl: false,
        linux_docker: false,
    };

    assert_eq!(
        signals.android_state(),
        APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED
    );
    assert_eq!(signals.android_evidence_refs(), Vec::<&'static str>::new());
    assert_eq!(signals.android_probe_refs(), Vec::<&'static str>::new());
    assert_eq!(
        signals.linux_state(),
        APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED
    );
    assert_eq!(signals.linux_evidence_refs(), Vec::<&'static str>::new());
    assert_eq!(signals.linux_probe_refs(), Vec::<&'static str>::new());
}

#[test]
fn detect_uses_environment_probes_and_keeps_probe_ref_counts_aligned() {
    let signals = HostCapabilitySignals::detect();

    assert_eq!(
        signals.android_evidence_refs().len(),
        usize::from(signals.android_adb)
    );
    assert_eq!(
        signals.android_probe_refs().len(),
        usize::from(signals.android_adb_path) + usize::from(signals.android_adb_sdk)
    );
    assert_eq!(
        signals.linux_evidence_refs().len(),
        usize::from(signals.linux_wsl) + usize::from(signals.linux_docker)
    );
    assert_eq!(
        signals.linux_probe_refs().len(),
        usize::from(signals.linux_wsl) + usize::from(signals.linux_docker)
    );
}
