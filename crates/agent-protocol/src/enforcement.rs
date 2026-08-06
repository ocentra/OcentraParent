use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, SchemaVersion};
use serde::{Deserialize, Serialize};

use crate::{
    activity::policy::{ParentActorReference, ParentEvidenceReference, PolicyAction, PolicyTarget},
    activity::policy_context::ParentDeviceReference,
    constants::enforcement as enforcement_constants,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentActionReference {
    pub action_reference_id: String,
    pub actor: ParentActorReference,
    pub policy_version: String,
    pub created_at: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 5] = [
        enforcement_constants::PLATFORM_WINDOWS,
        enforcement_constants::PLATFORM_LINUX,
        enforcement_constants::PLATFORM_MACOS,
        enforcement_constants::PLATFORM_ANDROID,
        enforcement_constants::PLATFORM_IOS,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 4] = [
        enforcement_constants::INTENT_SOURCE_PARENT_PORTAL,
        enforcement_constants::INTENT_SOURCE_PARENT_RULE,
        enforcement_constants::INTENT_SOURCE_LOCAL_POLICY_EVALUATOR,
        enforcement_constants::INTENT_SOURCE_SYSTEM_RECOVERY,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 4] = [
        enforcement_constants::ADAPTER_KIND_PROCESS_CONTROL,
        enforcement_constants::ADAPTER_KIND_NETWORK_CONTROL,
        enforcement_constants::ADAPTER_KIND_MANAGED_BROWSER_CONTROL,
        enforcement_constants::ADAPTER_KIND_TIMER_CONTROL,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 6] = [
        enforcement_constants::MODE_TERMINATE_PROCESS,
        enforcement_constants::MODE_BLOCK_PROCESS,
        enforcement_constants::MODE_TEMPORARY_BLOCK,
        enforcement_constants::MODE_TIME_LIMIT,
        enforcement_constants::MODE_ASK_PARENT,
        enforcement_constants::MODE_OBSERVE_ONLY,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    #[serde(rename = "manual-required")]
    ManualRequired,
}

impl EnforcementCapabilityState {
    const PROTOCOL_STRINGS: [&'static str; 6] = [
        enforcement_constants::CAPABILITY_SUPPORTED,
        enforcement_constants::CAPABILITY_UNAVAILABLE,
        enforcement_constants::CAPABILITY_DEGRADED,
        enforcement_constants::CAPABILITY_DRY_RUN,
        enforcement_constants::CAPABILITY_OBSERVE_ONLY,
        enforcement_constants::CAPABILITY_MANUAL_REQUIRED,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum EnforcementUnavailableReason {
    #[serde(rename = "unsupported-platform")]
    UnsupportedPlatform,
    #[serde(rename = "unsupported-action")]
    UnsupportedAction,
    #[serde(rename = "missing-permission")]
    MissingPermission,
    #[serde(rename = "missing-dependency")]
    MissingDependency,
    #[serde(rename = "adapter-unavailable")]
    AdapterUnavailable,
    #[serde(rename = "adapter-error")]
    AdapterError,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

impl EnforcementUnavailableReason {
    const PROTOCOL_STRINGS: [&'static str; 7] = [
        enforcement_constants::UNAVAILABLE_UNSUPPORTED_PLATFORM,
        enforcement_constants::UNAVAILABLE_UNSUPPORTED_ACTION,
        enforcement_constants::UNAVAILABLE_MISSING_PERMISSION,
        enforcement_constants::UNAVAILABLE_MISSING_DEPENDENCY,
        enforcement_constants::UNAVAILABLE_ADAPTER_UNAVAILABLE,
        enforcement_constants::UNAVAILABLE_ADAPTER_ERROR,
        enforcement_constants::UNAVAILABLE_MANUAL_REQUIRED,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 4] = [
        enforcement_constants::PERMISSION_ALLOWED,
        enforcement_constants::PERMISSION_MISSING,
        enforcement_constants::PERMISSION_NOT_REQUIRED,
        enforcement_constants::PERMISSION_UNKNOWN,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 4] = [
        enforcement_constants::DEPENDENCY_INSTALLED,
        enforcement_constants::DEPENDENCY_MISSING,
        enforcement_constants::DEPENDENCY_NOT_REQUIRED,
        enforcement_constants::DEPENDENCY_UNKNOWN,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 8] = [
        enforcement_constants::RESULT_WOULD_ENFORCE,
        enforcement_constants::RESULT_ACTUALLY_ENFORCED,
        enforcement_constants::RESULT_UNAVAILABLE,
        enforcement_constants::RESULT_FAILED,
        enforcement_constants::RESULT_EXPIRED,
        enforcement_constants::RESULT_ROLLED_BACK,
        enforcement_constants::RESULT_SUPERSEDED,
        enforcement_constants::RESULT_NO_OP,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 6] = [
        enforcement_constants::ROLLBACK_NOT_REQUIRED,
        enforcement_constants::ROLLBACK_AVAILABLE,
        enforcement_constants::ROLLBACK_REQUESTED,
        enforcement_constants::ROLLBACK_COMPLETED,
        enforcement_constants::ROLLBACK_UNAVAILABLE,
        enforcement_constants::ROLLBACK_FAILED,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    const PROTOCOL_STRINGS: [&'static str; 10] = [
        enforcement_constants::ADAPTER_PROCESS_TERMINATED,
        enforcement_constants::ADAPTER_PROCESS_ALREADY_EXITED,
        enforcement_constants::ADAPTER_LEFT_RUNNING_OBSERVE_ONLY,
        enforcement_constants::ADAPTER_DRY_RUN_NO_ACTION,
        enforcement_constants::ADAPTER_UNSUPPORTED_PLATFORM,
        enforcement_constants::ADAPTER_UNAVAILABLE,
        enforcement_constants::ADAPTER_FAILED,
        enforcement_constants::ADAPTER_TIMER_EXPIRED,
        enforcement_constants::ADAPTER_ROLLBACK_COMPLETED,
        enforcement_constants::ADAPTER_NO_OP,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    #[serde(rename = "recovery-needed")]
    RecoveryNeeded,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl EnforcementTimerEventKind {
    const PROTOCOL_STRINGS: [&'static str; 9] = [
        enforcement_constants::TIMER_CREATED,
        enforcement_constants::TIMER_EXTENDED,
        enforcement_constants::TIMER_EXPIRED,
        enforcement_constants::TIMER_CANCELLED,
        enforcement_constants::TIMER_RESTART_RECOVERED,
        enforcement_constants::TIMER_ROLLBACK_REQUESTED,
        enforcement_constants::TIMER_ROLLBACK_COMPLETED,
        enforcement_constants::TIMER_RECOVERY_NEEDED,
        enforcement_constants::TIMER_UNAVAILABLE,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
    #[serde(rename = "cancelled")]
    Cancelled,
}

impl EnforcementAuditEventKind {
    const PROTOCOL_STRINGS: [&'static str; 8] = [
        enforcement_constants::AUDIT_ATTEMPTED,
        enforcement_constants::AUDIT_SUCCEEDED,
        enforcement_constants::AUDIT_FAILED,
        enforcement_constants::AUDIT_ROLLBACK_REQUESTED,
        enforcement_constants::AUDIT_ROLLBACK_COMPLETED,
        enforcement_constants::AUDIT_EXPIRED,
        enforcement_constants::AUDIT_UNAVAILABLE,
        enforcement_constants::AUDIT_CANCELLED,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
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
pub struct EnforcementUnavailableStatus {
    pub schema_version: String,
    pub capability: EnforcementCapabilityStatus,
    pub unavailable_reason: EnforcementUnavailableReason,
    pub retryable: bool,
    pub checked_at: String,
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
    pub capability: EnforcementCapabilityStatus,
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
    pub unavailable_status: Option<EnforcementUnavailableStatus>,
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
    pub capability: EnforcementCapabilityStatus,
    pub unavailable_status: Option<EnforcementUnavailableStatus>,
    pub policy_version: String,
    pub evidence_references: Vec<ParentEvidenceReference>,
    pub actor: Option<ParentActorReference>,
    pub parent_override: Option<ParentActionReference>,
    pub journal_sequence: Option<String>,
    pub observed_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnforcementAuditJournalEvent {
    pub audit_event_id: String,
    pub action_id: String,
    pub result_id: String,
    pub audit_event_kind: EnforcementAuditEventKind,
    pub result_status: EnforcementResultStatus,
    pub adapter_result_code: EnforcementAdapterResultCode,
    pub capability_state: EnforcementCapabilityState,
    pub observed_at: String,
}

impl From<&EnforcementAuditEvent> for EnforcementAuditJournalEvent {
    fn from(audit: &EnforcementAuditEvent) -> Self {
        Self {
            audit_event_id: audit.audit_event_id.clone(),
            action_id: audit.action.action_id.clone(),
            result_id: audit.result.result_id.clone(),
            audit_event_kind: audit.audit_event_kind,
            result_status: audit.result.status,
            adapter_result_code: audit.result.adapter_result_code,
            capability_state: audit.capability.capability_state,
            observed_at: audit.observed_at.clone(),
        }
    }
}

impl DomainEvent for EnforcementAuditJournalEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(enforcement_constants::EVENT_AUDIT_JOURNAL_RECORDED)?,
            SchemaVersion::new(enforcement_constants::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        let mut value = String::from(enforcement_constants::EVENTING_AGGREGATE_AUDIT_PREFIX);
        value.push_str(&self.action_id);
        AggregateKey::parse(value)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value = String::from(enforcement_constants::EVENTING_IDEMPOTENCY_AUDIT_PREFIX);
        value.push_str(&self.audit_event_id);
        IdempotencyKey::parse(value)
    }
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
    pub unavailable_reason: Option<EnforcementUnavailableReason>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnforcementActiveTimerState {
    pub schema_version: String,
    pub state_id: String,
    pub action: EnforcementAction,
    pub result: EnforcementResult,
    pub audit_event: EnforcementAuditEvent,
    pub timer_event: EnforcementTimerEvent,
    pub stored_at: String,
}
