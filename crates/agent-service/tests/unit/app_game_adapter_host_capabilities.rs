use ocentra_parent_agent_protocol::app_game_adapter_execution_readiness::{
    APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE, APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED,
};
use ocentra_parent_agent_protocol::constants::v08_supported_adapter_runtime_proof as proof;
use ocentra_parent_screen_capture_adapter::linux_foreground_source::LinuxForegroundSourcePreflight;

use super::app_game_adapter_host_capabilities::{
    CapabilityState, EvidenceRefs, HostCapabilitySignals, ProbeRefs,
};

#[test]
fn android_probe_refs_distinguish_path_and_sdk_visibility() {
    let signals = HostCapabilitySignals {
        android_adb: true,
        android_adb_path: true,
        android_adb_sdk: true,
    };

    assert_eq!(
        signals.android_state(),
        CapabilityState(APP_GAME_ADAPTER_HOST_CAPABILITY_AVAILABLE)
    );
    assert_eq!(
        signals.android_evidence_refs(),
        EvidenceRefs(vec![String::from(proof::REF_ANDROID_ADB_HOST_TOOLCHAIN)])
    );
    assert_eq!(
        signals.android_probe_refs(),
        ProbeRefs(vec![
            String::from(proof::REF_ANDROID_ADB_PATH_PROBE),
            String::from(proof::REF_ANDROID_ADB_SDK_PROBE),
        ])
    );
}

#[test]
fn default_linux_state_is_unavailable_without_refs() {
    let signals = HostCapabilitySignals {
        android_adb: false,
        android_adb_path: false,
        android_adb_sdk: false,
    };

    assert_eq!(
        signals.linux_state(),
        CapabilityState(APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED)
    );
    assert_eq!(signals.linux_evidence_refs(), EvidenceRefs(Vec::new()));
    assert_eq!(signals.linux_probe_refs(), ProbeRefs(Vec::new()));
}

#[test]
fn missing_host_tools_report_not_detected_without_probe_refs() {
    let signals = HostCapabilitySignals {
        android_adb: false,
        android_adb_path: false,
        android_adb_sdk: false,
    };

    assert_eq!(
        signals.android_state(),
        CapabilityState(APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED)
    );
    assert_eq!(signals.android_evidence_refs(), EvidenceRefs(Vec::new()));
    assert_eq!(signals.android_probe_refs(), ProbeRefs(Vec::new()));
    assert_eq!(
        signals.linux_state(),
        CapabilityState(APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED)
    );
    assert_eq!(signals.linux_evidence_refs(), EvidenceRefs(Vec::new()));
    assert_eq!(signals.linux_probe_refs(), ProbeRefs(Vec::new()));
}

#[test]
fn unavailable_linux_preflight_yields_no_evidence_probe_or_proof_refs() {
    let signals = HostCapabilitySignals {
        android_adb: false,
        android_adb_path: false,
        android_adb_sdk: false,
    };
    let preflight = LinuxForegroundSourcePreflight::unavailable();

    assert_eq!(
        signals.linux_state_for(&preflight),
        CapabilityState(APP_GAME_ADAPTER_HOST_CAPABILITY_NOT_DETECTED)
    );
    assert_eq!(
        signals.linux_evidence_refs_for(&preflight),
        EvidenceRefs(Vec::new())
    );
    assert_eq!(
        signals.linux_probe_refs_for(&preflight),
        ProbeRefs(Vec::new())
    );
    assert_eq!(
        signals.linux_proof_refs_for(&preflight),
        EvidenceRefs(Vec::new())
    );
}
