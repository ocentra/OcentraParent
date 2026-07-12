use serde::{Deserialize, Serialize};

use crate::{
    integrity_alert_status_bridge::V08IntegrityAlertStatusBridgeReadModel,
    notification_provider_status_boundary::V08NotificationProviderStatusBoundaryReadModel,
    ParentPlatform,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08EnforcementIntegrityRuntimeAuditSurface {
    #[serde(rename = "app-game-time-limit")]
    AppGameTimeLimit,
    #[serde(rename = "managed-browser-session")]
    ManagedBrowserSession,
    #[serde(rename = "unmanaged-browser-process-fallback")]
    UnmanagedBrowserProcessFallback,
    #[serde(rename = "network-domain-observe-only")]
    NetworkDomainObserveOnly,
    #[serde(rename = "host-network-domain-filter")]
    HostNetworkDomainFilter,
    #[serde(rename = "notification-delivery")]
    NotificationDelivery,
    #[serde(rename = "integrity-heartbeat")]
    IntegrityHeartbeat,
    #[serde(rename = "tamper-uninstall-signal")]
    TamperUninstallSignal,
    #[serde(rename = "mobile-child-control")]
    MobileChildControl,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum V08EnforcementIntegrityRuntimeAuditResult {
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "rolled-back")]
    RolledBack,
    #[serde(rename = "superseded")]
    Superseded,
    #[serde(rename = "no-op")]
    NoOp,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "observe-only")]
    ObserveOnly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08EnforcementIntegrityRuntimeAuditExecution {
    #[serde(rename = "executed-supported-boundary")]
    ExecutedSupportedBoundary,
    #[serde(rename = "dry-run-no-adapter-execution")]
    DryRunNoAdapterExecution,
    #[serde(rename = "rejected-before-adapter")]
    RejectedBeforeAdapter,
    #[serde(rename = "manual-required-no-execution")]
    ManualRequiredNoExecution,
    #[serde(rename = "observe-only-no-execution")]
    ObserveOnlyNoExecution,
    #[serde(rename = "unavailable-no-execution")]
    UnavailableNoExecution,
    #[serde(rename = "unsupported-no-execution")]
    UnsupportedNoExecution,
    #[serde(rename = "recovery-needed-no-execution")]
    RecoveryNeededNoExecution,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08EnforcementIntegrityRuntimeAuditIntentState {
    #[serde(rename = "validated")]
    Validated,
    #[serde(rename = "observe-only")]
    ObserveOnly,
    #[serde(rename = "rejected-invalid")]
    RejectedInvalid,
    #[serde(rename = "rejected-stale")]
    RejectedStale,
    #[serde(rename = "rejected-wrong-device")]
    RejectedWrongDevice,
    #[serde(rename = "rejected-unsupported")]
    RejectedUnsupported,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08EnforcementIntegrityRuntimeAuditTimerState {
    #[serde(rename = "active-timer-backed")]
    ActiveTimerBacked,
    #[serde(rename = "expired-backed")]
    ExpiredBacked,
    #[serde(rename = "cancelled-backed")]
    CancelledBacked,
    #[serde(rename = "rollback-backed")]
    RollbackBacked,
    #[serde(rename = "recovery-needed")]
    RecoveryNeeded,
    #[serde(rename = "not-applicable")]
    NotApplicable,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08EnforcementIntegrityRuntimeAuditRollbackState {
    #[serde(rename = "not-needed")]
    NotNeeded,
    #[serde(rename = "rollback-token-backed")]
    RollbackTokenBacked,
    #[serde(rename = "rollback-completed")]
    RollbackCompleted,
    #[serde(rename = "rollback-required")]
    RollbackRequired,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08EnforcementIntegrityRuntimeAuditChildState {
    #[serde(rename = "status-ref-backed")]
    StatusRefBacked,
    #[serde(rename = "reason-ref-backed")]
    ReasonRefBacked,
    #[serde(rename = "approval-intent-backed")]
    ApprovalIntentBacked,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "not-claimed")]
    NotClaimed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum V08EnforcementIntegrityRuntimeAuditIntegrityState {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "permission-missing")]
    PermissionMissing,
    #[serde(rename = "adapter-unavailable")]
    AdapterUnavailable,
    #[serde(rename = "stale-heartbeat")]
    StaleHeartbeat,
    #[serde(rename = "service-stopped")]
    ServiceStopped,
    #[serde(rename = "uninstall-detection-manual-required")]
    UninstallDetectionManualRequired,
    #[serde(rename = "tamper-signal-manual-required")]
    TamperSignalManualRequired,
    #[serde(rename = "anti-tamper-not-claimed")]
    AntiTamperNotClaimed,
    #[serde(rename = "not-applicable")]
    NotApplicable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum V08EnforcementIntegrityRuntimeAuditAuditState {
    #[serde(rename = "audit-backed")]
    AuditBacked,
    #[serde(rename = "audit-required")]
    AuditRequired,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "not-claimed")]
    NotClaimed,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V08EnforcementIntegrityRuntimeAuditEntry {
    pub schema_version: String,
    pub audit_entry_id: String,
    pub surface: V08EnforcementIntegrityRuntimeAuditSurface,
    pub platform: ParentPlatform,
    pub result: V08EnforcementIntegrityRuntimeAuditResult,
    pub execution: V08EnforcementIntegrityRuntimeAuditExecution,
    pub intent_state: V08EnforcementIntegrityRuntimeAuditIntentState,
    pub timer_state: V08EnforcementIntegrityRuntimeAuditTimerState,
    pub rollback_state: V08EnforcementIntegrityRuntimeAuditRollbackState,
    pub child_state: V08EnforcementIntegrityRuntimeAuditChildState,
    pub integrity_state: V08EnforcementIntegrityRuntimeAuditIntegrityState,
    pub audit_state: V08EnforcementIntegrityRuntimeAuditAuditState,
    pub policy_decision_refs: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub adapter_outcome_refs: Vec<String>,
    pub audit_refs: Vec<String>,
    pub rollback_refs: Vec<String>,
    pub timer_refs: Vec<String>,
    pub child_status_refs: Vec<String>,
    pub integrity_refs: Vec<String>,
    pub parent_intent_refs: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
    pub boundary: String,
    pub broad_installed_app_blocking_claimed: bool,
    pub host_network_domain_blocking_claimed: bool,
    pub exact_active_tab_enforcement_claimed: bool,
    pub notification_delivery_claimed: bool,
    pub tamper_hardening_claimed: bool,
    pub mobile_privilege_claimed: bool,
    pub stealth_persistence_claimed: bool,
    pub privilege_escalation_claimed: bool,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V08EnforcementIntegrityRuntimeAuditReadModel {
    pub schema_version: String,
    pub read_model_id: String,
    pub generated_at: String,
    pub source_read_model_ids: Vec<String>,
    pub entries: Vec<V08EnforcementIntegrityRuntimeAuditEntry>,
    pub integrity_alert_status_bridge: V08IntegrityAlertStatusBridgeReadModel,
    pub notification_provider_status_boundary: V08NotificationProviderStatusBoundaryReadModel,
}
