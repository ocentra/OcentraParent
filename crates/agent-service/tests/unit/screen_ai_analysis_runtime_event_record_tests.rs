use serde_json::{Map, Value};
use std::primitive::str as TestStr;
use std::string::String as TestString;

use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::{
    generation::LocalAiChatGenerationResult, lifecycle::LocalAiGenerationState,
};
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::screen_evidence::{
    SCREEN_CATEGORY_SCHOOL, SCREEN_POLICY_CONFIDENCE_READY, SCREEN_PROVIDER_LOCAL_OCR,
    SCREEN_SERVICE_ANALYSIS_DEFAULT_ADAPTER_TIMEOUT_MS, SCREEN_SERVICE_ANALYSIS_MODEL_ID,
    SCREEN_SERVICE_ANALYSIS_MODEL_REFERENCE, SCREEN_SERVICE_ANALYSIS_PROVIDER_ID,
    SCREEN_SERVICE_ANALYSIS_RUNTIME_REF, SCREEN_WINRT_OCR_MODEL_ID, SCREEN_WINRT_OCR_RUNTIME_REF,
    SCREEN_WINRT_OCR_TEMPLATE_VERSION,
};

use super::{
    config::{ScreenAiAnalysisCycleClock, ScreenAiAnalysisCycleOutcome, ScreenOcrRedactionPolicy},
    event_record::{analysis_event_record, outcome_for_generation, screen_analysis_event},
    queue::QueuedScreenImage,
};

#[test]
fn local_ocr_analysis_event_is_recorded_with_runtime_metadata() {
    let image = queued_image();
    let generation = complete_generation(local_ocr_output_text());
    let clock = ScreenAiAnalysisCycleClock {
        epoch_seconds: 7,
        timestamp: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
    };

    let record = analysis_event_record(
        &image,
        None,
        &clock,
        &generation,
        &ScreenOcrRedactionPolicy::default(),
    );
    let outcome = outcome_for_generation(&image, &generation, &record);
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
        Some(SCREEN_PROVIDER_LOCAL_OCR)
    );
    assert_eq!(
        string_value(&event, constants::field::SCREEN_MODEL_RUNTIME_REF),
        Some(SCREEN_WINRT_OCR_RUNTIME_REF)
    );
    assert_eq!(
        string_value(&event, constants::field::SCREEN_MODEL_ID),
        Some(SCREEN_WINRT_OCR_MODEL_ID)
    );
    assert_eq!(
        string_value(&event, constants::field::SCREEN_TEMPLATE_VERSION),
        Some(SCREEN_WINRT_OCR_TEMPLATE_VERSION)
    );
    assert_eq!(
        string_value(&event, constants::field::SCREEN_OCR_TEXT_SNIPPETS),
        Some(constants::activity_store::TEST_SCREEN_OCR_SNIPPET_WIKIPEDIA)
    );
    assert_eq!(
        string_value(&event, constants::field::SCREEN_REDACTION_NOTES),
        Some(constants::activity_store::TEST_SCREEN_REDACTION_NOTE_PII)
    );
}

fn queued_image() -> QueuedScreenImage {
    QueuedScreenImage {
        queue_job_id: constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string(),
        custody_state: ocentra_parent_agent_protocol::screen_evidence::SCREEN_CUSTODY_TEMP_QUEUE
            .to_string(),
        image_digest: constants::activity_store::TEST_SCREEN_IMAGE_DIGEST.to_string(),
        image_bytes: constants::activity_store::TEST_SCREEN_PLAINTEXT_MARKER
            .as_bytes()
            .to_vec(),
    }
}

fn local_ocr_output_text() -> TestString {
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

fn complete_generation(output_text: TestString) -> LocalAiChatGenerationResult {
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

fn string_value(event: &ActivityEvent, field: impl std::fmt::Display) -> Option<&TestStr> {
    let field = field.to_string();
    match event.fields.get(field.as_str()) {
        Some(LogFieldValue::String(value)) => Some(value),
        _ => None,
    }
}
