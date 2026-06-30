use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityEvidenceKind, ActivityEvidenceRef, ActivityObserver,
    ActivitySource, ActivitySubject, ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::{
    generation::LocalAiChatGenerationResult, lifecycle::LocalAiGenerationState,
};
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::screen_evidence::{
    ScreenAnalysisResult, SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW, SCREEN_CUSTODY_JOURNAL,
    SCREEN_DELETION_DELETED, SCREEN_PROVIDER_LOCAL_OCR, SCREEN_PROVIDER_LOCAL_VISION,
    SCREEN_SERVICE_ANALYSIS_EVENT_ID_PREFIX, SCREEN_SERVICE_ANALYSIS_EVIDENCE_ID_PREFIX,
    SCREEN_SERVICE_ANALYSIS_RESULT_ID_PREFIX, SCREEN_SERVICE_ANALYSIS_SOURCE_ID,
};
use ocentra_parent_agent_protocol::ACTIVITY_SCHEMA_VERSION;

use crate::fields::fields_from_pairs;

use super::{
    config::ScreenOcrRedactionPolicy, queue::QueuedScreenImage, ScreenAiAnalysisCycleClock,
    ScreenAiAnalysisCycleOutcome,
};

#[path = "event_record/parsed_fields.rs"]
mod parsed_fields;
#[path = "event_record/policy_refs.rs"]
mod policy_refs;
#[path = "event_record/redaction_fields.rs"]
mod redaction_fields;

use parsed_fields::parsed_fields_from_generation;
use redaction_fields::screen_analysis_redaction_fields;

#[derive(Clone, Debug)]
pub(crate) struct ScreenAiAnalysisEventRecord {
    queue_job_id: String,
    image_digest: String,
    timestamp: String,
    summary: String,
    primary_category: String,
    confidence: f64,
    policy_eligible: bool,
    pub(super) provider_kind: String,
    model_runtime_ref: String,
    model_id: String,
    prompt_or_template_version: String,
    capture_reason: String,
    capture_scope: String,
    capability_status: String,
    policy_decision_ref: Option<String>,
    policy_action: Option<String>,
    policy_reason_codes: Vec<String>,
    parent_rule_refs: Vec<String>,
    parent_explanation_refs: Vec<String>,
    explanation_reasons: Vec<String>,
    deletion_reasons: Vec<String>,
    ocr_text_snippets: Vec<String>,
    redaction_notes: Vec<String>,
}

pub(crate) fn analysis_event_record(
    image: &QueuedScreenImage,
    metadata: Option<&ScreenAnalysisResult>,
    clock: &ScreenAiAnalysisCycleClock,
    generation: &LocalAiChatGenerationResult,
    redaction_policy: &ScreenOcrRedactionPolicy,
) -> ScreenAiAnalysisEventRecord {
    let parsed = parsed_fields_from_generation(generation, redaction_policy);
    let policy = policy_refs::service_policy_refs(&image.queue_job_id, parsed.policy_eligible);
    ScreenAiAnalysisEventRecord {
        queue_job_id: image.queue_job_id.clone(),
        image_digest: image.image_digest.clone(),
        timestamp: clock.timestamp.clone(),
        summary: parsed.summary,
        primary_category: parsed.category,
        confidence: parsed.confidence,
        policy_eligible: parsed.policy_eligible,
        provider_kind: parsed.provider_kind,
        model_runtime_ref: parsed.model_runtime_ref,
        model_id: parsed.model_id,
        prompt_or_template_version: parsed.template_version,
        capture_reason: capture_reason(metadata).to_string(),
        capture_scope: capture_scope(metadata).to_string(),
        capability_status: capability_status(metadata).to_string(),
        policy_decision_ref: policy.policy_decision_ref,
        policy_action: policy.policy_action,
        policy_reason_codes: policy.policy_reason_codes,
        parent_rule_refs: policy.parent_rule_refs,
        parent_explanation_refs: policy.parent_explanation_refs,
        explanation_reasons: policy.explanation_reasons,
        deletion_reasons: policy.deletion_reasons,
        ocr_text_snippets: parsed.ocr_text_snippets,
        redaction_notes: parsed.redaction_notes,
    }
}

pub(crate) fn outcome_for_generation(
    queue_job_id: &str,
    generation: &LocalAiChatGenerationResult,
    event_record: &ScreenAiAnalysisEventRecord,
) -> ScreenAiAnalysisCycleOutcome {
    if is_recorded_provider_kind(&event_record.provider_kind) {
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

fn is_recorded_provider_kind(provider_kind: &str) -> bool {
    provider_kind == SCREEN_PROVIDER_LOCAL_VISION || provider_kind == SCREEN_PROVIDER_LOCAL_OCR
}

pub(crate) fn screen_analysis_event(record: &ScreenAiAnalysisEventRecord) -> ActivityEvent {
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
    let mut fields = vec![
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
            record.model_runtime_ref.clone(),
        ),
        string_field(constants::field::SCREEN_MODEL_ID, record.model_id.clone()),
        string_field(
            constants::field::SCREEN_PROVIDER_KIND,
            record.provider_kind.clone(),
        ),
        string_field(
            constants::field::SCREEN_TEMPLATE_VERSION,
            record.prompt_or_template_version.clone(),
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
    ];
    fields.extend(policy_refs::screen_analysis_policy_fields(record));
    fields.extend(screen_analysis_redaction_fields(record));
    fields
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

fn capability_status(metadata: Option<&ScreenAnalysisResult>) -> &str {
    metadata
        .map(|result| result.capability_status.as_str())
        .unwrap_or(ActivityCaptureCapabilityStatus::Available.as_protocol_str())
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
