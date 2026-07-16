use serde::{Deserialize, Serialize};

use crate::{
    constants::enforcement as enforcement_constants, EnforcementAdapterKind,
    EnforcementCapabilityState, EnforcementMode, ParentPlatform,
};

macro_rules! protocol_str_lookup {
    ($self:expr, [$($value:expr),+ $(,)?]) => {{
        const VALUES: &[&str] = &[$($value),+];
        VALUES[*$self as usize]
    }};
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum EnforcementBroadAdapterCapability {
    #[serde(rename = "owned-process-terminate")]
    OwnedProcessTerminate,
    #[serde(rename = "app-time-limit")]
    AppTimeLimit,
    #[serde(rename = "broad-app-blocking")]
    BroadAppBlocking,
    #[serde(rename = "network-domain-blocking")]
    NetworkDomainBlocking,
    #[serde(rename = "managed-browser-service-command")]
    ManagedBrowserServiceCommand,
    #[serde(rename = "managed-browser-exact-url-control")]
    ManagedBrowserExactUrlControl,
    #[serde(rename = "unmanaged-browser-process-only")]
    UnmanagedBrowserProcessOnly,
    #[serde(rename = "unmanaged-browser-exact-evidence")]
    UnmanagedBrowserExactEvidence,
    #[serde(rename = "admin-anti-tamper-rollback")]
    AdminAntiTamperRollback,
}

impl EnforcementBroadAdapterCapability {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                enforcement_constants::BROAD_CAPABILITY_OWNED_PROCESS_TERMINATE,
                enforcement_constants::BROAD_CAPABILITY_APP_TIME_LIMIT,
                enforcement_constants::BROAD_CAPABILITY_BROAD_APP_BLOCKING,
                enforcement_constants::BROAD_CAPABILITY_NETWORK_DOMAIN_BLOCKING,
                enforcement_constants::BROAD_CAPABILITY_MANAGED_BROWSER_SERVICE_COMMAND,
                enforcement_constants::BROAD_CAPABILITY_MANAGED_BROWSER_EXACT_URL_CONTROL,
                enforcement_constants::BROAD_CAPABILITY_UNMANAGED_BROWSER_PROCESS_ONLY,
                enforcement_constants::BROAD_CAPABILITY_UNMANAGED_BROWSER_EXACT_EVIDENCE,
                enforcement_constants::BROAD_CAPABILITY_ADMIN_ANTI_TAMPER_ROLLBACK,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum EnforcementReadinessState {
    #[serde(rename = "implemented")]
    Implemented,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "not-claimed")]
    NotClaimed,
}

impl EnforcementReadinessState {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                enforcement_constants::READINESS_IMPLEMENTED,
                enforcement_constants::READINESS_MANUAL_REQUIRED,
                enforcement_constants::READINESS_UNAVAILABLE,
                enforcement_constants::READINESS_NOT_CLAIMED,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum EnforcementReadinessProofLevel {
    #[serde(rename = "real-service-proof")]
    RealServiceProof,
    #[serde(rename = "ci-mechanical-proof")]
    CiMechanicalProof,
    #[serde(rename = "manual-proof-required")]
    ManualProofRequired,
    #[serde(rename = "not-proved")]
    NotProved,
}

impl EnforcementReadinessProofLevel {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                enforcement_constants::PROOF_REAL_SERVICE,
                enforcement_constants::PROOF_CI_MECHANICAL,
                enforcement_constants::PROOF_MANUAL_REQUIRED,
                enforcement_constants::PROOF_NOT_PROVED,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum EnforcementReadinessRuntimeOwner {
    #[serde(rename = "rust-service")]
    RustService,
    #[serde(rename = "os-adapter")]
    OsAdapter,
    #[serde(rename = "managed-browser-boundary")]
    ManagedBrowserBoundary,
    #[serde(rename = "manual-proof")]
    ManualProof,
    #[serde(rename = "not-implemented")]
    NotImplemented,
}

impl EnforcementReadinessRuntimeOwner {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                enforcement_constants::RUNTIME_OWNER_RUST_SERVICE,
                enforcement_constants::RUNTIME_OWNER_OS_ADAPTER,
                enforcement_constants::RUNTIME_OWNER_MANAGED_BROWSER_BOUNDARY,
                enforcement_constants::RUNTIME_OWNER_MANUAL_PROOF,
                enforcement_constants::RUNTIME_OWNER_NOT_IMPLEMENTED,
            ]
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnforcementBroadAdapterReadinessEntry {
    pub schema_version: String,
    pub readiness_id: String,
    pub capability: EnforcementBroadAdapterCapability,
    pub platform: ParentPlatform,
    pub adapter_kind: EnforcementAdapterKind,
    pub capability_state: EnforcementCapabilityState,
    pub readiness_state: EnforcementReadinessState,
    pub proof_level: EnforcementReadinessProofLevel,
    pub runtime_owner: EnforcementReadinessRuntimeOwner,
    pub supported_modes: Vec<EnforcementMode>,
    pub claim_boundary: String,
    pub fallback_behavior: String,
    pub required_artifacts: Vec<String>,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnforcementBroadOsAdapterReadinessMatrix {
    pub schema_version: String,
    pub matrix_id: String,
    pub generated_at: String,
    pub entries: Vec<EnforcementBroadAdapterReadinessEntry>,
}
