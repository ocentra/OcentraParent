use serde::{Deserialize, Serialize};

use crate::{
    constants::v08_enforcement_policy_dispatch as dispatch, EnforcementAdapterKind,
    EnforcementCapabilityState, EnforcementMode, ParentActionReference, ParentActorReference,
    ParentDeviceReference, ParentEvidenceReference, ParentPlatform, PolicyAction, PolicyTarget,
    V08EnforcementProductControlParentAction, V08EnforcementProductControlSurface,
};

macro_rules! protocol_str_lookup {
    ($self:expr, [$($value:expr),+ $(,)?]) => {{
        const VALUES: &[&str] = &[$($value),+];
        VALUES[*$self as usize]
    }};
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
        protocol_str_lookup!(
            self,
            [
                dispatch::SOURCE_READY,
                "stale",
                "offline",
                "missing",
                "wrong-device",
                "wrong-route",
                dispatch::SOURCE_UNAVAILABLE,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
        protocol_str_lookup!(
            self,
            [
                dispatch::PROOF_IMPLEMENTED,
                dispatch::PROOF_REPORT_ONLY,
                "degraded",
                "unavailable",
                dispatch::PROOF_MANUAL_REQUIRED,
                dispatch::PROOF_SCAFFOLD,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
        protocol_str_lookup!(
            self,
            [
                dispatch::OUTCOME_DISPATCH_READY,
                "dry-run-only",
                dispatch::OUTCOME_REPORT_ONLY,
                dispatch::OUTCOME_MANUAL_REQUIRED,
                "degraded",
                "unavailable",
                dispatch::OUTCOME_REJECTED,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
        protocol_str_lookup!(
            self,
            [
                dispatch::REJECTION_NONE,
                "missing-actor",
                "wrong-device",
                "missing-policy-decision",
                "stale-policy-version",
                "missing-schedule-or-budget",
                "missing-evidence",
                dispatch::REJECTION_ADAPTER_MANUAL_REQUIRED,
                "adapter-unavailable",
                "source-not-ready",
                "route-not-authorized",
                dispatch::REJECTION_BROAD_CLAIM_NOT_PROVED,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
        protocol_str_lookup!(
            self,
            [
                dispatch::APPROVAL_NOT_REQUIRED,
                dispatch::APPROVAL_PENDING,
                "approved",
                "denied",
                "expired",
                "override-active",
                dispatch::APPROVAL_MANUAL_REQUIRED,
            ]
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
        protocol_str_lookup!(
            self,
            [
                dispatch::TIMER_NOT_REQUIRED,
                dispatch::TIMER_ACTIVE,
                dispatch::TIMER_RESTART_RECOVERED,
                "expired",
                "cancelled",
                "rollback-completed",
                dispatch::TIMER_RECOVERY_NEEDED,
            ]
        )
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
