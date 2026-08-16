use serde::{Deserialize, Serialize};

pub const SOCIAL_DASHBOARD_SCHEMA_VERSION: &str = "social-dashboard-ux-contract";
pub const SOCIAL_DASHBOARD_CUSTODY_CHILD_DEVICE_QUERY_STORE: &str = "child-device-query-store";
pub const SOCIAL_DASHBOARD_CAPABILITY_READY: &str = "service-backed-social-dashboard";
pub const SOCIAL_DASHBOARD_FAMILY_ID: &str = "family-social-dashboard";
pub const SOCIAL_DASHBOARD_CHILD_PROFILE_ID: &str = "child-social-dashboard";
pub const SOCIAL_DASHBOARD_PANEL_ACCOUNT_APPROVAL_QUEUE: &str = "account-approval-queue";
pub const SOCIAL_DASHBOARD_PANEL_FEED_VIDEO_GATES: &str = "feed-video-gates";
pub const SOCIAL_DASHBOARD_PANEL_NATIVE_APP_CAPABILITY: &str = "native-app-capability";
pub const SOCIAL_DASHBOARD_PANEL_CONNECTOR_BOUNDARIES: &str = "connector-boundaries";
pub const SOCIAL_DASHBOARD_PANEL_DECISION_MEMORY: &str = "decision-memory";
pub const SOCIAL_DASHBOARD_PANEL_SETTINGS_CUSTODY: &str = "settings-custody";
pub const SOCIAL_DASHBOARD_PANEL_MANUAL_REQUIRED_GAPS: &str = "manual-required-gaps";
pub const SOCIAL_DASHBOARD_STATUS_READY_FOR_REVIEW: &str = "ready-for-review";
pub const SOCIAL_DASHBOARD_STATUS_MANUAL_REQUIRED: &str = "manual-required";
pub const SOCIAL_DASHBOARD_STATUS_CONTRACT_ONLY: &str = "contract-only";
pub const SOCIAL_DASHBOARD_ACTION_OPEN_PARENT_APPROVAL: &str = "open-parent-approval";
pub const SOCIAL_DASHBOARD_ACTION_REVIEW_FEED_GATE: &str = "review-feed-gate";
pub const SOCIAL_DASHBOARD_ACTION_REVIEW_NATIVE_CAPABILITY: &str = "review-native-capability";
pub const SOCIAL_DASHBOARD_ACTION_REVIEW_CONNECTOR_BOUNDARY: &str = "review-connector-boundary";
pub const SOCIAL_DASHBOARD_ACTION_REVIEW_MEMORY_ENTRY: &str = "review-memory-entry";
pub const SOCIAL_DASHBOARD_ACTION_REVIEW_SETTINGS_CUSTODY: &str = "review-settings-custody";
pub const SOCIAL_DASHBOARD_ACTION_MANUAL_REVIEW: &str = "manual-review";
pub const SOCIAL_DASHBOARD_SEVERITY_INFO: &str = "info";
pub const SOCIAL_DASHBOARD_SEVERITY_WARNING: &str = "warning";
pub const SOCIAL_DASHBOARD_REASON_PARENT_REVIEW_NEEDED: &str = "parent-review-needed";
pub const SOCIAL_DASHBOARD_REASON_FEED_VIDEO_GATE_CANDIDATE: &str = "feed-video-gate-candidate";
pub const SOCIAL_DASHBOARD_REASON_NATIVE_APP_MANUAL_REQUIRED: &str = "native-app-manual-required";
pub const SOCIAL_DASHBOARD_REASON_CONNECTOR_BOUNDARY_MANUAL_REQUIRED: &str =
    "connector-boundary-manual-required";
pub const SOCIAL_DASHBOARD_REASON_MEMORY_CONTRACT_ONLY: &str = "memory-contract-only";
pub const SOCIAL_DASHBOARD_REASON_SETTINGS_CUSTODY_RUNTIME_GAP: &str =
    "settings-custody-runtime-gap";
pub const SOCIAL_DASHBOARD_REASON_PLATFORM_PROOF_GAP: &str = "platform-proof-gap";
pub const SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED: &str = "not-claimed";
pub const SOCIAL_DASHBOARD_EVIDENCE_ACCOUNT_APPROVAL_QUEUE: &str =
    "social-13-managed-browser-account-creation-gate";
pub const SOCIAL_DASHBOARD_EVIDENCE_FEED_VIDEO_GATES: &str =
    "social-14-managed-browser-feed-video-route-gate";
pub const SOCIAL_DASHBOARD_EVIDENCE_NATIVE_APP_CAPABILITY: &str =
    "social-16-android-native-app-capability-matrix";
pub const SOCIAL_DASHBOARD_EVIDENCE_CONNECTOR_BOUNDARIES: &str =
    "social-18-platform-connector-authorization-boundary";
pub const SOCIAL_DASHBOARD_EVIDENCE_DECISION_MEMORY: &str =
    "social-19-memory-cache-account-video-channel-decisions";
pub const SOCIAL_DASHBOARD_EVIDENCE_SETTINGS_CUSTODY: &str =
    "social-video-source-custody-settings-proof";
pub const SOCIAL_DASHBOARD_EVIDENCE_MANUAL_REQUIRED_GAPS: &str =
    "social-24-rollout-manual-required-status-labels";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialDashboardPanel {
    pub panel_id: String,
    pub panel_kind: String,
    pub status: String,
    pub primary_action: String,
    pub severity: String,
    pub sort_order: u64,
    pub source_evidence_refs: Vec<String>,
    pub reasons: Vec<String>,
    pub rendered_ui_claimed: bool,
    pub notification_claimed: bool,
    pub runtime_data_fetch_claimed: bool,
    pub policy_decision_claimed: bool,
    pub native_app_control_claimed: bool,
    pub connector_authorization_claimed: bool,
    pub enforcement_claimed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialDashboardClaimBoundaries {
    pub rendered_portal_ui: String,
    pub notification_delivery: String,
    pub runtime_data_fetch: String,
    pub policy_decision: String,
    pub native_app_control: String,
    pub connector_authorization: String,
    pub enforcement: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialDashboardUxSnapshot {
    pub schema_version: String,
    pub family_id: String,
    pub child_profile_id: String,
    pub generated_at: String,
    pub panels: Vec<SocialDashboardPanel>,
    pub claim_boundaries: SocialDashboardClaimBoundaries,
}
