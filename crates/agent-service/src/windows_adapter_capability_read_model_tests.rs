use std::collections::BTreeMap;

use ocentra_parent_agent_protocol::{
    constants::{
        self, enforcement, field, host_identity, windows_adapter_capability as windows_adapter,
    },
    policy_constants as policy, EnforcementReadinessState, ParentPlatform,
    WindowsAdapterCapabilityOutcome, WindowsAdapterCapabilitySurface,
};

use crate::windows_adapter_capability_read_model::windows_adapter_capability_proof;

#[test]
fn windows_adapter_capability_proof_links_app_domain_and_browser_boundaries() {
    let proof = windows_adapter_capability_proof(policy::TEST_EVALUATED_AT);
    let surface_counts = count_surfaces(&proof.entries);

    assert_eq!(proof.read_model_id, windows_adapter::READ_MODEL_ID_V0_8);
    assert_eq!(proof.entries.len(), 6);
    assert_eq!(surface_counts[windows_adapter::SURFACE_APP_TARGET], 1);
    assert_eq!(
        entry_for(&proof, WindowsAdapterCapabilitySurface::AppTarget).outcome,
        expected_manual_or_unavailable()
    );
    assert_eq!(
        entry_for(&proof, WindowsAdapterCapabilitySurface::DomainNetworkTarget)
            .linked_readiness_ids[0],
        enforcement::READINESS_ID_NETWORK_DOMAIN_BLOCKING
    );
    assert!(
        !entry_for(
            &proof,
            WindowsAdapterCapabilitySurface::ManagedBrowserTarget
        )
        .exact_url_claimed
    );
    assert!(
        entry_for(&proof, WindowsAdapterCapabilitySurface::AppTarget)
            .linked_host_identity_entry_ids
            .contains(&host_identity::ENTRY_ID_PACKAGE_IDENTITY.to_string())
    );
}

#[test]
fn windows_adapter_capability_proof_keeps_exact_url_and_broad_blocking_unclaimed() {
    let proof = windows_adapter_capability_proof(policy::TEST_EVALUATED_AT);
    let unmanaged = entry_for(
        &proof,
        WindowsAdapterCapabilitySurface::UnmanagedBrowserTarget,
    );
    let managed = entry_for(
        &proof,
        WindowsAdapterCapabilitySurface::ManagedBrowserTarget,
    );

    assert!(proof
        .entries
        .iter()
        .all(|entry| !entry.exact_url_claimed && !entry.broad_blocking_claimed));
    assert_eq!(
        managed.linked_readiness_ids,
        vec![
            enforcement::READINESS_ID_MANAGED_BROWSER_SERVICE_COMMAND.to_string(),
            enforcement::READINESS_ID_MANAGED_BROWSER_EXACT_URL.to_string()
        ]
    );
    assert_eq!(
        unmanaged.linked_readiness_ids,
        vec![
            enforcement::READINESS_ID_UNMANAGED_BROWSER_PROCESS_ONLY.to_string(),
            enforcement::READINESS_ID_UNMANAGED_BROWSER_EXACT_EVIDENCE.to_string()
        ]
    );
    assert_eq!(unmanaged.outcome, expected_unmanaged_outcome());
}

#[test]
fn windows_adapter_capability_proof_records_unsupported_os_and_rollback_audit_gates() {
    let proof = windows_adapter_capability_proof(policy::TEST_EVALUATED_AT);
    let unsupported = entry_for(&proof, WindowsAdapterCapabilitySurface::UnsupportedOsTarget);
    let rollback = entry_for(&proof, WindowsAdapterCapabilitySurface::RollbackAuditTarget);

    assert_eq!(unsupported.platform, ParentPlatform::Linux);
    assert_eq!(
        unsupported.readiness_state,
        EnforcementReadinessState::Unavailable
    );
    assert_eq!(
        unsupported.outcome,
        WindowsAdapterCapabilityOutcome::Unavailable
    );
    assert_eq!(
        rollback.linked_host_identity_entry_ids,
        vec![
            host_identity::ENTRY_ID_ROLLBACK_READINESS.to_string(),
            host_identity::ENTRY_ID_AUDIT_CUSTODY.to_string()
        ]
    );
    assert_eq!(
        rollback.required_artifacts,
        vec![windows_adapter::ARTIFACT_ROLLBACK_AUDIT.to_string()]
    );
}

#[test]
fn windows_adapter_capability_proof_serializes_for_runtime_preview() {
    let proof = windows_adapter_capability_proof(policy::TEST_EVALUATED_AT);
    let serialized = serde_json::to_value(proof).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized[field::READ_MODEL_ID],
        windows_adapter::READ_MODEL_ID_V0_8
    );
    let reparsed = serde_json::from_value::<
        ocentra_parent_agent_protocol::WindowsAdapterCapabilityProof,
    >(serialized)
    .expect(constants::error::AGENT_EVENT_SERIALIZES);
    assert_eq!(
        reparsed.entries[0].surface,
        WindowsAdapterCapabilitySurface::AppTarget
    );
    assert!(!reparsed.entries[0].broad_blocking_claimed);
    assert!(!reparsed.entries[2].exact_url_claimed);
    assert_eq!(
        reparsed.entries[4].outcome,
        WindowsAdapterCapabilityOutcome::Unavailable
    );
}

fn entry_for(
    proof: &ocentra_parent_agent_protocol::WindowsAdapterCapabilityProof,
    surface: WindowsAdapterCapabilitySurface,
) -> &ocentra_parent_agent_protocol::WindowsAdapterCapabilityProofEntry {
    proof
        .entries
        .iter()
        .find(|entry| entry.surface == surface)
        .expect(windows_adapter::READ_MODEL_ID_V0_8)
}

fn count_surfaces(
    entries: &[ocentra_parent_agent_protocol::WindowsAdapterCapabilityProofEntry],
) -> BTreeMap<&'static str, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts.entry(entry.surface.as_protocol_str()).or_default() += 1;
        counts
    })
}

#[cfg(windows)]
fn expected_manual_or_unavailable() -> WindowsAdapterCapabilityOutcome {
    WindowsAdapterCapabilityOutcome::ManualRequired
}

#[cfg(not(windows))]
fn expected_manual_or_unavailable() -> WindowsAdapterCapabilityOutcome {
    WindowsAdapterCapabilityOutcome::Unavailable
}

#[cfg(windows)]
fn expected_unmanaged_outcome() -> WindowsAdapterCapabilityOutcome {
    WindowsAdapterCapabilityOutcome::ProcessOnlyImplemented
}

#[cfg(not(windows))]
fn expected_unmanaged_outcome() -> WindowsAdapterCapabilityOutcome {
    WindowsAdapterCapabilityOutcome::Unavailable
}
