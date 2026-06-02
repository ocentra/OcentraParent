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
        serde_json::to_value(surfaces).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized
            .as_array()
            .expect(constants::error::AGENT_EVENT_SERIALIZES)
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
            entry(
                proof::ENTRY_ID_OWNED_PROCESS_TIMER,
                V08BroadAdapterRuntimeSurface::WindowsOwnedProcessAndTimerRuntimeBoundary,
                ParentPlatform::Windows,
                V08BroadAdapterRuntimeClaimState::ImplementedBoundary,
                V08BroadAdapterRuntimeEvidenceState::CompositeRuntimeProof,
            ),
            entry(
                proof::ENTRY_ID_NETWORK_DOMAIN_GATE,
                V08BroadAdapterRuntimeSurface::WindowsNetworkDomainRuntimeGate,
                ParentPlatform::Windows,
                V08BroadAdapterRuntimeClaimState::ManualRequired,
                V08BroadAdapterRuntimeEvidenceState::ManualArtifactRequired,
            ),
            entry(
                proof::ENTRY_ID_UNMANAGED_EXACT_EVIDENCE_GAP,
                V08BroadAdapterRuntimeSurface::WindowsUnmanagedBrowserExactEvidenceRuntimeGap,
                ParentPlatform::Windows,
                V08BroadAdapterRuntimeClaimState::NotClaimed,
                V08BroadAdapterRuntimeEvidenceState::NotImplemented,
            ),
        ],
    };
    let reparsed = serde_json::from_value::<V08BroadAdapterRuntimeProofReadModel>(
        serde_json::to_value(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
    )
    .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let claim_counts = count_claim_states(&reparsed.entries);

    assert_eq!(reparsed.read_model_id, proof::READ_MODEL_ID);
    assert_eq!(claim_counts[proof::CLAIM_IMPLEMENTED_BOUNDARY], 1);
    assert_eq!(claim_counts[proof::CLAIM_MANUAL_REQUIRED], 1);
    assert_eq!(claim_counts[proof::CLAIM_NOT_CLAIMED], 1);
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
        .all(|entry| !entry.managed_browser_exact_url_claimed));
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.unmanaged_browser_exact_evidence_claimed));
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.unsupported_platform_claimed));
    assert!(reparsed
        .entries
        .iter()
        .all(|entry| !entry.mobile_privilege_claimed));
}

fn entry(
    proof_entry_id: &str,
    runtime_surface: V08BroadAdapterRuntimeSurface,
    platform: ParentPlatform,
    product_claim_state: V08BroadAdapterRuntimeClaimState,
    evidence_state: V08BroadAdapterRuntimeEvidenceState,
) -> V08BroadAdapterRuntimeProofEntry {
    V08BroadAdapterRuntimeProofEntry {
        schema_version: policy_constants::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        proof_entry_id: proof_entry_id.to_string(),
        runtime_surface,
        platform,
        product_claim_state,
        evidence_state,
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
    }
}

fn count_claim_states(
    entries: &[V08BroadAdapterRuntimeProofEntry],
) -> BTreeMap<&'static str, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(entry.product_claim_state.as_protocol_str())
            .or_default() += 1;
        counts
    })
}
