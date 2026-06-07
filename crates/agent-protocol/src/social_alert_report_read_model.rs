use serde::{Deserialize, Serialize};

pub const SOCIAL_ALERT_REPORT_SCHEMA_VERSION: &str = "social-alert-report-read-model";
pub const SOCIAL_ALERT_REPORT_FAMILY_ID: &str = "family-social-alert-report-service";
pub const SOCIAL_ALERT_REPORT_CHILD_PROFILE_ID: &str = "child-social-alert-report-service";
pub const SOCIAL_ALERT_REPORT_CONTRACT_SCHEMA_VERSION: &str = "v0.6";
pub const SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED: &str = "not-claimed";
pub const SOCIAL_ALERT_REPORT_CAPABILITY_READY: &str = "service-backed-social-alert-report";
pub const SOCIAL_ALERT_REPORT_INTENT_HIGH_RISK: &str = "high-risk-signal";
pub const SOCIAL_ALERT_REPORT_INTENT_MANUAL_REQUIRED: &str = "manual-required";
pub const SOCIAL_ALERT_REPORT_STATUS_LOCAL_OUTBOX: &str = "local-outbox-eligible";
pub const SOCIAL_ALERT_REPORT_STATUS_MANUAL_REQUIRED: &str = "manual-required";
pub const SOCIAL_ALERT_REPORT_PRIORITY_URGENT: &str = "urgent";
pub const SOCIAL_ALERT_REPORT_PRIORITY_ATTENTION: &str = "attention";
pub const SOCIAL_ALERT_REPORT_SEVERITY_CRITICAL: &str = "critical";
pub const SOCIAL_ALERT_REPORT_SEVERITY_WARNING: &str = "warning";
pub const SOCIAL_ALERT_REPORT_DEVICE_ID: &str = "device-social-alert-report";
pub const SOCIAL_ALERT_REPORT_DEVICE_LABEL: &str = "Study Phone";
pub const SOCIAL_ALERT_REPORT_PLATFORM_ANDROID: &str = "android";
pub const SOCIAL_ALERT_REPORT_REASON_HIGH_RISK: &str = "social-high-risk-signal";
pub const SOCIAL_ALERT_REPORT_REASON_MANUAL_REQUIRED: &str = "social-manual-review-required";
pub const SOCIAL_ALERT_REPORT_PROVIDER_CHANNEL_IN_APP: &str = "in-app";
pub const SOCIAL_ALERT_REPORT_TITLE_HIGH_RISK: &str = "social.alert.highRisk.title";
pub const SOCIAL_ALERT_REPORT_BODY_HIGH_RISK: &str = "social.alert.highRisk.body";
pub const SOCIAL_ALERT_REPORT_TITLE_MANUAL_REQUIRED: &str = "social.alert.manualRequired.title";
pub const SOCIAL_ALERT_REPORT_BODY_MANUAL_REQUIRED: &str = "social.alert.manualRequired.body";
pub const SOCIAL_ALERT_REPORT_ACTION_OPEN_PARENT_REVIEW: &str =
    "social.alert.action.openParentReview";
pub const SOCIAL_ALERT_REPORT_ACTION_REVIEW_MANUALLY: &str = "social.alert.action.reviewManually";
pub const SOCIAL_ALERT_REPORT_PANEL_FEED_VIDEO_GATES: &str = "panel-feed-video-gates";
pub const SOCIAL_ALERT_REPORT_PANEL_MANUAL_REQUIRED_GAPS: &str = "panel-manual-required-gaps";
pub const SOCIAL_ALERT_REPORT_EXPLANATION_SNAPSHOT: &str =
    "social-explanation-snapshot-alert-report";
pub const SOCIAL_ALERT_REPORT_EXPLANATION_EVENT_FEED_VIDEO_GATE: &str =
    "social-explanation-event-feed-video-gate";
pub const SOCIAL_ALERT_REPORT_EXPLANATION_EVENT_MANUAL_REQUIRED: &str =
    "social-explanation-event-manual-required";
pub const SOCIAL_ALERT_REPORT_EVIDENCE_ROUTE_GATE: &str = "evidence-social-route-gate";
pub const SOCIAL_ALERT_REPORT_EVIDENCE_MANUAL_GAP: &str = "evidence-social-manual-gap";
pub const SOCIAL_ALERT_REPORT_EVIDENCE_KIND_POLICY_DECISION: &str = "policy-decision";
pub const SOCIAL_ALERT_REPORT_POLICY_HIGH_RISK: &str = "policy-ref-social-high-risk";
pub const SOCIAL_ALERT_REPORT_POLICY_MANUAL_REQUIRED: &str = "policy-ref-social-manual-required";
pub const SOCIAL_ALERT_REPORT_AUDIT_REF: &str = "audit-ref-social-alert-report";
pub const SOCIAL_ALERT_REPORT_PARENT_ACTION_ID: &str = "parent-action-social-review";
pub const SOCIAL_ALERT_REPORT_PARENT_ACTOR_ID: &str = "parent-local-account";
pub const SOCIAL_ALERT_REPORT_PARENT_ACTOR_ROLE: &str = "parent";
pub const SOCIAL_ALERT_REPORT_POLICY_VERSION: &str = "policy-social-alert-report-v1";
pub const SOCIAL_ALERT_REPORT_LOCAL_OUTBOX_REF: &str = "local-outbox-social-alert-report";
pub const SOCIAL_ALERT_REPORT_MANUAL_PROOF_REQUIRED: &str =
    "provider-delivery-runtime-proof-required";
pub const SOCIAL_ALERT_REPORT_PAYLOAD_ALERT_ID: &str = "alert-id";
pub const SOCIAL_ALERT_REPORT_PAYLOAD_FAMILY_DEVICE_SCOPE: &str = "family-device-scope";
pub const SOCIAL_ALERT_REPORT_PAYLOAD_SEVERITY: &str = "severity";
pub const SOCIAL_ALERT_REPORT_PAYLOAD_REASON_CODE: &str = "reason-code";
pub const SOCIAL_ALERT_REPORT_PAYLOAD_EVIDENCE_REF: &str = "evidence-ref";
pub const SOCIAL_ALERT_REPORT_PAYLOAD_POLICY_REF: &str = "policy-ref";
pub const SOCIAL_ALERT_REPORT_PAYLOAD_EXPLANATION_REF: &str = "explanation-ref";
pub const SOCIAL_ALERT_REPORT_PAYLOAD_PARENT_ACTION_LINK_REF: &str = "parent-action-link-ref";
pub const SOCIAL_ALERT_REPORT_DELIVERY_LOCAL_OUTBOX_ONLY: &str = "local-outbox-only";
pub const SOCIAL_ALERT_REPORT_DELIVERY_MANUAL_REQUIRED: &str = "manual-required";
pub const SOCIAL_ALERT_REPORT_ADAPTER_NOT_DISPATCHED: &str = "not-dispatched";
pub const SOCIAL_ALERT_REPORT_PROVIDER_PREFLIGHT_ADAPTER_REQUIRED: &str =
    "provider-adapter-required";
pub const SOCIAL_ALERT_REPORT_PROVIDER_PREFLIGHT_MANUAL_REQUIRED: &str = "manual-required";
pub const SOCIAL_ALERT_REPORT_PROVIDER_STATUS_MANUAL_REQUIRED: &str = "manual-required";
pub const SOCIAL_ALERT_REPORT_PROVIDER_STATUS_PROOF_MANUAL_ACTION: &str = "manual-action-required";
pub const SOCIAL_ALERT_REPORT_PROVIDER_DELIVERY_NOT_OBSERVED: &str = "not-observed";
pub const SOCIAL_ALERT_REPORT_PROVIDER_STATUS_HIGH_RISK: &str =
    "social-provider-status-social-alert-report-high-risk-service";
pub const SOCIAL_ALERT_REPORT_PROVIDER_STATUS_MANUAL: &str =
    "social-provider-status-social-alert-report-manual-required";
pub const SOCIAL_ALERT_REPORT_PROVIDER_ATTEMPT_HIGH_RISK: &str =
    "social-provider-attempt-not-started-social-alert-report-high-risk-service";
pub const SOCIAL_ALERT_REPORT_PROVIDER_ATTEMPT_MANUAL: &str =
    "social-provider-attempt-not-started-social-alert-report-manual-required";
pub const SOCIAL_ALERT_REPORT_PROVIDER_ADAPTER_REQUIRED: &str =
    "provider-adapter-required-social-alert-report-high-risk-service";
pub const SOCIAL_ALERT_REPORT_PROVIDER_CREDENTIALS_REQUIRED: &str =
    "provider-credentials-required-social-alert-report-high-risk-service";
pub const SOCIAL_ALERT_REPORT_PROVIDER_SMOKE_REQUIRED: &str =
    "provider-smoke-proof-required-social-alert-report-high-risk-service";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialAlertReportReadModelSnapshot {
    pub schema_version: String,
    pub family_id: String,
    pub child_profile_id: String,
    pub generated_at: String,
    pub intents: Vec<SocialAlertReportIntent>,
    pub provider_status_rows: Vec<SocialAlertReportProviderStatusRow>,
    pub claim_boundaries: SocialAlertReportClaimBoundaries,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialAlertReportClaimBoundaries {
    pub provider_delivery: String,
    pub report_delivery: String,
    pub parent_notification_ui: String,
    pub final_policy_decision: String,
    pub enforcement: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialAlertReportIntent {
    pub schema_version: String,
    pub alert_report_intent_id: String,
    pub intent_kind: String,
    pub intent_status: String,
    pub priority: String,
    pub severity: String,
    pub device: SocialAlertReportDeviceRef,
    pub notification_reason_code: String,
    pub provider_channel_preference: String,
    pub parent_title_token: String,
    pub parent_body_token: String,
    pub parent_action_token: String,
    pub dashboard_panel_refs: Vec<String>,
    pub explanation_snapshot_ref: String,
    pub explanation_event_refs: Vec<String>,
    pub evidence_references: Vec<SocialAlertReportEvidenceRef>,
    pub policy_refs: Vec<String>,
    pub audit_refs: Vec<String>,
    pub parent_report_ref: Option<String>,
    pub parent_action_ref: Option<SocialAlertReportParentActionRef>,
    pub local_outbox_record_ref: Option<String>,
    pub provider_attempt_refs: Vec<String>,
    pub provider_receipt_refs: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
    pub minimal_payload_fields: Vec<String>,
    pub delivery_claim_state: String,
    pub raw_account_data_included: bool,
    pub raw_video_content_included: bool,
    pub raw_message_content_included: bool,
    pub screenshot_included: bool,
    pub provider_delivery_attempted: bool,
    pub provider_delivery_observed: bool,
    pub provider_receipt_ingested: bool,
    pub cloud_routing_claimed: bool,
    pub parent_notification_ui_claimed: bool,
    pub report_delivery_claimed: bool,
    pub final_policy_decision_claimed: bool,
    pub enforcement_claimed: bool,
    pub adapter_dispatch_state: String,
    pub adapter_action_claimed: bool,
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialAlertReportProviderStatusRow {
    pub status_entry_id: String,
    pub source_intent_ref: String,
    pub source_preflight_status: String,
    pub provider_status: String,
    pub status_proof_state: String,
    pub delivery_claim_state: String,
    pub provider_attempt_ref: String,
    pub readiness_refs: Vec<String>,
    pub provider_receipt_refs: Vec<String>,
    pub manual_proof_requirements: Vec<String>,
    pub provider_delivery_implemented: bool,
    pub provider_delivery_observed: bool,
    pub delivered_notification_claimed: bool,
    pub sensitive_provider_payload_claimed: bool,
    pub provider_stores_child_evidence_claimed: bool,
    pub last_checked_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialAlertReportDeviceRef {
    pub device_id: String,
    pub child_profile_id: String,
    pub label: String,
    pub platform: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialAlertReportEvidenceRef {
    pub evidence_reference_id: String,
    pub kind: String,
    pub observed_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialAlertReportParentActionRef {
    pub action_reference_id: String,
    pub actor: SocialAlertReportParentActor,
    pub policy_version: String,
    pub created_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialAlertReportParentActor {
    pub actor_id: String,
    pub role: String,
}
