use serde::{Deserialize, Serialize};

use crate::ActivityEvidenceRef;

pub const APP_GAME_POLICY_READINESS_CUSTODY_CHILD_DEVICE_QUERY_STORE: &str =
    "child-device-query-store";
pub const APP_GAME_POLICY_READINESS_STATUS_READY: &str = "policy-ready";
pub const APP_GAME_POLICY_READINESS_STATUS_PARTIAL: &str = "policy-partial";
pub const APP_GAME_POLICY_READINESS_STATUS_NO_ROWS: &str = "policy-no-rows";
pub const APP_GAME_POLICY_READINESS_KIND_POLICY_EVIDENCE: &str = "policyEvidence";
pub const APP_GAME_POLICY_READINESS_KIND_APPROVAL_AUTHORITY: &str = "approvalAuthority";
pub const APP_GAME_POLICY_READINESS_KIND_APPROVAL_ACTION_RESULT: &str = "approvalActionResult";
pub const APP_GAME_POLICY_READINESS_KIND_PLATFORM_AUTHORITY: &str = "platformAuthority";
pub const APP_GAME_POLICY_READINESS_KIND_AI_CLASSIFIER_CONTEXT: &str = "aiClassifierContext";
pub const APP_GAME_POLICY_READINESS_KIND_CATEGORY_CANDIDATE: &str = "categoryCandidate";
pub const APP_GAME_POLICY_READINESS_KIND_CATEGORY_RISK_ROUTING: &str = "categoryRiskRouting";
pub const APP_GAME_POLICY_READINESS_KIND_UNKNOWN_REVIEW: &str = "unknownReview";
pub const APP_GAME_POLICY_READINESS_STATE_READY: &str = "ready";
pub const APP_GAME_POLICY_READINESS_STATE_MISSING: &str = "missing";
pub const APP_GAME_POLICY_READINESS_STATE_MANUAL_REQUIRED: &str = "manual-required";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGamePolicyReadinessRow {
    pub schema_version: u16,
    pub row_id: String,
    pub readiness_kind: String,
    pub readiness_state: String,
    pub row_count: u64,
    pub evidence_reference_ids: Vec<String>,
    pub evidence: Vec<ActivityEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGamePolicyReadinessReadModel {
    pub schema_version: u16,
    pub generated_at: String,
    pub custody_label: String,
    pub capability_status: String,
    pub returned: u64,
    pub policy_evaluation_ready: bool,
    pub category_routing_ready: bool,
    pub unknown_review_required: bool,
    pub manual_review_required: bool,
    pub adapter_dispatch_claimed: bool,
    pub evidence_claim_row_count: u64,
    pub identity_row_count: u64,
    pub approval_authority_row_count: u64,
    pub approval_action_result_row_count: u64,
    pub platform_authority_row_count: u64,
    pub ai_classifier_result_row_count: u64,
    pub category_candidate_row_count: u64,
    pub unknown_review_row_count: u64,
    pub rows: Vec<AppGamePolicyReadinessRow>,
}
