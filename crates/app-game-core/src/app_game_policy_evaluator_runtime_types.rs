use ocentra_eventing::error::EventingError;
use serde::{Deserialize, Serialize};

use crate::app_game_policy_target_compiler::references::AppGamePolicyAuditRef;
use crate::app_game_policy_target_compiler::types::AppGamePolicyCompilation;

const RUNTIME_REFERENCE_FIELDS: [&str; 3] = [
    "app_game.runtime_session_ref",
    "app_game.runtime_timer_ref",
    "app_game.bonus_approval_ref",
];

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct AppGamePolicyRuntimeReference<const KIND: usize>(String);

impl<const KIND: usize> AppGamePolicyRuntimeReference<KIND> {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(EventingError::EmptyValue {
                field: RUNTIME_REFERENCE_FIELDS[KIND],
            });
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<const KIND: usize> TryFrom<String> for AppGamePolicyRuntimeReference<KIND> {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl<const KIND: usize> From<AppGamePolicyRuntimeReference<KIND>> for String {
    fn from(value: AppGamePolicyRuntimeReference<KIND>) -> Self {
        value.0
    }
}

pub type AppGamePolicyRuntimeSessionRef = AppGamePolicyRuntimeReference<0>;
pub type AppGamePolicyRuntimeTimerRef = AppGamePolicyRuntimeReference<1>;
pub type AppGamePolicyBonusApprovalRef = AppGamePolicyRuntimeReference<2>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGamePolicySessionAccounting {
    Counted,
    Excluded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGamePolicyDurationSource {
    AuthoritativeSession,
    RecoveredJournal,
    ManualEstimate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGamePolicyScheduleState {
    NotRequired,
    Active,
    OutsideWindow,
    Stale,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "kebab-case")]
pub enum AppGamePolicyBonusState {
    None,
    Pending,
    Approved {
        additional_seconds: u64,
        approval_ref: AppGamePolicyBonusApprovalRef,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGamePolicyRuntimeSession {
    pub session_ref: AppGamePolicyRuntimeSessionRef,
    pub duration_seconds: u64,
    pub accounting: AppGamePolicySessionAccounting,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGamePolicyEvaluatorInput {
    pub compilation: AppGamePolicyCompilation,
    pub evaluation_audit_ref: AppGamePolicyAuditRef,
    pub budget_seconds: u64,
    pub warning_threshold_seconds: u64,
    pub sessions: Vec<AppGamePolicyRuntimeSession>,
    pub duration_source: AppGamePolicyDurationSource,
    pub schedule_state: AppGamePolicyScheduleState,
    pub bonus_state: AppGamePolicyBonusState,
    pub timer_ref: Option<AppGamePolicyRuntimeTimerRef>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGamePolicyRuntimeDecisionState {
    Observe,
    ApprovedBonusObserve,
    WarnOnly,
    AskParent,
    DryRunTimeLimit,
    ManualRequired,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGamePolicyRuntimeDecisionReason {
    WithinBudget,
    WarningThresholdReached,
    BudgetExceeded,
    ApprovedBonusActive,
    BonusApprovalPending,
    OutsideSchedule,
    StaleSchedule,
    UntrustedDurationSource,
    MissingTimerReference,
    CompilerManualRequired,
    CompilerRejected,
    DurationOverflow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGamePolicyRuntimeAdapterDispatchState {
    NotDispatched,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGamePolicyRuntimeDecision {
    pub state: AppGamePolicyRuntimeDecisionState,
    pub reason: AppGamePolicyRuntimeDecisionReason,
    pub consumed_seconds: u64,
    pub effective_budget_seconds: u64,
    pub remaining_seconds: u64,
    pub counted_session_refs: Vec<AppGamePolicyRuntimeSessionRef>,
    pub excluded_session_refs: Vec<AppGamePolicyRuntimeSessionRef>,
    pub timer_ref: Option<AppGamePolicyRuntimeTimerRef>,
    pub bonus_approval_ref: Option<AppGamePolicyBonusApprovalRef>,
    pub audit_ref: AppGamePolicyAuditRef,
    pub adapter_dispatch_state: AppGamePolicyRuntimeAdapterDispatchState,
}
