use serde::{Deserialize, Serialize};

use crate::ActivityEvidenceRef;

pub const APP_GAME_POLICY_EVALUATION_CUSTODY_CHILD_DEVICE_QUERY_STORE: &str =
    "child-device-query-store";
pub const APP_GAME_POLICY_EVALUATION_STATUS_READY: &str = "policy-evaluation-ready";
pub const APP_GAME_POLICY_EVALUATION_STATUS_MANUAL_REQUIRED: &str =
    "policy-evaluation-manual-required";
pub const APP_GAME_POLICY_EVALUATION_STATUS_NO_ROWS: &str = "policy-evaluation-no-rows";
pub const APP_GAME_POLICY_EVALUATION_KIND_TIME_LIMIT: &str = "timeLimit";
pub const APP_GAME_POLICY_EVALUATION_KIND_APPROVAL_REQUEST: &str = "approvalRequest";
pub const APP_GAME_POLICY_EVALUATION_KIND_CATEGORY_RISK_REVIEW: &str = "categoryRiskReview";
pub const APP_GAME_POLICY_EVALUATION_KIND_BLOCK_LAUNCH: &str = "blockLaunch";
pub const APP_GAME_POLICY_EVALUATION_REQUESTED_ACTION_TIME_LIMIT: &str = "time-limit";
pub const APP_GAME_POLICY_EVALUATION_REQUESTED_ACTION_ASK_PARENT: &str = "ask-parent";
pub const APP_GAME_POLICY_EVALUATION_REQUESTED_ACTION_WARN: &str = "warn";
pub const APP_GAME_POLICY_EVALUATION_REQUESTED_ACTION_BLOCK_LAUNCH: &str = "block-launch";
pub const APP_GAME_POLICY_EVALUATION_POLICY_ACTION_TIME_LIMIT: &str = "time-limit";
pub const APP_GAME_POLICY_EVALUATION_POLICY_ACTION_ASK_PARENT: &str = "ask-parent";
pub const APP_GAME_POLICY_EVALUATION_POLICY_ACTION_WARN: &str = "warn";
pub const APP_GAME_POLICY_EVALUATION_POLICY_ACTION_BLOCK: &str = "block";
pub const APP_GAME_POLICY_EVALUATION_DECISION_DRY_RUN_READY: &str = "dry-run-ready";
pub const APP_GAME_POLICY_EVALUATION_DECISION_MANUAL_REQUIRED: &str = "manual-required";
pub const APP_GAME_POLICY_EVALUATION_REJECTION_NONE: &str = "none";
pub const APP_GAME_POLICY_EVALUATION_REJECTION_MISSING_POLICY_EVIDENCE: &str =
    "missing-policy-evidence";
pub const APP_GAME_POLICY_EVALUATION_REJECTION_MISSING_APPROVAL_AUTHORITY: &str =
    "missing-approval-authority";
pub const APP_GAME_POLICY_EVALUATION_REJECTION_MISSING_PLATFORM_AUTHORITY: &str =
    "missing-platform-authority";
pub const APP_GAME_POLICY_EVALUATION_REJECTION_MISSING_CLASSIFIER_CONTEXT: &str =
    "missing-classifier-context";
pub const APP_GAME_POLICY_EVALUATION_REJECTION_BLOCK_LAUNCH_MANUAL_REQUIRED: &str =
    "block-launch-manual-required";
pub const APP_GAME_POLICY_EVALUATION_REASON_READY: &str = "app-game-policy-readiness-ready";
pub const APP_GAME_POLICY_EVALUATION_REASON_ADAPTER_DISPATCH_DISABLED: &str =
    "adapter-dispatch-disabled";
pub const APP_GAME_POLICY_EVALUATION_REASON_MANUAL_REQUIRED: &str =
    "app-game-policy-manual-required";
pub const APP_GAME_POLICY_EVALUATION_HANDOFF_DISABLED: &str = "disabled";
pub const APP_GAME_POLICY_EVALUATION_ADAPTER_NOT_DISPATCHED: &str = "not-dispatched";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGamePolicyEvaluationRow {
    pub schema_version: u16,
    pub evaluation_id: String,
    pub evaluation_kind: String,
    pub requested_action: String,
    pub policy_action: String,
    pub decision_state: String,
    pub rejection_reason: String,
    pub reason_codes: Vec<String>,
    pub required_readiness_kinds: Vec<String>,
    pub evidence_reference_ids: Vec<String>,
    pub evidence: Vec<ActivityEvidenceRef>,
    pub dry_run: bool,
    pub enforcement_handoff_state: String,
    pub adapter_dispatch_state: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGamePolicyEvaluationReadModel {
    pub schema_version: u16,
    pub generated_at: String,
    pub custody_label: String,
    pub capability_status: String,
    pub returned: u64,
    pub policy_evaluation_ready: bool,
    pub manual_review_required: bool,
    pub dry_run: bool,
    pub enforcement_handoff_state: String,
    pub adapter_dispatch_claimed: bool,
    pub readiness_row_count: u64,
    pub evaluated_row_count: u64,
    pub evidence_claim_row_count: u64,
    pub identity_row_count: u64,
    pub approval_authority_row_count: u64,
    pub approval_action_result_row_count: u64,
    pub platform_authority_row_count: u64,
    pub ai_classifier_result_row_count: u64,
    pub rows: Vec<AppGamePolicyEvaluationRow>,
}
