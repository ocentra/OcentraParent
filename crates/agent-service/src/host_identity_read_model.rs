use ocentra_parent_agent_protocol::{
    constants::host_identity, policy_constants as policy, EnforcementAdapterKind,
    EnforcementBroadAdapterCapability, EnforcementCapabilityState, EnforcementReadinessProofLevel,
    EnforcementReadinessRuntimeOwner, EnforcementReadinessState, HostIdentityEvidenceClass,
    HostIdentityEvidenceKind, HostIdentityReadModel, HostIdentityReadModelEntry, ParentPlatform,
};

pub(crate) fn host_identity_read_model(generated_at: &str) -> HostIdentityReadModel {
    HostIdentityReadModel {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: host_identity::READ_MODEL_ID_V0_8.to_string(),
        generated_at: generated_at.to_string(),
        platform: ParentPlatform::Windows,
        entries: host_identity_entries(generated_at),
    }
}

fn host_identity_entries(generated_at: &str) -> Vec<HostIdentityReadModelEntry> {
    let mut entries = Vec::new();
    entries.extend(inventory_and_process_entries(generated_at));
    entries.extend(package_and_trust_entries(generated_at));
    entries.extend(fallback_and_custody_entries(generated_at));
    entries
}

fn inventory_and_process_entries(generated_at: &str) -> Vec<HostIdentityReadModelEntry> {
    vec![
        manual_required_entry(
            host_identity::ENTRY_ID_INSTALLED_APP_INVENTORY,
            HostIdentityEvidenceKind::InstalledAppInventory,
            HostIdentityEvidenceClass::Inventory,
            host_identity::REQUIREMENT_INSTALLED_APP_INVENTORY,
            host_identity::ARTIFACTS_INSTALLED_APP_INVENTORY,
            host_identity::SIGNALS_INSTALLED_APP_INVENTORY,
            host_identity::FALLBACK_INSTALLED_APP_INVENTORY,
            generated_at,
        ),
        manual_required_entry(
            host_identity::ENTRY_ID_PROCESS_LINEAGE,
            HostIdentityEvidenceKind::ProcessLineage,
            HostIdentityEvidenceClass::Process,
            host_identity::REQUIREMENT_PROCESS_LINEAGE,
            host_identity::ARTIFACTS_PROCESS_LINEAGE,
            host_identity::SIGNALS_PROCESS_LINEAGE,
            host_identity::FALLBACK_PROCESS_LINEAGE,
            generated_at,
        ),
        manual_required_entry(
            host_identity::ENTRY_ID_EXECUTABLE_IDENTITY,
            HostIdentityEvidenceKind::ExecutableIdentity,
            HostIdentityEvidenceClass::Executable,
            host_identity::REQUIREMENT_EXECUTABLE_IDENTITY,
            host_identity::ARTIFACTS_EXECUTABLE_IDENTITY,
            host_identity::SIGNALS_EXECUTABLE_IDENTITY,
            host_identity::FALLBACK_EXECUTABLE_IDENTITY,
            generated_at,
        ),
    ]
}

fn package_and_trust_entries(generated_at: &str) -> Vec<HostIdentityReadModelEntry> {
    vec![
        manual_required_entry(
            host_identity::ENTRY_ID_PACKAGE_IDENTITY,
            HostIdentityEvidenceKind::PackageIdentity,
            HostIdentityEvidenceClass::Package,
            host_identity::REQUIREMENT_PACKAGE_IDENTITY,
            host_identity::ARTIFACTS_PACKAGE_IDENTITY,
            host_identity::SIGNALS_PACKAGE_IDENTITY,
            host_identity::FALLBACK_PACKAGE_IDENTITY,
            generated_at,
        ),
        manual_required_entry(
            host_identity::ENTRY_ID_PUBLISHER_SIGNATURE,
            HostIdentityEvidenceKind::PublisherSignature,
            HostIdentityEvidenceClass::PublisherSignature,
            host_identity::REQUIREMENT_PUBLISHER_SIGNATURE,
            host_identity::ARTIFACTS_PUBLISHER_SIGNATURE,
            host_identity::SIGNALS_PUBLISHER_SIGNATURE,
            host_identity::FALLBACK_PUBLISHER_SIGNATURE,
            generated_at,
        ),
        manual_required_entry(
            host_identity::ENTRY_ID_INVENTORY_PROCESS_LINK,
            HostIdentityEvidenceKind::InventoryProcessLink,
            HostIdentityEvidenceClass::Inventory,
            host_identity::REQUIREMENT_INVENTORY_PROCESS_LINK,
            host_identity::ARTIFACTS_INVENTORY_PROCESS_LINK,
            host_identity::SIGNALS_INVENTORY_PROCESS_LINK,
            host_identity::FALLBACK_INVENTORY_PROCESS_LINK,
            generated_at,
        ),
    ]
}

fn fallback_and_custody_entries(generated_at: &str) -> Vec<HostIdentityReadModelEntry> {
    vec![
        unavailable_entry(
            host_identity::ENTRY_ID_UNSUPPORTED_IDENTITY,
            HostIdentityEvidenceKind::UnsupportedIdentity,
            HostIdentityEvidenceClass::Package,
            host_identity::REQUIREMENT_UNSUPPORTED_IDENTITY,
            host_identity::ARTIFACTS_UNSUPPORTED_IDENTITY,
            host_identity::SIGNALS_UNSUPPORTED_IDENTITY,
            host_identity::FALLBACK_UNSUPPORTED_IDENTITY,
            generated_at,
        ),
        not_claimed_entry(
            host_identity::ENTRY_ID_ROLLBACK_READINESS,
            HostIdentityEvidenceKind::RollbackReadiness,
            HostIdentityEvidenceClass::Rollback,
            host_identity::REQUIREMENT_ROLLBACK_READINESS,
            host_identity::ARTIFACTS_ROLLBACK_READINESS,
            host_identity::SIGNALS_ROLLBACK_READINESS,
            host_identity::FALLBACK_ROLLBACK_READINESS,
            generated_at,
        ),
        manual_required_entry(
            host_identity::ENTRY_ID_AUDIT_CUSTODY,
            HostIdentityEvidenceKind::AuditCustody,
            HostIdentityEvidenceClass::Audit,
            host_identity::REQUIREMENT_AUDIT_CUSTODY,
            host_identity::ARTIFACTS_AUDIT_CUSTODY,
            host_identity::SIGNALS_AUDIT_CUSTODY,
            host_identity::FALLBACK_AUDIT_CUSTODY,
            generated_at,
        ),
    ]
}

fn manual_required_entry(
    read_model_entry_id: &str,
    evidence_kind: HostIdentityEvidenceKind,
    evidence_class: HostIdentityEvidenceClass,
    host_evidence_requirement: &str,
    required_evidence_artifacts: &[&str],
    acceptance_signals: &[&str],
    fallback_behavior: &str,
    last_checked_at: &str,
) -> HostIdentityReadModelEntry {
    host_identity_entry(
        read_model_entry_id,
        evidence_kind,
        evidence_class,
        EnforcementCapabilityState::ManualRequired,
        EnforcementReadinessState::ManualRequired,
        EnforcementReadinessProofLevel::ManualProofRequired,
        EnforcementReadinessRuntimeOwner::ManualProof,
        host_evidence_requirement,
        required_evidence_artifacts,
        acceptance_signals,
        fallback_behavior,
        last_checked_at,
    )
}

fn unavailable_entry(
    read_model_entry_id: &str,
    evidence_kind: HostIdentityEvidenceKind,
    evidence_class: HostIdentityEvidenceClass,
    host_evidence_requirement: &str,
    required_evidence_artifacts: &[&str],
    acceptance_signals: &[&str],
    fallback_behavior: &str,
    last_checked_at: &str,
) -> HostIdentityReadModelEntry {
    host_identity_entry(
        read_model_entry_id,
        evidence_kind,
        evidence_class,
        EnforcementCapabilityState::Unavailable,
        EnforcementReadinessState::Unavailable,
        EnforcementReadinessProofLevel::ManualProofRequired,
        EnforcementReadinessRuntimeOwner::ManualProof,
        host_evidence_requirement,
        required_evidence_artifacts,
        acceptance_signals,
        fallback_behavior,
        last_checked_at,
    )
}

fn not_claimed_entry(
    read_model_entry_id: &str,
    evidence_kind: HostIdentityEvidenceKind,
    evidence_class: HostIdentityEvidenceClass,
    host_evidence_requirement: &str,
    required_evidence_artifacts: &[&str],
    acceptance_signals: &[&str],
    fallback_behavior: &str,
    last_checked_at: &str,
) -> HostIdentityReadModelEntry {
    host_identity_entry(
        read_model_entry_id,
        evidence_kind,
        evidence_class,
        EnforcementCapabilityState::ManualRequired,
        EnforcementReadinessState::NotClaimed,
        EnforcementReadinessProofLevel::NotProved,
        EnforcementReadinessRuntimeOwner::NotImplemented,
        host_evidence_requirement,
        required_evidence_artifacts,
        acceptance_signals,
        fallback_behavior,
        last_checked_at,
    )
}

fn host_identity_entry(
    read_model_entry_id: &str,
    evidence_kind: HostIdentityEvidenceKind,
    evidence_class: HostIdentityEvidenceClass,
    capability_state: EnforcementCapabilityState,
    readiness_state: EnforcementReadinessState,
    proof_level: EnforcementReadinessProofLevel,
    runtime_owner: EnforcementReadinessRuntimeOwner,
    host_evidence_requirement: &str,
    required_evidence_artifacts: &[&str],
    acceptance_signals: &[&str],
    fallback_behavior: &str,
    last_checked_at: &str,
) -> HostIdentityReadModelEntry {
    HostIdentityReadModelEntry {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_entry_id: read_model_entry_id.to_string(),
        evidence_kind,
        evidence_class,
        capability: EnforcementBroadAdapterCapability::BroadAppBlocking,
        platform: ParentPlatform::Windows,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        capability_state,
        readiness_state,
        proof_level,
        runtime_owner,
        host_evidence_requirement: host_evidence_requirement.to_string(),
        required_evidence_artifacts: strings(required_evidence_artifacts),
        acceptance_signals: strings(acceptance_signals),
        fallback_behavior: fallback_behavior.to_string(),
        safe_for_broad_app_blocking: false,
        last_checked_at: last_checked_at.to_string(),
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
