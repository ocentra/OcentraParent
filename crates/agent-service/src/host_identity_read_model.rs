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

#[derive(Clone, Debug)]
pub(crate) struct GeneratedAtText(pub(crate) String);

pub(crate) fn host_identity_read_model(generated_at: GeneratedAtText) -> HostIdentityReadModel {
    let entries = host_identity_entries(&generated_at);
    let generated_at = generated_at.0;
    HostIdentityReadModel {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        read_model_id: host_identity::READ_MODEL_ID_V0_8.to_string(),
        generated_at,
        platform: ParentPlatform::Windows,
        entries,
    }
}

fn host_identity_entries(generated_at: &GeneratedAtText) -> Vec<HostIdentityReadModelEntry> {
    let mut entries = Vec::new();
    entries.extend(inventory_and_process_entries(generated_at));
    entries.extend(package_and_trust_entries(generated_at));
    entries.extend(fallback_and_custody_entries(generated_at));
    entries
}

fn inventory_and_process_entries(
    generated_at: &GeneratedAtText,
) -> Vec<HostIdentityReadModelEntry> {
    vec![
        host_identity_entry(
            &HostIdentityEntrySpec {
                read_model_entry_id: host_identity::ENTRY_ID_INSTALLED_APP_INVENTORY,
                evidence_kind: HostIdentityEvidenceKind::InstalledAppInventory,
                evidence_class: HostIdentityEvidenceClass::Inventory,
                host_evidence_requirement: host_identity::REQUIREMENT_INSTALLED_APP_INVENTORY,
                required_evidence_artifacts: host_identity::ARTIFACTS_INSTALLED_APP_INVENTORY,
                acceptance_signals: host_identity::SIGNALS_INSTALLED_APP_INVENTORY,
                fallback_behavior: host_identity::FALLBACK_INSTALLED_APP_INVENTORY,
            },
            &HostIdentityReadinessSpec {
                capability_state: EnforcementCapabilityState::ManualRequired,
                readiness_state: EnforcementReadinessState::ManualRequired,
                proof_level: EnforcementReadinessProofLevel::ManualProofRequired,
                runtime_owner: EnforcementReadinessRuntimeOwner::ManualProof,
            },
            generated_at,
        ),
        host_identity_entry(
            &HostIdentityEntrySpec {
                read_model_entry_id: host_identity::ENTRY_ID_PROCESS_LINEAGE,
                evidence_kind: HostIdentityEvidenceKind::ProcessLineage,
                evidence_class: HostIdentityEvidenceClass::Process,
                host_evidence_requirement: host_identity::REQUIREMENT_PROCESS_LINEAGE,
                required_evidence_artifacts: host_identity::ARTIFACTS_PROCESS_LINEAGE,
                acceptance_signals: host_identity::SIGNALS_PROCESS_LINEAGE,
                fallback_behavior: host_identity::FALLBACK_PROCESS_LINEAGE,
            },
            &HostIdentityReadinessSpec {
                capability_state: EnforcementCapabilityState::ManualRequired,
                readiness_state: EnforcementReadinessState::ManualRequired,
                proof_level: EnforcementReadinessProofLevel::ManualProofRequired,
                runtime_owner: EnforcementReadinessRuntimeOwner::ManualProof,
            },
            generated_at,
        ),
        host_identity_entry(
            &HostIdentityEntrySpec {
                read_model_entry_id: host_identity::ENTRY_ID_EXECUTABLE_IDENTITY,
                evidence_kind: HostIdentityEvidenceKind::ExecutableIdentity,
                evidence_class: HostIdentityEvidenceClass::Executable,
                host_evidence_requirement: host_identity::REQUIREMENT_EXECUTABLE_IDENTITY,
                required_evidence_artifacts: host_identity::ARTIFACTS_EXECUTABLE_IDENTITY,
                acceptance_signals: host_identity::SIGNALS_EXECUTABLE_IDENTITY,
                fallback_behavior: host_identity::FALLBACK_EXECUTABLE_IDENTITY,
            },
            &HostIdentityReadinessSpec {
                capability_state: EnforcementCapabilityState::ManualRequired,
                readiness_state: EnforcementReadinessState::ManualRequired,
                proof_level: EnforcementReadinessProofLevel::ManualProofRequired,
                runtime_owner: EnforcementReadinessRuntimeOwner::ManualProof,
            },
            generated_at,
        ),
    ]
}

fn package_and_trust_entries(generated_at: &GeneratedAtText) -> Vec<HostIdentityReadModelEntry> {
    vec![
        host_identity_entry(
            &HostIdentityEntrySpec {
                read_model_entry_id: host_identity::ENTRY_ID_PACKAGE_IDENTITY,
                evidence_kind: HostIdentityEvidenceKind::PackageIdentity,
                evidence_class: HostIdentityEvidenceClass::Package,
                host_evidence_requirement: host_identity::REQUIREMENT_PACKAGE_IDENTITY,
                required_evidence_artifacts: host_identity::ARTIFACTS_PACKAGE_IDENTITY,
                acceptance_signals: host_identity::SIGNALS_PACKAGE_IDENTITY,
                fallback_behavior: host_identity::FALLBACK_PACKAGE_IDENTITY,
            },
            &HostIdentityReadinessSpec {
                capability_state: EnforcementCapabilityState::ManualRequired,
                readiness_state: EnforcementReadinessState::ManualRequired,
                proof_level: EnforcementReadinessProofLevel::ManualProofRequired,
                runtime_owner: EnforcementReadinessRuntimeOwner::ManualProof,
            },
            generated_at,
        ),
        host_identity_entry(
            &HostIdentityEntrySpec {
                read_model_entry_id: host_identity::ENTRY_ID_PUBLISHER_SIGNATURE,
                evidence_kind: HostIdentityEvidenceKind::PublisherSignature,
                evidence_class: HostIdentityEvidenceClass::PublisherSignature,
                host_evidence_requirement: host_identity::REQUIREMENT_PUBLISHER_SIGNATURE,
                required_evidence_artifacts: host_identity::ARTIFACTS_PUBLISHER_SIGNATURE,
                acceptance_signals: host_identity::SIGNALS_PUBLISHER_SIGNATURE,
                fallback_behavior: host_identity::FALLBACK_PUBLISHER_SIGNATURE,
            },
            &HostIdentityReadinessSpec {
                capability_state: EnforcementCapabilityState::ManualRequired,
                readiness_state: EnforcementReadinessState::ManualRequired,
                proof_level: EnforcementReadinessProofLevel::ManualProofRequired,
                runtime_owner: EnforcementReadinessRuntimeOwner::ManualProof,
            },
            generated_at,
        ),
        host_identity_entry(
            &HostIdentityEntrySpec {
                read_model_entry_id: host_identity::ENTRY_ID_INVENTORY_PROCESS_LINK,
                evidence_kind: HostIdentityEvidenceKind::InventoryProcessLink,
                evidence_class: HostIdentityEvidenceClass::Inventory,
                host_evidence_requirement: host_identity::REQUIREMENT_INVENTORY_PROCESS_LINK,
                required_evidence_artifacts: host_identity::ARTIFACTS_INVENTORY_PROCESS_LINK,
                acceptance_signals: host_identity::SIGNALS_INVENTORY_PROCESS_LINK,
                fallback_behavior: host_identity::FALLBACK_INVENTORY_PROCESS_LINK,
            },
            &HostIdentityReadinessSpec {
                capability_state: EnforcementCapabilityState::ManualRequired,
                readiness_state: EnforcementReadinessState::ManualRequired,
                proof_level: EnforcementReadinessProofLevel::ManualProofRequired,
                runtime_owner: EnforcementReadinessRuntimeOwner::ManualProof,
            },
            generated_at,
        ),
    ]
}

fn fallback_and_custody_entries(generated_at: &GeneratedAtText) -> Vec<HostIdentityReadModelEntry> {
    vec![
        host_identity_entry(
            &HostIdentityEntrySpec {
                read_model_entry_id: host_identity::ENTRY_ID_UNSUPPORTED_IDENTITY,
                evidence_kind: HostIdentityEvidenceKind::UnsupportedIdentity,
                evidence_class: HostIdentityEvidenceClass::Package,
                host_evidence_requirement: host_identity::REQUIREMENT_UNSUPPORTED_IDENTITY,
                required_evidence_artifacts: host_identity::ARTIFACTS_UNSUPPORTED_IDENTITY,
                acceptance_signals: host_identity::SIGNALS_UNSUPPORTED_IDENTITY,
                fallback_behavior: host_identity::FALLBACK_UNSUPPORTED_IDENTITY,
            },
            &HostIdentityReadinessSpec {
                capability_state: EnforcementCapabilityState::Unavailable,
                readiness_state: EnforcementReadinessState::Unavailable,
                proof_level: EnforcementReadinessProofLevel::ManualProofRequired,
                runtime_owner: EnforcementReadinessRuntimeOwner::ManualProof,
            },
            generated_at,
        ),
        host_identity_entry(
            &HostIdentityEntrySpec {
                read_model_entry_id: host_identity::ENTRY_ID_ROLLBACK_READINESS,
                evidence_kind: HostIdentityEvidenceKind::RollbackReadiness,
                evidence_class: HostIdentityEvidenceClass::Rollback,
                host_evidence_requirement: host_identity::REQUIREMENT_ROLLBACK_READINESS,
                required_evidence_artifacts: host_identity::ARTIFACTS_ROLLBACK_READINESS,
                acceptance_signals: host_identity::SIGNALS_ROLLBACK_READINESS,
                fallback_behavior: host_identity::FALLBACK_ROLLBACK_READINESS,
            },
            &HostIdentityReadinessSpec {
                capability_state: EnforcementCapabilityState::ManualRequired,
                readiness_state: EnforcementReadinessState::NotClaimed,
                proof_level: EnforcementReadinessProofLevel::NotProved,
                runtime_owner: EnforcementReadinessRuntimeOwner::NotImplemented,
            },
            generated_at,
        ),
        host_identity_entry(
            &HostIdentityEntrySpec {
                read_model_entry_id: host_identity::ENTRY_ID_AUDIT_CUSTODY,
                evidence_kind: HostIdentityEvidenceKind::AuditCustody,
                evidence_class: HostIdentityEvidenceClass::Audit,
                host_evidence_requirement: host_identity::REQUIREMENT_AUDIT_CUSTODY,
                required_evidence_artifacts: host_identity::ARTIFACTS_AUDIT_CUSTODY,
                acceptance_signals: host_identity::SIGNALS_AUDIT_CUSTODY,
                fallback_behavior: host_identity::FALLBACK_AUDIT_CUSTODY,
            },
            &HostIdentityReadinessSpec {
                capability_state: EnforcementCapabilityState::ManualRequired,
                readiness_state: EnforcementReadinessState::ManualRequired,
                proof_level: EnforcementReadinessProofLevel::ManualProofRequired,
                runtime_owner: EnforcementReadinessRuntimeOwner::ManualProof,
            },
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

fn host_identity_entry(
    spec: &HostIdentityEntrySpec,
    readiness: &HostIdentityReadinessSpec,
    last_checked_at: &GeneratedAtText,
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
        required_evidence_artifacts: spec
            .required_evidence_artifacts
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        acceptance_signals: spec
            .acceptance_signals
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        fallback_behavior: spec.fallback_behavior.to_string(),
        safe_for_broad_app_blocking: false,
        last_checked_at: last_checked_at.0.clone(),
    }
}
