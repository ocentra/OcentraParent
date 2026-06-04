use serde::{Deserialize, Serialize};

use crate::{
    constants::v08_enforcement_policy_dispatch as dispatch, EnforcementAdapterKind,
    EnforcementCapabilityState, EnforcementMode, ParentActionReference, ParentActorReference,
    ParentDeviceReference, ParentEvidenceReference, ParentPlatform, PolicyAction, PolicyTarget,
    V08EnforcementProductControlParentAction, V08EnforcementProductControlSurface,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementPolicyDispatchSourceState {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "wrong-device")]
    WrongDevice,
    #[serde(rename = "wrong-route")]
    WrongRoute,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl EnforcementPolicyDispatchSourceState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Ready => dispatch::SOURCE_READY,
            Self::Unavailable => dispatch::SOURCE_UNAVAILABLE,
            Self::Stale => "stale",
            Self::Offline => "offline",
            Self::Missing => "missing",
            Self::WrongDevice => "wrong-device",
            Self::WrongRoute => "wrong-route",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementPolicyDispatchProofLevel {
    #[serde(rename = "implemented")]
    Implemented,
    #[serde(rename = "report-only")]
    ReportOnly,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "scaffold")]
    Scaffold,
}

impl EnforcementPolicyDispatchProofLevel {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Implemented => dispatch::PROOF_IMPLEMENTED,
            Self::ReportOnly => dispatch::PROOF_REPORT_ONLY,
            Self::ManualRequired => dispatch::PROOF_MANUAL_REQUIRED,
            Self::Scaffold => dispatch::PROOF_SCAFFOLD,
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementPolicyDispatchOutcomeState {
    #[serde(rename = "dispatch-ready")]
    DispatchReady,
    #[serde(rename = "dry-run-only")]
    DryRunOnly,
    #[serde(rename = "report-only")]
    ReportOnly,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "rejected")]
    Rejected,
}

impl EnforcementPolicyDispatchOutcomeState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::DispatchReady => dispatch::OUTCOME_DISPATCH_READY,
            Self::ReportOnly => dispatch::OUTCOME_REPORT_ONLY,
            Self::ManualRequired => dispatch::OUTCOME_MANUAL_REQUIRED,
            Self::Rejected => dispatch::OUTCOME_REJECTED,
            Self::DryRunOnly => dispatch::OUTCOME_DRY_RUN_ONLY,
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementPolicyDispatchRejectionReason {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "missing-actor")]
    MissingActor,
    #[serde(rename = "wrong-device")]
    WrongDevice,
    #[serde(rename = "missing-policy-decision")]
    MissingPolicyDecision,
    #[serde(rename = "stale-policy-version")]
    StalePolicyVersion,
    #[serde(rename = "missing-schedule-or-budget")]
    MissingScheduleOrBudget,
    #[serde(rename = "missing-evidence")]
    MissingEvidence,
    #[serde(rename = "adapter-manual-required")]
    AdapterManualRequired,
    #[serde(rename = "adapter-unavailable")]
    AdapterUnavailable,
    #[serde(rename = "source-not-ready")]
    SourceNotReady,
    #[serde(rename = "route-not-authorized")]
    RouteNotAuthorized,
    #[serde(rename = "broad-claim-not-proved")]
    BroadClaimNotProved,
}

impl EnforcementPolicyDispatchRejectionReason {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::None => dispatch::REJECTION_NONE,
            Self::AdapterManualRequired => dispatch::REJECTION_ADAPTER_MANUAL_REQUIRED,
            Self::BroadClaimNotProved => dispatch::REJECTION_BROAD_CLAIM_NOT_PROVED,
            Self::MissingActor => "missing-actor",
            Self::WrongDevice => "wrong-device",
            Self::MissingPolicyDecision => "missing-policy-decision",
            Self::StalePolicyVersion => "stale-policy-version",
            Self::MissingScheduleOrBudget => "missing-schedule-or-budget",
            Self::MissingEvidence => "missing-evidence",
            Self::AdapterUnavailable => "adapter-unavailable",
            Self::SourceNotReady => "source-not-ready",
            Self::RouteNotAuthorized => "route-not-authorized",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementPolicyDispatchApprovalState {
    #[serde(rename = "not-required")]
    NotRequired,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "denied")]
    Denied,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "override-active")]
    OverrideActive,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

impl EnforcementPolicyDispatchApprovalState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::NotRequired => dispatch::APPROVAL_NOT_REQUIRED,
            Self::Pending => dispatch::APPROVAL_PENDING,
            Self::ManualRequired => dispatch::APPROVAL_MANUAL_REQUIRED,
            Self::Approved => "approved",
            Self::Denied => "denied",
            Self::Expired => "expired",
            Self::OverrideActive => "override-active",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementPolicyDispatchTimerState {
    #[serde(rename = "not-required")]
    NotRequired,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "restart-recovered")]
    RestartRecovered,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "rollback-completed")]
    RollbackCompleted,
    #[serde(rename = "recovery-needed")]
    RecoveryNeeded,
}

impl EnforcementPolicyDispatchTimerState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::NotRequired => dispatch::TIMER_NOT_REQUIRED,
            Self::Active => dispatch::TIMER_ACTIVE,
            Self::RestartRecovered => dispatch::TIMER_RESTART_RECOVERED,
            Self::RecoveryNeeded => dispatch::TIMER_RECOVERY_NEEDED,
            Self::Expired => "expired",
            Self::Cancelled => "cancelled",
            Self::RollbackCompleted => "rollback-completed",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnforcementPolicyDispatchCapabilityMatrixRow {
    pub matrix_id: String,
    pub surface: V08EnforcementProductControlSurface,
    pub platform: ParentPlatform,
    pub adapter_kind: EnforcementAdapterKind,
    pub requested_action: V08EnforcementProductControlParentAction,
    pub mode: EnforcementMode,
    pub capability_state: EnforcementCapabilityState,
    pub proof_level: EnforcementPolicyDispatchProofLevel,
    pub outcome_state: EnforcementPolicyDispatchOutcomeState,
    pub rejection_reason: EnforcementPolicyDispatchRejectionReason,
    pub source_state: EnforcementPolicyDispatchSourceState,
    pub child_reason_code: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnforcementPolicyDispatchIntent {
    pub schema_version: String,
    pub intent_id: String,
    pub actor: ParentActorReference,
    pub device: ParentDeviceReference,
    pub policy_decision_id: String,
    pub policy_decision_ref: String,
    pub policy_version: String,
    pub target: PolicyTarget,
    pub requested_policy_action: PolicyAction,
    pub requested_parent_action: V08EnforcementProductControlParentAction,
    pub schedule_ref: String,
    pub evidence_references: Vec<ParentEvidenceReference>,
    pub approval_ref: Option<ParentActionReference>,
    pub route_ref: String,
    pub source_state: EnforcementPolicyDispatchSourceState,
    pub dry_run: bool,
    pub requested_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnforcementPolicyDispatchReadModelEntry {
    pub schema_version: String,
    pub intent: EnforcementPolicyDispatchIntent,
    pub matrix_row: EnforcementPolicyDispatchCapabilityMatrixRow,
    pub approval_state: EnforcementPolicyDispatchApprovalState,
    pub timer_state: EnforcementPolicyDispatchTimerState,
    pub audit_refs: Vec<String>,
    pub timer_refs: Vec<String>,
    pub child_reason_code: String,
    pub reason_codes: Vec<String>,
    pub dispatched_at: Option<String>,
    pub next_check_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnforcementPolicyDispatchReadModel {
    pub schema_version: String,
    pub read_model_id: String,
    pub generated_at: String,
    pub entries: Vec<EnforcementPolicyDispatchReadModelEntry>,
}
