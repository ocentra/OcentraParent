use ocentra_parent_agent_protocol::constants::host_identity;
use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAdapterKind, EnforcementCapabilityState, ParentPlatform,
};
use ocentra_parent_agent_protocol::enforcement_readiness::{
    EnforcementBroadAdapterCapability, EnforcementReadinessProofLevel,
    EnforcementReadinessRuntimeOwner, EnforcementReadinessState,
};
use ocentra_parent_agent_protocol::host_identity::{
    HostIdentityEvidenceClass, HostIdentityEvidenceKind, HostIdentityReadModel,
    HostIdentityReadModelEntry,
};
use ocentra_parent_agent_protocol::policy_constants as policy;

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
            &entry_spec(
                host_identity::ENTRY_ID_INSTALLED_APP_INVENTORY,
                HostIdentityEvidenceKind::InstalledAppInventory,
                HostIdentityEvidenceClass::Inventory,
                host_identity::REQUIREMENT_INSTALLED_APP_INVENTORY,
                host_identity::ARTIFACTS_INSTALLED_APP_INVENTORY,
                host_identity::SIGNALS_INSTALLED_APP_INVENTORY,
                host_identity::FALLBACK_INSTALLED_APP_INVENTORY,
            ),
            generated_at,
        ),
        manual_required_entry(
            &entry_spec(
                host_identity::ENTRY_ID_PROCESS_LINEAGE,
                HostIdentityEvidenceKind::ProcessLineage,
                HostIdentityEvidenceClass::Process,
                host_identity::REQUIREMENT_PROCESS_LINEAGE,
                host_identity::ARTIFACTS_PROCESS_LINEAGE,
                host_identity::SIGNALS_PROCESS_LINEAGE,
                host_identity::FALLBACK_PROCESS_LINEAGE,
            ),
            generated_at,
        ),
        manual_required_entry(
            &entry_spec(
                host_identity::ENTRY_ID_EXECUTABLE_IDENTITY,
                HostIdentityEvidenceKind::ExecutableIdentity,
                HostIdentityEvidenceClass::Executable,
                host_identity::REQUIREMENT_EXECUTABLE_IDENTITY,
                host_identity::ARTIFACTS_EXECUTABLE_IDENTITY,
                host_identity::SIGNALS_EXECUTABLE_IDENTITY,
                host_identity::FALLBACK_EXECUTABLE_IDENTITY,
            ),
            generated_at,
        ),
    ]
}

fn package_and_trust_entries(generated_at: &str) -> Vec<HostIdentityReadModelEntry> {
    vec![
        manual_required_entry(
            &entry_spec(
                host_identity::ENTRY_ID_PACKAGE_IDENTITY,
                HostIdentityEvidenceKind::PackageIdentity,
                HostIdentityEvidenceClass::Package,
                host_identity::REQUIREMENT_PACKAGE_IDENTITY,
                host_identity::ARTIFACTS_PACKAGE_IDENTITY,
                host_identity::SIGNALS_PACKAGE_IDENTITY,
                host_identity::FALLBACK_PACKAGE_IDENTITY,
            ),
            generated_at,
        ),
        manual_required_entry(
            &entry_spec(
                host_identity::ENTRY_ID_PUBLISHER_SIGNATURE,
                HostIdentityEvidenceKind::PublisherSignature,
                HostIdentityEvidenceClass::PublisherSignature,
                host_identity::REQUIREMENT_PUBLISHER_SIGNATURE,
                host_identity::ARTIFACTS_PUBLISHER_SIGNATURE,
                host_identity::SIGNALS_PUBLISHER_SIGNATURE,
                host_identity::FALLBACK_PUBLISHER_SIGNATURE,
            ),
            generated_at,
        ),
        manual_required_entry(
            &entry_spec(
                host_identity::ENTRY_ID_INVENTORY_PROCESS_LINK,
                HostIdentityEvidenceKind::InventoryProcessLink,
                HostIdentityEvidenceClass::Inventory,
                host_identity::REQUIREMENT_INVENTORY_PROCESS_LINK,
                host_identity::ARTIFACTS_INVENTORY_PROCESS_LINK,
                host_identity::SIGNALS_INVENTORY_PROCESS_LINK,
                host_identity::FALLBACK_INVENTORY_PROCESS_LINK,
            ),
            generated_at,
        ),
    ]
}

fn fallback_and_custody_entries(generated_at: &str) -> Vec<HostIdentityReadModelEntry> {
    vec![
        unavailable_entry(
            &entry_spec(
                host_identity::ENTRY_ID_UNSUPPORTED_IDENTITY,
                HostIdentityEvidenceKind::UnsupportedIdentity,
                HostIdentityEvidenceClass::Package,
                host_identity::REQUIREMENT_UNSUPPORTED_IDENTITY,
                host_identity::ARTIFACTS_UNSUPPORTED_IDENTITY,
                host_identity::SIGNALS_UNSUPPORTED_IDENTITY,
                host_identity::FALLBACK_UNSUPPORTED_IDENTITY,
            ),
            generated_at,
        ),
        not_claimed_entry(
            &entry_spec(
                host_identity::ENTRY_ID_ROLLBACK_READINESS,
                HostIdentityEvidenceKind::RollbackReadiness,
                HostIdentityEvidenceClass::Rollback,
                host_identity::REQUIREMENT_ROLLBACK_READINESS,
                host_identity::ARTIFACTS_ROLLBACK_READINESS,
                host_identity::SIGNALS_ROLLBACK_READINESS,
                host_identity::FALLBACK_ROLLBACK_READINESS,
            ),
            generated_at,
        ),
        manual_required_entry(
            &entry_spec(
                host_identity::ENTRY_ID_AUDIT_CUSTODY,
                HostIdentityEvidenceKind::AuditCustody,
                HostIdentityEvidenceClass::Audit,
                host_identity::REQUIREMENT_AUDIT_CUSTODY,
                host_identity::ARTIFACTS_AUDIT_CUSTODY,
                host_identity::SIGNALS_AUDIT_CUSTODY,
                host_identity::FALLBACK_AUDIT_CUSTODY,
            ),
            generated_at,
        ),
    ]
}

struct HostIdentityEntrySpec {
    read_model_entry_id: &'static str,
    evidence_kind: HostIdentityEvidenceKind,
    evidence_class: HostIdentityEvidenceClass,
    host_evidence_requirement: &'static str,
    required_evidence_artifacts: &'static [&'static str],
    acceptance_signals: &'static [&'static str],
    fallback_behavior: &'static str,
}

struct HostIdentityReadinessSpec {
    capability_state: EnforcementCapabilityState,
    readiness_state: EnforcementReadinessState,
    proof_level: EnforcementReadinessProofLevel,
    runtime_owner: EnforcementReadinessRuntimeOwner,
}

fn entry_spec(
    read_model_entry_id: &'static str,
    evidence_kind: HostIdentityEvidenceKind,
    evidence_class: HostIdentityEvidenceClass,
    host_evidence_requirement: &'static str,
    required_evidence_artifacts: &'static [&'static str],
    acceptance_signals: &'static [&'static str],
    fallback_behavior: &'static str,
) -> HostIdentityEntrySpec {
    HostIdentityEntrySpec {
        read_model_entry_id,
        evidence_kind,
        evidence_class,
        host_evidence_requirement,
        required_evidence_artifacts,
        acceptance_signals,
        fallback_behavior,
    }
}

fn manual_required_entry(
    spec: &HostIdentityEntrySpec,
    last_checked_at: &str,
) -> HostIdentityReadModelEntry {
    host_identity_entry(
        spec,
        &HostIdentityReadinessSpec {
            capability_state: EnforcementCapabilityState::ManualRequired,
            readiness_state: EnforcementReadinessState::ManualRequired,
            proof_level: EnforcementReadinessProofLevel::ManualProofRequired,
            runtime_owner: EnforcementReadinessRuntimeOwner::ManualProof,
        },
        last_checked_at,
    )
}

fn unavailable_entry(
    spec: &HostIdentityEntrySpec,
    last_checked_at: &str,
) -> HostIdentityReadModelEntry {
    host_identity_entry(
        spec,
        &HostIdentityReadinessSpec {
            capability_state: EnforcementCapabilityState::Unavailable,
            readiness_state: EnforcementReadinessState::Unavailable,
            proof_level: EnforcementReadinessProofLevel::ManualProofRequired,
            runtime_owner: EnforcementReadinessRuntimeOwner::ManualProof,
        },
        last_checked_at,
    )
}

fn not_claimed_entry(
    spec: &HostIdentityEntrySpec,
    last_checked_at: &str,
) -> HostIdentityReadModelEntry {
    host_identity_entry(
        spec,
        &HostIdentityReadinessSpec {
            capability_state: EnforcementCapabilityState::ManualRequired,
            readiness_state: EnforcementReadinessState::NotClaimed,
            proof_level: EnforcementReadinessProofLevel::NotProved,
            runtime_owner: EnforcementReadinessRuntimeOwner::NotImplemented,
        },
        last_checked_at,
    )
}

fn host_identity_entry(
    spec: &HostIdentityEntrySpec,
    readiness: &HostIdentityReadinessSpec,
    last_checked_at: &str,
) -> HostIdentityReadModelEntry {
    HostIdentityReadModelEntry {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_entry_id: spec.read_model_entry_id.to_string(),
        evidence_kind: spec.evidence_kind,
        evidence_class: spec.evidence_class,
        capability: EnforcementBroadAdapterCapability::BroadAppBlocking,
        platform: ParentPlatform::Windows,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        capability_state: readiness.capability_state,
        readiness_state: readiness.readiness_state,
        proof_level: readiness.proof_level,
        runtime_owner: readiness.runtime_owner,
        host_evidence_requirement: spec.host_evidence_requirement.to_string(),
        required_evidence_artifacts: strings(spec.required_evidence_artifacts),
        acceptance_signals: strings(spec.acceptance_signals),
        fallback_behavior: spec.fallback_behavior.to_string(),
        safe_for_broad_app_blocking: false,
        last_checked_at: last_checked_at.to_string(),
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
