use serde::{Deserialize, Serialize};

use crate::{
    constants::host_identity as host_identity_constants, EnforcementAdapterKind,
    EnforcementBroadAdapterCapability, EnforcementCapabilityState, EnforcementReadinessProofLevel,
    EnforcementReadinessRuntimeOwner, EnforcementReadinessState, ParentPlatform,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HostIdentityEvidenceKind {
    #[serde(rename = "installed-app-inventory")]
    InstalledAppInventory,
    #[serde(rename = "process-lineage")]
    ProcessLineage,
    #[serde(rename = "executable-identity")]
    ExecutableIdentity,
    #[serde(rename = "package-identity")]
    PackageIdentity,
    #[serde(rename = "publisher-signature")]
    PublisherSignature,
    #[serde(rename = "inventory-process-link")]
    InventoryProcessLink,
    #[serde(rename = "unsupported-identity")]
    UnsupportedIdentity,
    #[serde(rename = "rollback-readiness")]
    RollbackReadiness,
    #[serde(rename = "audit-custody")]
    AuditCustody,
}

impl HostIdentityEvidenceKind {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::InstalledAppInventory => host_identity_constants::KIND_INSTALLED_APP_INVENTORY,
            Self::ProcessLineage => host_identity_constants::KIND_PROCESS_LINEAGE,
            Self::ExecutableIdentity => host_identity_constants::KIND_EXECUTABLE_IDENTITY,
            Self::PackageIdentity => host_identity_constants::KIND_PACKAGE_IDENTITY,
            Self::PublisherSignature => host_identity_constants::KIND_PUBLISHER_SIGNATURE,
            Self::InventoryProcessLink => host_identity_constants::KIND_INVENTORY_PROCESS_LINK,
            Self::UnsupportedIdentity => host_identity_constants::KIND_UNSUPPORTED_IDENTITY,
            Self::RollbackReadiness => host_identity_constants::KIND_ROLLBACK_READINESS,
            Self::AuditCustody => host_identity_constants::KIND_AUDIT_CUSTODY,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HostIdentityEvidenceClass {
    #[serde(rename = "inventory")]
    Inventory,
    #[serde(rename = "process")]
    Process,
    #[serde(rename = "executable")]
    Executable,
    #[serde(rename = "package")]
    Package,
    #[serde(rename = "publisher-signature")]
    PublisherSignature,
    #[serde(rename = "rollback")]
    Rollback,
    #[serde(rename = "audit")]
    Audit,
}

impl HostIdentityEvidenceClass {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Inventory => host_identity_constants::CLASS_INVENTORY,
            Self::Process => host_identity_constants::CLASS_PROCESS,
            Self::Executable => host_identity_constants::CLASS_EXECUTABLE,
            Self::Package => host_identity_constants::CLASS_PACKAGE,
            Self::PublisherSignature => host_identity_constants::CLASS_PUBLISHER_SIGNATURE,
            Self::Rollback => host_identity_constants::CLASS_ROLLBACK,
            Self::Audit => host_identity_constants::CLASS_AUDIT,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostIdentityReadModelEntry {
    pub schema_version: String,
    pub read_model_entry_id: String,
    pub evidence_kind: HostIdentityEvidenceKind,
    pub evidence_class: HostIdentityEvidenceClass,
    pub capability: EnforcementBroadAdapterCapability,
    pub platform: ParentPlatform,
    pub adapter_kind: EnforcementAdapterKind,
    pub capability_state: EnforcementCapabilityState,
    pub readiness_state: EnforcementReadinessState,
    pub proof_level: EnforcementReadinessProofLevel,
    pub runtime_owner: EnforcementReadinessRuntimeOwner,
    pub host_evidence_requirement: String,
    pub required_evidence_artifacts: Vec<String>,
    pub acceptance_signals: Vec<String>,
    pub fallback_behavior: String,
    pub safe_for_broad_app_blocking: bool,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostIdentityReadModel {
    pub schema_version: String,
    pub read_model_id: String,
    pub generated_at: String,
    pub platform: ParentPlatform,
    pub entries: Vec<HostIdentityReadModelEntry>,
}
