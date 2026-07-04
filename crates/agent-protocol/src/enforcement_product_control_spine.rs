use serde::{Deserialize, Serialize};

use crate::{constants::v08_enforcement_product_control_spine as spine, ParentPlatform};

macro_rules! protocol_str_lookup {
    ($self:expr, [$($value:expr),+ $(,)?]) => {{
        const VALUES: &[&str] = &[$($value),+];
        VALUES[*$self as usize]
    }};
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
        protocol_str_lookup!(
            self,
            [
                spine::SURFACE_OWNED_PROCESS,
                spine::SURFACE_APP_TIME_LIMIT,
                spine::SURFACE_MANAGED_BROWSER_SESSION,
                spine::SURFACE_UNMANAGED_BROWSER_PROCESS,
                spine::SURFACE_POLICY_DRY_RUN,
                spine::SURFACE_APPROVAL_OVERRIDE,
                spine::SURFACE_RESTART_RECOVERY,
                spine::SURFACE_ROLLBACK_AUDIT,
                spine::SURFACE_CHILD_EXPLANATION,
                spine::SURFACE_BROAD_APP,
                spine::SURFACE_NETWORK_DOMAIN,
                spine::SURFACE_MANAGED_EXACT_URL,
                spine::SURFACE_UNMANAGED_EXACT_URL,
                spine::SURFACE_PERMISSION_LOSS,
                spine::SURFACE_TAMPER_UNINSTALL,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
        protocol_str_lookup!(
            self,
            [
                spine::KIND_PROCESS,
                spine::KIND_APP_GAME,
                spine::KIND_MANAGED_BROWSER,
                spine::KIND_UNMANAGED_BROWSER,
                spine::KIND_NETWORK_DOMAIN,
                spine::KIND_POLICY,
                spine::KIND_RECOVERY,
                spine::KIND_AUDIT,
                spine::KIND_CHILD_EXPLANATION,
                spine::KIND_INTEGRITY,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
        protocol_str_lookup!(
            self,
            [
                spine::CAPABILITY_OWNED_PROCESS_TERMINATE,
                spine::CAPABILITY_APP_TIME_LIMIT,
                spine::CAPABILITY_APP_BLOCKING,
                spine::CAPABILITY_NETWORK_DOMAIN_BLOCKING,
                spine::CAPABILITY_MANAGED_BROWSER_CONTROL,
                spine::CAPABILITY_UNMANAGED_BROWSER_DETECTION,
                spine::CAPABILITY_TYPED_PROTOCOL_BRIDGE,
                spine::CAPABILITY_NOTIFICATIONS,
                spine::CAPABILITY_PACKAGE_LIFECYCLE,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
        protocol_str_lookup!(
            self,
            [
                spine::STATUS_IMPLEMENTED,
                spine::STATUS_MANUAL_REQUIRED,
                spine::STATUS_NOT_IMPLEMENTED,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
        protocol_str_lookup!(
            self,
            [
                spine::CLAIM_IMPLEMENTED_BOUNDARY,
                spine::CLAIM_DEGRADED_BOUNDARY,
                spine::CLAIM_DRY_RUN_ONLY,
                spine::CLAIM_MANUAL_REQUIRED,
                spine::CLAIM_UNAVAILABLE,
                spine::CLAIM_NOT_CLAIMED,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
        protocol_str_lookup!(
            self,
            [
                spine::EXECUTES_REAL_SERVICE,
                spine::RETURNS_DRY_RUN_PREVIEW,
                spine::RETURNS_DEGRADED_NOOP,
                spine::RETURNS_MANUAL_REQUIRED,
                spine::RETURNS_UNAVAILABLE,
                spine::NOT_INVOKED,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
        protocol_str_lookup!(
            self,
            [
                spine::DEVICE_POLICY_CONTROL_CAPABLE,
                spine::DEVICE_POLICY_PREVIEW_ONLY,
                spine::DEVICE_POLICY_REPORT_ONLY,
                spine::DEVICE_POLICY_MANUAL_REQUIRED,
                spine::DEVICE_POLICY_UNAVAILABLE,
                spine::DEVICE_POLICY_NOT_CLAIMED,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
        protocol_str_lookup!(
            self,
            [
                spine::ACTION_OBSERVE,
                spine::ACTION_WARN,
                spine::ACTION_TIME_LIMIT,
                spine::ACTION_BLOCK_SCOPED_PROCESS,
                spine::ACTION_ASK_PARENT,
                spine::ACTION_DRY_RUN_PREVIEW,
                spine::ACTION_REPORT_ONLY,
            ]
        )
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
