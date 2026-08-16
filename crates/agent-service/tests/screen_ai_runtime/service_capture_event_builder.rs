use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::activity::ActivityEventKind;
use ocentra_parent_agent_protocol::activity::ActivityEvidenceKind;
use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::activity::ActivityObserver;
use ocentra_parent_agent_protocol::activity::ActivitySource;
use ocentra_parent_agent_protocol::activity::ActivitySubject;
use ocentra_parent_agent_protocol::activity::ActivitySubjectKind;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
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

use crate::screen_ai_cadence_runtime_event::ScreenAiServiceCaptureRecord;

use crate::test_text::TestText;

const DEFAULT_MAX_RETRY_COUNT: u64 = 0;
const DEFAULT_SETTING_VERSION: u64 = 1;

#[derive(Clone, Copy)]
pub(crate) struct ScreenIdPrefix(pub(crate) &'static str);

pub(crate) type ScreenText = TestText;

impl std::fmt::Display for ScreenIdPrefix {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.0)
    }
}

pub(crate) struct ScreenAiServiceCaptureIds {
    pub(crate) queue_job_id: TestText,
    result_id: TestText,
    event_id: TestText,
    evidence_id: TestText,
}

impl ScreenAiServiceCaptureIds {
    pub(crate) fn new(
        queue_job_id_prefix: impl std::fmt::Display,
        result_id_prefix: impl std::fmt::Display,
        event_id_prefix: impl std::fmt::Display,
        evidence_id_prefix: impl std::fmt::Display,
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
    image_digest: &TestText,
) -> ScreenAnalysisQueueJob {
    ScreenAnalysisQueueJob {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        queue_job_id: ids.queue_job_id.to_string(),
        created_at: record.clock.timestamp.clone(),
        not_before: record.clock.timestamp.clone(),
        expires_at: record
            .clock
            .expires_after_seconds(record.temporary_image_ttl_seconds)
            .to_string(),
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
        deletion_status: SCREEN_DELETION_REQUIRED.to_string(),
        deletion_proof_ref: None,
        custody_state: SCREEN_CUSTODY_TEMP_QUEUE.to_string(),
    }
}

pub(crate) fn screen_analysis_event(
    record: &ScreenAiServiceCaptureRecord<'_>,
    ids: &ScreenAiServiceCaptureIds,
    job: &ScreenAnalysisQueueJob,
    image_digest: &TestText,
) -> ActivityEvent {
    let evidence = screen_analysis_evidence(ids, job, image_digest);
    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: ids.event_id.to_string(),
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
        fields: fields_from_pairs_test(screen_analysis_fields(record, ids, job, image_digest)),
        evidence,
    }
}

fn screen_analysis_evidence(
    ids: &ScreenAiServiceCaptureIds,
    _job: &ScreenAnalysisQueueJob,
    image_digest: &TestText,
) -> Vec<ActivityEvidenceRef> {
    vec![ActivityEvidenceRef {
        evidence_id: ids.evidence_id.to_string(),
        kind: ActivityEvidenceKind::Screenshot,
        digest: Some(image_digest.to_string()),
        uri: None,
    }]
}

fn screen_analysis_fields(
    record: &ScreenAiServiceCaptureRecord<'_>,
    ids: &ScreenAiServiceCaptureIds,
    job: &ScreenAnalysisQueueJob,
    image_digest: &TestText,
) -> Vec<(TestText, LogFieldValue)> {
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
) -> Vec<(TestText, LogFieldValue)> {
    vec![
        string_field(
            constants::field::SCREEN_ANALYSIS_RESULT_ID,
            ids.result_id.to_string(),
        ),
        string_field(
            constants::field::SCREEN_QUEUE_JOB_ID,
            job.queue_job_id.clone(),
        ),
        string_field(
            constants::field::SCREEN_SUMMARY,
            constants::activity_surface::SUMMARY_READY,
        ),
        string_field(
            constants::field::SCREEN_PRIMARY_CATEGORY,
            SCREEN_CATEGORY_UNKNOWN,
        ),
    ]
}

fn screen_analysis_model_fields(
    record: &ScreenAiServiceCaptureRecord<'_>,
) -> Vec<(TestText, LogFieldValue)> {
    vec![
        number_field(
            constants::field::SCREEN_CONFIDENCE,
            SCREEN_SERVICE_METADATA_CONFIDENCE,
        ),
        string_field(
            constants::field::SCREEN_IMAGE_DELETION_STATE,
            SCREEN_DELETION_REQUIRED,
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
    image_digest: &TestText,
    image: &CapturedScreenImage,
) -> Vec<(TestText, LogFieldValue)> {
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

fn suffixed_id(prefix: impl std::fmt::Display, epoch_seconds: u64, tick_index: u64) -> TestText {
    let mut id = prefix.to_string();
    id.push_str(&epoch_seconds.to_string());
    id.push(constants::delimiter::HYPHEN);
    id.push_str(&tick_index.to_string());
    TestText::from_display(id)
}

fn fields_from_pairs_test(pairs: Vec<(TestText, LogFieldValue)>) -> LogFields {
    let mut fields = LogFields::new();
    for (key, value) in pairs {
        fields.insert(key.to_string(), value);
    }
    fields
}

fn string_field(
    key: impl std::fmt::Display,
    value: impl std::fmt::Display,
) -> (TestText, LogFieldValue) {
    (
        TestText::from_display(key),
        LogFieldValue::String(value.to_string()),
    )
}

fn number_field(key: impl std::fmt::Display, value: f64) -> (TestText, LogFieldValue) {
    (TestText::from_display(key), LogFieldValue::Number(value))
}

fn bool_field(key: impl std::fmt::Display, value: bool) -> (TestText, LogFieldValue) {
    (TestText::from_display(key), LogFieldValue::Boolean(value))
}
