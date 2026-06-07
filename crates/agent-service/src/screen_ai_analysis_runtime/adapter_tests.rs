use serde_json::{Map, Value};
use std::path::Path;

use ocentra_parent_agent_protocol::{
    constants, LocalAiChatGenerationResult, LocalAiGenerationState, SCREEN_CATEGORY_SCHOOL,
    SCREEN_POLICY_CONFIDENCE_READY, SCREEN_PROVIDER_LOCAL_OCR, SCREEN_PROVIDER_SERVICE_METADATA,
    SCREEN_SERVICE_ANALYSIS_DEFAULT_ADAPTER_TIMEOUT_MS, SCREEN_SERVICE_ANALYSIS_MODEL_ID,
    SCREEN_SERVICE_ANALYSIS_MODEL_REFERENCE, SCREEN_SERVICE_ANALYSIS_PROVIDER_ID,
    SCREEN_SERVICE_ANALYSIS_RUNTIME_REF, SCREEN_WINRT_OCR_MODEL_ID, SCREEN_WINRT_OCR_RUNTIME_REF,
    SCREEN_WINRT_OCR_TEMPLATE_VERSION,
};

use super::{adapter::parsed_generation_output, adapter_process::is_windows_batch_adapter};

#[test]
fn parsed_generation_output_preserves_local_ocr_runtime_metadata() {
    let parsed = parsed_generation_output(&complete_generation(adapter_output_text(
        SCREEN_PROVIDER_LOCAL_OCR,
    )))
    .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(parsed.provider_kind, SCREEN_PROVIDER_LOCAL_OCR);
    assert_eq!(parsed.model_runtime_ref, SCREEN_WINRT_OCR_RUNTIME_REF);
    assert_eq!(parsed.model_id, SCREEN_WINRT_OCR_MODEL_ID);
    assert_eq!(
        parsed.prompt_or_template_version,
        SCREEN_WINRT_OCR_TEMPLATE_VERSION
    );
    assert_eq!(
        parsed.ocr_text_snippets,
        vec![constants::activity_store::TEST_SCREEN_OCR_SNIPPET_WIKIPEDIA.to_string()]
    );
    assert_eq!(
        parsed.redaction_notes,
        vec![constants::activity_store::TEST_SCREEN_REDACTION_NOTE_PII.to_string()]
    );
    assert_eq!(parsed.confidence, SCREEN_POLICY_CONFIDENCE_READY);
    assert!(parsed.policy_eligible);
}

#[test]
fn parsed_generation_output_rejects_unknown_provider_kind() {
    let parsed = parsed_generation_output(&complete_generation(adapter_output_text(
        SCREEN_PROVIDER_SERVICE_METADATA,
    )));

    assert_eq!(parsed, None);
}

#[test]
fn parsed_generation_output_applies_service_ocr_redaction() {
    let parsed = parsed_generation_output(&complete_generation(sensitive_adapter_output_text()))
        .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        parsed.ocr_text_snippets,
        vec![
            constants::activity_store::TEST_SCREEN_OCR_SNIPPET_EMAIL_REDACTED.to_string(),
            constants::activity_store::TEST_SCREEN_OCR_SNIPPET_PHONE_REDACTED.to_string()
        ]
    );
    assert!(!parsed
        .ocr_text_snippets
        .iter()
        .any(|snippet| snippet == constants::activity_store::TEST_SCREEN_OCR_SNIPPET_EMAIL_RAW));
    assert!(!parsed.ocr_text_snippets.iter().any(
        |snippet| snippet == constants::activity_store::TEST_SCREEN_OCR_SNIPPET_CREDENTIAL_RAW
    ));
    assert!(parsed
        .redaction_notes
        .iter()
        .any(|note| note == constants::local_ai_runtime::SCREEN_OCR_REDACTION_NOTE_PII));
    assert!(parsed
        .redaction_notes
        .iter()
        .any(|note| note == constants::local_ai_runtime::SCREEN_OCR_REDACTION_NOTE_CREDENTIAL));
}

#[test]
fn adapter_launch_detects_windows_batch_wrappers() {
    assert!(is_windows_batch_adapter(Path::new(
        constants::local_ai_runtime::TEST_SCREEN_SERVICE_WINRT_OCR_ADAPTER_CMD
    )));
    assert!(is_windows_batch_adapter(Path::new(
        constants::local_ai_runtime::TEST_SCREEN_SERVICE_WINRT_OCR_ADAPTER_BAT
    )));
    assert!(!is_windows_batch_adapter(Path::new(
        constants::local_ai_runtime::TEST_SCREEN_SERVICE_WINRT_OCR_ADAPTER_EXE
    )));
}

fn adapter_output_text(provider_kind: &str) -> String {
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
        Value::from(provider_kind),
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

fn sensitive_adapter_output_text() -> String {
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
        constants::field::SCREEN_OCR_TEXT_SNIPPETS.to_string(),
        Value::from(vec![
            constants::activity_store::TEST_SCREEN_OCR_SNIPPET_EMAIL_RAW,
            constants::activity_store::TEST_SCREEN_OCR_SNIPPET_CREDENTIAL_RAW,
            constants::activity_store::TEST_SCREEN_OCR_SNIPPET_PHONE_RAW,
        ]),
    );
    output.insert(
        constants::field::SCREEN_REDACTION_NOTES.to_string(),
        Value::from(Vec::<String>::new()),
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
