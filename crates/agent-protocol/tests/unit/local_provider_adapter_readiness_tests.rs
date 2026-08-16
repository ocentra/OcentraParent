use super::{
    constants, LocalAiAdapterBoundary, LocalAiAdapterProbeState, LocalAiAdapterReadinessState,
    LocalAiExecutionState, LocalAiProviderConfigurationState, LocalAiProviderPrivacyMode,
    LocalAiProviderSource, LocalProviderAdapterProbe,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn local_provider_adapter_probe_serializes_no_execution_readiness_status() {
    let probe = LocalProviderAdapterProbe {
        provider_id: constants::local_ai_runtime::PROVIDER_ID_UNCONFIGURED.to_string(),
        privacy_mode: LocalAiProviderPrivacyMode::LocalOnly,
        adapter_boundary: LocalAiAdapterBoundary::StatusOnly,
        execution_state: LocalAiExecutionState::Disabled,
        provider_source: LocalAiProviderSource::Unavailable,
        probe_state: LocalAiAdapterProbeState::ProbeUnavailable,
        configuration_state: LocalAiProviderConfigurationState::LocalProviderUnconfigured,
        readiness_state: LocalAiAdapterReadinessState::AdapterNotReady,
        execution_allowed: false,
        last_checked_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        unavailable_reason: Some(
            constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED.to_string(),
        ),
    };

    let serialized = serde_json::to_value(probe).expect_value("adapter probe serializes");

    assert_eq!(
        serialized["probeState"],
        constants::local_ai_runtime::ADAPTER_PROBE_STATE_UNAVAILABLE
    );
    assert_eq!(
        serialized["configurationState"],
        constants::local_ai_runtime::PROVIDER_CONFIGURATION_UNCONFIGURED
    );
    assert_eq!(
        serialized["readinessState"],
        constants::local_ai_runtime::ADAPTER_READINESS_STATE_NOT_READY
    );
    assert_eq!(serialized["executionAllowed"], false);
}

#[test]
fn local_provider_adapter_readiness_state_serializes_every_boundary_state() {
    let states = vec![
        (
            LocalAiAdapterReadinessState::AdapterNotReady,
            constants::local_ai_runtime::ADAPTER_READINESS_STATE_NOT_READY,
        ),
        (
            LocalAiAdapterReadinessState::AdapterReady,
            constants::local_ai_runtime::ADAPTER_READINESS_STATE_READY,
        ),
        (
            LocalAiAdapterReadinessState::AdapterReadinessInvalid,
            constants::local_ai_runtime::ADAPTER_READINESS_STATE_INVALID,
        ),
    ];

    for (state, expected) in states {
        let serialized = serde_json::to_value(state).expect_value("adapter readiness serializes");

        assert_eq!(serialized, expected);
    }
}
