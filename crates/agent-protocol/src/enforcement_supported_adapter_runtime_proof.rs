use serde::{Deserialize, Serialize};

use crate::{constants::v08_supported_adapter_runtime_proof as proof, ParentPlatform};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08SupportedAdapterRuntimeBoundary {
    #[serde(rename = "windows-app-game-owned-process-time-limit")]
    WindowsAppGameOwnedProcessTimeLimit,
    #[serde(rename = "windows-network-flow-observe-policy-handoff")]
    WindowsNetworkFlowObservePolicyHandoff,
    #[serde(rename = "windows-broad-installed-app-blocking-manual-gate")]
    WindowsBroadInstalledAppBlockingManualGate,
    #[serde(rename = "windows-host-network-domain-blocking-manual-gate")]
    WindowsHostNetworkDomainBlockingManualGate,
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
        match self {
            Self::WindowsAppGameOwnedProcessTimeLimit => proof::ENTRY_ID_APP_GAME_TIMER,
            Self::WindowsNetworkFlowObservePolicyHandoff => proof::ENTRY_ID_NETWORK_OBSERVE,
            Self::WindowsBroadInstalledAppBlockingManualGate => proof::ENTRY_ID_BROAD_APP_MANUAL,
            Self::WindowsHostNetworkDomainBlockingManualGate => proof::ENTRY_ID_HOST_NETWORK_MANUAL,
            Self::WindowsManagedExactActiveTabNotClaimed => {
                proof::ENTRY_ID_EXACT_ACTIVE_TAB_NOT_CLAIMED
            }
            Self::WindowsAdapterPermissionDependencyDegraded => proof::ENTRY_ID_PERMISSION_DEGRADED,
            Self::LinuxHostAdapterUnavailable => proof::ENTRY_ID_LINUX_UNAVAILABLE,
            Self::MacosHostAdapterUnsupported => proof::ENTRY_ID_MACOS_UNSUPPORTED,
            Self::AndroidMobileControlManualGate => proof::ENTRY_ID_ANDROID_MANUAL,
            Self::IosMobileControlManualGate => proof::ENTRY_ID_IOS_MANUAL,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08SupportedAdapterCapability {
    #[serde(rename = "app-game-owned-process-time-limit")]
    AppGameOwnedProcessTimeLimit,
    #[serde(rename = "network-flow-observe-policy-handoff")]
    NetworkFlowObservePolicyHandoff,
    #[serde(rename = "broad-installed-app-blocking")]
    BroadInstalledAppBlocking,
    #[serde(rename = "host-network-domain-blocking")]
    HostNetworkDomainBlocking,
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
        match self {
            Self::AppGameOwnedProcessTimeLimit => proof::CAPABILITY_APP_GAME_TIMER,
            Self::NetworkFlowObservePolicyHandoff => proof::CAPABILITY_NETWORK_OBSERVE,
            Self::BroadInstalledAppBlocking => proof::CAPABILITY_BROAD_APP_BLOCKING,
            Self::HostNetworkDomainBlocking => proof::CAPABILITY_HOST_NETWORK_BLOCKING,
            Self::ManagedExactActiveTabEnforcement => proof::CAPABILITY_MANAGED_EXACT_ACTIVE_TAB,
            Self::AdapterPermissionDependency => proof::CAPABILITY_PERMISSION_DEPENDENCY,
            Self::DesktopHostPlatformAdapter => proof::CAPABILITY_DESKTOP_HOST,
            Self::MobileChildControlAdapter => proof::CAPABILITY_MOBILE_CHILD_CONTROL,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
        match self {
            Self::ImplementedBoundary => proof::STATE_IMPLEMENTED_BOUNDARY,
            Self::ManualRequired => proof::STATE_MANUAL_REQUIRED,
            Self::Unavailable => proof::STATE_UNAVAILABLE,
            Self::NotClaimed => proof::STATE_NOT_CLAIMED,
            Self::Unsupported => proof::STATE_UNSUPPORTED,
            Self::Degraded => proof::STATE_DEGRADED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
        match self {
            Self::SupportedBoundaryProved => proof::RESULT_SUPPORTED_BOUNDARY_PROVED,
            Self::ManualProofRequired => proof::RESULT_MANUAL_PROOF_REQUIRED,
            Self::TargetUnavailable => proof::RESULT_TARGET_UNAVAILABLE,
            Self::NotClaimed => proof::RESULT_NOT_CLAIMED,
            Self::UnsupportedPlatform => proof::RESULT_UNSUPPORTED_PLATFORM,
            Self::DegradedPermissionOrDependency => proof::RESULT_DEGRADED_PERMISSION_OR_DEPENDENCY,
        }
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
