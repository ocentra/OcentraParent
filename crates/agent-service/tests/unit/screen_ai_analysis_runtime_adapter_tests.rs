use serde_json::{Map, Value};
use std::io::Error as IoError;
#[cfg(windows)]
use std::path::Path;
use std::primitive::str as TestStr;
use std::string::String as TestString;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::generation::LocalAiChatGenerationResult;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiGenerationState;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_CATEGORY_SCHOOL;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_POLICY_CONFIDENCE_READY;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_PROVIDER_LOCAL_OCR;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_PROVIDER_SERVICE_METADATA;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_DEFAULT_ADAPTER_TIMEOUT_MS;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_MODEL_ID;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_MODEL_REFERENCE;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_PROVIDER_ID;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_RUNTIME_REF;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_WINRT_OCR_MODEL_ID;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_WINRT_OCR_RUNTIME_REF;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_WINRT_OCR_TEMPLATE_VERSION;

#[cfg(windows)]
use super::adapter_process::is_windows_batch_adapter;
use super::{
    adapter::parsed_generation_output_with_policy, adapter_runtime_status,
    config::ScreenOcrRedactionPolicy,
};

type TestResult = Result<(), IoError>;

#[test]
fn adapter_runtime_status_fails_closed_without_a_configured_command() {
    let status = adapter_runtime_status(None, constants::activity_store::TEST_FIRST_OBSERVED_AT);

    assert_eq!(
        status.adapter_boundary,
        ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiAdapterBoundary::LocalAdapterUnavailable
    );
    assert_eq!(
        status.execution_state,
        ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiExecutionState::Disabled
    );
    assert_eq!(
        status.unavailable_reason.as_deref(),
        Some(constants::local_ai_runtime::UNAVAILABLE_REASON_RUNTIME_BINARY_UNCONFIGURED)
    );
    assert_eq!(
        status.last_checked_at,
        constants::activity_store::TEST_FIRST_OBSERVED_AT
    );
}

#[test]
fn parsed_generation_output_preserves_local_ocr_runtime_metadata() -> TestResult {
    let parsed = parsed_generation_output_with_policy(
        &complete_generation(adapter_output_text(SCREEN_PROVIDER_LOCAL_OCR)),
        &ScreenOcrRedactionPolicy::default(),
    )
    .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?;

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

    Ok(())
}

#[test]
fn parsed_generation_output_rejects_unknown_provider_kind() {
    let parsed = parsed_generation_output_with_policy(
        &complete_generation(adapter_output_text(SCREEN_PROVIDER_SERVICE_METADATA)),
        &ScreenOcrRedactionPolicy::default(),
    );

    assert_eq!(parsed, None);
}

#[test]
fn parsed_generation_output_applies_service_ocr_redaction() -> TestResult {
    let parsed = parsed_generation_output_with_policy(
        &complete_generation(sensitive_adapter_output_text()),
        &ScreenOcrRedactionPolicy::default(),
    )
    .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?;

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

    Ok(())
}

#[test]
fn parsed_generation_output_respects_parent_selected_disabled_ocr_text() -> TestResult {
    let parsed = parsed_generation_output_with_policy(
        &complete_generation(sensitive_adapter_output_text()),
        &ScreenOcrRedactionPolicy {
            ocr_text_enabled: false,
            snippet_limit: 0,
            redaction_mode: constants::local_ai_runtime::SCREEN_OCR_REDACTION_MODE_DISABLED
                .to_string(),
            text_retention_mode: constants::local_ai_runtime::SCREEN_OCR_TEXT_RETENTION_DISABLED
                .to_string(),
            credential_suppression_enabled: true,
            pii_redaction_enabled: false,
            parent_setting_ref: Some(
                constants::activity_store::TEST_SCREEN_PARENT_SETTING_REF.to_string(),
            ),
            setting_version: Some(1),
        },
    )
    .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?;

    assert!(parsed.ocr_text_snippets.is_empty());
    assert_eq!(
        parsed.redaction_notes,
        vec![constants::local_ai_runtime::SCREEN_OCR_REDACTION_NOTE_DISABLED.to_string()]
    );

    Ok(())
}

#[test]
fn parsed_generation_output_respects_parent_selected_bounded_snippets() -> TestResult {
    let parsed = parsed_generation_output_with_policy(
        &complete_generation(sensitive_adapter_output_text()),
        &ScreenOcrRedactionPolicy {
            ocr_text_enabled: true,
            snippet_limit: 1,
            redaction_mode:
                constants::local_ai_runtime::SCREEN_OCR_REDACTION_MODE_LOCAL_SENSITIVE_TEXT
                    .to_string(),
            text_retention_mode:
                constants::local_ai_runtime::SCREEN_OCR_TEXT_RETENTION_BOUNDED_SNIPPETS.to_string(),
            credential_suppression_enabled: true,
            pii_redaction_enabled: false,
            parent_setting_ref: Some(
                constants::activity_store::TEST_SCREEN_PARENT_SETTING_REF.to_string(),
            ),
            setting_version: Some(2),
        },
    )
    .ok_or_else(|| IoError::other(constants::error::AGENT_EVENT_SERIALIZES))?;

    assert_eq!(
        parsed.ocr_text_snippets,
        vec![constants::activity_store::TEST_SCREEN_OCR_SNIPPET_EMAIL_RAW.to_string()]
    );
    assert!(!parsed
        .redaction_notes
        .iter()
        .any(|note| note == constants::local_ai_runtime::SCREEN_OCR_REDACTION_NOTE_PII));
    assert!(parsed
        .redaction_notes
        .iter()
        .any(|note| note == constants::local_ai_runtime::SCREEN_OCR_REDACTION_NOTE_CREDENTIAL));

    Ok(())
}

#[cfg(windows)]
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

fn adapter_output_text(provider_kind: &TestStr) -> TestString {
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

fn sensitive_adapter_output_text() -> TestString {
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
        Value::from(Vec::<TestString>::new()),
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
