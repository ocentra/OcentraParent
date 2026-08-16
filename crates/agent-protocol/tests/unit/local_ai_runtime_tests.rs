use super::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute, LocalAiAdapterBoundary, LocalAiCapabilityFlag, LocalAiDegradedState,
    LocalAiExecutionState, LocalAiModelLoadState, LocalAiProviderPrivacyMode,
    LocalAiProviderSource, LocalAiResourceClass, LocalModelRuntimeStatus, LogFields,
    AGENT_PROTOCOL_SCHEMA_VERSION,
};
use crate::local_ai_runtime::status::LocalProviderCapability;
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn local_model_runtime_status_serializes_to_typescript_contract_shape() {
    let status = LocalModelRuntimeStatus {
        runtime_reference_id: constants::local_ai_runtime::RUNTIME_REFERENCE_DEV_UNCONFIGURED
            .to_string(),
        provider_id: constants::local_ai_runtime::PROVIDER_ID_UNCONFIGURED.to_string(),
        model_id: constants::local_ai_runtime::MODEL_ID_UNCONFIGURED.to_string(),
        model_reference: constants::local_ai_runtime::MODEL_REFERENCE_UNCONFIGURED.to_string(),
        privacy_mode: LocalAiProviderPrivacyMode::LocalOnly,
        adapter_boundary: LocalAiAdapterBoundary::LocalAdapterUnavailable,
        execution_state: LocalAiExecutionState::Disabled,
        provider_source: LocalAiProviderSource::Unavailable,
        load_state: LocalAiModelLoadState::Unavailable,
        capability_flags: vec![],
        resource_class: LocalAiResourceClass::Cpu,
        degraded_state: LocalAiDegradedState::ProviderUnavailable,
        last_checked_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        unavailable_reason: Some(
            constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED.to_string(),
        ),
    };

    let serialized = serde_json::to_value(status).expect_value("local AI status serializes");

    assert_eq!(
        serialized["runtimeReferenceId"],
        constants::local_ai_runtime::RUNTIME_REFERENCE_DEV_UNCONFIGURED
    );
    assert_eq!(
        serialized["loadState"],
        constants::local_ai_runtime::LOAD_STATE_UNAVAILABLE
    );
    assert_eq!(
        serialized["privacyMode"],
        constants::local_ai_runtime::PRIVACY_MODE_LOCAL_ONLY
    );
    assert_eq!(
        serialized["adapterBoundary"],
        constants::local_ai_runtime::ADAPTER_BOUNDARY_LOCAL_ADAPTER_UNAVAILABLE
    );
    assert_eq!(
        serialized["executionState"],
        constants::local_ai_runtime::EXECUTION_STATE_DISABLED
    );
    assert_eq!(
        serialized["providerSource"],
        constants::local_ai_runtime::PROVIDER_SOURCE_UNAVAILABLE
    );
    assert_eq!(
        serialized["degradedState"],
        constants::local_ai_runtime::DEGRADED_PROVIDER_UNAVAILABLE
    );
    assert_eq!(
        serialized["unavailableReason"],
        constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED
    );
}

#[test]
fn local_model_load_state_serializes_every_safe_lifecycle_state() {
    let states = vec![
        (
            LocalAiModelLoadState::Unavailable,
            constants::local_ai_runtime::LOAD_STATE_UNAVAILABLE,
        ),
        (
            LocalAiModelLoadState::Loading,
            constants::local_ai_runtime::LOAD_STATE_LOADING,
        ),
        (
            LocalAiModelLoadState::Loaded,
            constants::local_ai_runtime::LOAD_STATE_LOADED,
        ),
        (
            LocalAiModelLoadState::Degraded,
            constants::local_ai_runtime::LOAD_STATE_DEGRADED,
        ),
        (
            LocalAiModelLoadState::Failed,
            constants::local_ai_runtime::LOAD_STATE_FAILED,
        ),
    ];

    for (state, expected) in states {
        let serialized = serde_json::to_value(state).expect_value("load state serializes");

        assert_eq!(serialized, expected);
    }
}

#[test]
fn local_provider_capability_serializes_local_only_without_remote_ai() {
    let capability = LocalProviderCapability {
        provider_id: constants::local_ai_runtime::PROVIDER_ID_UNCONFIGURED.to_string(),
        supported_tasks: vec![LocalAiCapabilityFlag::SafetyDecision],
        resource_class: LocalAiResourceClass::Cpu,
        privacy_mode: LocalAiProviderPrivacyMode::LocalOnly,
        fallback_order: 1,
    };

    let serialized =
        serde_json::to_value(capability).expect_value("provider capability serializes");

    assert_eq!(
        serialized["privacyMode"],
        constants::local_ai_runtime::PRIVACY_MODE_LOCAL_ONLY
    );
    assert_eq!(
        serialized["supportedTasks"][0],
        constants::local_ai_runtime::CAPABILITY_SAFETY_DECISION
    );
}

#[test]
fn local_ai_runtime_status_command_serializes_to_typescript_contract_shape() {
    let command = AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: "cmd-local-ai-status".to_string(),
        sent_at: "2026-05-21T09:18:00Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: "windows".to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentLocalAiRuntimeStatusGet,
        payload: LogFields::new(),
    };

    let serialized = serde_json::to_value(command).expect_value("command serializes");

    assert_eq!(serialized["command"], "agent.local-ai.runtime.status.get");
}
