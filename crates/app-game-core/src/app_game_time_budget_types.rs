use ocentra_parent_agent_protocol::app_game::AppGameSessionSummary;
use serde::{Deserialize, Serialize};

use crate::app_game_policy_evaluator_runtime::types::{
    AppGamePolicyBonusApprovalRef, AppGamePolicyDurationSource, AppGamePolicyRuntimeDecision,
    AppGamePolicyRuntimeSessionRef, AppGamePolicyRuntimeTimerRef,
};
use crate::app_game_policy_target_compiler::references::{
    AppGamePolicyAuditRef, AppGamePolicyEvidenceRef, AppGamePolicyScheduleRef,
};
use crate::app_game_policy_target_compiler::types::AppGamePolicyCompilation;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameTimeBudgetPeriod {
    Daily,
    Weekly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameTimeBudgetDurationMode {
    Running,
    Foreground,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "kebab-case")]
pub enum AppGameTimeBudgetSchedule {
    NotRequired,
    Active {
        schedule_ref: AppGamePolicyScheduleRef,
        evidence_refs: Vec<AppGamePolicyEvidenceRef>,
    },
    OutsideWindow {
        schedule_ref: AppGamePolicyScheduleRef,
        evidence_refs: Vec<AppGamePolicyEvidenceRef>,
    },
    Stale {
        schedule_ref: AppGamePolicyScheduleRef,
        evidence_refs: Vec<AppGamePolicyEvidenceRef>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "kebab-case")]
pub enum AppGameTimeBudgetBonus {
    None,
    Pending {
        request_audit_ref: AppGamePolicyAuditRef,
    },
    Approved {
        additional_seconds: u64,
        approval_ref: AppGamePolicyBonusApprovalRef,
        approval_audit_ref: AppGamePolicyAuditRef,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "kebab-case")]
pub enum AppGameTimeBudgetTimer {
    NotRequired,
    Active {
        timer_ref: AppGamePolicyRuntimeTimerRef,
    },
    Recovered {
        timer_ref: AppGamePolicyRuntimeTimerRef,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppGameTimeBudgetInput {
    pub compilation: AppGamePolicyCompilation,
    pub evaluation_audit_ref: AppGamePolicyAuditRef,
    pub period: AppGameTimeBudgetPeriod,
    pub budget_seconds: u64,
    pub warning_threshold_seconds: u64,
    pub stored_sessions: Vec<AppGameSessionSummary>,
    pub duration_mode: AppGameTimeBudgetDurationMode,
    pub duration_source: AppGamePolicyDurationSource,
    pub schedule: AppGameTimeBudgetSchedule,
    pub bonus: AppGameTimeBudgetBonus,
    pub timer: AppGameTimeBudgetTimer,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGameTimeBudgetDecision {
    pub runtime_decision: AppGamePolicyRuntimeDecision,
    pub period: AppGameTimeBudgetPeriod,
    pub duration_mode: AppGameTimeBudgetDurationMode,
    pub stored_session_refs: Vec<AppGamePolicyRuntimeSessionRef>,
    pub schedule_ref: Option<AppGamePolicyScheduleRef>,
    pub schedule_evidence_refs: Vec<AppGamePolicyEvidenceRef>,
    pub bonus_audit_ref: Option<AppGamePolicyAuditRef>,
    pub recovered_timer_ref: Option<AppGamePolicyRuntimeTimerRef>,
}
