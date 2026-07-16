use std::{collections::BTreeMap, error::Error, io::Error as IoError};

use ocentra_parent_agent_protocol::constants::{
    enforcement, field, host_identity, windows_adapter_capability as windows_adapter,
};
use ocentra_parent_agent_protocol::enforcement::ParentPlatform;
use ocentra_parent_agent_protocol::enforcement_readiness::EnforcementReadinessState;
use ocentra_parent_agent_protocol::policy_constants as policy;
use ocentra_parent_agent_protocol::windows_adapter_capability::{
    WindowsAdapterCapabilityOutcome, WindowsAdapterCapabilitySurface,
};

use super::test_text::{count_for_display, TestText};
use crate::host_identity_read_model::GeneratedAtText;
use crate::windows_adapter_capability_read_model::windows_adapter_capability_proof;

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn windows_adapter_capability_proof_links_app_domain_and_browser_boundaries() -> TestResult {
    let proof =
        windows_adapter_capability_proof(GeneratedAtText(policy::TEST_EVALUATED_AT.to_string()));
    let surface_counts = count_surfaces(&proof.entries);

    assert_eq!(proof.read_model_id, windows_adapter::READ_MODEL_ID_V0_8);
    assert_eq!(proof.entries.len(), 6);
    assert_eq!(
        count_for_display(&surface_counts, windows_adapter::SURFACE_APP_TARGET),
        1
    );
    assert_eq!(
        entry_for(&proof, WindowsAdapterCapabilitySurface::AppTarget)?.outcome,
        expected_manual_or_unavailable()
    );
    assert_eq!(
        entry_for(&proof, WindowsAdapterCapabilitySurface::DomainNetworkTarget)?
            .linked_readiness_ids[0],
        enforcement::READINESS_ID_NETWORK_DOMAIN_BLOCKING
    );
    assert!(
        !entry_for(
            &proof,
            WindowsAdapterCapabilitySurface::ManagedBrowserTarget
        )?
        .exact_url_claimed
    );
    assert!(
        entry_for(&proof, WindowsAdapterCapabilitySurface::AppTarget)?
            .linked_host_identity_entry_ids
            .contains(&host_identity::ENTRY_ID_PACKAGE_IDENTITY.to_string())
    );

    Ok(())
}

#[test]
fn windows_adapter_capability_proof_keeps_exact_url_and_broad_blocking_unclaimed() -> TestResult {
    let proof =
        windows_adapter_capability_proof(GeneratedAtText(policy::TEST_EVALUATED_AT.to_string()));
    let unmanaged = entry_for(
        &proof,
        WindowsAdapterCapabilitySurface::UnmanagedBrowserTarget,
    )?;
    let managed = entry_for(
        &proof,
        WindowsAdapterCapabilitySurface::ManagedBrowserTarget,
    )?;

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

    Ok(())
}

#[test]
fn windows_adapter_capability_proof_records_unsupported_os_and_rollback_audit_gates() -> TestResult
{
    let proof =
        windows_adapter_capability_proof(GeneratedAtText(policy::TEST_EVALUATED_AT.to_string()));
    let unsupported = entry_for(&proof, WindowsAdapterCapabilitySurface::UnsupportedOsTarget)?;
    let rollback = entry_for(&proof, WindowsAdapterCapabilitySurface::RollbackAuditTarget)?;

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

    Ok(())
}

#[test]
fn windows_adapter_capability_proof_serializes_for_runtime_preview() -> TestResult {
    let proof =
        windows_adapter_capability_proof(GeneratedAtText(policy::TEST_EVALUATED_AT.to_string()));
    let serialized = serde_json::to_value(proof)?;

    assert_eq!(
        serialized[field::READ_MODEL_ID],
        windows_adapter::READ_MODEL_ID_V0_8
    );
    let reparsed = serde_json::from_value::<
        ocentra_parent_agent_protocol::windows_adapter_capability::WindowsAdapterCapabilityProof,
    >(serialized)?;
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

    Ok(())
}

fn entry_for(
    proof: &ocentra_parent_agent_protocol::windows_adapter_capability::WindowsAdapterCapabilityProof,
    surface: WindowsAdapterCapabilitySurface,
) -> Result<
    &ocentra_parent_agent_protocol::windows_adapter_capability::WindowsAdapterCapabilityProofEntry,
    IoError,
> {
    proof
        .entries
        .iter()
        .find(|entry| entry.surface == surface)
        .ok_or_else(|| IoError::other(windows_adapter::READ_MODEL_ID_V0_8))
}

fn count_surfaces(
    entries: &[ocentra_parent_agent_protocol::windows_adapter_capability::WindowsAdapterCapabilityProofEntry],
) -> BTreeMap<TestText, usize> {
    entries.iter().fold(BTreeMap::new(), |mut counts, entry| {
        *counts
            .entry(TestText::from_display(entry.surface.as_protocol_str()))
            .or_default() += 1;
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
