use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey, RuntimeRole, SchemaVersion};
use serde::{Deserialize, Serialize};

use crate::{constants, ActivityEvidenceRef};

pub mod screen_household_mesh_input;
pub mod screen_runtime_input;

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
pub const SCREEN_DELETION_DELETE_FAILED_SUMMARY: &str = "Screen evidence deletion failed.";
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
pub const SCREEN_SERVICE_DELETION_OUTBOX_EXTENSION: &str = "deletion-outbox";
pub const SCREEN_SERVICE_DELETION_OUTBOX_QUARANTINE_EXTENSION: &str = "deletion-outbox-quarantine";
pub const SCREEN_SERVICE_DELETION_OUTBOX_CORRUPT_ID_PREFIX: &str =
    "screen-retention-outbox-corrupt-";
pub const SCREEN_SERVICE_DELETION_OUTBOX_QUARANTINE_PROOF_PREFIX: &str =
    "screen-retention-outbox-quarantine-";
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

const SCREEN_RUNTIME_PHASES: [ScreenRuntimePhase; 9] = [
    ScreenRuntimePhase::CaptureObserved,
    ScreenRuntimePhase::QueueEncrypted,
    ScreenRuntimePhase::AiAnalysisRequested,
    ScreenRuntimePhase::AiAnalysisCompleted,
    ScreenRuntimePhase::SummaryCommitted,
    ScreenRuntimePhase::PolicyDecisionCompleted,
    ScreenRuntimePhase::ActionDryRunRecorded,
    ScreenRuntimePhase::DeletionCommitted,
    ScreenRuntimePhase::PortalReadModelUpdated,
];

const SCREEN_RUNTIME_PHASE_EVENT_TYPES: [&str; 9] = [
    constants::screen_flow::EVENT_SCREEN_CAPTURE_OBSERVED,
    constants::screen_flow::EVENT_SCREEN_QUEUE_ENCRYPTED,
    constants::screen_flow::EVENT_SCREEN_AI_ANALYSIS_REQUESTED,
    constants::screen_flow::EVENT_SCREEN_AI_ANALYSIS_COMPLETED,
    constants::screen_flow::EVENT_SCREEN_SUMMARY_COMMITTED,
    constants::screen_flow::EVENT_SCREEN_POLICY_DECISION_COMPLETED,
    constants::screen_flow::EVENT_SCREEN_ACTION_DRY_RUN_RECORDED,
    constants::screen_flow::EVENT_SCREEN_DELETION_COMMITTED,
    constants::screen_flow::EVENT_SCREEN_PORTAL_READ_MODEL_UPDATED,
];

const SCREEN_RUNTIME_PHASE_SUBSCRIBER_IDS: [&str; 9] = [
    constants::screen_flow::SUBSCRIBER_SCREEN_CAPTURE_OBSERVER,
    constants::screen_flow::SUBSCRIBER_SCREEN_QUEUE_WRITER,
    constants::screen_flow::SUBSCRIBER_SCREEN_AI_REQUEST,
    constants::screen_flow::SUBSCRIBER_SCREEN_AI_COMPLETE,
    constants::screen_flow::SUBSCRIBER_SCREEN_SUMMARY_WRITER,
    constants::screen_flow::SUBSCRIBER_SCREEN_POLICY_DECISION,
    constants::screen_flow::SUBSCRIBER_SCREEN_ACTION_DRY_RUN,
    constants::screen_flow::SUBSCRIBER_SCREEN_DELETION_WORKER,
    constants::screen_flow::SUBSCRIBER_SCREEN_PORTAL_READ_MODEL,
];

const SCREEN_RUNTIME_PHASE_TARGET_HANDLERS: [&str; 9] = [
    constants::screen_flow::TARGET_SCREEN_CAPTURE_OBSERVER,
    constants::screen_flow::TARGET_SCREEN_QUEUE_WRITER,
    constants::screen_flow::TARGET_SCREEN_AI_ANALYZER,
    constants::screen_flow::TARGET_SCREEN_AI_ANALYZER,
    constants::screen_flow::TARGET_SCREEN_SUMMARY_WRITER,
    constants::screen_flow::TARGET_SCREEN_POLICY_ENGINE,
    constants::screen_flow::TARGET_SCREEN_ACTION_DRY_RUN,
    constants::screen_flow::TARGET_SCREEN_DELETION_WORKER,
    constants::screen_flow::TARGET_SCREEN_PORTAL_READ_MODEL,
];

const SCREEN_RUNTIME_PHASE_RUNTIME_ROLES: [&str; 9] = [
    constants::eventing_source::ROLE_AGENT,
    constants::eventing_source::ROLE_AGENT,
    constants::eventing_source::ROLE_ANALYZER,
    constants::eventing_source::ROLE_ANALYZER,
    constants::eventing_source::ROLE_AUDIT_WRITER,
    constants::eventing_source::ROLE_DECISION_ENGINE,
    constants::eventing_source::ROLE_SIDE_EFFECT_ADAPTER,
    constants::eventing_source::ROLE_AUDIT_WRITER,
    constants::eventing_source::ROLE_READ_MODEL,
];

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenRuntimePhase {
    CaptureObserved,
    QueueEncrypted,
    AiAnalysisRequested,
    AiAnalysisCompleted,
    SummaryCommitted,
    PolicyDecisionCompleted,
    ActionDryRunRecorded,
    DeletionCommitted,
    PortalReadModelUpdated,
}

impl ScreenRuntimePhase {
    pub fn ordered_chain() -> &'static [Self] {
        &SCREEN_RUNTIME_PHASES
    }

    pub fn event_type(self) -> &'static str {
        SCREEN_RUNTIME_PHASE_EVENT_TYPES[self as usize]
    }

    pub fn subscriber_id(self) -> &'static str {
        SCREEN_RUNTIME_PHASE_SUBSCRIBER_IDS[self as usize]
    }

    pub fn target_handler(self) -> &'static str {
        SCREEN_RUNTIME_PHASE_TARGET_HANDLERS[self as usize]
    }

    pub fn runtime_role(self) -> Result<RuntimeRole, EventingError> {
        RuntimeRole::parse(SCREEN_RUNTIME_PHASE_RUNTIME_ROLES[self as usize])
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenEvidenceScope {
    EncryptedLocalImage,
    DeletedQueryStoreSummary,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenAiAuditState {
    NotRequested,
    Requested,
    Completed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenPolicyState {
    NotReady,
    ReadyForDryRun,
    Completed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenActionState {
    NotReady,
    DryRunRecorded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenDeletionState {
    Pending,
    Committed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenRuntimeClaimBoundary {
    pub raw_image_available_to_ai_provider: bool,
    pub raw_image_available_to_policy: bool,
    pub raw_image_available_to_portal: bool,
    pub adapter_action_executed: bool,
}

impl ScreenRuntimeClaimBoundary {
    pub fn child_owned_no_raw_escape() -> Self {
        Self {
            raw_image_available_to_ai_provider: false,
            raw_image_available_to_policy: false,
            raw_image_available_to_portal: false,
            adapter_action_executed: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScreenRuntimeEventPayload {
    pub phase: ScreenRuntimePhase,
    pub queue_job_id: String,
    pub screen_analysis_result_id: String,
    pub capture_reason: String,
    pub capture_scope: String,
    pub image_digest: String,
    pub summary: String,
    pub model_runtime_ref: String,
    pub model_id: String,
    pub prompt_or_template_version: String,
    pub policy_decision_ref: Option<String>,
    pub policy_action: Option<String>,
    pub parent_rule_ref: Option<String>,
    pub action_ref: Option<String>,
    pub deletion_proof_ref: Option<String>,
    pub portal_read_model_ref: Option<String>,
    pub previous_phase_ref: Option<String>,
    pub capture_event_ref: String,
    pub queue_event_ref: Option<String>,
    pub ai_request_ref: Option<String>,
    pub ai_result_ref: Option<String>,
    pub summary_ref: Option<String>,
    pub evidence_scope: ScreenEvidenceScope,
    pub ai_audit_state: ScreenAiAuditState,
    pub policy_state: ScreenPolicyState,
    pub action_state: ScreenActionState,
    pub deletion_state: ScreenDeletionState,
    pub custody_state: String,
    pub claim_boundary: ScreenRuntimeClaimBoundary,
    pub observed_at: String,
}

impl DomainEvent for ScreenRuntimeEventPayload {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(self.phase.event_type())?,
            SchemaVersion::new(constants::screen_flow::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(screen_runtime_aggregate_key(&self.queue_job_id))
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value = String::from(constants::screen_flow::IDEMPOTENCY_SCREEN_RUNTIME_PREFIX);
        value.push_str(self.phase.event_type());
        value.push(constants::delimiter::HYPHEN);
        value.push_str(&screen_runtime_aggregate_key(&self.queue_job_id));
        value.push(constants::delimiter::HYPHEN);
        value.push_str(&self.observed_at);
        IdempotencyKey::parse(value)
    }
}

const SCREEN_HOUSEHOLD_MESH_PHASES: [ScreenHouseholdMeshPhase; 8] = [
    ScreenHouseholdMeshPhase::WorkQueued,
    ScreenHouseholdMeshPhase::OfferPublished,
    ScreenHouseholdMeshPhase::ClaimRequested,
    ScreenHouseholdMeshPhase::ClaimGranted,
    ScreenHouseholdMeshPhase::LeaseCreated,
    ScreenHouseholdMeshPhase::ProviderResultReturned,
    ScreenHouseholdMeshPhase::ChildResultAccepted,
    ScreenHouseholdMeshPhase::PolicyRequested,
];

const SCREEN_HOUSEHOLD_MESH_PHASE_EVENT_TYPES: [&str; 8] = [
    constants::screen_flow::EVENT_SCREEN_MESH_WORK_QUEUED,
    constants::screen_flow::EVENT_SCREEN_MESH_OFFER_PUBLISHED,
    constants::screen_flow::EVENT_SCREEN_MESH_CLAIM_REQUESTED,
    constants::screen_flow::EVENT_SCREEN_MESH_CLAIM_GRANTED,
    constants::screen_flow::EVENT_SCREEN_MESH_LEASE_CREATED,
    constants::screen_flow::EVENT_SCREEN_MESH_PROVIDER_RESULT_RETURNED,
    constants::screen_flow::EVENT_SCREEN_MESH_CHILD_RESULT_ACCEPTED,
    constants::screen_flow::EVENT_SCREEN_MESH_POLICY_REQUESTED,
];

const SCREEN_HOUSEHOLD_MESH_PHASE_SUBSCRIBER_IDS: [&str; 8] = [
    constants::screen_flow::SUBSCRIBER_SCREEN_MESH_WORK_QUEUE,
    constants::screen_flow::SUBSCRIBER_SCREEN_MESH_OFFER,
    constants::screen_flow::SUBSCRIBER_SCREEN_MESH_CLAIM_REQUEST,
    constants::screen_flow::SUBSCRIBER_SCREEN_MESH_CLAIM_GRANT,
    constants::screen_flow::SUBSCRIBER_SCREEN_MESH_LEASE,
    constants::screen_flow::SUBSCRIBER_SCREEN_MESH_PROVIDER_RESULT,
    constants::screen_flow::SUBSCRIBER_SCREEN_MESH_CHILD_VALIDATION,
    constants::screen_flow::SUBSCRIBER_SCREEN_MESH_POLICY_REQUEST,
];

const SCREEN_HOUSEHOLD_MESH_PHASE_TARGET_HANDLERS: [&str; 8] = [
    constants::screen_flow::TARGET_SCREEN_MESH_CHILD_LEDGER,
    constants::screen_flow::TARGET_SCREEN_MESH_BRIDGE,
    constants::screen_flow::TARGET_SCREEN_MESH_BRIDGE,
    constants::screen_flow::TARGET_SCREEN_MESH_CHILD_LEDGER,
    constants::screen_flow::TARGET_SCREEN_MESH_CHILD_LEDGER,
    constants::screen_flow::TARGET_SCREEN_MESH_PROVIDER_WORKER,
    constants::screen_flow::TARGET_SCREEN_MESH_CHILD_VALIDATOR,
    constants::screen_flow::TARGET_SCREEN_POLICY_ENGINE,
];

const SCREEN_HOUSEHOLD_MESH_PHASE_RUNTIME_ROLES: [&str; 8] = [
    constants::eventing_source::ROLE_AUDIT_WRITER,
    constants::eventing_source::ROLE_AGENT,
    constants::eventing_source::ROLE_AGENT,
    constants::eventing_source::ROLE_AUDIT_WRITER,
    constants::eventing_source::ROLE_AUDIT_WRITER,
    constants::eventing_source::ROLE_ANALYZER,
    constants::eventing_source::ROLE_AUDIT_WRITER,
    constants::eventing_source::ROLE_DECISION_ENGINE,
];

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenHouseholdMeshPhase {
    WorkQueued,
    OfferPublished,
    ClaimRequested,
    ClaimGranted,
    LeaseCreated,
    ProviderResultReturned,
    ChildResultAccepted,
    PolicyRequested,
}

impl ScreenHouseholdMeshPhase {
    pub fn ordered_chain() -> &'static [Self] {
        &SCREEN_HOUSEHOLD_MESH_PHASES
    }

    pub fn event_type(self) -> &'static str {
        SCREEN_HOUSEHOLD_MESH_PHASE_EVENT_TYPES[self as usize]
    }

    pub fn subscriber_id(self) -> &'static str {
        SCREEN_HOUSEHOLD_MESH_PHASE_SUBSCRIBER_IDS[self as usize]
    }

    pub fn target_handler(self) -> &'static str {
        SCREEN_HOUSEHOLD_MESH_PHASE_TARGET_HANDLERS[self as usize]
    }

    pub fn runtime_role(self) -> Result<RuntimeRole, EventingError> {
        RuntimeRole::parse(SCREEN_HOUSEHOLD_MESH_PHASE_RUNTIME_ROLES[self as usize])
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenMeshPayloadMode {
    RedactedSummary,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenMeshClaimState {
    NotRequested,
    Requested,
    Granted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenMeshLeaseState {
    NotCreated,
    Active,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenMeshProviderResultState {
    NotReturned,
    Returned,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenMeshChildValidationState {
    NotReady,
    Requested,
    Accepted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenMeshPolicyState {
    NotReady,
    Ready,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenMeshResultRejectionReason {
    DuplicateResult,
    ExpiredLease,
    WrongProvider,
    WrongClaim,
    EvidenceMismatch,
    CustodyMismatch,
    RawImageTransfer,
    ProviderAuthorityViolation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenMeshCustodyBoundary {
    pub raw_screenshot_transferred: bool,
    pub raw_screenshot_retained_by_provider: bool,
    pub provider_can_publish_policy: bool,
    pub provider_can_publish_enforcement: bool,
    pub child_agent_validates_before_policy: bool,
}

impl ScreenMeshCustodyBoundary {
    pub fn child_owned_worker_only() -> Self {
        Self {
            raw_screenshot_transferred: false,
            raw_screenshot_retained_by_provider: false,
            provider_can_publish_policy: false,
            provider_can_publish_enforcement: false,
            child_agent_validates_before_policy: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenHouseholdMeshResultValidation {
    pub accepted: bool,
    pub rejection_reason: Option<ScreenMeshResultRejectionReason>,
    pub policy_may_run: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScreenHouseholdMeshEventPayload {
    pub phase: ScreenHouseholdMeshPhase,
    pub queue_job_id: String,
    pub screen_evidence_ref: String,
    pub payload_ref: String,
    pub payload_mode: ScreenMeshPayloadMode,
    pub provider_peer_id: String,
    pub claim_id: String,
    pub lease_id: String,
    pub provider_result_ref: Option<String>,
    pub policy_decision_ref: Option<String>,
    pub previous_phase_ref: Option<String>,
    pub custody_label: String,
    pub claim_state: ScreenMeshClaimState,
    pub lease_state: ScreenMeshLeaseState,
    pub provider_result_state: ScreenMeshProviderResultState,
    pub child_validation_state: ScreenMeshChildValidationState,
    pub policy_state: ScreenMeshPolicyState,
    pub custody_boundary: ScreenMeshCustodyBoundary,
    pub observed_at: String,
}

impl DomainEvent for ScreenHouseholdMeshEventPayload {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(self.phase.event_type())?,
            SchemaVersion::new(constants::screen_flow::EVENT_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(screen_runtime_aggregate_key(&self.queue_job_id))
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        let mut value = String::from(constants::screen_flow::IDEMPOTENCY_SCREEN_MESH_PREFIX);
        value.push_str(self.phase.event_type());
        value.push(constants::delimiter::HYPHEN);
        value.push_str(&screen_runtime_aggregate_key(&self.queue_job_id));
        value.push(constants::delimiter::HYPHEN);
        value.push_str(&self.observed_at);
        IdempotencyKey::parse(value)
    }
}

fn screen_runtime_aggregate_key(queue_job_id: &str) -> String {
    let mut value = String::from(constants::screen_flow::AGGREGATE_SCREEN_QUEUE_PREFIX);
    value.push_str(queue_job_id);
    value
}
