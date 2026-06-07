use serde::{Deserialize, Serialize};

use crate::ActivityEvidenceRef;

pub const SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST: &str = "manualParentTestCapture";
pub const SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW: &str = "activeWindow";
pub const SCREEN_CAPABILITY_READY: &str = "ready";
pub const SCREEN_CAPABILITY_DISABLED_BY_PARENT: &str = "disabledByParent";
pub const SCREEN_QUEUE_STATUS_QUEUED: &str = "queued";
pub const SCREEN_QUEUE_STATUS_EXPIRED: &str = "expired";
pub const SCREEN_QUEUE_STATUS_DELETED: &str = "deleted";
pub const SCREEN_QUEUE_STATUS_FAILED: &str = "failed";
pub const SCREEN_DELETION_REQUIRED: &str = "deletionRequired";
pub const SCREEN_DELETION_DELETED: &str = "deleted";
pub const SCREEN_DELETION_EXPIRED_DELETED: &str = "expiredDeleted";
pub const SCREEN_DELETION_DELETE_FAILED: &str = "deleteFailed";
pub const SCREEN_CUSTODY_TEMP_QUEUE: &str = "child-device-temp-queue";
pub const SCREEN_CUSTODY_JOURNAL: &str = "child-device-journal";
pub const SCREEN_CUSTODY_QUERY_STORE: &str = "child-device-query-store";
pub const SCREEN_SERVICE_CADENCE_RUNTIME_ENABLED_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_CADENCE_RUNTIME_ENABLED";
pub const SCREEN_SERVICE_FOREGROUND_RUNTIME_ENABLED_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_FOREGROUND_RUNTIME_ENABLED";
pub const SCREEN_SERVICE_ANALYSIS_ENABLED_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_ANALYSIS_ENABLED";
pub const SCREEN_SERVICE_ANALYSIS_RUNTIME_ENABLED_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_ANALYSIS_RUNTIME_ENABLED";
pub const SCREEN_SERVICE_ANALYSIS_ADAPTER_COMMAND_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_ANALYSIS_ADAPTER_COMMAND";
pub const SCREEN_SERVICE_ANALYSIS_POLL_SECONDS_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_ANALYSIS_POLL_SECONDS";
pub const SCREEN_SERVICE_ANALYSIS_MAX_JOBS_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_ANALYSIS_MAX_JOBS";
pub const SCREEN_SERVICE_ANALYSIS_MAX_TICKS_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_ANALYSIS_MAX_TICKS";
pub const SCREEN_SERVICE_ANALYSIS_ADAPTER_TIMEOUT_MS_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_ANALYSIS_ADAPTER_TIMEOUT_MS";
pub const SCREEN_SERVICE_RETENTION_SWEEPER_RUNTIME_ENABLED_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_RETENTION_SWEEPER_RUNTIME_ENABLED";
pub const SCREEN_SERVICE_RETENTION_SWEEPER_POLL_SECONDS_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_RETENTION_SWEEPER_POLL_SECONDS";
pub const SCREEN_SERVICE_RETENTION_SWEEPER_MAX_SWEEPS_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_RETENTION_SWEEPER_MAX_SWEEPS";
pub const SCREEN_SERVICE_RETENTION_SWEEPER_MAX_TICKS_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_RETENTION_SWEEPER_MAX_TICKS";
pub const SCREEN_SERVICE_CADENCE_ENABLED_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_CADENCE_ENABLED";
pub const SCREEN_SERVICE_FOREGROUND_ENABLED_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_FOREGROUND_ENABLED";
pub const SCREEN_SERVICE_CADENCE_SECONDS_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_CADENCE_SECONDS";
pub const SCREEN_SERVICE_FOREGROUND_POLL_SECONDS_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_FOREGROUND_POLL_SECONDS";
pub const SCREEN_SERVICE_FOREGROUND_MIN_GAP_SECONDS_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_FOREGROUND_MIN_GAP_SECONDS";
pub const SCREEN_SERVICE_CADENCE_MAX_CAPTURES_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_CADENCE_MAX_CAPTURES";
pub const SCREEN_SERVICE_FOREGROUND_MAX_CAPTURES_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_FOREGROUND_MAX_CAPTURES";
pub const SCREEN_SERVICE_CADENCE_MAX_TICKS_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_CADENCE_MAX_TICKS";
pub const SCREEN_SERVICE_FOREGROUND_MAX_TICKS_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_FOREGROUND_MAX_TICKS";
pub const SCREEN_SERVICE_QUEUE_DIR_ENV: &str = "OCENTRA_PARENT_SCREEN_SERVICE_QUEUE_DIR";
pub const SCREEN_SERVICE_QUEUE_MAX_PENDING_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_QUEUE_MAX_PENDING";
pub const SCREEN_SERVICE_QUEUE_MAX_PENDING_DEFAULT: u64 = 3;
pub const SCREEN_SERVICE_TEMPORARY_IMAGE_TTL_SECONDS_ENV: &str =
    "OCENTRA_PARENT_SCREEN_SERVICE_TEMPORARY_IMAGE_TTL_SECONDS";
pub const SCREEN_SERVICE_TEMPORARY_IMAGE_TTL_SECONDS_DEFAULT: u64 = 300;
pub const SCREEN_SERVICE_DEFAULT_QUEUE_DIR_NAME: &str = "ocentra-parent-screen-evidence";
pub const SCREEN_SERVICE_QUEUE_JOB_ID_PREFIX: &str = "screen-service-queue-job-";
pub const SCREEN_SERVICE_RESULT_ID_PREFIX: &str = "screen-service-analysis-result-";
pub const SCREEN_SERVICE_EVENT_ID_PREFIX: &str = "screen-service-analysis-event-";
pub const SCREEN_SERVICE_EVIDENCE_ID_PREFIX: &str = "screen-service-screenshot-";
pub const SCREEN_SERVICE_FOREGROUND_QUEUE_JOB_ID_PREFIX: &str =
    "screen-service-foreground-queue-job-";
pub const SCREEN_SERVICE_FOREGROUND_RESULT_ID_PREFIX: &str =
    "screen-service-foreground-analysis-result-";
pub const SCREEN_SERVICE_FOREGROUND_EVENT_ID_PREFIX: &str =
    "screen-service-foreground-analysis-event-";
pub const SCREEN_SERVICE_FOREGROUND_EVIDENCE_ID_PREFIX: &str =
    "screen-service-foreground-screenshot-";
pub const SCREEN_SERVICE_SOURCE_ID: &str = "screen-service-cadence-runtime";
pub const SCREEN_SERVICE_FOREGROUND_SOURCE_ID: &str = "screen-service-foreground-runtime";
pub const SCREEN_SERVICE_FOREGROUND_KEY_WINDOW_PREFIX: &str = "window-";
pub const SCREEN_SERVICE_FOREGROUND_KEY_PID_PREFIX: &str = "pid-";
pub const SCREEN_SERVICE_FOREGROUND_KEY_TITLE_PREFIX: &str = "title-";
pub const SCREEN_SERVICE_FOREGROUND_KEY_APP_PREFIX: &str = "app-";
pub const SCREEN_SERVICE_FOREGROUND_KEY_STATUS_PREFIX: &str = "status-";
pub const SCREEN_SERVICE_ADAPTER_ID: &str = "screen-active-window-adapter";
pub const SCREEN_SERVICE_LOCAL_USER_REF: &str = "local-user-service";
pub const SCREEN_SERVICE_PARENT_SETTING_REF: &str = "parent-setting-screen-service";
pub const SCREEN_SERVICE_MODEL_RUNTIME_REF: &str = "screen-service-deterministic-runtime";
pub const SCREEN_SERVICE_MODEL_ID: &str = "screen-service-cadence-metadata-v1";
pub const SCREEN_SERVICE_FOREGROUND_MODEL_ID: &str = "screen-service-foreground-metadata-v1";
pub const SCREEN_SERVICE_TEMPLATE_VERSION: &str = "screen-service-cadence-summary-v1";
pub const SCREEN_SERVICE_FOREGROUND_TEMPLATE_VERSION: &str = "screen-service-foreground-summary-v1";
pub const SCREEN_SERVICE_ANALYSIS_SOURCE_ID: &str = "screen-service-analysis-runtime";
pub const SCREEN_SERVICE_ANALYSIS_RESULT_ID_PREFIX: &str =
    "screen-service-adapter-analysis-result-";
pub const SCREEN_SERVICE_ANALYSIS_EVENT_ID_PREFIX: &str = "screen-service-adapter-analysis-event-";
pub const SCREEN_SERVICE_ANALYSIS_EVIDENCE_ID_PREFIX: &str = "screen-service-adapter-evidence-";
pub const SCREEN_SERVICE_ANALYSIS_RUNTIME_REF: &str = "screen-service-local-adapter-runtime";
pub const SCREEN_SERVICE_ANALYSIS_PROVIDER_ID: &str = "screen-service-local-adapter";
pub const SCREEN_SERVICE_ANALYSIS_MODEL_REFERENCE: &str = "artifact:screen_service_local_adapter";
pub const SCREEN_SERVICE_ANALYSIS_MODEL_ID: &str = "screen-service-local-analysis-v1";
pub const SCREEN_SERVICE_ANALYSIS_TEMPLATE_VERSION: &str = "screen-service-analysis-adapter-v1";
pub const SCREEN_SERVICE_ANALYSIS_FIELD_IMAGE_BASE64: &str = "imageBase64";
pub const SCREEN_SERVICE_RETENTION_SWEEPER_SOURCE_ID: &str =
    "screen-service-retention-sweeper-runtime";
pub const SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX: &str =
    "screen-service-retention-delete-proof-";
pub const SCREEN_SERVICE_RETENTION_RESULT_ID_PREFIX: &str = "screen-service-retention-result-";
pub const SCREEN_SERVICE_RETENTION_EVENT_ID_PREFIX: &str = "screen-service-retention-event-";
pub const SCREEN_SERVICE_RETENTION_EVIDENCE_ID_PREFIX: &str = "screen-service-retention-evidence-";
pub const SCREEN_SERVICE_RETENTION_MODEL_RUNTIME_REF: &str = "screen-service-retention-sweeper";
pub const SCREEN_SERVICE_RETENTION_MODEL_ID: &str = "screen-service-retention-sweeper-v1";
pub const SCREEN_SERVICE_RETENTION_TEMPLATE_VERSION: &str = "screen-service-retention-expiry-v1";
pub const SCREEN_SERVICE_RETENTION_SUMMARY_EXPIRED_DELETED: &str =
    "Expired screen evidence was deleted by the local retention sweeper.";
pub const SCREEN_SERVICE_ANALYSIS_SUMMARY_UNAVAILABLE: &str =
    "Local screen analysis adapter is unavailable for this queued capture.";
pub const SCREEN_SERVICE_ANALYSIS_SUMMARY_INVALID: &str =
    "Local screen analysis adapter output was rejected before policy.";
pub const SCREEN_SERVICE_SUMMARY_CAPTURED: &str =
    "Timed screen capture was queued by the local service cadence.";
pub const SCREEN_SERVICE_FOREGROUND_SUMMARY_CAPTURED: &str =
    "Foreground screen capture was queued by the local service watcher.";
pub const SCREEN_SERVICE_TEST_QUEUE_RECORD_LINE: &str = "{}\n";
pub const SCREEN_CATEGORY_UNKNOWN: &str = "unknown";
pub const SCREEN_CATEGORY_SCHOOL: &str = "school";
pub const SCREEN_PROVIDER_SERVICE_METADATA: &str = "serviceCaptureMetadata";
pub const SCREEN_PROVIDER_LOCAL_OCR: &str = "localOcr";
pub const SCREEN_PROVIDER_LOCAL_VISION: &str = "localVision";
pub const SCREEN_PROVIDER_LOCAL_VISION_UNAVAILABLE: &str = "localVisionUnavailable";
pub const SCREEN_WINRT_OCR_RUNTIME_REF: &str = "windows-winrt-ocr-local-runtime";
pub const SCREEN_WINRT_OCR_MODEL_ID: &str = "windows-winrt-ocr";
pub const SCREEN_WINRT_OCR_TEMPLATE_VERSION: &str = "screen-ocr-worker-winrt-v1";
pub const SCREEN_IMAGE_FORMAT_PNG: &str = "png";
pub const SCREEN_POLICY_CONFIDENCE_READY: f64 = 0.88;
pub const SCREEN_SERVICE_METADATA_CONFIDENCE: f64 = 0.2;
pub const SCREEN_SERVICE_UNAVAILABLE_CONFIDENCE: f64 = 0.0;
pub const SCREEN_SERVICE_ANALYSIS_DEFAULT_POLL_SECONDS: u64 = 5;
pub const SCREEN_SERVICE_ANALYSIS_DEFAULT_MAX_QUEUE_SCAN: usize = 16;
pub const SCREEN_SERVICE_ANALYSIS_DEFAULT_ADAPTER_TIMEOUT_MS: u64 = 30000;

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
    pub policy_decision_ref: Option<String>,
    pub policy_action: Option<String>,
    #[serde(default)]
    pub policy_reason_codes: Vec<String>,
    #[serde(default)]
    pub parent_rule_refs: Vec<String>,
    #[serde(default)]
    pub local_model_runtime_refs: Vec<String>,
    #[serde(default)]
    pub parent_explanation_refs: Vec<String>,
    #[serde(default)]
    pub explanation_reasons: Vec<String>,
    #[serde(default)]
    pub deletion_reasons: Vec<String>,
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
