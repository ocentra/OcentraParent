use serde::{Deserialize, Serialize};

use crate::{
    constants::enforcement as enforcement_constants, ParentActorReference, ParentDeviceReference,
    ParentEvidenceReference, PolicyAction, PolicyTarget,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentPlatform {
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "macos")]
    Macos,
    #[serde(rename = "android")]
    Android,
    #[serde(rename = "ios")]
    Ios,
}

impl ParentPlatform {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Windows => enforcement_constants::PLATFORM_WINDOWS,
            Self::Linux => enforcement_constants::PLATFORM_LINUX,
            Self::Macos => enforcement_constants::PLATFORM_MACOS,
            Self::Android => enforcement_constants::PLATFORM_ANDROID,
            Self::Ios => enforcement_constants::PLATFORM_IOS,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementIntentSource {
    #[serde(rename = "parent-portal")]
    ParentPortal,
    #[serde(rename = "parent-rule")]
    ParentRule,
    #[serde(rename = "local-policy-evaluator")]
    LocalPolicyEvaluator,
    #[serde(rename = "system-recovery")]
    SystemRecovery,
}

impl EnforcementIntentSource {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ParentPortal => enforcement_constants::INTENT_SOURCE_PARENT_PORTAL,
            Self::ParentRule => enforcement_constants::INTENT_SOURCE_PARENT_RULE,
            Self::LocalPolicyEvaluator => {
                enforcement_constants::INTENT_SOURCE_LOCAL_POLICY_EVALUATOR
            }
            Self::SystemRecovery => enforcement_constants::INTENT_SOURCE_SYSTEM_RECOVERY,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementAdapterKind {
    #[serde(rename = "process-control")]
    ProcessControl,
    #[serde(rename = "network-control")]
    NetworkControl,
    #[serde(rename = "managed-browser-control")]
    ManagedBrowserControl,
    #[serde(rename = "timer-control")]
    TimerControl,
}

impl EnforcementAdapterKind {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ProcessControl => enforcement_constants::ADAPTER_KIND_PROCESS_CONTROL,
            Self::NetworkControl => enforcement_constants::ADAPTER_KIND_NETWORK_CONTROL,
            Self::ManagedBrowserControl => {
                enforcement_constants::ADAPTER_KIND_MANAGED_BROWSER_CONTROL
            }
            Self::TimerControl => enforcement_constants::ADAPTER_KIND_TIMER_CONTROL,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementMode {
    #[serde(rename = "terminate-process")]
    TerminateProcess,
    #[serde(rename = "block-process")]
    BlockProcess,
    #[serde(rename = "temporary-block")]
    TemporaryBlock,
    #[serde(rename = "time-limit")]
    TimeLimit,
    #[serde(rename = "ask-parent")]
    AskParent,
    #[serde(rename = "observe-only")]
    ObserveOnly,
}

impl EnforcementMode {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::TerminateProcess => enforcement_constants::MODE_TERMINATE_PROCESS,
            Self::BlockProcess => enforcement_constants::MODE_BLOCK_PROCESS,
            Self::TemporaryBlock => enforcement_constants::MODE_TEMPORARY_BLOCK,
            Self::TimeLimit => enforcement_constants::MODE_TIME_LIMIT,
            Self::AskParent => enforcement_constants::MODE_ASK_PARENT,
            Self::ObserveOnly => enforcement_constants::MODE_OBSERVE_ONLY,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementCapabilityState {
    #[serde(rename = "supported")]
    Supported,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "dry-run")]
    DryRun,
    #[serde(rename = "observe-only")]
    ObserveOnly,
}

impl EnforcementCapabilityState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Supported => enforcement_constants::CAPABILITY_SUPPORTED,
            Self::Unavailable => enforcement_constants::CAPABILITY_UNAVAILABLE,
            Self::Degraded => enforcement_constants::CAPABILITY_DEGRADED,
            Self::DryRun => enforcement_constants::CAPABILITY_DRY_RUN,
            Self::ObserveOnly => enforcement_constants::CAPABILITY_OBSERVE_ONLY,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementPermissionState {
    #[serde(rename = "allowed")]
    Allowed,
    #[serde(rename = "missing-permission")]
    MissingPermission,
    #[serde(rename = "not-required")]
    NotRequired,
    #[serde(rename = "unknown")]
    Unknown,
}

impl EnforcementPermissionState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Allowed => enforcement_constants::PERMISSION_ALLOWED,
            Self::MissingPermission => enforcement_constants::PERMISSION_MISSING,
            Self::NotRequired => enforcement_constants::PERMISSION_NOT_REQUIRED,
            Self::Unknown => enforcement_constants::PERMISSION_UNKNOWN,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementDependencyState {
    #[serde(rename = "installed")]
    Installed,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "not-required")]
    NotRequired,
    #[serde(rename = "unknown")]
    Unknown,
}

impl EnforcementDependencyState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Installed => enforcement_constants::DEPENDENCY_INSTALLED,
            Self::Missing => enforcement_constants::DEPENDENCY_MISSING,
            Self::NotRequired => enforcement_constants::DEPENDENCY_NOT_REQUIRED,
            Self::Unknown => enforcement_constants::DEPENDENCY_UNKNOWN,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementResultStatus {
    #[serde(rename = "would-enforce")]
    WouldEnforce,
    #[serde(rename = "actually-enforced")]
    ActuallyEnforced,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "rolled-back")]
    RolledBack,
    #[serde(rename = "superseded")]
    Superseded,
    #[serde(rename = "no-op")]
    NoOp,
}

impl EnforcementResultStatus {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::WouldEnforce => enforcement_constants::RESULT_WOULD_ENFORCE,
            Self::ActuallyEnforced => enforcement_constants::RESULT_ACTUALLY_ENFORCED,
            Self::Unavailable => enforcement_constants::RESULT_UNAVAILABLE,
            Self::Failed => enforcement_constants::RESULT_FAILED,
            Self::Expired => enforcement_constants::RESULT_EXPIRED,
            Self::RolledBack => enforcement_constants::RESULT_ROLLED_BACK,
            Self::Superseded => enforcement_constants::RESULT_SUPERSEDED,
            Self::NoOp => enforcement_constants::RESULT_NO_OP,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementRollbackState {
    #[serde(rename = "not-required")]
    NotRequired,
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "failed")]
    Failed,
}

impl EnforcementRollbackState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::NotRequired => enforcement_constants::ROLLBACK_NOT_REQUIRED,
            Self::Available => enforcement_constants::ROLLBACK_AVAILABLE,
            Self::Requested => enforcement_constants::ROLLBACK_REQUESTED,
            Self::Completed => enforcement_constants::ROLLBACK_COMPLETED,
            Self::Unavailable => enforcement_constants::ROLLBACK_UNAVAILABLE,
            Self::Failed => enforcement_constants::ROLLBACK_FAILED,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementAdapterResultCode {
    #[serde(rename = "process-terminated")]
    ProcessTerminated,
    #[serde(rename = "process-already-exited")]
    ProcessAlreadyExited,
    #[serde(rename = "left-running-observe-only")]
    LeftRunningObserveOnly,
    #[serde(rename = "dry-run-no-action")]
    DryRunNoAction,
    #[serde(rename = "unsupported-platform")]
    UnsupportedPlatform,
    #[serde(rename = "adapter-unavailable")]
    AdapterUnavailable,
    #[serde(rename = "adapter-failed")]
    AdapterFailed,
    #[serde(rename = "timer-expired")]
    TimerExpired,
    #[serde(rename = "rollback-completed")]
    RollbackCompleted,
    #[serde(rename = "no-op")]
    NoOp,
}

impl EnforcementAdapterResultCode {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ProcessTerminated => enforcement_constants::ADAPTER_PROCESS_TERMINATED,
            Self::ProcessAlreadyExited => enforcement_constants::ADAPTER_PROCESS_ALREADY_EXITED,
            Self::LeftRunningObserveOnly => {
                enforcement_constants::ADAPTER_LEFT_RUNNING_OBSERVE_ONLY
            }
            Self::DryRunNoAction => enforcement_constants::ADAPTER_DRY_RUN_NO_ACTION,
            Self::UnsupportedPlatform => enforcement_constants::ADAPTER_UNSUPPORTED_PLATFORM,
            Self::AdapterUnavailable => enforcement_constants::ADAPTER_UNAVAILABLE,
            Self::AdapterFailed => enforcement_constants::ADAPTER_FAILED,
            Self::TimerExpired => enforcement_constants::ADAPTER_TIMER_EXPIRED,
            Self::RollbackCompleted => enforcement_constants::ADAPTER_ROLLBACK_COMPLETED,
            Self::NoOp => enforcement_constants::ADAPTER_NO_OP,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementTimerEventKind {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "extended")]
    Extended,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "restart-recovered")]
    RestartRecovered,
    #[serde(rename = "rollback-requested")]
    RollbackRequested,
    #[serde(rename = "rollback-completed")]
    RollbackCompleted,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl EnforcementTimerEventKind {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Created => enforcement_constants::TIMER_CREATED,
            Self::Extended => enforcement_constants::TIMER_EXTENDED,
            Self::Expired => enforcement_constants::TIMER_EXPIRED,
            Self::Cancelled => enforcement_constants::TIMER_CANCELLED,
            Self::RestartRecovered => enforcement_constants::TIMER_RESTART_RECOVERED,
            Self::RollbackRequested => enforcement_constants::TIMER_ROLLBACK_REQUESTED,
            Self::RollbackCompleted => enforcement_constants::TIMER_ROLLBACK_COMPLETED,
            Self::Unavailable => enforcement_constants::TIMER_UNAVAILABLE,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementAuditEventKind {
    #[serde(rename = "attempted")]
    Attempted,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "rollback-requested")]
    RollbackRequested,
    #[serde(rename = "rollback-completed")]
    RollbackCompleted,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl EnforcementAuditEventKind {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Attempted => enforcement_constants::AUDIT_ATTEMPTED,
            Self::Succeeded => enforcement_constants::AUDIT_SUCCEEDED,
            Self::Failed => enforcement_constants::AUDIT_FAILED,
            Self::RollbackRequested => enforcement_constants::AUDIT_ROLLBACK_REQUESTED,
            Self::RollbackCompleted => enforcement_constants::AUDIT_ROLLBACK_COMPLETED,
            Self::Expired => enforcement_constants::AUDIT_EXPIRED,
            Self::Unavailable => enforcement_constants::AUDIT_UNAVAILABLE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentActionReference {
    pub action_reference_id: String,
    pub actor: ParentActorReference,
    pub policy_version: String,
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnforcementCapabilityStatus {
    pub schema_version: String,
    pub platform: ParentPlatform,
    pub adapter_kind: EnforcementAdapterKind,
    pub capability_state: EnforcementCapabilityState,
    pub permission_state: EnforcementPermissionState,
    pub dependency_state: EnforcementDependencyState,
    pub supported_actions: Vec<EnforcementMode>,
    pub degraded_reason: Option<String>,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnforcementIntent {
    pub schema_version: String,
    pub intent_id: String,
    pub source: EnforcementIntentSource,
    pub actor: Option<ParentActorReference>,
    pub device: ParentDeviceReference,
    pub policy_decision_id: String,
    pub target: PolicyTarget,
    pub requested_action: PolicyAction,
    pub evidence_references: Vec<ParentEvidenceReference>,
    pub parent_approval: Option<ParentActionReference>,
    pub idempotency_key: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnforcementAction {
    pub schema_version: String,
    pub action_id: String,
    pub intent_id: String,
    pub policy_decision_id: String,
    pub policy_action: PolicyAction,
    pub adapter_kind: EnforcementAdapterKind,
    pub platform: ParentPlatform,
    pub target: PolicyTarget,
    pub mode: EnforcementMode,
    pub reason_codes: Vec<String>,
    pub evidence_references: Vec<ParentEvidenceReference>,
    pub local_ai_result_id: Option<String>,
    pub parent_approval: Option<ParentActionReference>,
    pub dry_run: bool,
    pub requested_at: String,
    pub expires_at: Option<String>,
    pub rollback_token: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnforcementResult {
    pub schema_version: String,
    pub result_id: String,
    pub action_id: String,
    pub status: EnforcementResultStatus,
    pub adapter_result_code: EnforcementAdapterResultCode,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub rollback_token: Option<String>,
    pub rollback_state: EnforcementRollbackState,
    pub unavailable_reason: Option<String>,
    pub failed_reason: Option<String>,
    pub next_check_at: Option<String>,
    pub capability: EnforcementCapabilityStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnforcementAuditEvent {
    pub schema_version: String,
    pub audit_event_id: String,
    pub audit_event_kind: EnforcementAuditEventKind,
    pub action: EnforcementAction,
    pub result: EnforcementResult,
    pub policy_version: String,
    pub evidence_references: Vec<ParentEvidenceReference>,
    pub actor: Option<ParentActorReference>,
    pub parent_override: Option<ParentActionReference>,
    pub journal_sequence: Option<String>,
    pub observed_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnforcementTimerEvent {
    pub schema_version: String,
    pub timer_event_id: String,
    pub timer_event_kind: EnforcementTimerEventKind,
    pub action_id: String,
    pub policy_decision_id: String,
    pub evidence_references: Vec<ParentEvidenceReference>,
    pub scheduled_at: String,
    pub effective_at: Option<String>,
    pub rollback_token: Option<String>,
    pub recovered_after_restart: bool,
    pub unavailable_reason: Option<String>,
}
