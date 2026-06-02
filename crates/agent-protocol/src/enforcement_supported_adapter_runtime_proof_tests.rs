use std::collections::BTreeMap;

use crate::{
    constants::{self, v08_supported_adapter_runtime_proof as proof},
    policy_constants, AgentCommandName, AgentEventName, ParentPlatform,
    V08SupportedAdapterAuditReferenceState, V08SupportedAdapterCapability,
    V08SupportedAdapterPlatformSupportState, V08SupportedAdapterRefusalReason,
    V08SupportedAdapterResult, V08SupportedAdapterRollbackReferenceState,
    V08SupportedAdapterRuntimeBoundary, V08SupportedAdapterRuntimeProofEntry,
    V08SupportedAdapterRuntimeProofReadModel, V08SupportedAdapterRuntimeState,
    V08SupportedAdapterTargetIdentityState,
};

#[test]
fn supported_adapter_runtime_command_and_event_names_are_stable() {
    assert_eq!(
        serde_json::to_value(AgentCommandName::AgentEnforcementSupportedAdapterRuntimeProofGet)
            .expect(constants::error::AGENT_EVENT_SERIALIZES),
        "agent.enforcement.supported-adapter-runtime-proof.get"
    );
    assert_eq!(
        serde_json::to_value(AgentEventName::AgentEnforcementSupportedAdapterRuntimeProofReported)
            .expect(constants::error::AGENT_EVENT_SERIALIZES),
        "agent.enforcement.supported-adapter-runtime-proof.reported"
    );
}

#[test]
fn supported_adapter_runtime_states_have_stable_protocol_strings() {
    let boundaries = [
        V08SupportedAdapterRuntimeBoundary::WindowsAppGameOwnedProcessTimeLimit,
        V08SupportedAdapterRuntimeBoundary::WindowsNetworkFlowObservePolicyHandoff,
        V08SupportedAdapterRuntimeBoundary::WindowsBroadInstalledAppBlockingManualGate,
        V08SupportedAdapterRuntimeBoundary::WindowsHostNetworkDomainBlockingManualGate,
        V08SupportedAdapterRuntimeBoundary::WindowsManagedExactActiveTabNotClaimed,
        V08SupportedAdapterRuntimeBoundary::WindowsAdapterPermissionDependencyDegraded,
        V08SupportedAdapterRuntimeBoundary::LinuxHostAdapterUnavailable,
        V08SupportedAdapterRuntimeBoundary::MacosHostAdapterUnsupported,
        V08SupportedAdapterRuntimeBoundary::AndroidMobileControlManualGate,
        V08SupportedAdapterRuntimeBoundary::IosMobileControlManualGate,
    ];
    let serialized =
        serde_json::to_value(boundaries).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized
            .as_array()
            .expect(constants::error::AGENT_EVENT_SERIALIZES)
            .len(),
        10
    );
    assert_eq!(
        boundaries[0].as_protocol_str(),
        proof::ENTRY_ID_APP_GAME_TIMER
    );
    assert_eq!(boundaries[9].as_protocol_str(), proof::ENTRY_ID_IOS_MANUAL);
    assert_eq!(
        V08SupportedAdapterRuntimeState::ImplementedBoundary.as_protocol_str(),
        proof::STATE_IMPLEMENTED_BOUNDARY
    );
    assert_eq!(
        V08SupportedAdapterRuntimeState::Degraded.as_protocol_str(),
        proof::STATE_DEGRADED
    );
    assert_eq!(
        V08SupportedAdapterResult::SupportedBoundaryProved.as_protocol_str(),
        proof::RESULT_SUPPORTED_BOUNDARY_PROVED
    );
    assert_eq!(
        V08SupportedAdapterCapability::NetworkFlowObservePolicyHandoff.as_protocol_str(),
        proof::CAPABILITY_NETWORK_OBSERVE
    );
}

#[test]
fn supported_adapter_runtime_read_model_serializes_honest_non_claims() {
    let read_model = read_model_fixture();
    let reparsed = serde_json::from_value::<V08SupportedAdapterRuntimeProofReadModel>(
        serde_json::to_value(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
    )
    .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let state_counts = count_runtime_states(&reparsed.entries);

    assert_eq!(reparsed.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(state_counts[proof::STATE_IMPLEMENTED_BOUNDARY], 1);
    assert_eq!(state_counts[proof::STATE_MANUAL_REQUIRED], 1);
    assert_eq!(state_counts[proof::STATE_NOT_CLAIMED], 1);
    assert_eq!(state_counts[proof::STATE_DEGRADED], 1);
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.broad_installed_app_blocking_claimed));
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.network_domain_blocking_claimed));
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.exact_active_tab_enforcement_claimed));
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.notification_delivery_claimed));
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.tamper_hardening_claimed));
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.mobile_control_claimed));
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.unsupported_platform_behavior_claimed));
}

fn read_model_fixture() -> V08SupportedAdapterRuntimeProofReadModel {
    V08SupportedAdapterRuntimeProofReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: proof::READ_MODEL_ID.to_string(),
        generated_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source_read_model_ids: vec![proof::SOURCE_BROAD_ADAPTER_PROOF.to_string()],
        entries: vec![
            entry(
                proof::ENTRY_ID_APP_GAME_TIMER,
                V08SupportedAdapterRuntimeBoundary::WindowsAppGameOwnedProcessTimeLimit,
                ParentPlatform::Windows,
                V08SupportedAdapterCapability::AppGameOwnedProcessTimeLimit,
                V08SupportedAdapterRuntimeState::ImplementedBoundary,
                V08SupportedAdapterResult::SupportedBoundaryProved,
            ),
            entry(
                proof::ENTRY_ID_BROAD_APP_MANUAL,
                V08SupportedAdapterRuntimeBoundary::WindowsBroadInstalledAppBlockingManualGate,
                ParentPlatform::Windows,
                V08SupportedAdapterCapability::BroadInstalledAppBlocking,
                V08SupportedAdapterRuntimeState::ManualRequired,
                V08SupportedAdapterResult::ManualProofRequired,
            ),
            entry(
                proof::ENTRY_ID_EXACT_ACTIVE_TAB_NOT_CLAIMED,
                V08SupportedAdapterRuntimeBoundary::WindowsManagedExactActiveTabNotClaimed,
                ParentPlatform::Windows,
                V08SupportedAdapterCapability::ManagedExactActiveTabEnforcement,
                V08SupportedAdapterRuntimeState::NotClaimed,
                V08SupportedAdapterResult::NotClaimed,
            ),
            entry(
                proof::ENTRY_ID_PERMISSION_DEGRADED,
                V08SupportedAdapterRuntimeBoundary::WindowsAdapterPermissionDependencyDegraded,
                ParentPlatform::Windows,
                V08SupportedAdapterCapability::AdapterPermissionDependency,
                V08SupportedAdapterRuntimeState::Degraded,
                V08SupportedAdapterResult::DegradedPermissionOrDependency,
            ),
        ],
    }
}

fn entry(
    proof_entry_id: &str,
    runtime_boundary: V08SupportedAdapterRuntimeBoundary,
    platform: ParentPlatform,
    adapter_capability: V08SupportedAdapterCapability,
    runtime_state: V08SupportedAdapterRuntimeState,
    adapter_result: V08SupportedAdapterResult,
) -> V08SupportedAdapterRuntimeProofEntry {
    V08SupportedAdapterRuntimeProofEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        proof_entry_id: proof_entry_id.to_string(),
        runtime_boundary,
        platform,
        adapter_capability,
        runtime_state,
        adapter_result,
        platform_support_state: V08SupportedAdapterPlatformSupportState::ManualRequired,
        target_identity_state: V08SupportedAdapterTargetIdentityState::InsufficientForBroadTarget,
        rollback_reference_state: V08SupportedAdapterRollbackReferenceState::ManualRequired,
        audit_reference_state: V08SupportedAdapterAuditReferenceState::ManualRequired,
        refusal_reason: V08SupportedAdapterRefusalReason::ManualArtifactRequired,
        evidence_refs: vec![proof::REF_APP_SESSION_EVIDENCE.to_string()],
        linked_proof_commands: vec![proof::COMMAND_APP_TIME_LIMIT_ADAPTER.to_string()],
        linked_proof_artifacts: vec![proof::ARTIFACT_APP_TIME_LIMIT_PROOF.to_string()],
        manual_proof_requirements: vec![proof::REQUIREMENT_ROLLBACK.to_string()],
        claim_boundary: proof::CLAIM_BROAD_APP_MANUAL.to_string(),
        fallback_behavior: proof::FALLBACK_BROAD_APP_MANUAL.to_string(),
        broad_installed_app_blocking_claimed: false,
        network_domain_blocking_claimed: false,
        exact_active_tab_enforcement_claimed: false,
        notification_delivery_claimed: false,
        tamper_hardening_claimed: false,
        mobile_control_claimed: false,
        unsupported_platform_behavior_claimed: false,
        last_checked_at: policy_constants::TEST_EVALUATED_AT.to_string(),
    }
}

fn count_runtime_states(
    entries: &[V08SupportedAdapterRuntimeProofEntry],
) -> BTreeMap<&'static str, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(entry.runtime_state.as_protocol_str())
            .or_default() += 1;
        counts
    })
}
