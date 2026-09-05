use std::collections::BTreeMap;

use ocentra_parent_agent_protocol::constants::v08_supported_adapter_runtime_proof as proof;
use ocentra_parent_agent_protocol::enforcement::ParentPlatform;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRefusalReason;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterResult;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRuntimeBoundary;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRuntimeProofEntry;
use ocentra_parent_agent_protocol::enforcement_supported_adapter_runtime_proof::V08SupportedAdapterRuntimeState;
use ocentra_parent_agent_protocol::policy_constants;

use super::enforcement_api::enforcement_supported_adapter_runtime_proof_read_model::{
    v08_supported_adapter_runtime_proof_read_model, GeneratedAtTextRef,
};
use super::test_text::{count_for_display, TestText};
use crate::test_require_some::require_some;

#[test]
fn supported_adapter_runtime_proof_read_model_preserves_honest_states() {
    let read_model = v08_supported_adapter_runtime_proof_read_model(GeneratedAtTextRef(
        policy_constants::TEST_EVALUATED_AT,
    ));
    let state_counts = count_states(&read_model.entries);
    let platform_counts = count_platforms(&read_model.entries);

    assert_eq!(read_model.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(read_model.entries.len(), 13);
    assert_eq!(
        state_count(&state_counts, proof::STATE_IMPLEMENTED_BOUNDARY),
        1
    );
    assert_eq!(state_count(&state_counts, proof::STATE_MANUAL_REQUIRED), 8);
    assert_eq!(state_count(&state_counts, proof::STATE_NOT_CLAIMED), 1);
    assert_eq!(state_count(&state_counts, proof::STATE_DEGRADED), 1);
    assert_eq!(state_count(&state_counts, proof::STATE_UNAVAILABLE), 1);
    assert_eq!(state_count(&state_counts, proof::STATE_UNSUPPORTED), 1);
    assert_eq!(
        platform_count(
            &platform_counts,
            policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
        ),
        9
    );
    assert_eq!(
        platform_count(&platform_counts, ParentPlatform::Linux.as_protocol_str()),
        1
    );
    assert_eq!(
        platform_count(&platform_counts, ParentPlatform::Macos.as_protocol_str()),
        1
    );
    assert_eq!(
        platform_count(&platform_counts, ParentPlatform::Android.as_protocol_str()),
        1
    );
    assert_eq!(
        platform_count(&platform_counts, ParentPlatform::Ios.as_protocol_str()),
        1
    );
    assert!(read_model
        .source_read_model_ids
        .contains(&proof::SOURCE_POLICY_DISPATCH_PROOF.to_string()));
    assert!(read_model
        .source_read_model_ids
        .contains(&proof::SOURCE_NETWORK_FLOW_EVIDENCE.to_string()));
    assert!(read_model
        .source_read_model_ids
        .contains(&proof::SOURCE_WINDOWS_ADAPTER_ARTIFACT_INGESTION_PROOF.to_string()));
}

#[test]
fn supported_adapter_runtime_proof_keeps_exact_boundaries() {
    let read_model = v08_supported_adapter_runtime_proof_read_model(GeneratedAtTextRef(
        policy_constants::TEST_EVALUATED_AT,
    ));

    assert_boundary_state(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::WindowsAppGameOwnedProcessTimeLimit,
        V08SupportedAdapterRuntimeState::ManualRequired,
    );
    let app_timer = entry_for(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::WindowsAppGameOwnedProcessTimeLimit,
    );
    assert_eq!(
        app_timer.adapter_result,
        V08SupportedAdapterResult::ManualProofRequired
    );
    assert_eq!(
        app_timer.refusal_reason,
        V08SupportedAdapterRefusalReason::ManualArtifactRequired
    );
    assert!(app_timer
        .manual_proof_requirements
        .contains(&proof::REQUIREMENT_ROLLBACK.to_string()));
    assert_boundary_state(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::WindowsNetworkFlowObservePolicyHandoff,
        V08SupportedAdapterRuntimeState::ImplementedBoundary,
    );
    assert_boundary_state(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::WindowsBroadInstalledAppBlockingManualGate,
        V08SupportedAdapterRuntimeState::ManualRequired,
    );
    assert_boundary_state(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::WindowsManagedExactActiveTabNotClaimed,
        V08SupportedAdapterRuntimeState::NotClaimed,
    );
    assert_boundary_state(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::WindowsBroadInstalledAppArtifactStatus,
        V08SupportedAdapterRuntimeState::ManualRequired,
    );
    assert_boundary_state(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::WindowsHostNetworkDomainArtifactStatus,
        V08SupportedAdapterRuntimeState::ManualRequired,
    );
    assert_boundary_state(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::WindowsManagedBrowserArtifactStatus,
        V08SupportedAdapterRuntimeState::ManualRequired,
    );
    assert_boundary_state(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::LinuxHostAdapterUnavailable,
        V08SupportedAdapterRuntimeState::Unavailable,
    );
    assert_boundary_state(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::MacosHostAdapterUnsupported,
        V08SupportedAdapterRuntimeState::Unsupported,
    );
    assert_boundary_state(
        &read_model.entries,
        V08SupportedAdapterRuntimeBoundary::WindowsAdapterPermissionDependencyDegraded,
        V08SupportedAdapterRuntimeState::Degraded,
    );
}

#[test]
fn supported_adapter_runtime_proof_does_not_upgrade_claim_flags() {
    let read_model = v08_supported_adapter_runtime_proof_read_model(GeneratedAtTextRef(
        policy_constants::TEST_EVALUATED_AT,
    ));

    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.broad_installed_app_blocking_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.network_domain_blocking_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.exact_active_tab_enforcement_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.notification_delivery_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.tamper_hardening_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.mobile_control_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.unsupported_platform_behavior_claimed));
}

fn entry_for(
    entries: &[V08SupportedAdapterRuntimeProofEntry],
    boundary: V08SupportedAdapterRuntimeBoundary,
) -> &V08SupportedAdapterRuntimeProofEntry {
    require_some(
        entries
            .iter()
            .find(|entry| entry.runtime_boundary == boundary),
        proof::READ_MODEL_ID,
    )
}

fn assert_boundary_state(
    entries: &[V08SupportedAdapterRuntimeProofEntry],
    boundary: V08SupportedAdapterRuntimeBoundary,
    runtime_state: V08SupportedAdapterRuntimeState,
) {
    let entry = entry_for(entries, boundary);

    assert_eq!(entry.runtime_state, runtime_state);
}

fn count_states(entries: &[V08SupportedAdapterRuntimeProofEntry]) -> BTreeMap<TestText, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(TestText::from_display(
                entry.runtime_state.as_protocol_str(),
            ))
            .or_default() += 1;
        counts
    })
}

fn state_count(counts: &BTreeMap<TestText, usize>, state: impl std::fmt::Display) -> usize {
    count_for_display(counts, state)
}

fn count_platforms(entries: &[V08SupportedAdapterRuntimeProofEntry]) -> BTreeMap<TestText, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(TestText::from_display(entry.platform.as_protocol_str()))
            .or_default() += 1;
        counts
    })
}

fn platform_count(counts: &BTreeMap<TestText, usize>, platform: impl std::fmt::Display) -> usize {
    count_for_display(counts, platform)
}
