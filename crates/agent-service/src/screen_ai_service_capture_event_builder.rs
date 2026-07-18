use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::activity::ActivityEventKind;
use ocentra_parent_agent_protocol::activity::ActivityEvidenceKind;
use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::activity::ActivityObserver;
use ocentra_parent_agent_protocol::activity::ActivitySource;
use ocentra_parent_agent_protocol::activity::ActivitySubject;
use ocentra_parent_agent_protocol::activity::ActivitySubjectKind;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::screen_evidence::ScreenAnalysisQueueJob;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_CATEGORY_UNKNOWN;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_CUSTODY_JOURNAL;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_CUSTODY_TEMP_QUEUE;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_DELETION_REQUIRED;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_IMAGE_FORMAT_PNG;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_PROVIDER_SERVICE_METADATA;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_QUEUE_STATUS_QUEUED;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ADAPTER_ID;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_LOCAL_USER_REF;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_METADATA_CONFIDENCE;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_MODEL_RUNTIME_REF;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_PARENT_SETTING_REF;
use ocentra_parent_agent_protocol::ACTIVITY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::SCREEN_EVIDENCE_SCHEMA_VERSION;
use ocentra_parent_screen_capture_adapter::CapturedScreenImage;

use crate::{
    fields::fields_from_pairs, screen_ai_cadence_runtime_event::ScreenAiServiceCaptureRecord,
};

const DEFAULT_MAX_RETRY_COUNT: u64 = 0;
const DEFAULT_SETTING_VERSION: u64 = 1;

#[derive(Clone, Copy)]
pub(crate) struct ScreenIdPrefix(pub(crate) &'static str);

#[derive(Clone)]
pub(crate) struct ScreenText(pub(crate) String);

impl ScreenText {
    pub(crate) fn from_display(value: impl std::fmt::Display) -> Self {
        Self(value.to_string())
    }
}

#[derive(Clone)]
struct ScreenFieldEntry {
    key: &'static str,
    value: LogFieldValue,
}

pub(crate) struct ScreenAiServiceCaptureIds {
    pub(crate) queue_job_id: String,
    result_id: String,
    event_id: String,
    evidence_id: String,
}

impl ScreenAiServiceCaptureIds {
    pub(crate) fn new(
        queue_job_id_prefix: ScreenIdPrefix,
        result_id_prefix: ScreenIdPrefix,
        event_id_prefix: ScreenIdPrefix,
        evidence_id_prefix: ScreenIdPrefix,
        epoch_seconds: u64,
        sequence_index: u64,
    ) -> Self {
        Self {
            queue_job_id: suffixed_id(queue_job_id_prefix, epoch_seconds, sequence_index).0,
            result_id: suffixed_id(result_id_prefix, epoch_seconds, sequence_index).0,
            event_id: suffixed_id(event_id_prefix, epoch_seconds, sequence_index).0,
            evidence_id: suffixed_id(evidence_id_prefix, epoch_seconds, sequence_index).0,
        }
    }
}

pub(crate) fn screen_queue_job(
    record: &ScreenAiServiceCaptureRecord<'_>,
    ids: &ScreenAiServiceCaptureIds,
    image_digest: &ScreenText,
) -> ScreenAnalysisQueueJob {
    ScreenAnalysisQueueJob {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        queue_job_id: ids.queue_job_id.clone(),
        created_at: record.clock.timestamp.clone(),
        not_before: record.clock.timestamp.clone(),
        expires_at: record
            .clock
            .expires_after_seconds(record.temporary_image_ttl_seconds)
            .0,
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
        encrypted_image_ref: format!("screen-evidence:{}", ids.queue_job_id),
        image_digest: image_digest.0.clone(),
        image_byte_size: record.image.png_bytes.len() as u64,
        image_format: SCREEN_IMAGE_FORMAT_PNG.to_string(),
        status: SCREEN_QUEUE_STATUS_QUEUED.to_string(),
        attempt_count: 0,
        max_retry_count: DEFAULT_MAX_RETRY_COUNT,
        failure_reason: None,
        unavailable_reason: None,
        deletion_required: true,
        deleted_at: None,
        deletion_status: SCREEN_DELETION_REQUIRED.to_string(),
        deletion_proof_ref: None,
        custody_state: SCREEN_CUSTODY_TEMP_QUEUE.to_string(),
    }
}

pub(crate) fn screen_analysis_event(
    record: &ScreenAiServiceCaptureRecord<'_>,
    ids: &ScreenAiServiceCaptureIds,
    job: &ScreenAnalysisQueueJob,
    image_digest: &ScreenText,
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
            display_name: None,
        },
        fields: fields_from_pairs(
            screen_analysis_fields(record, ids, job, image_digest)
                .into_iter()
                .map(|entry| (entry.key, entry.value))
                .collect(),
        ),
        evidence,
    }
}

fn screen_analysis_evidence(
    ids: &ScreenAiServiceCaptureIds,
    _job: &ScreenAnalysisQueueJob,
    image_digest: &ScreenText,
) -> Vec<ActivityEvidenceRef> {
    vec![ActivityEvidenceRef {
        evidence_id: ids.evidence_id.clone(),
        kind: ActivityEvidenceKind::Screenshot,
        digest: Some(image_digest.0.clone()),
        uri: None,
    }]
}

fn screen_analysis_fields(
    record: &ScreenAiServiceCaptureRecord<'_>,
    ids: &ScreenAiServiceCaptureIds,
    job: &ScreenAnalysisQueueJob,
    image_digest: &ScreenText,
) -> Vec<ScreenFieldEntry> {
    let mut fields = Vec::new();
    fields.extend(screen_analysis_identity_fields(ids, job));
    fields.extend(screen_analysis_model_fields(record));
    fields.extend(screen_analysis_capture_fields(
        job,
        image_digest,
        record.image,
    ));
    fields
}

fn screen_analysis_identity_fields(
    ids: &ScreenAiServiceCaptureIds,
    job: &ScreenAnalysisQueueJob,
) -> Vec<ScreenFieldEntry> {
    vec![
        string_field(
            ScreenFieldKey(constants::field::SCREEN_ANALYSIS_RESULT_ID),
            ScreenText(ids.result_id.clone()),
        ),
        string_field(
            ScreenFieldKey(constants::field::SCREEN_QUEUE_JOB_ID),
            ScreenText(job.queue_job_id.clone()),
        ),
        string_field(
            ScreenFieldKey(constants::field::SCREEN_SUMMARY),
            ScreenText(constants::activity_surface::SUMMARY_READY.to_string()),
        ),
        string_field(
            ScreenFieldKey(constants::field::SCREEN_PRIMARY_CATEGORY),
            ScreenText(SCREEN_CATEGORY_UNKNOWN.to_string()),
        ),
    ]
}

fn screen_analysis_model_fields(
    record: &ScreenAiServiceCaptureRecord<'_>,
) -> Vec<ScreenFieldEntry> {
    vec![
        number_field(
            ScreenFieldKey(constants::field::SCREEN_CONFIDENCE),
            SCREEN_SERVICE_METADATA_CONFIDENCE,
        ),
        string_field(
            ScreenFieldKey(constants::field::SCREEN_IMAGE_DELETION_STATE),
            ScreenText(SCREEN_DELETION_REQUIRED.to_string()),
        ),
        bool_field(
            ScreenFieldKey(constants::field::SCREEN_POLICY_ELIGIBLE),
            false,
        ),
        string_field(
            ScreenFieldKey(constants::field::SCREEN_MODEL_RUNTIME_REF),
            ScreenText(SCREEN_SERVICE_MODEL_RUNTIME_REF.to_string()),
        ),
        string_field(
            ScreenFieldKey(constants::field::SCREEN_MODEL_ID),
            ScreenText(record.model_id.to_string()),
        ),
        string_field(
            ScreenFieldKey(constants::field::SCREEN_PROVIDER_KIND),
            ScreenText(SCREEN_PROVIDER_SERVICE_METADATA.to_string()),
        ),
        string_field(
            ScreenFieldKey(constants::field::SCREEN_TEMPLATE_VERSION),
            ScreenText(record.template_version.to_string()),
        ),
    ]
}

fn screen_analysis_capture_fields(
    job: &ScreenAnalysisQueueJob,
    image_digest: &ScreenText,
    image: &CapturedScreenImage,
) -> Vec<ScreenFieldEntry> {
    vec![
        string_field(
            ScreenFieldKey(constants::field::SCREEN_CAPTURE_REASON),
            ScreenText(job.capture_reason.clone()),
        ),
        string_field(
            ScreenFieldKey(constants::field::SCREEN_CAPTURE_SCOPE),
            ScreenText(job.capture_scope.clone()),
        ),
        string_field(
            ScreenFieldKey(constants::field::CAPABILITY_STATUS),
            ScreenText(image.metadata.status.as_protocol_str().to_string()),
        ),
        string_field(
            ScreenFieldKey(constants::field::SCREEN_IMAGE_DIGEST),
            ScreenText(image_digest.0.clone()),
        ),
        string_field(
            ScreenFieldKey(constants::field::SCREEN_CUSTODY_STATE),
            ScreenText(SCREEN_CUSTODY_JOURNAL.to_string()),
        ),
    ]
}

fn suffixed_id(prefix: ScreenIdPrefix, epoch_seconds: u64, tick_index: u64) -> ScreenText {
    let mut id = String::from(prefix.0);
    id.push_str(&epoch_seconds.to_string());
    id.push(constants::delimiter::HYPHEN);
    id.push_str(&tick_index.to_string());
    ScreenText(id)
}

fn string_field(key: ScreenFieldKey, value: ScreenText) -> ScreenFieldEntry {
    ScreenFieldEntry {
        key: key.0,
        value: LogFieldValue::String(value.0),
    }
}

fn number_field(key: ScreenFieldKey, value: f64) -> ScreenFieldEntry {
    ScreenFieldEntry {
        key: key.0,
        value: LogFieldValue::Number(value),
    }
}

fn bool_field(key: ScreenFieldKey, value: bool) -> ScreenFieldEntry {
    ScreenFieldEntry {
        key: key.0,
        value: LogFieldValue::Boolean(value),
    }
}

#[derive(Clone, Copy)]
struct ScreenFieldKey(&'static str);
