use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityEvidenceKind, ActivityEvidenceRef,
    ActivityObserver, ActivitySource, ActivitySubject, ActivitySubjectKind, LogFieldValue,
    ScreenAnalysisQueueJob, ACTIVITY_SCHEMA_VERSION, SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW,
    SCREEN_CATEGORY_UNKNOWN, SCREEN_CUSTODY_JOURNAL, SCREEN_CUSTODY_TEMP_QUEUE,
    SCREEN_DELETION_DELETED, SCREEN_EVIDENCE_SCHEMA_VERSION, SCREEN_IMAGE_FORMAT_PNG,
    SCREEN_PROVIDER_SERVICE_METADATA, SCREEN_QUEUE_STATUS_QUEUED, SCREEN_SERVICE_ADAPTER_ID,
    SCREEN_SERVICE_LOCAL_USER_REF, SCREEN_SERVICE_METADATA_CONFIDENCE,
    SCREEN_SERVICE_MODEL_RUNTIME_REF, SCREEN_SERVICE_PARENT_SETTING_REF,
};
use ocentra_parent_screen_capture_adapter::CapturedScreenImage;

use crate::{
    fields::fields_from_pairs, screen_ai_cadence_runtime_event::ScreenAiServiceCaptureRecord,
};

const DEFAULT_MAX_RETRY_COUNT: u64 = 0;
const DEFAULT_SETTING_VERSION: u64 = 1;

pub(crate) struct ScreenAiServiceCaptureIds {
    pub(crate) queue_job_id: String,
    result_id: String,
    event_id: String,
    evidence_id: String,
}

impl ScreenAiServiceCaptureIds {
    pub(crate) fn new(
        queue_job_id_prefix: &str,
        result_id_prefix: &str,
        event_id_prefix: &str,
        evidence_id_prefix: &str,
        epoch_seconds: u64,
        sequence_index: u64,
    ) -> Self {
        Self {
            queue_job_id: suffixed_id(queue_job_id_prefix, epoch_seconds, sequence_index),
            result_id: suffixed_id(result_id_prefix, epoch_seconds, sequence_index),
            event_id: suffixed_id(event_id_prefix, epoch_seconds, sequence_index),
            evidence_id: suffixed_id(evidence_id_prefix, epoch_seconds, sequence_index),
        }
    }
}

pub(crate) fn screen_queue_job(
    record: &ScreenAiServiceCaptureRecord<'_>,
    ids: &ScreenAiServiceCaptureIds,
    image_digest: &str,
) -> ScreenAnalysisQueueJob {
    ScreenAnalysisQueueJob {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        queue_job_id: ids.queue_job_id.clone(),
        created_at: record.clock.timestamp.clone(),
        not_before: record.clock.timestamp.clone(),
        expires_at: record.clock.timestamp.clone(),
        last_attempt_at: None,
        capture_reason: record.capture_reason.to_string(),
        capture_scope: SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW.to_string(),
        source_id: record.source_id.to_string(),
        adapter_id: SCREEN_SERVICE_ADAPTER_ID.to_string(),
        device_ref: constants::peer::LOCAL_DEV_AGENT.to_string(),
        local_user_ref: SCREEN_SERVICE_LOCAL_USER_REF.to_string(),
        parent_setting_ref: SCREEN_SERVICE_PARENT_SETTING_REF.to_string(),
        setting_version: DEFAULT_SETTING_VERSION,
        related_evidence_refs: Vec::new(),
        encrypted_image_ref: record.paths.queue_dir.to_string_lossy().to_string(),
        image_digest: image_digest.to_string(),
        image_byte_size: record.image.png_bytes.len() as u64,
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

pub(crate) fn screen_analysis_event(
    record: &ScreenAiServiceCaptureRecord<'_>,
    ids: &ScreenAiServiceCaptureIds,
    job: &ScreenAnalysisQueueJob,
    image_digest: &str,
) -> ActivityEvent {
    let evidence = screen_analysis_evidence(ids, job, image_digest);
    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: ids.event_id.clone(),
        observed_at: record.clock.timestamp.clone(),
        source: ActivitySource {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: std::env::consts::OS.to_string(),
            observer: ActivityObserver::LocalAi,
            source_id: record.source_id.to_string(),
        },
        kind: ActivityEventKind::ScreenAnalysisSummarized,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Device,
            subject_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            display_name: record.image.metadata.title.clone(),
        },
        fields: fields_from_pairs(screen_analysis_fields(record, ids, job, image_digest)),
        evidence,
    }
}

fn screen_analysis_evidence(
    ids: &ScreenAiServiceCaptureIds,
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
    record: &ScreenAiServiceCaptureRecord<'_>,
    ids: &ScreenAiServiceCaptureIds,
    job: &ScreenAnalysisQueueJob,
    image_digest: &str,
) -> Vec<(&'static str, LogFieldValue)> {
    let mut fields = Vec::new();
    fields.extend(screen_analysis_identity_fields(record, ids, job));
    fields.extend(screen_analysis_model_fields(record));
    fields.extend(screen_analysis_capture_fields(
        job,
        image_digest,
        record.image,
    ));
    fields
}

fn screen_analysis_identity_fields(
    record: &ScreenAiServiceCaptureRecord<'_>,
    ids: &ScreenAiServiceCaptureIds,
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
        string_field(constants::field::SCREEN_SUMMARY, record.summary),
        string_field(
            constants::field::SCREEN_PRIMARY_CATEGORY,
            SCREEN_CATEGORY_UNKNOWN,
        ),
    ]
}

fn screen_analysis_model_fields(
    record: &ScreenAiServiceCaptureRecord<'_>,
) -> Vec<(&'static str, LogFieldValue)> {
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
        string_field(constants::field::SCREEN_MODEL_ID, record.model_id),
        string_field(
            constants::field::SCREEN_PROVIDER_KIND,
            SCREEN_PROVIDER_SERVICE_METADATA,
        ),
        string_field(
            constants::field::SCREEN_TEMPLATE_VERSION,
            record.template_version,
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
