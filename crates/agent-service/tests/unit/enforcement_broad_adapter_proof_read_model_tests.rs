use std::collections::BTreeMap;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::constants::enforcement_broad_adapter_proof as proof;
use ocentra_parent_agent_protocol::enforcement::ParentPlatform;
use ocentra_parent_agent_protocol::enforcement_broad_adapter_proof::V08BroadAdapterRuntimeClaimState;
use ocentra_parent_agent_protocol::enforcement_broad_adapter_proof::V08BroadAdapterRuntimeProofEntry;
use ocentra_parent_agent_protocol::enforcement_broad_adapter_proof::V08BroadAdapterRuntimeProofReadModel;
use ocentra_parent_agent_protocol::enforcement_broad_adapter_proof::V08BroadAdapterRuntimeSurface;
use ocentra_parent_agent_protocol::policy_constants;

use super::enforcement_api::enforcement_broad_adapter_proof_read_model::{
    v08_broad_adapter_proof_read_model, GeneratedAtTextRef,
};
use super::test_text::{count_for_display, TestText};
use crate::test_invariants::require_some;

#[test]
fn broad_adapter_proof_read_model_preserves_honest_runtime_states() {
    let read_model =
        v08_broad_adapter_proof_read_model(GeneratedAtTextRef(policy_constants::TEST_EVALUATED_AT));
    let claim_counts = count_claims(&read_model.entries);
    let platform_counts = count_platforms(&read_model.entries);

    assert_eq!(read_model.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(read_model.entries.len(), 10);
    assert_eq!(
        claim_count(&claim_counts, proof::CLAIM_IMPLEMENTED_BOUNDARY),
        2
    );
    assert_eq!(claim_count(&claim_counts, proof::CLAIM_MANUAL_REQUIRED), 6);
    assert_eq!(claim_count(&claim_counts, proof::CLAIM_UNAVAILABLE), 1);
    assert_eq!(claim_count(&claim_counts, proof::CLAIM_NOT_CLAIMED), 1);
    assert_eq!(
        platform_count(
            &platform_counts,
            policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS
        ),
        6
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
        .contains(&proof::SOURCE_BROWSER_DOMAIN_ADAPTER_PROOF.to_string()));
    assert!(read_model
        .source_read_model_ids
        .contains(&proof::SOURCE_OS_ADAPTER_MANUAL_ARTIFACT_GATES.to_string()));
}

#[test]
fn broad_adapter_proof_read_model_keeps_surface_outcomes_exact() {
    let read_model =
        v08_broad_adapter_proof_read_model(GeneratedAtTextRef(policy_constants::TEST_EVALUATED_AT));

    assert_surface_state(
        &read_model.entries,
        V08BroadAdapterRuntimeSurface::WindowsOwnedProcessAndTimerRuntimeBoundary,
        V08BroadAdapterRuntimeClaimState::ImplementedBoundary,
    );
    assert_surface_state(
        &read_model.entries,
        V08BroadAdapterRuntimeSurface::WindowsManagedBrowserSessionRuntimeBoundary,
        V08BroadAdapterRuntimeClaimState::ImplementedBoundary,
    );
    assert_surface_state(
        &read_model.entries,
        V08BroadAdapterRuntimeSurface::WindowsNetworkDomainRuntimeGate,
        V08BroadAdapterRuntimeClaimState::ManualRequired,
    );
    assert_surface_state(
        &read_model.entries,
        V08BroadAdapterRuntimeSurface::WindowsUnmanagedBrowserExactEvidenceRuntimeGap,
        V08BroadAdapterRuntimeClaimState::NotClaimed,
    );
    assert_surface_state(
        &read_model.entries,
        V08BroadAdapterRuntimeSurface::LinuxHostRuntimeUnavailable,
        V08BroadAdapterRuntimeClaimState::Unavailable,
    );
}

#[test]
fn broad_adapter_proof_read_model_does_not_upgrade_claim_flags() {
    let read_model =
        v08_broad_adapter_proof_read_model(GeneratedAtTextRef(policy_constants::TEST_EVALUATED_AT));

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
        .all(|entry| !entry.managed_browser_exact_url_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.unmanaged_browser_exact_evidence_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.unsupported_platform_claimed));
    assert!(read_model
        .entries
        .iter()
        .all(|entry| !entry.mobile_privilege_claimed));
}

fn entry_for(
    entries: &[V08BroadAdapterRuntimeProofEntry],
    surface: V08BroadAdapterRuntimeSurface,
) -> &V08BroadAdapterRuntimeProofEntry {
    require_some(
        entries
            .iter()
            .find(|entry| entry.runtime_surface == surface),
        proof::READ_MODEL_ID,
    )
}

fn assert_surface_state(
    entries: &[V08BroadAdapterRuntimeProofEntry],
    surface: V08BroadAdapterRuntimeSurface,
    product_claim_state: V08BroadAdapterRuntimeClaimState,
) {
    let entry = entry_for(entries, surface);

    assert_eq!(entry.product_claim_state, product_claim_state);
}

fn count_claims(entries: &[V08BroadAdapterRuntimeProofEntry]) -> BTreeMap<TestText, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(TestText::from_display(
                entry.product_claim_state.as_protocol_str(),
            ))
            .or_default() += 1;
        counts
    })
}

fn claim_count(counts: &BTreeMap<TestText, usize>, claim: impl std::fmt::Display) -> usize {
    count_for_display(counts, claim)
}

fn count_platforms(entries: &[V08BroadAdapterRuntimeProofEntry]) -> BTreeMap<TestText, usize> {
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
