use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, ActivityEvent, ActivityEventKind,
    ActivityEvidenceKind, ActivityEvidenceRef, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind, LocalAiChatGenerationResult, LocalAiGenerationState, LogFieldValue,
    ScreenAnalysisResult, ACTIVITY_SCHEMA_VERSION, SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW,
    SCREEN_CATEGORY_UNKNOWN, SCREEN_CUSTODY_JOURNAL, SCREEN_DELETION_DELETED,
    SCREEN_PROVIDER_LOCAL_VISION, SCREEN_PROVIDER_LOCAL_VISION_UNAVAILABLE,
    SCREEN_SERVICE_ANALYSIS_EVENT_ID_PREFIX, SCREEN_SERVICE_ANALYSIS_EVIDENCE_ID_PREFIX,
    SCREEN_SERVICE_ANALYSIS_MODEL_ID, SCREEN_SERVICE_ANALYSIS_RESULT_ID_PREFIX,
    SCREEN_SERVICE_ANALYSIS_RUNTIME_REF, SCREEN_SERVICE_ANALYSIS_SOURCE_ID,
    SCREEN_SERVICE_ANALYSIS_SUMMARY_INVALID, SCREEN_SERVICE_ANALYSIS_SUMMARY_UNAVAILABLE,
    SCREEN_SERVICE_ANALYSIS_TEMPLATE_VERSION, SCREEN_SERVICE_UNAVAILABLE_CONFIDENCE,
};

use crate::fields::fields_from_pairs;

use super::{
    adapter::parsed_generation_output, queue::QueuedScreenImage, ScreenAiAnalysisCycleClock,
    ScreenAiAnalysisCycleOutcome,
};

#[derive(Clone, Debug)]
pub(super) struct ScreenAiAnalysisEventRecord {
    queue_job_id: String,
    image_digest: String,
    timestamp: String,
    summary: String,
    primary_category: String,
    confidence: f64,
    policy_eligible: bool,
    pub(super) provider_kind: String,
    capture_reason: String,
    capture_scope: String,
    capability_status: String,
}

pub(super) fn analysis_event_record(
    image: &QueuedScreenImage,
    metadata: Option<&ScreenAnalysisResult>,
    clock: &ScreenAiAnalysisCycleClock,
    generation: &LocalAiChatGenerationResult,
) -> ScreenAiAnalysisEventRecord {
    let parsed = parsed_generation_output(generation);
    let (summary, category, confidence, policy_eligible, provider_kind) = match parsed {
        Some(output) => {
            let policy_eligible = output.policy_eligible
                && output.confidence
                    >= ocentra_parent_agent_protocol::SCREEN_POLICY_CONFIDENCE_READY;
            (
                output.summary,
                output.primary_category,
                output.confidence,
                policy_eligible,
                SCREEN_PROVIDER_LOCAL_VISION.to_string(),
            )
        }
        None if generation.generation_state == LocalAiGenerationState::Complete => (
            SCREEN_SERVICE_ANALYSIS_SUMMARY_INVALID.to_string(),
            SCREEN_CATEGORY_UNKNOWN.to_string(),
            SCREEN_SERVICE_UNAVAILABLE_CONFIDENCE,
            false,
            SCREEN_PROVIDER_LOCAL_VISION_UNAVAILABLE.to_string(),
        ),
        None => (
            SCREEN_SERVICE_ANALYSIS_SUMMARY_UNAVAILABLE.to_string(),
            SCREEN_CATEGORY_UNKNOWN.to_string(),
            SCREEN_SERVICE_UNAVAILABLE_CONFIDENCE,
            false,
            SCREEN_PROVIDER_LOCAL_VISION_UNAVAILABLE.to_string(),
        ),
    };
    ScreenAiAnalysisEventRecord {
        queue_job_id: image.queue_job_id.clone(),
        image_digest: image.image_digest.clone(),
        timestamp: clock.timestamp.clone(),
        summary,
        primary_category: category,
        confidence,
        policy_eligible,
        provider_kind,
        capture_reason: capture_reason(metadata).to_string(),
        capture_scope: capture_scope(metadata).to_string(),
        capability_status: metadata
            .map(|result| result.capability_status.as_str())
            .unwrap_or(ActivityCaptureCapabilityStatus::Available.as_protocol_str())
            .to_string(),
    }
}

pub(super) fn outcome_for_generation(
    queue_job_id: &str,
    generation: &LocalAiChatGenerationResult,
    event_record: &ScreenAiAnalysisEventRecord,
) -> ScreenAiAnalysisCycleOutcome {
    if event_record.provider_kind == SCREEN_PROVIDER_LOCAL_VISION {
        return ScreenAiAnalysisCycleOutcome::Recorded {
            queue_job_id: queue_job_id.to_string(),
            provider_kind: event_record.provider_kind.clone(),
        };
    }
    if generation.generation_state == LocalAiGenerationState::Complete {
        return ScreenAiAnalysisCycleOutcome::InvalidOutput {
            queue_job_id: queue_job_id.to_string(),
        };
    }
    ScreenAiAnalysisCycleOutcome::ProviderUnavailable {
        queue_job_id: queue_job_id.to_string(),
    }
}

pub(super) fn screen_analysis_event(record: &ScreenAiAnalysisEventRecord) -> ActivityEvent {
    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: prefixed_id(
            SCREEN_SERVICE_ANALYSIS_EVENT_ID_PREFIX,
            &record.queue_job_id,
        ),
        observed_at: record.timestamp.clone(),
        source: ActivitySource {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: std::env::consts::OS.to_string(),
            observer: ActivityObserver::LocalAi,
            source_id: SCREEN_SERVICE_ANALYSIS_SOURCE_ID.to_string(),
        },
        kind: ActivityEventKind::ScreenAnalysisSummarized,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Device,
            subject_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            display_name: None,
        },
        fields: fields_from_pairs(screen_analysis_fields(record)),
        evidence: vec![ActivityEvidenceRef {
            evidence_id: prefixed_id(
                SCREEN_SERVICE_ANALYSIS_EVIDENCE_ID_PREFIX,
                &record.queue_job_id,
            ),
            kind: ActivityEvidenceKind::Screenshot,
            digest: Some(record.image_digest.clone()),
            uri: None,
        }],
    }
}

fn screen_analysis_fields(
    record: &ScreenAiAnalysisEventRecord,
) -> Vec<(&'static str, LogFieldValue)> {
    vec![
        string_field(
            constants::field::SCREEN_ANALYSIS_RESULT_ID,
            prefixed_id(
                SCREEN_SERVICE_ANALYSIS_RESULT_ID_PREFIX,
                &record.queue_job_id,
            ),
        ),
        string_field(
            constants::field::SCREEN_QUEUE_JOB_ID,
            record.queue_job_id.clone(),
        ),
        string_field(constants::field::SCREEN_SUMMARY, record.summary.clone()),
        string_field(
            constants::field::SCREEN_PRIMARY_CATEGORY,
            record.primary_category.clone(),
        ),
        number_field(constants::field::SCREEN_CONFIDENCE, record.confidence),
        string_field(
            constants::field::SCREEN_IMAGE_DELETION_STATE,
            SCREEN_DELETION_DELETED,
        ),
        bool_field(
            constants::field::SCREEN_POLICY_ELIGIBLE,
            record.policy_eligible,
        ),
        string_field(
            constants::field::SCREEN_MODEL_RUNTIME_REF,
            SCREEN_SERVICE_ANALYSIS_RUNTIME_REF,
        ),
        string_field(
            constants::field::SCREEN_MODEL_ID,
            SCREEN_SERVICE_ANALYSIS_MODEL_ID,
        ),
        string_field(
            constants::field::SCREEN_PROVIDER_KIND,
            record.provider_kind.clone(),
        ),
        string_field(
            constants::field::SCREEN_TEMPLATE_VERSION,
            SCREEN_SERVICE_ANALYSIS_TEMPLATE_VERSION,
        ),
        string_field(
            constants::field::SCREEN_CAPTURE_REASON,
            record.capture_reason.clone(),
        ),
        string_field(
            constants::field::SCREEN_CAPTURE_SCOPE,
            record.capture_scope.clone(),
        ),
        string_field(
            constants::field::CAPABILITY_STATUS,
            record.capability_status.clone(),
        ),
        string_field(
            constants::field::SCREEN_IMAGE_DIGEST,
            record.image_digest.clone(),
        ),
        string_field(
            constants::field::SCREEN_CUSTODY_STATE,
            SCREEN_CUSTODY_JOURNAL,
        ),
    ]
}

fn capture_reason(metadata: Option<&ScreenAnalysisResult>) -> &str {
    metadata
        .map(|result| result.capture_reason.as_str())
        .unwrap_or(constants::activity_capture::SCREEN_TRIGGER_TIMED_CADENCE)
}

fn capture_scope(metadata: Option<&ScreenAnalysisResult>) -> &str {
    metadata
        .map(|result| result.capture_scope.as_str())
        .unwrap_or(SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW)
}

fn prefixed_id(prefix: &str, value: &str) -> String {
    let mut id = String::from(prefix);
    id.push_str(value);
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
