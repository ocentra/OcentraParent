use serde::{Deserialize, Serialize};

pub const SOCIAL_AUDIT_EXPLANATION_SCHEMA_VERSION: &str = "social-audit-explanation-read-model";
pub const SOCIAL_AUDIT_EXPLANATION_SNAPSHOT_ID: &str = "social-audit-explanation-service-snapshot";
pub const SOCIAL_AUDIT_EXPLANATION_FAMILY_ID: &str = "family-social-audit-service";
pub const SOCIAL_AUDIT_EXPLANATION_CHILD_PROFILE_ID: &str = "child-social-audit-service";
pub const SOCIAL_AUDIT_EXPLANATION_POLICY_VERSION: &str = "policy-version-social-audit-service";
pub const SOCIAL_AUDIT_EXPLANATION_AUDIT_REF: &str = "parent-evidence-social-audit-service";
pub const SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED: &str = "not-claimed";
pub const SOCIAL_AUDIT_EXPLANATION_STATUS_READY_FOR_PARENT: &str = "ready-for-parent";
pub const SOCIAL_AUDIT_EXPLANATION_STATUS_MANUAL_REQUIRED: &str = "manual-required";
pub const SOCIAL_AUDIT_EXPLANATION_STATUS_CONTRACT_ONLY: &str = "contract-only";
pub const SOCIAL_AUDIT_EXPLANATION_DECISION_PARENT_RECORDED: &str = "parent-recorded";
pub const SOCIAL_AUDIT_EXPLANATION_DECISION_CANDIDATE_ONLY: &str = "candidate-only";
pub const SOCIAL_AUDIT_EXPLANATION_DECISION_MANUAL_REQUIRED: &str = "manual-required";
pub const SOCIAL_AUDIT_EXPLANATION_AUDIENCE_PARENT: &str = "parent";
pub const SOCIAL_AUDIT_EXPLANATION_ACTION_PARENT_REVIEW: &str = "parent-review-candidate";
pub const SOCIAL_AUDIT_EXPLANATION_ACTION_WARN: &str = "warn-candidate";
pub const SOCIAL_AUDIT_EXPLANATION_ACTION_ALLOW: &str = "allow-candidate";
pub const SOCIAL_AUDIT_EXPLANATION_ACTION_MANUAL_REVIEW: &str = "manual-review-candidate";
pub const SOCIAL_AUDIT_EXPLANATION_REASON_EVIDENCE_LINKED: &str = "evidence-linked";
pub const SOCIAL_AUDIT_EXPLANATION_REASON_POLICY_CANDIDATE_LINKED: &str = "policy-candidate-linked";
pub const SOCIAL_AUDIT_EXPLANATION_REASON_PARENT_DECISION_LINKED: &str = "parent-decision-linked";
pub const SOCIAL_AUDIT_EXPLANATION_REASON_NATIVE_APP_MANUAL_REQUIRED: &str =
    "native-app-manual-required";
pub const SOCIAL_AUDIT_EXPLANATION_REASON_CONNECTOR_BOUNDARY_LINKED: &str =
    "connector-boundary-linked";
pub const SOCIAL_AUDIT_EXPLANATION_REASON_MEMORY_LINKED: &str = "memory-linked";
pub const SOCIAL_AUDIT_EXPLANATION_REASON_MANUAL_REVIEW_REQUIRED: &str = "manual-review-required";
pub const SOCIAL_AUDIT_EXPLANATION_REASON_MISSING_RUNTIME_PROOF: &str = "missing-runtime-proof";
pub const SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_PARENT_RULE_MATCH: &str = "parent-rule-match";
pub const SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_SOCIAL_RISK_HIGH: &str = "social-risk-high";
pub const SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_VIDEO_SAFETY_RISK: &str = "video-safety-risk";
pub const SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_MANUAL_REQUIRED: &str = "manual-required";
pub const SOCIAL_AUDIT_EXPLANATION_EVIDENCE_POLICY_CANDIDATE: &str = "policy-candidate";
pub const SOCIAL_AUDIT_EXPLANATION_EVIDENCE_PARENT_APPROVAL: &str = "parent-approval";
pub const SOCIAL_AUDIT_EXPLANATION_EVIDENCE_ROUTE_EVIDENCE: &str = "route-evidence";
pub const SOCIAL_AUDIT_EXPLANATION_EVIDENCE_NATIVE_CAPABILITY: &str = "native-capability";
pub const SOCIAL_AUDIT_EXPLANATION_EVIDENCE_CONNECTOR_BOUNDARY: &str = "connector-boundary";
pub const SOCIAL_AUDIT_EXPLANATION_EVIDENCE_DECISION_MEMORY: &str = "decision-memory";
pub const SOCIAL_AUDIT_EXPLANATION_EVIDENCE_MANUAL_GAP: &str = "manual-gap";
pub const SOCIAL_AUDIT_EXPLANATION_SUBJECT_ACCOUNT_APPROVAL: &str = "account-approval";
pub const SOCIAL_AUDIT_EXPLANATION_SUBJECT_FEED_VIDEO_GATE: &str = "feed-video-gate";
pub const SOCIAL_AUDIT_EXPLANATION_SUBJECT_NATIVE_APP_GAP: &str = "native-app-gap";
pub const SOCIAL_AUDIT_EXPLANATION_SUBJECT_CONNECTOR_BOUNDARY: &str = "connector-boundary";
pub const SOCIAL_AUDIT_EXPLANATION_SUBJECT_DECISION_MEMORY: &str = "decision-memory";
pub const SOCIAL_AUDIT_EXPLANATION_SUBJECT_MANUAL_REQUIRED_GAP: &str = "manual-required-gap";
pub const SOCIAL_AUDIT_EXPLANATION_REF_APPROVAL_REQUEST: &str =
    "parent-evidence-approval-request-service";
pub const SOCIAL_AUDIT_EXPLANATION_REF_APPROVAL_DECISION: &str =
    "parent-evidence-approval-decision-service";
pub const SOCIAL_AUDIT_EXPLANATION_REF_NATIVE_CAPABILITY: &str =
    "parent-evidence-native-capability-service";
pub const SOCIAL_AUDIT_EXPLANATION_REF_CONNECTOR_BOUNDARY: &str =
    "parent-evidence-connector-boundary-service";
pub const SOCIAL_AUDIT_EXPLANATION_REF_DECISION_MEMORY: &str =
    "parent-evidence-decision-memory-service";
pub const SOCIAL_AUDIT_EXPLANATION_REF_MANUAL_GAP: &str = "parent-evidence-manual-gap-service";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialAuditExplanationEvidenceLink {
    pub evidence_kind: String,
    pub evidence_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialAuditExplanationEntry {
    pub event_id: String,
    pub subject_kind: String,
    pub status: String,
    pub decision_state: String,
    pub audience: String,
    pub policy_version_ref: Option<String>,
    pub action_candidate: String,
    pub policy_reason_codes: Vec<String>,
    pub explanation_reasons: Vec<String>,
    pub evidence_links: Vec<SocialAuditExplanationEvidenceLink>,
    pub audit_refs: Vec<String>,
    pub parent_approval_request_ref: Option<String>,
    pub parent_approval_decision_ref: Option<String>,
    pub decision_memory_ref: Option<String>,
    pub connector_boundary_ref: Option<String>,
    pub native_capability_ref: Option<String>,
    pub manual_required_ref: Option<String>,
    pub runtime_audit_store_claimed: bool,
    pub rendered_explanation_ui_claimed: bool,
    pub notification_delivered_claimed: bool,
    pub raw_account_data_included: bool,
    pub raw_video_content_included: bool,
    pub raw_message_content_included: bool,
    pub connector_authorization_claimed: bool,
    pub native_app_control_claimed: bool,
    pub final_policy_decision_claimed: bool,
    pub enforcement_claimed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialAuditExplanationClaimBoundaries {
    pub runtime_audit_store: String,
    pub rendered_explanation_ui: String,
    pub notification_delivery: String,
    pub raw_account_video_message_content: String,
    pub connector_authorization: String,
    pub native_app_control: String,
    pub final_policy_decision: String,
    pub enforcement: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialAuditExplanationSnapshot {
    pub schema_version: String,
    pub snapshot_id: String,
    pub family_id: String,
    pub child_profile_id: String,
    pub captured_at: String,
    pub entries: Vec<SocialAuditExplanationEntry>,
    pub claim_boundaries: SocialAuditExplanationClaimBoundaries,
}
