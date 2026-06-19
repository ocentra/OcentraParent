use ocentra_parent_agent_core::enforcement_readiness::broad_os_adapter_readiness;
use ocentra_parent_agent_protocol::{
    constants::{enforcement, host_identity, windows_adapter_capability as windows_adapter},
    policy_constants as policy, EnforcementAdapterKind, EnforcementBroadAdapterCapability,
    EnforcementBroadAdapterReadinessEntry, EnforcementBroadOsAdapterReadinessMatrix,
    EnforcementCapabilityState, EnforcementMode, EnforcementReadinessProofLevel,
    EnforcementReadinessRuntimeOwner, EnforcementReadinessState, HostIdentityEvidenceKind,
    HostIdentityReadModel, ParentPlatform, WindowsAdapterCapabilityOutcome,
    WindowsAdapterCapabilityProof, WindowsAdapterCapabilityProofEntry,
    WindowsAdapterCapabilitySurface,
};

use crate::host_identity_read_model::host_identity_read_model;

pub(crate) fn windows_adapter_capability_proof(
    generated_at: &str,
) -> WindowsAdapterCapabilityProof {
    let readiness = broad_os_adapter_readiness(generated_at);
    let host_identity_model = host_identity_read_model(generated_at);

    WindowsAdapterCapabilityProof {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: windows_adapter::READ_MODEL_ID_V0_8.to_string(),
        generated_at: generated_at.to_string(),
        platform: ParentPlatform::Windows,
        entries: vec![
            app_target_entry(&readiness, &host_identity_model, generated_at),
            domain_network_target_entry(&readiness, generated_at),
            managed_browser_target_entry(&readiness, generated_at),
            unmanaged_browser_target_entry(&readiness, generated_at),
            unsupported_os_target_entry(generated_at),
            rollback_audit_target_entry(&readiness, &host_identity_model, generated_at),
        ],
    }
}

struct CapabilityProofSpec {
    proof_entry_id: &'static str,
    surface: WindowsAdapterCapabilitySurface,
    linked_readiness_ids: Vec<String>,
    linked_host_identity_entry_ids: Vec<String>,
    outcome: WindowsAdapterCapabilityOutcome,
    claim_boundary: &'static str,
    fallback_behavior: &'static str,
    exact_url_claimed: bool,
    broad_blocking_claimed: bool,
    required_artifacts: &'static [&'static str],
}

fn app_target_entry(
    readiness: &EnforcementBroadOsAdapterReadinessMatrix,
    host_identity_model: &HostIdentityReadModel,
    generated_at: &str,
) -> WindowsAdapterCapabilityProofEntry {
    let primary = readiness_entry(
        readiness,
        EnforcementBroadAdapterCapability::BroadAppBlocking,
    );
    let host_entries = host_identity_entries(
        host_identity_model,
        &[
            HostIdentityEvidenceKind::InstalledAppInventory,
            HostIdentityEvidenceKind::ProcessLineage,
            HostIdentityEvidenceKind::ExecutableIdentity,
            HostIdentityEvidenceKind::PackageIdentity,
            HostIdentityEvidenceKind::PublisherSignature,
            HostIdentityEvidenceKind::InventoryProcessLink,
        ],
    );

    proof_entry(
        primary,
        CapabilityProofSpec {
            proof_entry_id: windows_adapter::ENTRY_ID_APP_TARGET,
            surface: WindowsAdapterCapabilitySurface::AppTarget,
            linked_readiness_ids: vec![primary.readiness_id.clone()],
            linked_host_identity_entry_ids: host_entries,
            outcome: manual_or_unavailable_outcome(primary.readiness_state),
            claim_boundary: windows_adapter::CLAIM_BOUNDARY_APP_TARGET,
            fallback_behavior: windows_adapter::FALLBACK_APP_TARGET,
            exact_url_claimed: false,
            broad_blocking_claimed: false,
            required_artifacts: &[windows_adapter::ARTIFACT_WINDOWS_APP_IDENTITY],
        },
        generated_at,
    )
}

fn domain_network_target_entry(
    readiness: &EnforcementBroadOsAdapterReadinessMatrix,
    generated_at: &str,
) -> WindowsAdapterCapabilityProofEntry {
    let primary = readiness_entry(
        readiness,
        EnforcementBroadAdapterCapability::NetworkDomainBlocking,
    );

    proof_entry(
        primary,
        CapabilityProofSpec {
            proof_entry_id: windows_adapter::ENTRY_ID_DOMAIN_NETWORK_TARGET,
            surface: WindowsAdapterCapabilitySurface::DomainNetworkTarget,
            linked_readiness_ids: vec![primary.readiness_id.clone()],
            linked_host_identity_entry_ids: Vec::new(),
            outcome: manual_or_unavailable_outcome(primary.readiness_state),
            claim_boundary: windows_adapter::CLAIM_BOUNDARY_DOMAIN_NETWORK_TARGET,
            fallback_behavior: windows_adapter::FALLBACK_DOMAIN_NETWORK_TARGET,
            exact_url_claimed: false,
            broad_blocking_claimed: false,
            required_artifacts: &[windows_adapter::ARTIFACT_WINDOWS_DOMAIN_FILTER],
        },
        generated_at,
    )
}

fn managed_browser_target_entry(
    readiness: &EnforcementBroadOsAdapterReadinessMatrix,
    generated_at: &str,
) -> WindowsAdapterCapabilityProofEntry {
    let primary = readiness_entry(
        readiness,
        EnforcementBroadAdapterCapability::ManagedBrowserServiceCommand,
    );
    let exact_url = readiness_entry(
        readiness,
        EnforcementBroadAdapterCapability::ManagedBrowserExactUrlControl,
    );

    proof_entry(
        primary,
        CapabilityProofSpec {
            proof_entry_id: windows_adapter::ENTRY_ID_MANAGED_BROWSER_TARGET,
            surface: WindowsAdapterCapabilitySurface::ManagedBrowserTarget,
            linked_readiness_ids: vec![
                primary.readiness_id.clone(),
                exact_url.readiness_id.clone(),
            ],
            linked_host_identity_entry_ids: Vec::new(),
            outcome: manual_or_unavailable_outcome(primary.readiness_state),
            claim_boundary: windows_adapter::CLAIM_BOUNDARY_MANAGED_BROWSER_TARGET,
            fallback_behavior: windows_adapter::FALLBACK_MANAGED_BROWSER_TARGET,
            exact_url_claimed: false,
            broad_blocking_claimed: false,
            required_artifacts: &[windows_adapter::ARTIFACT_WINDOWS_MANAGED_BROWSER],
        },
        generated_at,
    )
}

fn unmanaged_browser_target_entry(
    readiness: &EnforcementBroadOsAdapterReadinessMatrix,
    generated_at: &str,
) -> WindowsAdapterCapabilityProofEntry {
    let primary = readiness_entry(
        readiness,
        EnforcementBroadAdapterCapability::UnmanagedBrowserProcessOnly,
    );
    let exact_evidence = readiness_entry(
        readiness,
        EnforcementBroadAdapterCapability::UnmanagedBrowserExactEvidence,
    );

    proof_entry(
        primary,
        CapabilityProofSpec {
            proof_entry_id: windows_adapter::ENTRY_ID_UNMANAGED_BROWSER_TARGET,
            surface: WindowsAdapterCapabilitySurface::UnmanagedBrowserTarget,
            linked_readiness_ids: vec![
                primary.readiness_id.clone(),
                exact_evidence.readiness_id.clone(),
            ],
            linked_host_identity_entry_ids: Vec::new(),
            outcome: unmanaged_outcome(primary.readiness_state),
            claim_boundary: windows_adapter::CLAIM_BOUNDARY_UNMANAGED_BROWSER_TARGET,
            fallback_behavior: windows_adapter::FALLBACK_UNMANAGED_BROWSER_TARGET,
            exact_url_claimed: false,
            broad_blocking_claimed: false,
            required_artifacts: artifacts_for_process_only(primary.readiness_state),
        },
        generated_at,
    )
}

fn unsupported_os_target_entry(generated_at: &str) -> WindowsAdapterCapabilityProofEntry {
    WindowsAdapterCapabilityProofEntry {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        proof_entry_id: windows_adapter::ENTRY_ID_UNSUPPORTED_OS_TARGET.to_string(),
        surface: WindowsAdapterCapabilitySurface::UnsupportedOsTarget,
        platform: ParentPlatform::Linux,
        primary_capability: EnforcementBroadAdapterCapability::BroadAppBlocking,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        capability_state: EnforcementCapabilityState::Unavailable,
        readiness_state: EnforcementReadinessState::Unavailable,
        proof_level: EnforcementReadinessProofLevel::ManualProofRequired,
        runtime_owner: EnforcementReadinessRuntimeOwner::ManualProof,
        supported_modes: Vec::new(),
        linked_readiness_ids: vec![enforcement::READINESS_ID_BROAD_APP_BLOCKING.to_string()],
        linked_host_identity_entry_ids: vec![
            host_identity::ENTRY_ID_UNSUPPORTED_IDENTITY.to_string()
        ],
        outcome: WindowsAdapterCapabilityOutcome::Unavailable,
        claim_boundary: windows_adapter::CLAIM_BOUNDARY_UNSUPPORTED_OS_TARGET.to_string(),
        fallback_behavior: windows_adapter::FALLBACK_UNSUPPORTED_OS_TARGET.to_string(),
        exact_url_claimed: false,
        broad_blocking_claimed: false,
        required_artifacts: vec![windows_adapter::ARTIFACT_UNSUPPORTED_OS.to_string()],
        last_checked_at: generated_at.to_string(),
    }
}

fn rollback_audit_target_entry(
    readiness: &EnforcementBroadOsAdapterReadinessMatrix,
    host_identity_model: &HostIdentityReadModel,
    generated_at: &str,
) -> WindowsAdapterCapabilityProofEntry {
    let primary = readiness_entry(
        readiness,
        EnforcementBroadAdapterCapability::AdminAntiTamperRollback,
    );
    let host_entries = host_identity_entries(
        host_identity_model,
        &[
            HostIdentityEvidenceKind::RollbackReadiness,
            HostIdentityEvidenceKind::AuditCustody,
        ],
    );

    proof_entry(
        primary,
        CapabilityProofSpec {
            proof_entry_id: windows_adapter::ENTRY_ID_ROLLBACK_AUDIT_TARGET,
            surface: WindowsAdapterCapabilitySurface::RollbackAuditTarget,
            linked_readiness_ids: vec![primary.readiness_id.clone()],
            linked_host_identity_entry_ids: host_entries,
            outcome: manual_or_unavailable_outcome(primary.readiness_state),
            claim_boundary: windows_adapter::CLAIM_BOUNDARY_ROLLBACK_AUDIT_TARGET,
            fallback_behavior: windows_adapter::FALLBACK_ROLLBACK_AUDIT_TARGET,
            exact_url_claimed: false,
            broad_blocking_claimed: false,
            required_artifacts: &[windows_adapter::ARTIFACT_ROLLBACK_AUDIT],
        },
        generated_at,
    )
}

fn proof_entry(
    primary: &EnforcementBroadAdapterReadinessEntry,
    spec: CapabilityProofSpec,
    generated_at: &str,
) -> WindowsAdapterCapabilityProofEntry {
    WindowsAdapterCapabilityProofEntry {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        proof_entry_id: spec.proof_entry_id.to_string(),
        surface: spec.surface,
        platform: primary.platform,
        primary_capability: primary.capability,
        adapter_kind: primary.adapter_kind,
        capability_state: primary.capability_state,
        readiness_state: primary.readiness_state,
        proof_level: primary.proof_level,
        runtime_owner: primary.runtime_owner,
        supported_modes: spec_supported_modes(primary),
        linked_readiness_ids: spec.linked_readiness_ids,
        linked_host_identity_entry_ids: spec.linked_host_identity_entry_ids,
        outcome: spec.outcome,
        claim_boundary: spec.claim_boundary.to_string(),
        fallback_behavior: spec.fallback_behavior.to_string(),
        exact_url_claimed: spec.exact_url_claimed,
        broad_blocking_claimed: spec.broad_blocking_claimed,
        required_artifacts: strings(spec.required_artifacts),
        last_checked_at: generated_at.to_string(),
    }
}

fn readiness_entry(
    readiness: &EnforcementBroadOsAdapterReadinessMatrix,
    capability: EnforcementBroadAdapterCapability,
) -> &EnforcementBroadAdapterReadinessEntry {
    readiness
        .entries
        .iter()
        .find(|entry| entry.capability == capability)
        .expect(enforcement::READINESS_MATRIX_ID_V0_8_BROAD_OS_ADAPTER)
}

fn host_identity_entries(
    model: &HostIdentityReadModel,
    evidence_kinds: &[HostIdentityEvidenceKind],
) -> Vec<String> {
    evidence_kinds
        .iter()
        .map(|evidence_kind| {
            model
                .entries
                .iter()
                .find(|entry| entry.evidence_kind == *evidence_kind)
                .map(|entry| entry.read_model_entry_id.clone())
                .expect(host_identity::READ_MODEL_ID_V0_8)
        })
        .collect()
}

fn spec_supported_modes(primary: &EnforcementBroadAdapterReadinessEntry) -> Vec<EnforcementMode> {
    primary.supported_modes.clone()
}

fn manual_or_unavailable_outcome(
    readiness_state: EnforcementReadinessState,
) -> WindowsAdapterCapabilityOutcome {
    if readiness_state == EnforcementReadinessState::Unavailable {
        WindowsAdapterCapabilityOutcome::Unavailable
    } else {
        WindowsAdapterCapabilityOutcome::ManualRequired
    }
}

fn unmanaged_outcome(
    readiness_state: EnforcementReadinessState,
) -> WindowsAdapterCapabilityOutcome {
    if readiness_state == EnforcementReadinessState::Implemented {
        WindowsAdapterCapabilityOutcome::ProcessOnlyImplemented
    } else {
        WindowsAdapterCapabilityOutcome::Unavailable
    }
}

fn artifacts_for_process_only(
    readiness_state: EnforcementReadinessState,
) -> &'static [&'static str] {
    if readiness_state == EnforcementReadinessState::Implemented {
        &[]
    } else {
        &[windows_adapter::ARTIFACT_WINDOWS_UNMANAGED_BROWSER]
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
