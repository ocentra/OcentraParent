use serde::{Deserialize, Serialize};

use crate::ActivityEvidenceRef;

pub const SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST: &str = "manualParentTestCapture";
pub const SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW: &str = "activeWindow";
pub const SCREEN_CAPABILITY_READY: &str = "ready";
pub const SCREEN_CAPABILITY_DISABLED_BY_PARENT: &str = "disabledByParent";
pub const SCREEN_QUEUE_STATUS_QUEUED: &str = "queued";
pub const SCREEN_QUEUE_STATUS_DELETED: &str = "deleted";
pub const SCREEN_DELETION_REQUIRED: &str = "deletionRequired";
pub const SCREEN_DELETION_DELETED: &str = "deleted";
pub const SCREEN_CUSTODY_TEMP_QUEUE: &str = "child-device-temp-queue";
pub const SCREEN_CUSTODY_JOURNAL: &str = "child-device-journal";
pub const SCREEN_CUSTODY_QUERY_STORE: &str = "child-device-query-store";
pub const SCREEN_CATEGORY_SCHOOL: &str = "school";
pub const SCREEN_PROVIDER_LOCAL_VISION: &str = "localVision";
pub const SCREEN_IMAGE_FORMAT_PNG: &str = "png";
pub const SCREEN_POLICY_CONFIDENCE_READY: f64 = 0.88;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenAnalysisQueueJob {
    pub schema_version: u16,
    pub queue_job_id: String,
    pub created_at: String,
    pub not_before: String,
    pub expires_at: String,
    pub last_attempt_at: Option<String>,
    pub capture_reason: String,
    pub capture_scope: String,
    pub source_id: String,
    pub adapter_id: String,
    pub device_ref: String,
    pub local_user_ref: String,
    pub parent_setting_ref: String,
    pub setting_version: u64,
    pub related_evidence_refs: Vec<ActivityEvidenceRef>,
    pub encrypted_image_ref: String,
    pub image_digest: String,
    pub image_byte_size: u64,
    pub image_format: String,
    pub status: String,
    pub attempt_count: u64,
    pub max_retry_count: u64,
    pub failure_reason: Option<String>,
    pub unavailable_reason: Option<String>,
    pub deletion_required: bool,
    pub deleted_at: Option<String>,
    pub deletion_status: String,
    pub deletion_proof_ref: Option<String>,
    pub custody_state: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenCategoryCandidate {
    pub category: String,
    pub confidence: f64,
    pub evidence_refs: Vec<ActivityEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenRiskSignalCandidate {
    pub signal: String,
    pub confidence: f64,
    pub evidence_refs: Vec<ActivityEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenAnalysisResult {
    pub schema_version: u16,
    pub screen_analysis_result_id: String,
    pub queue_job_id: String,
    pub analyzed_at: String,
    pub model_runtime_ref: String,
    pub model_id: String,
    pub provider_kind: String,
    pub prompt_or_template_version: String,
    pub capture_reason: String,
    pub capture_scope: String,
    pub capability_status: String,
    pub summary: String,
    pub visible_category_candidates: Vec<ScreenCategoryCandidate>,
    pub primary_category: Option<String>,
    pub risk_signals: Vec<ScreenRiskSignalCandidate>,
    pub ocr_text_snippets: Vec<String>,
    pub redaction_notes: Vec<String>,
    pub confidence: f64,
    pub uncertainty_reason: Option<String>,
    pub source_evidence_refs: Vec<ActivityEvidenceRef>,
    pub image_digest: String,
    pub raw_image_retained: bool,
    pub image_deletion_state: String,
    pub custody_state: String,
    pub policy_eligible: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenEvidenceQueueHealth {
    pub schema_version: u16,
    pub generated_at: String,
    pub custody_state: String,
    pub pending_count: u64,
    pub expired_count: u64,
    pub delete_pending_count: u64,
    pub delete_failed_count: u64,
    pub latest_queue_job_id: Option<String>,
    pub latest_status: Option<String>,
    pub last_successful_analysis_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenEvidenceRecentSummary {
    pub schema_version: u16,
    pub generated_at: String,
    pub custody_state: String,
    pub limit: u64,
    pub returned: u64,
    pub queue_health: ScreenEvidenceQueueHealth,
    pub latest_result_id: Option<String>,
    pub latest_summary: Option<String>,
    pub latest_primary_category: Option<String>,
    pub latest_confidence: Option<f64>,
    pub latest_image_deletion_state: Option<String>,
    pub latest_policy_eligible: Option<bool>,
    pub evidence: Vec<ActivityEvidenceRef>,
    pub results: Vec<ScreenAnalysisResult>,
}
