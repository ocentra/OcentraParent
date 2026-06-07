use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, ActivityEvent, ActivityEventKind,
    ActivityEvidenceKind, ActivityEvidenceRef, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind, LocalAiChatGenerationResult, LocalAiGenerationState, LogFieldValue,
    ScreenAnalysisResult, ACTIVITY_SCHEMA_VERSION, SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW,
    SCREEN_CUSTODY_JOURNAL, SCREEN_DELETION_DELETED, SCREEN_PROVIDER_LOCAL_OCR,
    SCREEN_PROVIDER_LOCAL_VISION, SCREEN_SERVICE_ANALYSIS_EVENT_ID_PREFIX,
    SCREEN_SERVICE_ANALYSIS_EVIDENCE_ID_PREFIX, SCREEN_SERVICE_ANALYSIS_RESULT_ID_PREFIX,
    SCREEN_SERVICE_ANALYSIS_SOURCE_ID,
};

use crate::fields::fields_from_pairs;

use super::{queue::QueuedScreenImage, ScreenAiAnalysisCycleClock, ScreenAiAnalysisCycleOutcome};

mod parsed_fields;
mod policy_refs;
mod redaction_fields;

use parsed_fields::parsed_fields_from_generation;
use redaction_fields::screen_analysis_redaction_fields;

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

pub(super) fn analysis_event_record(
    image: &QueuedScreenImage,
    metadata: Option<&ScreenAnalysisResult>,
    clock: &ScreenAiAnalysisCycleClock,
    generation: &LocalAiChatGenerationResult,
) -> ScreenAiAnalysisEventRecord {
    let parsed = parsed_fields_from_generation(generation);
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

pub(super) fn outcome_for_generation(
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

#[cfg(test)]
mod tests {
    use serde_json::{Map, Value};

    use ocentra_parent_agent_protocol::{
        SCREEN_CATEGORY_SCHOOL, SCREEN_POLICY_CONFIDENCE_READY, SCREEN_PROVIDER_LOCAL_OCR,
        SCREEN_SERVICE_ANALYSIS_DEFAULT_ADAPTER_TIMEOUT_MS, SCREEN_SERVICE_ANALYSIS_MODEL_ID,
        SCREEN_SERVICE_ANALYSIS_MODEL_REFERENCE, SCREEN_SERVICE_ANALYSIS_PROVIDER_ID,
        SCREEN_SERVICE_ANALYSIS_RUNTIME_REF, SCREEN_WINRT_OCR_MODEL_ID,
        SCREEN_WINRT_OCR_RUNTIME_REF, SCREEN_WINRT_OCR_TEMPLATE_VERSION,
    };

    use super::*;

    #[test]
    fn local_ocr_analysis_event_is_recorded_with_runtime_metadata() {
        let image = queued_image();
        let generation = complete_generation(local_ocr_output_text());
        let clock = ScreenAiAnalysisCycleClock::from_parts(
            7,
            constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        );

        let record = analysis_event_record(&image, None, &clock, &generation);
        let outcome = outcome_for_generation(&image.queue_job_id, &generation, &record);
        let event = screen_analysis_event(&record);

        assert_eq!(
            outcome,
            ScreenAiAnalysisCycleOutcome::Recorded {
                queue_job_id: constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string(),
                provider_kind: SCREEN_PROVIDER_LOCAL_OCR.to_string()
            }
        );
        assert_eq!(
            string_value(&event, constants::field::SCREEN_PROVIDER_KIND),
            SCREEN_PROVIDER_LOCAL_OCR
        );
        assert_eq!(
            string_value(&event, constants::field::SCREEN_MODEL_RUNTIME_REF),
            SCREEN_WINRT_OCR_RUNTIME_REF
        );
        assert_eq!(
            string_value(&event, constants::field::SCREEN_MODEL_ID),
            SCREEN_WINRT_OCR_MODEL_ID
        );
        assert_eq!(
            string_value(&event, constants::field::SCREEN_TEMPLATE_VERSION),
            SCREEN_WINRT_OCR_TEMPLATE_VERSION
        );
        assert_eq!(
            string_value(&event, constants::field::SCREEN_OCR_TEXT_SNIPPETS),
            constants::activity_store::TEST_SCREEN_OCR_SNIPPET_WIKIPEDIA
        );
        assert_eq!(
            string_value(&event, constants::field::SCREEN_REDACTION_NOTES),
            constants::activity_store::TEST_SCREEN_REDACTION_NOTE_PII
        );
    }

    fn queued_image() -> QueuedScreenImage {
        QueuedScreenImage {
            queue_job_id: constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string(),
            custody_state: ocentra_parent_agent_protocol::SCREEN_CUSTODY_TEMP_QUEUE.to_string(),
            image_digest: constants::activity_store::TEST_SCREEN_IMAGE_DIGEST.to_string(),
            image_bytes: constants::activity_store::TEST_SCREEN_PLAINTEXT_MARKER
                .as_bytes()
                .to_vec(),
        }
    }

    fn local_ocr_output_text() -> String {
        let mut output = Map::new();
        output.insert(
            constants::field::SCREEN_SUMMARY.to_string(),
            Value::from(constants::activity_store::TEST_SCREEN_SUMMARY),
        );
        output.insert(
            constants::field::SCREEN_PRIMARY_CATEGORY.to_string(),
            Value::from(SCREEN_CATEGORY_SCHOOL),
        );
        output.insert(
            constants::field::SCREEN_CONFIDENCE.to_string(),
            Value::from(SCREEN_POLICY_CONFIDENCE_READY),
        );
        output.insert(
            constants::field::SCREEN_POLICY_ELIGIBLE.to_string(),
            Value::from(true),
        );
        output.insert(
            constants::field::SCREEN_PROVIDER_KIND.to_string(),
            Value::from(SCREEN_PROVIDER_LOCAL_OCR),
        );
        output.insert(
            constants::field::SCREEN_MODEL_RUNTIME_REF.to_string(),
            Value::from(SCREEN_WINRT_OCR_RUNTIME_REF),
        );
        output.insert(
            constants::field::SCREEN_MODEL_ID.to_string(),
            Value::from(SCREEN_WINRT_OCR_MODEL_ID),
        );
        output.insert(
            constants::field::SCREEN_TEMPLATE_VERSION.to_string(),
            Value::from(SCREEN_WINRT_OCR_TEMPLATE_VERSION),
        );
        output.insert(
            constants::field::SCREEN_OCR_TEXT_SNIPPETS.to_string(),
            Value::from(vec![
                constants::activity_store::TEST_SCREEN_OCR_SNIPPET_WIKIPEDIA,
            ]),
        );
        output.insert(
            constants::field::SCREEN_REDACTION_NOTES.to_string(),
            Value::from(vec![
                constants::activity_store::TEST_SCREEN_REDACTION_NOTE_PII,
            ]),
        );
        Value::Object(output).to_string()
    }

    fn complete_generation(output_text: String) -> LocalAiChatGenerationResult {
        LocalAiChatGenerationResult {
            local_ai_result_id: constants::activity_store::TEST_SCREEN_RESULT_ID.to_string(),
            runtime_reference_id: SCREEN_SERVICE_ANALYSIS_RUNTIME_REF.to_string(),
            provider_id: SCREEN_SERVICE_ANALYSIS_PROVIDER_ID.to_string(),
            model_id: SCREEN_SERVICE_ANALYSIS_MODEL_ID.to_string(),
            model_reference: SCREEN_SERVICE_ANALYSIS_MODEL_REFERENCE.to_string(),
            generation_state: LocalAiGenerationState::Complete,
            output_text: Some(output_text),
            prompt_char_count: 1,
            max_output_tokens: constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
            timeout_ms: SCREEN_SERVICE_ANALYSIS_DEFAULT_ADAPTER_TIMEOUT_MS,
            duration_ms: 1,
            exit_code: Some(0),
            stderr_byte_size: 0,
            unavailable_reason: None,
        }
    }

    fn string_value<'a>(event: &'a ActivityEvent, field: &str) -> &'a str {
        match event.fields.get(field) {
            Some(LogFieldValue::String(value)) => value,
            _ => unreachable!(),
        }
    }
}
