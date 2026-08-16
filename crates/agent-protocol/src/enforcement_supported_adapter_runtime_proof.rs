use serde::{Deserialize, Serialize};

use crate::{constants::v08_supported_adapter_runtime_proof as proof, ParentPlatform};

macro_rules! protocol_str_lookup {
    ($self:expr, [$($value:expr),+ $(,)?]) => {{
        const VALUES: &[&str] = &[$($value),+];
        VALUES[*$self as usize]
    }};
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum V08SupportedAdapterRuntimeBoundary {
    #[serde(rename = "windows-app-game-owned-process-time-limit")]
    WindowsAppGameOwnedProcessTimeLimit,
    #[serde(rename = "windows-network-flow-observe-policy-handoff")]
    WindowsNetworkFlowObservePolicyHandoff,
    #[serde(rename = "windows-broad-installed-app-blocking-manual-gate")]
    WindowsBroadInstalledAppBlockingManualGate,
    #[serde(rename = "windows-host-network-domain-blocking-manual-gate")]
    WindowsHostNetworkDomainBlockingManualGate,
    #[serde(rename = "windows-broad-installed-app-artifact-status")]
    WindowsBroadInstalledAppArtifactStatus,
    #[serde(rename = "windows-host-network-domain-artifact-status")]
    WindowsHostNetworkDomainArtifactStatus,
    #[serde(rename = "windows-managed-browser-artifact-status")]
    WindowsManagedBrowserArtifactStatus,
    #[serde(rename = "windows-managed-exact-active-tab-not-claimed")]
    WindowsManagedExactActiveTabNotClaimed,
    #[serde(rename = "windows-adapter-permission-dependency-degraded")]
    WindowsAdapterPermissionDependencyDegraded,
    #[serde(rename = "linux-host-adapter-unavailable")]
    LinuxHostAdapterUnavailable,
    #[serde(rename = "macos-host-adapter-unsupported")]
    MacosHostAdapterUnsupported,
    #[serde(rename = "android-mobile-control-manual-gate")]
    AndroidMobileControlManualGate,
    #[serde(rename = "ios-mobile-control-manual-gate")]
    IosMobileControlManualGate,
}

impl V08SupportedAdapterRuntimeBoundary {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                proof::ENTRY_ID_APP_GAME_TIMER,
                proof::ENTRY_ID_NETWORK_OBSERVE,
                proof::ENTRY_ID_BROAD_APP_MANUAL,
                proof::ENTRY_ID_HOST_NETWORK_MANUAL,
                proof::ENTRY_ID_BROAD_APP_ARTIFACT_STATUS,
                proof::ENTRY_ID_HOST_NETWORK_ARTIFACT_STATUS,
                proof::ENTRY_ID_MANAGED_BROWSER_ARTIFACT_STATUS,
                proof::ENTRY_ID_EXACT_ACTIVE_TAB_NOT_CLAIMED,
                proof::ENTRY_ID_PERMISSION_DEGRADED,
                proof::ENTRY_ID_LINUX_UNAVAILABLE,
                proof::ENTRY_ID_MACOS_UNSUPPORTED,
                proof::ENTRY_ID_ANDROID_MANUAL,
                proof::ENTRY_ID_IOS_MANUAL,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum V08SupportedAdapterCapability {
    #[serde(rename = "app-game-owned-process-time-limit")]
    AppGameOwnedProcessTimeLimit,
    #[serde(rename = "network-flow-observe-policy-handoff")]
    NetworkFlowObservePolicyHandoff,
    #[serde(rename = "broad-installed-app-blocking")]
    BroadInstalledAppBlocking,
    #[serde(rename = "host-network-domain-blocking")]
    HostNetworkDomainBlocking,
    #[serde(rename = "broad-installed-app-artifact-status")]
    BroadInstalledAppArtifactStatus,
    #[serde(rename = "host-network-domain-artifact-status")]
    HostNetworkDomainArtifactStatus,
    #[serde(rename = "managed-browser-artifact-status")]
    ManagedBrowserArtifactStatus,
    #[serde(rename = "managed-exact-active-tab-enforcement")]
    ManagedExactActiveTabEnforcement,
    #[serde(rename = "adapter-permission-dependency")]
    AdapterPermissionDependency,
    #[serde(rename = "desktop-host-platform-adapter")]
    DesktopHostPlatformAdapter,
    #[serde(rename = "mobile-child-control-adapter")]
    MobileChildControlAdapter,
}

impl V08SupportedAdapterCapability {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                proof::CAPABILITY_APP_GAME_TIMER,
                proof::CAPABILITY_NETWORK_OBSERVE,
                proof::CAPABILITY_BROAD_APP_BLOCKING,
                proof::CAPABILITY_HOST_NETWORK_BLOCKING,
                proof::CAPABILITY_BROAD_APP_ARTIFACT_STATUS,
                proof::CAPABILITY_HOST_NETWORK_ARTIFACT_STATUS,
                proof::CAPABILITY_MANAGED_BROWSER_ARTIFACT_STATUS,
                proof::CAPABILITY_MANAGED_EXACT_ACTIVE_TAB,
                proof::CAPABILITY_PERMISSION_DEPENDENCY,
                proof::CAPABILITY_DESKTOP_HOST,
                proof::CAPABILITY_MOBILE_CHILD_CONTROL,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum V08SupportedAdapterRuntimeState {
    #[serde(rename = "implemented-boundary")]
    ImplementedBoundary,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "not-claimed")]
    NotClaimed,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "degraded")]
    Degraded,
}

impl V08SupportedAdapterRuntimeState {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                proof::STATE_IMPLEMENTED_BOUNDARY,
                proof::STATE_MANUAL_REQUIRED,
                proof::STATE_UNAVAILABLE,
                proof::STATE_NOT_CLAIMED,
                proof::STATE_UNSUPPORTED,
                proof::STATE_DEGRADED,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum V08SupportedAdapterResult {
    #[serde(rename = "supported-boundary-proved")]
    SupportedBoundaryProved,
    #[serde(rename = "manual-proof-required")]
    ManualProofRequired,
    #[serde(rename = "target-unavailable")]
    TargetUnavailable,
    #[serde(rename = "not-claimed")]
    NotClaimed,
    #[serde(rename = "unsupported-platform")]
    UnsupportedPlatform,
    #[serde(rename = "degraded-permission-or-dependency")]
    DegradedPermissionOrDependency,
}

impl V08SupportedAdapterResult {
    pub fn as_protocol_str(&self) -> &'static str {
        protocol_str_lookup!(
            self,
            [
                proof::RESULT_SUPPORTED_BOUNDARY_PROVED,
                proof::RESULT_MANUAL_PROOF_REQUIRED,
                proof::RESULT_TARGET_UNAVAILABLE,
                proof::RESULT_NOT_CLAIMED,
                proof::RESULT_UNSUPPORTED_PLATFORM,
                proof::RESULT_DEGRADED_PERMISSION_OR_DEPENDENCY,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08SupportedAdapterPlatformSupportState {
    #[serde(rename = "supported-on-windows")]
    SupportedOnWindows,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable-on-target")]
    UnavailableOnTarget,
    #[serde(rename = "unsupported-platform")]
    UnsupportedPlatform,
    #[serde(rename = "degraded")]
    Degraded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08SupportedAdapterTargetIdentityState {
    #[serde(rename = "process-session-evidence-backed")]
    ProcessSessionEvidenceBacked,
    #[serde(rename = "network-flow-evidence-backed")]
    NetworkFlowEvidenceBacked,
    #[serde(rename = "insufficient-for-broad-target")]
    InsufficientForBroadTarget,
    #[serde(rename = "not-applicable")]
    NotApplicable,
    #[serde(rename = "unsupported-platform-target")]
    UnsupportedPlatformTarget,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08SupportedAdapterRollbackReferenceState {
    #[serde(rename = "timer-recovery-backed")]
    TimerRecoveryBacked,
    #[serde(rename = "observe-only-not-needed")]
    ObserveOnlyNotNeeded,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "not-claimed")]
    NotClaimed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08SupportedAdapterAuditReferenceState {
    #[serde(rename = "audit-reference-backed")]
    AuditReferenceBacked,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "not-claimed")]
    NotClaimed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08SupportedAdapterRefusalReason {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "manual-artifact-required")]
    ManualArtifactRequired,
    #[serde(rename = "target-unavailable")]
    TargetUnavailable,
    #[serde(rename = "not-claimed-boundary")]
    NotClaimedBoundary,
    #[serde(rename = "unsupported-platform")]
    UnsupportedPlatform,
    #[serde(rename = "permission-or-dependency-degraded")]
    PermissionOrDependencyDegraded,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V08SupportedAdapterRuntimeProofEntry {
    pub schema_version: String,
    pub proof_entry_id: String,
    pub runtime_boundary: V08SupportedAdapterRuntimeBoundary,
    pub platform: ParentPlatform,
    pub adapter_capability: V08SupportedAdapterCapability,
    pub runtime_state: V08SupportedAdapterRuntimeState,
    pub adapter_result: V08SupportedAdapterResult,
    pub platform_support_state: V08SupportedAdapterPlatformSupportState,
    pub target_identity_state: V08SupportedAdapterTargetIdentityState,
    pub rollback_reference_state: V08SupportedAdapterRollbackReferenceState,
    pub audit_reference_state: V08SupportedAdapterAuditReferenceState,
    pub refusal_reason: V08SupportedAdapterRefusalReason,
    pub evidence_refs: Vec<String>,
    pub linked_proof_commands: Vec<String>,
    pub linked_proof_artifacts: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
    pub claim_boundary: String,
    pub fallback_behavior: String,
    pub broad_installed_app_blocking_claimed: bool,
    pub network_domain_blocking_claimed: bool,
    pub exact_active_tab_enforcement_claimed: bool,
    pub notification_delivery_claimed: bool,
    pub tamper_hardening_claimed: bool,
    pub mobile_control_claimed: bool,
    pub unsupported_platform_behavior_claimed: bool,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V08SupportedAdapterRuntimeProofReadModel {
    pub schema_version: String,
    pub read_model_id: String,
    pub generated_at: String,
    pub source_read_model_ids: Vec<String>,
    pub entries: Vec<V08SupportedAdapterRuntimeProofEntry>,
}
