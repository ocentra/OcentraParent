use serde::{Deserialize, Serialize};

use crate::{
    constants::v08_os_adapter_product_proof as proof_constants, EnforcementAdapterKind,
    EnforcementCapabilityState, EnforcementMode, EnforcementReadinessProofLevel,
    EnforcementReadinessRuntimeOwner, EnforcementReadinessState, EnforcementResultStatus,
    EnforcementRollbackState, ParentPlatform,
};

macro_rules! protocol_str_lookup {
    ($self:expr, [$($value:expr),+ $(,)?]) => {{
        const VALUES: &[&str] = &[$($value),+];
        VALUES[*$self as usize]
    }};
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum V08OsAdapterProductProofSurface {
    #[serde(rename = "owned-process-terminate")]
    OwnedProcessTerminate,
    #[serde(rename = "app-time-limit-lifecycle")]
    AppTimeLimitLifecycle,
    #[serde(rename = "broad-app-blocking")]
    BroadAppBlocking,
    #[serde(rename = "network-domain-blocking")]
    NetworkDomainBlocking,
    #[serde(rename = "managed-browser-service-command")]
    ManagedBrowserServiceCommand,
    #[serde(rename = "managed-browser-exact-url")]
    ManagedBrowserExactUrl,
    #[serde(rename = "unmanaged-browser-process-only")]
    UnmanagedBrowserProcessOnly,
    #[serde(rename = "unmanaged-browser-exact-evidence")]
    UnmanagedBrowserExactEvidence,
    #[serde(rename = "restart-recovery")]
    RestartRecovery,
    #[serde(rename = "parent-cancel-override")]
    ParentCancelOverride,
    #[serde(rename = "audit-custody")]
    AuditCustody,
    #[serde(rename = "rollback-artifact-gate")]
    RollbackArtifactGate,
}

impl V08OsAdapterProductProofSurface {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                proof_constants::SURFACE_OWNED_PROCESS_TERMINATE,
                proof_constants::SURFACE_APP_TIME_LIMIT_LIFECYCLE,
                proof_constants::SURFACE_BROAD_APP_BLOCKING,
                proof_constants::SURFACE_NETWORK_DOMAIN_BLOCKING,
                proof_constants::SURFACE_MANAGED_BROWSER_SERVICE_COMMAND,
                proof_constants::SURFACE_MANAGED_BROWSER_EXACT_URL,
                proof_constants::SURFACE_UNMANAGED_BROWSER_PROCESS_ONLY,
                proof_constants::SURFACE_UNMANAGED_BROWSER_EXACT_EVIDENCE,
                proof_constants::SURFACE_RESTART_RECOVERY,
                proof_constants::SURFACE_PARENT_CANCEL_OVERRIDE,
                proof_constants::SURFACE_AUDIT_CUSTODY,
                proof_constants::SURFACE_ROLLBACK_ARTIFACT_GATE,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum V08OsAdapterProductProofTimerRecoveryState {
    #[serde(rename = "not-required")]
    NotRequired,
    #[serde(rename = "restart-recovered")]
    RestartRecovered,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl V08OsAdapterProductProofTimerRecoveryState {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                proof_constants::TIMER_STATE_NOT_REQUIRED,
                proof_constants::TIMER_STATE_RESTART_RECOVERED,
                proof_constants::TIMER_STATE_CANCELLED,
                proof_constants::TIMER_STATE_EXPIRED,
                proof_constants::TIMER_STATE_MANUAL_REQUIRED,
                proof_constants::TIMER_STATE_UNAVAILABLE,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum V08OsAdapterProductProofAuditState {
    #[serde(rename = "journaled")]
    Journaled,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl V08OsAdapterProductProofAuditState {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                proof_constants::AUDIT_STATE_JOURNALED,
                proof_constants::AUDIT_STATE_MANUAL_REQUIRED,
                proof_constants::AUDIT_STATE_UNAVAILABLE,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum V08OsAdapterProductProofParentOverrideState {
    #[serde(rename = "not-required")]
    NotRequired,
    #[serde(rename = "cancel-supported")]
    CancelSupported,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl V08OsAdapterProductProofParentOverrideState {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                proof_constants::PARENT_OVERRIDE_NOT_REQUIRED,
                proof_constants::PARENT_OVERRIDE_CANCEL_SUPPORTED,
                proof_constants::PARENT_OVERRIDE_MANUAL_REQUIRED,
                proof_constants::PARENT_OVERRIDE_UNAVAILABLE,
            ]
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V08OsAdapterProductProofEntry {
    pub schema_version: String,
    pub proof_entry_id: String,
    pub surface: V08OsAdapterProductProofSurface,
    pub platform: ParentPlatform,
    pub adapter_kind: EnforcementAdapterKind,
    pub capability_state: EnforcementCapabilityState,
    pub readiness_state: EnforcementReadinessState,
    pub proof_level: EnforcementReadinessProofLevel,
    pub runtime_owner: EnforcementReadinessRuntimeOwner,
    pub supported_modes: Vec<EnforcementMode>,
    pub result_status: EnforcementResultStatus,
    pub rollback_state: EnforcementRollbackState,
    pub timer_recovery_state: V08OsAdapterProductProofTimerRecoveryState,
    pub audit_state: V08OsAdapterProductProofAuditState,
    pub parent_override_state: V08OsAdapterProductProofParentOverrideState,
    pub linked_readiness_ids: Vec<String>,
    pub linked_capability_entry_ids: Vec<String>,
    pub linked_artifact_gate_entry_ids: Vec<String>,
    pub capability_requirement: String,
    pub proof_requirement: String,
    pub claim_boundary: String,
    pub fallback_behavior: String,
    pub claim_upgrade_allowed: bool,
    pub broad_blocking_claimed: bool,
    pub exact_url_claimed: bool,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V08OsAdapterProductProofReadModel {
    pub schema_version: String,
    pub read_model_id: String,
    pub generated_at: String,
    pub source_read_model_ids: Vec<String>,
    pub entries: Vec<V08OsAdapterProductProofEntry>,
}
