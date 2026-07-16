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
fn supported_adapter_runtime_command_and_event_names_are_stable(
) -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        serde_json::to_value(AgentCommandName::AgentEnforcementSupportedAdapterRuntimeProofGet)?,
        "agent.enforcement.supported-adapter-runtime-proof.get"
    );
    assert_eq!(
        serde_json::to_value(AgentEventName::AgentEnforcementSupportedAdapterRuntimeProofReported)?,
        "agent.enforcement.supported-adapter-runtime-proof.reported"
    );
    Ok(())
}

#[test]
fn supported_adapter_runtime_states_have_stable_protocol_strings(
) -> Result<(), Box<dyn std::error::Error>> {
    let boundaries = [
        V08SupportedAdapterRuntimeBoundary::WindowsAppGameOwnedProcessTimeLimit,
        V08SupportedAdapterRuntimeBoundary::WindowsNetworkFlowObservePolicyHandoff,
        V08SupportedAdapterRuntimeBoundary::WindowsBroadInstalledAppBlockingManualGate,
        V08SupportedAdapterRuntimeBoundary::WindowsHostNetworkDomainBlockingManualGate,
        V08SupportedAdapterRuntimeBoundary::WindowsBroadInstalledAppArtifactStatus,
        V08SupportedAdapterRuntimeBoundary::WindowsHostNetworkDomainArtifactStatus,
        V08SupportedAdapterRuntimeBoundary::WindowsManagedBrowserArtifactStatus,
        V08SupportedAdapterRuntimeBoundary::WindowsManagedExactActiveTabNotClaimed,
        V08SupportedAdapterRuntimeBoundary::WindowsAdapterPermissionDependencyDegraded,
        V08SupportedAdapterRuntimeBoundary::LinuxHostAdapterUnavailable,
        V08SupportedAdapterRuntimeBoundary::MacosHostAdapterUnsupported,
        V08SupportedAdapterRuntimeBoundary::AndroidMobileControlManualGate,
        V08SupportedAdapterRuntimeBoundary::IosMobileControlManualGate,
    ];
    let serialized = serde_json::to_value(boundaries)?;

    let serialized = serialized
        .as_array()
        .ok_or_else(|| std::io::Error::other(constants::error::AGENT_EVENT_SERIALIZES))?;
    assert_eq!(serialized.len(), 13);
    assert_eq!(
        boundaries[0].as_protocol_str(),
        proof::ENTRY_ID_APP_GAME_TIMER
    );
    assert_eq!(
        boundaries[4].as_protocol_str(),
        proof::ENTRY_ID_BROAD_APP_ARTIFACT_STATUS
    );
    assert_eq!(
        boundaries[5].as_protocol_str(),
        proof::ENTRY_ID_HOST_NETWORK_ARTIFACT_STATUS
    );
    assert_eq!(boundaries[12].as_protocol_str(), proof::ENTRY_ID_IOS_MANUAL);
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
    assert_eq!(
        V08SupportedAdapterCapability::ManagedBrowserArtifactStatus.as_protocol_str(),
        proof::CAPABILITY_MANAGED_BROWSER_ARTIFACT_STATUS
    );
    Ok(())
}

#[test]
fn supported_adapter_runtime_read_model_serializes_honest_non_claims(
) -> Result<(), Box<dyn std::error::Error>> {
    let read_model = read_model_fixture();
    let reparsed = serde_json::from_value::<V08SupportedAdapterRuntimeProofReadModel>(
        serde_json::to_value(read_model)?,
    )?;
    let state_counts = count_runtime_states(&reparsed.entries);

    assert_eq!(reparsed.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(
        runtime_state_count(
            &state_counts,
            V08SupportedAdapterRuntimeState::ImplementedBoundary
        ),
        1
    );
    assert_eq!(
        runtime_state_count(
            &state_counts,
            V08SupportedAdapterRuntimeState::ManualRequired
        ),
        1
    );
    assert_eq!(
        runtime_state_count(&state_counts, V08SupportedAdapterRuntimeState::NotClaimed),
        1
    );
    assert_eq!(
        runtime_state_count(&state_counts, V08SupportedAdapterRuntimeState::Degraded),
        1
    );
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
    Ok(())
}

fn read_model_fixture() -> V08SupportedAdapterRuntimeProofReadModel {
    V08SupportedAdapterRuntimeProofReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: proof::READ_MODEL_ID.to_string(),
        generated_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source_read_model_ids: vec![proof::SOURCE_BROAD_ADAPTER_PROOF.to_string()],
        entries: vec![
            entry(
                V08SupportedAdapterRuntimeBoundary::WindowsAppGameOwnedProcessTimeLimit,
                ParentPlatform::Windows,
                V08SupportedAdapterCapability::AppGameOwnedProcessTimeLimit,
                V08SupportedAdapterRuntimeState::ImplementedBoundary,
                V08SupportedAdapterResult::SupportedBoundaryProved,
            ),
            entry(
                V08SupportedAdapterRuntimeBoundary::WindowsBroadInstalledAppBlockingManualGate,
                ParentPlatform::Windows,
                V08SupportedAdapterCapability::BroadInstalledAppBlocking,
                V08SupportedAdapterRuntimeState::ManualRequired,
                V08SupportedAdapterResult::ManualProofRequired,
            ),
            entry(
                V08SupportedAdapterRuntimeBoundary::WindowsManagedExactActiveTabNotClaimed,
                ParentPlatform::Windows,
                V08SupportedAdapterCapability::ManagedExactActiveTabEnforcement,
                V08SupportedAdapterRuntimeState::NotClaimed,
                V08SupportedAdapterResult::NotClaimed,
            ),
            entry(
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
    runtime_boundary: V08SupportedAdapterRuntimeBoundary,
    platform: ParentPlatform,
    adapter_capability: V08SupportedAdapterCapability,
    runtime_state: V08SupportedAdapterRuntimeState,
    adapter_result: V08SupportedAdapterResult,
) -> V08SupportedAdapterRuntimeProofEntry {
    V08SupportedAdapterRuntimeProofEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        proof_entry_id: runtime_boundary.as_protocol_str().to_string(),
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
) -> Vec<(V08SupportedAdapterRuntimeState, usize)> {
    entries.iter().fold(Vec::new(), |mut counts, entry| {
        if let Some((_, count)) = counts
            .iter_mut()
            .find(|(state, _)| *state == entry.runtime_state)
        {
            *count += 1;
        } else {
            counts.push((entry.runtime_state, 1));
        }
        counts
    })
}

fn runtime_state_count(
    counts: &[(V08SupportedAdapterRuntimeState, usize)],
    state: V08SupportedAdapterRuntimeState,
) -> usize {
    counts
        .iter()
        .find(|(entry_state, _)| *entry_state == state)
        .map(|(_, count)| *count)
        .unwrap_or_default()
}
