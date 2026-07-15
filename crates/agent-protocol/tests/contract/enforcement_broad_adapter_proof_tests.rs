use ocentra_eventing::expect_value::ExpectValue;
use std::collections::BTreeMap;

use crate::{
    constants::{self, enforcement_broad_adapter_proof as proof},
    policy_constants, ParentPlatform, V08BroadAdapterRuntimeClaimState,
    V08BroadAdapterRuntimeEvidenceState, V08BroadAdapterRuntimeProofEntry,
    V08BroadAdapterRuntimeProofReadModel, V08BroadAdapterRuntimeSurface,
};

#[test]
fn broad_adapter_runtime_surfaces_have_stable_protocol_strings() {
    let surfaces = [
        V08BroadAdapterRuntimeSurface::WindowsOwnedProcessAndTimerRuntimeBoundary,
        V08BroadAdapterRuntimeSurface::WindowsManagedBrowserSessionRuntimeBoundary,
        V08BroadAdapterRuntimeSurface::WindowsBroadInstalledAppRuntimeGate,
        V08BroadAdapterRuntimeSurface::WindowsNetworkDomainRuntimeGate,
        V08BroadAdapterRuntimeSurface::WindowsManagedBrowserExactUrlRuntimeGate,
        V08BroadAdapterRuntimeSurface::WindowsUnmanagedBrowserExactEvidenceRuntimeGap,
        V08BroadAdapterRuntimeSurface::LinuxHostRuntimeUnavailable,
        V08BroadAdapterRuntimeSurface::MacosHostRuntimeManualGate,
        V08BroadAdapterRuntimeSurface::AndroidMobileRuntimeManualGate,
        V08BroadAdapterRuntimeSurface::IosMobileRuntimeManualGate,
    ];
    let serialized =
        serde_json::to_value(surfaces).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized
            .as_array()
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES)
            .len(),
        10
    );
    assert_eq!(
        surfaces[0].as_protocol_str(),
        proof::SURFACE_OWNED_PROCESS_TIMER
    );
    assert_eq!(
        surfaces[9].as_protocol_str(),
        proof::SURFACE_IOS_MANUAL_GATE
    );
}

#[test]
fn broad_adapter_runtime_states_serialize_as_contract_values() {
    assert_eq!(
        V08BroadAdapterRuntimeClaimState::ImplementedBoundary.as_protocol_str(),
        proof::CLAIM_IMPLEMENTED_BOUNDARY
    );
    assert_eq!(
        V08BroadAdapterRuntimeClaimState::ManualRequired.as_protocol_str(),
        proof::CLAIM_MANUAL_REQUIRED
    );
    assert_eq!(
        V08BroadAdapterRuntimeEvidenceState::CompositeRuntimeProof.as_protocol_str(),
        proof::EVIDENCE_COMPOSITE_RUNTIME_PROOF
    );
    assert_eq!(
        V08BroadAdapterRuntimeEvidenceState::NotImplemented.as_protocol_str(),
        proof::EVIDENCE_NOT_IMPLEMENTED
    );
}

#[test]
fn broad_adapter_runtime_read_model_serializes_honest_non_claims() {
    let read_model = V08BroadAdapterRuntimeProofReadModel {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: proof::READ_MODEL_ID.to_string(),
        generated_at: policy_constants::TEST_EVALUATED_AT.to_string(),
        source_read_model_ids: vec![proof::SOURCE_BROAD_OS_ADAPTER_PROOF.to_string()],
        entries: vec![
            V08BroadAdapterRuntimeProofEntry {
                schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
                proof_entry_id: proof::ENTRY_ID_OWNED_PROCESS_TIMER.to_string(),
                runtime_surface:
                    V08BroadAdapterRuntimeSurface::WindowsOwnedProcessAndTimerRuntimeBoundary,
                platform: ParentPlatform::Windows,
                product_claim_state: V08BroadAdapterRuntimeClaimState::ImplementedBoundary,
                evidence_state: V08BroadAdapterRuntimeEvidenceState::CompositeRuntimeProof,
                source_proof_ids: vec![proof::SOURCE_BROAD_OS_ADAPTER_PROOF.to_string()],
                linked_proof_commands: vec![proof::COMMAND_BROAD_OS_ADAPTER_PROOF.to_string()],
                linked_proof_artifacts: vec![proof::ARTIFACT_BROAD_OS_ADAPTER_PROOF.to_string()],
                manual_proof_requirements: vec![proof::REQUIREMENT_ROLLBACK.to_string()],
                claim_boundary: proof::CLAIM_NETWORK_DOMAIN_GATE.to_string(),
                fallback_behavior: proof::FALLBACK_NETWORK_DOMAIN_GATE.to_string(),
                broad_installed_app_blocking_claimed: false,
                network_domain_blocking_claimed: false,
                managed_browser_exact_url_claimed: false,
                unmanaged_browser_exact_evidence_claimed: false,
                unsupported_platform_claimed: false,
                mobile_privilege_claimed: false,
                last_checked_at: policy_constants::TEST_EVALUATED_AT.to_string(),
            },
            V08BroadAdapterRuntimeProofEntry {
                schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
                proof_entry_id: proof::ENTRY_ID_NETWORK_DOMAIN_GATE.to_string(),
                runtime_surface: V08BroadAdapterRuntimeSurface::WindowsNetworkDomainRuntimeGate,
                platform: ParentPlatform::Windows,
                product_claim_state: V08BroadAdapterRuntimeClaimState::ManualRequired,
                evidence_state: V08BroadAdapterRuntimeEvidenceState::ManualArtifactRequired,
                source_proof_ids: vec![proof::SOURCE_BROAD_OS_ADAPTER_PROOF.to_string()],
                linked_proof_commands: vec![proof::COMMAND_BROAD_OS_ADAPTER_PROOF.to_string()],
                linked_proof_artifacts: vec![proof::ARTIFACT_BROAD_OS_ADAPTER_PROOF.to_string()],
                manual_proof_requirements: vec![proof::REQUIREMENT_ROLLBACK.to_string()],
                claim_boundary: proof::CLAIM_NETWORK_DOMAIN_GATE.to_string(),
                fallback_behavior: proof::FALLBACK_NETWORK_DOMAIN_GATE.to_string(),
                broad_installed_app_blocking_claimed: false,
                network_domain_blocking_claimed: false,
                managed_browser_exact_url_claimed: false,
                unmanaged_browser_exact_evidence_claimed: false,
                unsupported_platform_claimed: false,
                mobile_privilege_claimed: false,
                last_checked_at: policy_constants::TEST_EVALUATED_AT.to_string(),
            },
            V08BroadAdapterRuntimeProofEntry {
                schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
                proof_entry_id: proof::ENTRY_ID_UNMANAGED_EXACT_EVIDENCE_GAP.to_string(),
                runtime_surface:
                    V08BroadAdapterRuntimeSurface::WindowsUnmanagedBrowserExactEvidenceRuntimeGap,
                platform: ParentPlatform::Windows,
                product_claim_state: V08BroadAdapterRuntimeClaimState::NotClaimed,
                evidence_state: V08BroadAdapterRuntimeEvidenceState::NotImplemented,
                source_proof_ids: vec![proof::SOURCE_BROAD_OS_ADAPTER_PROOF.to_string()],
                linked_proof_commands: vec![proof::COMMAND_BROAD_OS_ADAPTER_PROOF.to_string()],
                linked_proof_artifacts: vec![proof::ARTIFACT_BROAD_OS_ADAPTER_PROOF.to_string()],
                manual_proof_requirements: vec![proof::REQUIREMENT_ROLLBACK.to_string()],
                claim_boundary: proof::CLAIM_NETWORK_DOMAIN_GATE.to_string(),
                fallback_behavior: proof::FALLBACK_NETWORK_DOMAIN_GATE.to_string(),
                broad_installed_app_blocking_claimed: false,
                network_domain_blocking_claimed: false,
                managed_browser_exact_url_claimed: false,
                unmanaged_browser_exact_evidence_claimed: false,
                unsupported_platform_claimed: false,
                mobile_privilege_claimed: false,
                last_checked_at: policy_constants::TEST_EVALUATED_AT.to_string(),
            },
        ],
    };
    let reparsed = serde_json::from_value::<V08BroadAdapterRuntimeProofReadModel>(
        serde_json::to_value(read_model).expect_value(constants::error::AGENT_EVENT_SERIALIZES),
    )
    .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let claim_counts: BTreeMap<&'static str, usize> =
        reparsed
            .entries
            .iter()
            .fold(BTreeMap::new(), |mut counts, entry| {
                *counts
                    .entry(entry.product_claim_state.as_protocol_str())
                    .or_default() += 1;
                counts
            });

    assert_eq!(reparsed.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(claim_counts[proof::CLAIM_IMPLEMENTED_BOUNDARY], 1);
    assert_eq!(claim_counts[proof::CLAIM_MANUAL_REQUIRED], 1);
    assert_eq!(claim_counts[proof::CLAIM_NOT_CLAIMED], 1);
    assert_broad_adapter_runtime_honest_non_claims(&reparsed.entries);
}

fn assert_broad_adapter_runtime_honest_non_claims(entries: &[V08BroadAdapterRuntimeProofEntry]) {
    assert!(entries
        .iter()
        .all(|entry| !entry.broad_installed_app_blocking_claimed));
    assert!(entries
        .iter()
        .all(|entry| !entry.network_domain_blocking_claimed));
    assert!(entries
        .iter()
        .all(|entry| !entry.managed_browser_exact_url_claimed));
    assert!(entries
        .iter()
        .all(|entry| !entry.unmanaged_browser_exact_evidence_claimed));
    assert!(entries
        .iter()
        .all(|entry| !entry.unsupported_platform_claimed));
    assert!(entries.iter().all(|entry| !entry.mobile_privilege_claimed));
}
