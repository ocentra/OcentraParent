use serde::{Deserialize, Serialize};

use crate::app_game_policy_evaluator_runtime::types::{
    AppGamePolicyEvaluatorInput, AppGamePolicyRuntimeDecision, AppGamePolicyRuntimeSessionRef,
};
use crate::app_game_policy_target_compiler::references::AppGamePolicyEvidenceRef;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameNativeGameSessionKind {
    KnownGame,
    LauncherOnly,
    LauncherGameCandidate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameNativeGameCandidateApprovalState {
    NotRequired,
    ParentApproved,
    Pending,
    Denied,
    Expired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameNativeGameAdvisorySignalKind {
    Rating,
    UserGeneratedContent,
    Multiplayer,
    PurchaseCapable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGameNativeGameAdvisorySignal {
    pub kind: AppGameNativeGameAdvisorySignalKind,
    pub evidence_ref: AppGamePolicyEvidenceRef,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGameNativeGameBudgetSession {
    pub session_ref: AppGamePolicyRuntimeSessionRef,
    pub duration_seconds: u64,
    pub kind: AppGameNativeGameSessionKind,
    pub candidate_approval_state: AppGameNativeGameCandidateApprovalState,
    pub advisory_signals: Vec<AppGameNativeGameAdvisorySignal>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGameNativeGameBudgetInput {
    pub evaluator_input: AppGamePolicyEvaluatorInput,
    pub sessions: Vec<AppGameNativeGameBudgetSession>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGameNativeGameBudgetDecision {
    pub runtime_decision: AppGamePolicyRuntimeDecision,
    pub counted_known_game_session_refs: Vec<AppGamePolicyRuntimeSessionRef>,
    pub counted_parent_approved_candidate_session_refs: Vec<AppGamePolicyRuntimeSessionRef>,
    pub excluded_launcher_only_session_refs: Vec<AppGamePolicyRuntimeSessionRef>,
    pub excluded_unapproved_candidate_session_refs: Vec<AppGamePolicyRuntimeSessionRef>,
    pub advisory_signals: Vec<AppGameNativeGameAdvisorySignal>,
}
