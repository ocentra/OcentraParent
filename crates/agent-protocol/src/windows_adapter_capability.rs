use serde::{Deserialize, Serialize};

use crate::{
    constants::windows_adapter_capability as windows_adapter_constants, EnforcementAdapterKind,
    EnforcementBroadAdapterCapability, EnforcementCapabilityState, EnforcementMode,
    EnforcementReadinessProofLevel, EnforcementReadinessRuntimeOwner, EnforcementReadinessState,
    ParentPlatform,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowsAdapterCapabilitySurface {
    #[serde(rename = "app-target")]
    AppTarget,
    #[serde(rename = "domain-network-target")]
    DomainNetworkTarget,
    #[serde(rename = "managed-browser-target")]
    ManagedBrowserTarget,
    #[serde(rename = "unmanaged-browser-target")]
    UnmanagedBrowserTarget,
    #[serde(rename = "unsupported-os-target")]
    UnsupportedOsTarget,
    #[serde(rename = "rollback-audit-target")]
    RollbackAuditTarget,
}

impl WindowsAdapterCapabilitySurface {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::AppTarget => windows_adapter_constants::SURFACE_APP_TARGET,
            Self::DomainNetworkTarget => windows_adapter_constants::SURFACE_DOMAIN_NETWORK_TARGET,
            Self::ManagedBrowserTarget => windows_adapter_constants::SURFACE_MANAGED_BROWSER_TARGET,
            Self::UnmanagedBrowserTarget => {
                windows_adapter_constants::SURFACE_UNMANAGED_BROWSER_TARGET
            }
            Self::UnsupportedOsTarget => windows_adapter_constants::SURFACE_UNSUPPORTED_OS_TARGET,
            Self::RollbackAuditTarget => windows_adapter_constants::SURFACE_ROLLBACK_AUDIT_TARGET,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowsAdapterCapabilityOutcome {
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "process-only-implemented")]
    ProcessOnlyImplemented,
    #[serde(rename = "not-claimed")]
    NotClaimed,
}

impl WindowsAdapterCapabilityOutcome {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ManualRequired => windows_adapter_constants::OUTCOME_MANUAL_REQUIRED,
            Self::Unavailable => windows_adapter_constants::OUTCOME_UNAVAILABLE,
            Self::ProcessOnlyImplemented => {
                windows_adapter_constants::OUTCOME_PROCESS_ONLY_IMPLEMENTED
            }
            Self::NotClaimed => windows_adapter_constants::OUTCOME_NOT_CLAIMED,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsAdapterCapabilityProofEntry {
    pub schema_version: String,
    pub proof_entry_id: String,
    pub surface: WindowsAdapterCapabilitySurface,
    pub platform: ParentPlatform,
    pub primary_capability: EnforcementBroadAdapterCapability,
    pub adapter_kind: EnforcementAdapterKind,
    pub capability_state: EnforcementCapabilityState,
    pub readiness_state: EnforcementReadinessState,
    pub proof_level: EnforcementReadinessProofLevel,
    pub runtime_owner: EnforcementReadinessRuntimeOwner,
    pub supported_modes: Vec<EnforcementMode>,
    pub linked_readiness_ids: Vec<String>,
    pub linked_host_identity_entry_ids: Vec<String>,
    pub outcome: WindowsAdapterCapabilityOutcome,
    pub claim_boundary: String,
    pub fallback_behavior: String,
    pub exact_url_claimed: bool,
    pub broad_blocking_claimed: bool,
    pub required_artifacts: Vec<String>,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsAdapterCapabilityProof {
    pub schema_version: String,
    pub read_model_id: String,
    pub generated_at: String,
    pub platform: ParentPlatform,
    pub entries: Vec<WindowsAdapterCapabilityProofEntry>,
}
