use serde::{Deserialize, Serialize};

use crate::{constants::v08_enforcement_product_control_spine as spine, ParentPlatform};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08EnforcementProductControlSurface {
    #[serde(rename = "windows-owned-process-time-limit")]
    WindowsOwnedProcessTimeLimit,
    #[serde(rename = "windows-app-time-limit-lifecycle")]
    WindowsAppTimeLimitLifecycle,
    #[serde(rename = "windows-managed-browser-session-intervention")]
    WindowsManagedBrowserSessionIntervention,
    #[serde(rename = "windows-unmanaged-browser-process-fallback")]
    WindowsUnmanagedBrowserProcessFallback,
    #[serde(rename = "windows-policy-dry-run-preview")]
    WindowsPolicyDryRunPreview,
    #[serde(rename = "windows-approval-override-audit")]
    WindowsApprovalOverrideAudit,
    #[serde(rename = "windows-restart-recovery-timer")]
    WindowsRestartRecoveryTimer,
    #[serde(rename = "windows-rollback-audit-boundary")]
    WindowsRollbackAuditBoundary,
    #[serde(rename = "windows-child-facing-explanation")]
    WindowsChildFacingExplanation,
    #[serde(rename = "windows-broad-app-blocking")]
    WindowsBroadAppBlocking,
    #[serde(rename = "windows-network-domain-blocking")]
    WindowsNetworkDomainBlocking,
    #[serde(rename = "windows-managed-exact-url-control")]
    WindowsManagedExactUrlControl,
    #[serde(rename = "windows-unmanaged-exact-url-not-claimed")]
    WindowsUnmanagedExactUrlNotClaimed,
    #[serde(rename = "windows-permission-loss-alerts")]
    WindowsPermissionLossAlerts,
    #[serde(rename = "windows-tamper-uninstall-alerts")]
    WindowsTamperUninstallAlerts,
}

impl V08EnforcementProductControlSurface {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::WindowsOwnedProcessTimeLimit => spine::SURFACE_OWNED_PROCESS,
            Self::WindowsAppTimeLimitLifecycle => spine::SURFACE_APP_TIME_LIMIT,
            Self::WindowsManagedBrowserSessionIntervention => {
                spine::SURFACE_MANAGED_BROWSER_SESSION
            }
            Self::WindowsUnmanagedBrowserProcessFallback => {
                spine::SURFACE_UNMANAGED_BROWSER_PROCESS
            }
            Self::WindowsPolicyDryRunPreview => spine::SURFACE_POLICY_DRY_RUN,
            Self::WindowsApprovalOverrideAudit => spine::SURFACE_APPROVAL_OVERRIDE,
            Self::WindowsRestartRecoveryTimer => spine::SURFACE_RESTART_RECOVERY,
            Self::WindowsRollbackAuditBoundary => spine::SURFACE_ROLLBACK_AUDIT,
            Self::WindowsChildFacingExplanation => spine::SURFACE_CHILD_EXPLANATION,
            Self::WindowsBroadAppBlocking => spine::SURFACE_BROAD_APP,
            Self::WindowsNetworkDomainBlocking => spine::SURFACE_NETWORK_DOMAIN,
            Self::WindowsManagedExactUrlControl => spine::SURFACE_MANAGED_EXACT_URL,
            Self::WindowsUnmanagedExactUrlNotClaimed => spine::SURFACE_UNMANAGED_EXACT_URL,
            Self::WindowsPermissionLossAlerts => spine::SURFACE_PERMISSION_LOSS,
            Self::WindowsTamperUninstallAlerts => spine::SURFACE_TAMPER_UNINSTALL,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08EnforcementProductControlSurfaceKind {
    #[serde(rename = "process")]
    Process,
    #[serde(rename = "app-game")]
    AppGame,
    #[serde(rename = "managed-browser")]
    ManagedBrowser,
    #[serde(rename = "unmanaged-browser")]
    UnmanagedBrowser,
    #[serde(rename = "network-domain")]
    NetworkDomain,
    #[serde(rename = "policy")]
    Policy,
    #[serde(rename = "recovery")]
    Recovery,
    #[serde(rename = "audit")]
    Audit,
    #[serde(rename = "child-explanation")]
    ChildExplanation,
    #[serde(rename = "integrity")]
    Integrity,
}

impl V08EnforcementProductControlSurfaceKind {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Process => spine::KIND_PROCESS,
            Self::AppGame => spine::KIND_APP_GAME,
            Self::ManagedBrowser => spine::KIND_MANAGED_BROWSER,
            Self::UnmanagedBrowser => spine::KIND_UNMANAGED_BROWSER,
            Self::NetworkDomain => spine::KIND_NETWORK_DOMAIN,
            Self::Policy => spine::KIND_POLICY,
            Self::Recovery => spine::KIND_RECOVERY,
            Self::Audit => spine::KIND_AUDIT,
            Self::ChildExplanation => spine::KIND_CHILD_EXPLANATION,
            Self::Integrity => spine::KIND_INTEGRITY,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08EnforcementProductControlCapabilityName {
    #[serde(rename = "owned-process-terminate")]
    OwnedProcessTerminate,
    #[serde(rename = "app-time-limit")]
    AppTimeLimit,
    #[serde(rename = "app-blocking")]
    AppBlocking,
    #[serde(rename = "network-domain-blocking")]
    NetworkDomainBlocking,
    #[serde(rename = "managed-browser-control")]
    ManagedBrowserControl,
    #[serde(rename = "unmanaged-browser-detection")]
    UnmanagedBrowserDetection,
    #[serde(rename = "typed-protocol-bridge")]
    TypedProtocolBridge,
    #[serde(rename = "notifications")]
    Notifications,
    #[serde(rename = "package-lifecycle")]
    PackageLifecycle,
}

impl V08EnforcementProductControlCapabilityName {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::OwnedProcessTerminate => spine::CAPABILITY_OWNED_PROCESS_TERMINATE,
            Self::AppTimeLimit => spine::CAPABILITY_APP_TIME_LIMIT,
            Self::AppBlocking => spine::CAPABILITY_APP_BLOCKING,
            Self::NetworkDomainBlocking => spine::CAPABILITY_NETWORK_DOMAIN_BLOCKING,
            Self::ManagedBrowserControl => spine::CAPABILITY_MANAGED_BROWSER_CONTROL,
            Self::UnmanagedBrowserDetection => spine::CAPABILITY_UNMANAGED_BROWSER_DETECTION,
            Self::TypedProtocolBridge => spine::CAPABILITY_TYPED_PROTOCOL_BRIDGE,
            Self::Notifications => spine::CAPABILITY_NOTIFICATIONS,
            Self::PackageLifecycle => spine::CAPABILITY_PACKAGE_LIFECYCLE,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08EnforcementProductControlCapabilityStatus {
    #[serde(rename = "implemented")]
    Implemented,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "not-implemented")]
    NotImplemented,
}

impl V08EnforcementProductControlCapabilityStatus {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Implemented => spine::STATUS_IMPLEMENTED,
            Self::ManualRequired => spine::STATUS_MANUAL_REQUIRED,
            Self::NotImplemented => spine::STATUS_NOT_IMPLEMENTED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08EnforcementProductControlClaimState {
    #[serde(rename = "implemented-boundary")]
    ImplementedBoundary,
    #[serde(rename = "degraded-boundary")]
    DegradedBoundary,
    #[serde(rename = "dry-run-only")]
    DryRunOnly,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "not-claimed")]
    NotClaimed,
}

impl V08EnforcementProductControlClaimState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ImplementedBoundary => spine::CLAIM_IMPLEMENTED_BOUNDARY,
            Self::DegradedBoundary => spine::CLAIM_DEGRADED_BOUNDARY,
            Self::DryRunOnly => spine::CLAIM_DRY_RUN_ONLY,
            Self::ManualRequired => spine::CLAIM_MANUAL_REQUIRED,
            Self::Unavailable => spine::CLAIM_UNAVAILABLE,
            Self::NotClaimed => spine::CLAIM_NOT_CLAIMED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08EnforcementProductControlExecutionState {
    #[serde(rename = "executes-real-service")]
    ExecutesRealService,
    #[serde(rename = "returns-dry-run-preview")]
    ReturnsDryRunPreview,
    #[serde(rename = "returns-degraded-noop")]
    ReturnsDegradedNoop,
    #[serde(rename = "returns-manual-required")]
    ReturnsManualRequired,
    #[serde(rename = "returns-unavailable")]
    ReturnsUnavailable,
    #[serde(rename = "not-invoked")]
    NotInvoked,
}

impl V08EnforcementProductControlExecutionState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ExecutesRealService => spine::EXECUTES_REAL_SERVICE,
            Self::ReturnsDryRunPreview => spine::RETURNS_DRY_RUN_PREVIEW,
            Self::ReturnsDegradedNoop => spine::RETURNS_DEGRADED_NOOP,
            Self::ReturnsManualRequired => spine::RETURNS_MANUAL_REQUIRED,
            Self::ReturnsUnavailable => spine::RETURNS_UNAVAILABLE,
            Self::NotInvoked => spine::NOT_INVOKED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08EnforcementProductControlDevicePolicyState {
    #[serde(rename = "control-capable")]
    ControlCapable,
    #[serde(rename = "preview-only")]
    PreviewOnly,
    #[serde(rename = "report-only")]
    ReportOnly,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "not-claimed")]
    NotClaimed,
}

impl V08EnforcementProductControlDevicePolicyState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ControlCapable => spine::DEVICE_POLICY_CONTROL_CAPABLE,
            Self::PreviewOnly => spine::DEVICE_POLICY_PREVIEW_ONLY,
            Self::ReportOnly => spine::DEVICE_POLICY_REPORT_ONLY,
            Self::ManualRequired => spine::DEVICE_POLICY_MANUAL_REQUIRED,
            Self::Unavailable => spine::DEVICE_POLICY_UNAVAILABLE,
            Self::NotClaimed => spine::DEVICE_POLICY_NOT_CLAIMED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08EnforcementProductControlParentAction {
    #[serde(rename = "observe")]
    Observe,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "time-limit")]
    TimeLimit,
    #[serde(rename = "block-scoped-process")]
    BlockScopedProcess,
    #[serde(rename = "ask-parent")]
    AskParent,
    #[serde(rename = "dry-run-preview")]
    DryRunPreview,
    #[serde(rename = "report-only")]
    ReportOnly,
}

impl V08EnforcementProductControlParentAction {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Observe => spine::ACTION_OBSERVE,
            Self::Warn => spine::ACTION_WARN,
            Self::TimeLimit => spine::ACTION_TIME_LIMIT,
            Self::BlockScopedProcess => spine::ACTION_BLOCK_SCOPED_PROCESS,
            Self::AskParent => spine::ACTION_ASK_PARENT,
            Self::DryRunPreview => spine::ACTION_DRY_RUN_PREVIEW,
            Self::ReportOnly => spine::ACTION_REPORT_ONLY,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V08EnforcementProductControlSpineEntry {
    pub schema_version: String,
    pub entry_id: String,
    pub surface: V08EnforcementProductControlSurface,
    pub surface_kind: V08EnforcementProductControlSurfaceKind,
    pub platform: ParentPlatform,
    pub capability: V08EnforcementProductControlCapabilityName,
    pub capability_status: V08EnforcementProductControlCapabilityStatus,
    pub product_claim_state: V08EnforcementProductControlClaimState,
    pub adapter_execution_state: V08EnforcementProductControlExecutionState,
    pub device_policy_state: V08EnforcementProductControlDevicePolicyState,
    pub parent_visible_actions: Vec<V08EnforcementProductControlParentAction>,
    pub linked_proof_commands: Vec<String>,
    pub linked_proof_artifacts: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
    pub claim_boundary: String,
    pub fallback_behavior: String,
    pub broad_app_blocking_claimed: bool,
    pub network_domain_blocking_claimed: bool,
    pub managed_exact_url_blocking_claimed: bool,
    pub unmanaged_exact_url_claimed: bool,
    pub tamper_resistance_claimed: bool,
    pub notification_delivery_claimed: bool,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V08EnforcementProductControlSpineReadModel {
    pub schema_version: String,
    pub read_model_id: String,
    pub generated_at: String,
    pub source_read_model_ids: Vec<String>,
    pub entries: Vec<V08EnforcementProductControlSpineEntry>,
}
