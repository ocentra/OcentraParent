use serde::{Deserialize, Serialize};

use crate::{
    constants::host_identity as host_identity_constants, EnforcementAdapterKind,
    EnforcementBroadAdapterCapability, EnforcementCapabilityState, EnforcementReadinessProofLevel,
    EnforcementReadinessRuntimeOwner, EnforcementReadinessState, ParentPlatform,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 9] = [
        host_identity_constants::KIND_INSTALLED_APP_INVENTORY,
        host_identity_constants::KIND_PROCESS_LINEAGE,
        host_identity_constants::KIND_EXECUTABLE_IDENTITY,
        host_identity_constants::KIND_PACKAGE_IDENTITY,
        host_identity_constants::KIND_PUBLISHER_SIGNATURE,
        host_identity_constants::KIND_INVENTORY_PROCESS_LINK,
        host_identity_constants::KIND_UNSUPPORTED_IDENTITY,
        host_identity_constants::KIND_ROLLBACK_READINESS,
        host_identity_constants::KIND_AUDIT_CUSTODY,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 7] = [
        host_identity_constants::CLASS_INVENTORY,
        host_identity_constants::CLASS_PROCESS,
        host_identity_constants::CLASS_EXECUTABLE,
        host_identity_constants::CLASS_PACKAGE,
        host_identity_constants::CLASS_PUBLISHER_SIGNATURE,
        host_identity_constants::CLASS_ROLLBACK,
        host_identity_constants::CLASS_AUDIT,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
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
