use std::{fs, path::Path};

use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use ocentra_parent_agent_core::{JournalKey, ScreenEvidenceQueue, JOURNAL_KEY_BYTES};
use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityEvidenceKind, ActivityEvidenceRef,
    ActivityObserver, ActivitySource, ActivitySubject, ActivitySubjectKind, LogFieldValue,
    ScreenAnalysisQueueJob, ACTIVITY_SCHEMA_VERSION, SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW,
    SCREEN_CATEGORY_UNKNOWN, SCREEN_CUSTODY_JOURNAL, SCREEN_CUSTODY_TEMP_QUEUE,
    SCREEN_DELETION_DELETED, SCREEN_EVIDENCE_SCHEMA_VERSION, SCREEN_IMAGE_FORMAT_PNG,
    SCREEN_PROVIDER_SERVICE_METADATA, SCREEN_QUEUE_STATUS_QUEUED, SCREEN_SERVICE_ADAPTER_ID,
    SCREEN_SERVICE_EVENT_ID_PREFIX, SCREEN_SERVICE_EVIDENCE_ID_PREFIX,
    SCREEN_SERVICE_LOCAL_USER_REF, SCREEN_SERVICE_METADATA_CONFIDENCE, SCREEN_SERVICE_MODEL_ID,
    SCREEN_SERVICE_MODEL_RUNTIME_REF, SCREEN_SERVICE_PARENT_SETTING_REF,
    SCREEN_SERVICE_QUEUE_JOB_ID_PREFIX, SCREEN_SERVICE_RESULT_ID_PREFIX, SCREEN_SERVICE_SOURCE_ID,
    SCREEN_SERVICE_SUMMARY_CAPTURED, SCREEN_SERVICE_TEMPLATE_VERSION,
};
use ocentra_parent_screen_capture_adapter::CapturedScreenImage;
use sha2::{Digest, Sha256};

use crate::{
    activity_capture::{record_activity_events_to_paths, ActivityCaptureError},
    fields::fields_from_pairs,
    screen_ai_cadence_runtime::{ScreenAiCadenceRuntimeConfig, ScreenAiCadenceTickClock},
};

const DEFAULT_MAX_RETRY_COUNT: u64 = 0;
const DEFAULT_SETTING_VERSION: u64 = 1;

pub(crate) fn record_captured_screen_image_to_paths(
    config: &ScreenAiCadenceRuntimeConfig,
    image: &CapturedScreenImage,
    clock: ScreenAiCadenceTickClock,
    tick_index: u64,
) -> Result<String, ActivityCaptureError> {
    let key = load_or_create_screen_key(&config.journal_key_path)?;
    let image_digest = digest_image(&image.png_bytes);
    let ids = ScreenAiCadenceIds::new(clock.epoch_seconds, tick_index);
    let job = screen_queue_job(config, &ids, &image_digest, image, &clock.timestamp);
    ScreenEvidenceQueue::open(&config.queue_dir, key)?
        .append_encrypted_image(&job, &image.png_bytes)?;
    let event = screen_analysis_event(&ids, &job, &image_digest, image, &clock.timestamp);
    record_activity_events_to_paths(
        &config.journal_path,
        &config.journal_key_path,
        &config.store_path,
        &[event],
    )?;
    Ok(ids.queue_job_id)
}

struct ScreenAiCadenceIds {
    queue_job_id: String,
    result_id: String,
    event_id: String,
    evidence_id: String,
}

impl ScreenAiCadenceIds {
    fn new(epoch_seconds: u64, tick_index: u64) -> Self {
        Self {
            queue_job_id: suffixed_id(
                SCREEN_SERVICE_QUEUE_JOB_ID_PREFIX,
                epoch_seconds,
                tick_index,
            ),
            result_id: suffixed_id(SCREEN_SERVICE_RESULT_ID_PREFIX, epoch_seconds, tick_index),
            event_id: suffixed_id(SCREEN_SERVICE_EVENT_ID_PREFIX, epoch_seconds, tick_index),
            evidence_id: suffixed_id(SCREEN_SERVICE_EVIDENCE_ID_PREFIX, epoch_seconds, tick_index),
        }
    }
}

fn screen_queue_job(
    config: &ScreenAiCadenceRuntimeConfig,
    ids: &ScreenAiCadenceIds,
    image_digest: &str,
    image: &CapturedScreenImage,
    timestamp: &str,
) -> ScreenAnalysisQueueJob {
    ScreenAnalysisQueueJob {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        queue_job_id: ids.queue_job_id.clone(),
        created_at: timestamp.to_string(),
        not_before: timestamp.to_string(),
        expires_at: timestamp.to_string(),
        last_attempt_at: None,
        capture_reason: constants::activity_capture::SCREEN_TRIGGER_TIMED_CADENCE.to_string(),
        capture_scope: SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW.to_string(),
        source_id: SCREEN_SERVICE_SOURCE_ID.to_string(),
        adapter_id: SCREEN_SERVICE_ADAPTER_ID.to_string(),
        device_ref: constants::peer::LOCAL_DEV_AGENT.to_string(),
        local_user_ref: SCREEN_SERVICE_LOCAL_USER_REF.to_string(),
        parent_setting_ref: SCREEN_SERVICE_PARENT_SETTING_REF.to_string(),
        setting_version: DEFAULT_SETTING_VERSION,
        related_evidence_refs: Vec::new(),
        encrypted_image_ref: config.queue_dir.to_string_lossy().to_string(),
        image_digest: image_digest.to_string(),
        image_byte_size: image.png_bytes.len() as u64,
        image_format: SCREEN_IMAGE_FORMAT_PNG.to_string(),
        status: SCREEN_QUEUE_STATUS_QUEUED.to_string(),
        attempt_count: 0,
        max_retry_count: DEFAULT_MAX_RETRY_COUNT,
        failure_reason: None,
        unavailable_reason: None,
        deletion_required: true,
        deleted_at: None,
        deletion_status: SCREEN_DELETION_DELETED.to_string(),
        deletion_proof_ref: None,
        custody_state: SCREEN_CUSTODY_TEMP_QUEUE.to_string(),
    }
}

fn screen_analysis_event(
    ids: &ScreenAiCadenceIds,
    job: &ScreenAnalysisQueueJob,
    image_digest: &str,
    image: &CapturedScreenImage,
    timestamp: &str,
) -> ActivityEvent {
    let evidence = screen_analysis_evidence(ids, job, image_digest);
    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: ids.event_id.clone(),
        observed_at: timestamp.to_string(),
        source: ActivitySource {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: std::env::consts::OS.to_string(),
            observer: ActivityObserver::LocalAi,
            source_id: SCREEN_SERVICE_SOURCE_ID.to_string(),
        },
        kind: ActivityEventKind::ScreenAnalysisSummarized,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Device,
            subject_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            display_name: image.metadata.title.clone(),
        },
        fields: fields_from_pairs(screen_analysis_fields(ids, job, image_digest, image)),
        evidence,
    }
}

fn screen_analysis_evidence(
    ids: &ScreenAiCadenceIds,
    job: &ScreenAnalysisQueueJob,
    image_digest: &str,
) -> Vec<ActivityEvidenceRef> {
    vec![ActivityEvidenceRef {
        evidence_id: ids.evidence_id.clone(),
        kind: ActivityEvidenceKind::Screenshot,
        digest: Some(image_digest.to_string()),
        uri: Some(job.encrypted_image_ref.clone()),
    }]
}

fn screen_analysis_fields(
    ids: &ScreenAiCadenceIds,
    job: &ScreenAnalysisQueueJob,
    image_digest: &str,
    image: &CapturedScreenImage,
) -> Vec<(&'static str, LogFieldValue)> {
    let mut fields = Vec::new();
    fields.extend(screen_analysis_identity_fields(ids, job));
    fields.extend(screen_analysis_model_fields());
    fields.extend(screen_analysis_capture_fields(job, image_digest, image));
    fields
}

fn screen_analysis_identity_fields(
    ids: &ScreenAiCadenceIds,
    job: &ScreenAnalysisQueueJob,
) -> Vec<(&'static str, LogFieldValue)> {
    vec![
        string_field(
            constants::field::SCREEN_ANALYSIS_RESULT_ID,
            ids.result_id.clone(),
        ),
        string_field(
            constants::field::SCREEN_QUEUE_JOB_ID,
            job.queue_job_id.clone(),
        ),
        string_field(
            constants::field::SCREEN_SUMMARY,
            SCREEN_SERVICE_SUMMARY_CAPTURED,
        ),
        string_field(
            constants::field::SCREEN_PRIMARY_CATEGORY,
            SCREEN_CATEGORY_UNKNOWN,
        ),
    ]
}

fn screen_analysis_model_fields() -> Vec<(&'static str, LogFieldValue)> {
    vec![
        number_field(
            constants::field::SCREEN_CONFIDENCE,
            SCREEN_SERVICE_METADATA_CONFIDENCE,
        ),
        string_field(
            constants::field::SCREEN_IMAGE_DELETION_STATE,
            SCREEN_DELETION_DELETED,
        ),
        bool_field(constants::field::SCREEN_POLICY_ELIGIBLE, false),
        string_field(
            constants::field::SCREEN_MODEL_RUNTIME_REF,
            SCREEN_SERVICE_MODEL_RUNTIME_REF,
        ),
        string_field(constants::field::SCREEN_MODEL_ID, SCREEN_SERVICE_MODEL_ID),
        string_field(
            constants::field::SCREEN_PROVIDER_KIND,
            SCREEN_PROVIDER_SERVICE_METADATA,
        ),
        string_field(
            constants::field::SCREEN_TEMPLATE_VERSION,
            SCREEN_SERVICE_TEMPLATE_VERSION,
        ),
    ]
}

fn screen_analysis_capture_fields(
    job: &ScreenAnalysisQueueJob,
    image_digest: &str,
    image: &CapturedScreenImage,
) -> Vec<(&'static str, LogFieldValue)> {
    vec![
        string_field(
            constants::field::SCREEN_CAPTURE_REASON,
            job.capture_reason.clone(),
        ),
        string_field(
            constants::field::SCREEN_CAPTURE_SCOPE,
            job.capture_scope.clone(),
        ),
        string_field(
            constants::field::CAPABILITY_STATUS,
            image.metadata.status.as_protocol_str(),
        ),
        string_field(
            constants::field::SCREEN_IMAGE_DIGEST,
            image_digest.to_string(),
        ),
        string_field(
            constants::field::SCREEN_CUSTODY_STATE,
            SCREEN_CUSTODY_JOURNAL,
        ),
    ]
}

fn load_or_create_screen_key(path: &Path) -> Result<JournalKey, ActivityCaptureError> {
    match fs::read(path) {
        Ok(bytes) => journal_key_from_bytes(&bytes),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let key = JournalKey::generate();
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(path, key.as_bytes())?;
            Ok(key)
        }
        Err(_) => Err(ActivityCaptureError::Io),
    }
}

fn journal_key_from_bytes(bytes: &[u8]) -> Result<JournalKey, ActivityCaptureError> {
    if bytes.len() != JOURNAL_KEY_BYTES {
        return Err(ActivityCaptureError::InvalidKeyLength);
    }
    let mut key = [0; JOURNAL_KEY_BYTES];
    key.copy_from_slice(bytes);
    Ok(JournalKey::from_bytes(key))
}

fn digest_image(image_bytes: &[u8]) -> String {
    BASE64_URL_SAFE_NO_PAD.encode(Sha256::digest(image_bytes))
}

fn suffixed_id(prefix: &str, epoch_seconds: u64, tick_index: u64) -> String {
    let mut id = String::from(prefix);
    id.push_str(&epoch_seconds.to_string());
    id.push(constants::delimiter::HYPHEN);
    id.push_str(&tick_index.to_string());
    id
}

fn string_field(key: &'static str, value: impl Into<String>) -> (&'static str, LogFieldValue) {
    (key, LogFieldValue::String(value.into()))
}

fn number_field(key: &'static str, value: f64) -> (&'static str, LogFieldValue) {
    (key, LogFieldValue::Number(value))
}

fn bool_field(key: &'static str, value: bool) -> (&'static str, LogFieldValue) {
    (key, LogFieldValue::Boolean(value))
}
