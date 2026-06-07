use std::{
    path::Path,
    process::Stdio,
    time::{Duration, Instant},
};

use base64::prelude::{Engine as _, BASE64_STANDARD};
use ocentra_parent_agent_protocol::{
    constants, LocalAiAdapterBoundary, LocalAiCapabilityFlag, LocalAiChatGenerationResult,
    LocalAiDegradedState, LocalAiExecutionState, LocalAiGenerationState, LocalAiModelLoadState,
    LocalAiProviderPrivacyMode, LocalAiProviderSource, LocalAiResourceClass,
    LocalModelRuntimeStatus, ScreenAnalysisResult, SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW,
    SCREEN_EVIDENCE_SCHEMA_VERSION, SCREEN_PROVIDER_LOCAL_OCR, SCREEN_PROVIDER_LOCAL_VISION,
    SCREEN_SERVICE_ANALYSIS_FIELD_IMAGE_BASE64, SCREEN_SERVICE_ANALYSIS_MODEL_ID,
    SCREEN_SERVICE_ANALYSIS_MODEL_REFERENCE, SCREEN_SERVICE_ANALYSIS_PROVIDER_ID,
    SCREEN_SERVICE_ANALYSIS_RUNTIME_REF, SCREEN_SERVICE_ANALYSIS_TEMPLATE_VERSION,
};
use serde_json::{Map, Value};
use tokio::{io::AsyncWriteExt, time::timeout};

use super::{
    adapter_output_fields::optional_string_array, adapter_process::adapter_process_command,
    adapter_redaction::apply_service_ocr_redaction, queue::QueuedScreenImage,
    ScreenAiAnalysisRuntimeConfig,
};

#[derive(Clone, Debug, PartialEq)]
pub(super) struct ScreenAiAnalysisAdapterOutput {
    pub(super) summary: String,
    pub(super) primary_category: String,
    pub(super) confidence: f64,
    pub(super) policy_eligible: bool,
    pub(super) provider_kind: String,
    pub(super) model_runtime_ref: String,
    pub(super) model_id: String,
    pub(super) prompt_or_template_version: String,
    pub(super) ocr_text_snippets: Vec<String>,
    pub(super) redaction_notes: Vec<String>,
}

pub(super) async fn run_adapter(
    config: &ScreenAiAnalysisRuntimeConfig,
    image: &QueuedScreenImage,
    metadata: Option<&ScreenAnalysisResult>,
) -> LocalAiChatGenerationResult {
    let Some(command) = config.adapter_command.as_ref() else {
        return unavailable_generation(config, image, 0);
    };
    if !command.is_file() {
        return unavailable_generation(config, image, 0);
    }
    let request = adapter_request(image, metadata);
    let request_bytes =
        serde_json::to_vec(&request).expect(constants::error::AGENT_EVENT_SERIALIZES);
    let started = Instant::now();
    let mut process = adapter_process_command(command);
    let mut child = match process
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(_) => return failed_generation(config, image, request_bytes.len() as u64, 0),
    };
    if let Some(mut stdin) = child.stdin.take() {
        if stdin.write_all(&request_bytes).await.is_err() {
            return failed_generation(config, image, request_bytes.len() as u64, 0);
        }
    }
    match timeout(
        Duration::from_millis(config.adapter_timeout_ms),
        child.wait_with_output(),
    )
    .await
    {
        Ok(Ok(output)) => generation_from_process_output(
            config,
            image,
            request_bytes.len() as u64,
            started.elapsed().as_millis() as u64,
            output,
        ),
        Ok(Err(_)) => failed_generation(
            config,
            image,
            request_bytes.len() as u64,
            started.elapsed().as_millis() as u64,
        ),
        Err(_) => timed_out_generation(config, image, request_bytes.len() as u64),
    }
}

pub(super) fn runtime_status(command: Option<&Path>, timestamp: &str) -> LocalModelRuntimeStatus {
    let available = command.is_some_and(Path::is_file);
    LocalModelRuntimeStatus {
        runtime_reference_id: SCREEN_SERVICE_ANALYSIS_RUNTIME_REF.to_string(),
        provider_id: SCREEN_SERVICE_ANALYSIS_PROVIDER_ID.to_string(),
        model_id: SCREEN_SERVICE_ANALYSIS_MODEL_ID.to_string(),
        model_reference: SCREEN_SERVICE_ANALYSIS_MODEL_REFERENCE.to_string(),
        privacy_mode: LocalAiProviderPrivacyMode::LocalOnly,
        adapter_boundary: if available {
            LocalAiAdapterBoundary::LocalAdapterReady
        } else {
            LocalAiAdapterBoundary::LocalAdapterUnavailable
        },
        execution_state: if available {
            LocalAiExecutionState::DryRunReady
        } else {
            LocalAiExecutionState::Disabled
        },
        provider_source: if available {
            LocalAiProviderSource::LocalConfig
        } else {
            LocalAiProviderSource::Unavailable
        },
        load_state: if available {
            LocalAiModelLoadState::Loaded
        } else {
            LocalAiModelLoadState::Unavailable
        },
        capability_flags: if available {
            vec![
                LocalAiCapabilityFlag::Classification,
                LocalAiCapabilityFlag::SafetyDecision,
            ]
        } else {
            Vec::new()
        },
        resource_class: if available {
            LocalAiResourceClass::Cpu
        } else {
            LocalAiResourceClass::RemoteUnavailable
        },
        degraded_state: if available {
            LocalAiDegradedState::None
        } else {
            LocalAiDegradedState::ProviderUnavailable
        },
        last_checked_at: timestamp.to_string(),
        unavailable_reason: if available {
            None
        } else {
            Some(
                constants::local_ai_runtime::UNAVAILABLE_REASON_RUNTIME_BINARY_UNCONFIGURED
                    .to_string(),
            )
        },
    }
}

pub(super) fn parsed_generation_output(
    generation: &LocalAiChatGenerationResult,
) -> Option<ScreenAiAnalysisAdapterOutput> {
    if generation.generation_state != LocalAiGenerationState::Complete {
        return None;
    }
    let output = generation.output_text.as_ref()?;
    let parsed = serde_json::from_str::<Value>(output).ok()?;
    let summary = required_string(&parsed, constants::field::SCREEN_SUMMARY)?;
    let primary_category = required_string(&parsed, constants::field::SCREEN_PRIMARY_CATEGORY)?;
    let confidence = required_f64(&parsed, constants::field::SCREEN_CONFIDENCE)?;
    if !(0.0..=1.0).contains(&confidence) {
        return None;
    }
    let provider_kind = output_provider_kind(&parsed)?;
    Some(apply_service_ocr_redaction(ScreenAiAnalysisAdapterOutput {
        summary,
        primary_category,
        confidence,
        policy_eligible: required_bool(&parsed, constants::field::SCREEN_POLICY_ELIGIBLE)?,
        provider_kind,
        model_runtime_ref: optional_string(&parsed, constants::field::SCREEN_MODEL_RUNTIME_REF)
            .unwrap_or_else(|| SCREEN_SERVICE_ANALYSIS_RUNTIME_REF.to_string()),
        model_id: optional_string(&parsed, constants::field::SCREEN_MODEL_ID)
            .unwrap_or_else(|| SCREEN_SERVICE_ANALYSIS_MODEL_ID.to_string()),
        prompt_or_template_version: optional_string(
            &parsed,
            constants::field::SCREEN_TEMPLATE_VERSION,
        )
        .unwrap_or_else(|| SCREEN_SERVICE_ANALYSIS_TEMPLATE_VERSION.to_string()),
        ocr_text_snippets: optional_string_array(
            &parsed,
            constants::field::SCREEN_OCR_TEXT_SNIPPETS,
        ),
        redaction_notes: optional_string_array(&parsed, constants::field::SCREEN_REDACTION_NOTES),
    }))
}

fn output_provider_kind(value: &Value) -> Option<String> {
    let provider_kind = optional_string(value, constants::field::SCREEN_PROVIDER_KIND)
        .unwrap_or_else(|| SCREEN_PROVIDER_LOCAL_VISION.to_string());
    if provider_kind == SCREEN_PROVIDER_LOCAL_VISION || provider_kind == SCREEN_PROVIDER_LOCAL_OCR {
        Some(provider_kind)
    } else {
        None
    }
}

fn adapter_request(image: &QueuedScreenImage, metadata: Option<&ScreenAnalysisResult>) -> Value {
    let mut request = Map::new();
    request.insert(
        constants::field::SCHEMA_VERSION.to_string(),
        Value::from(u64::from(SCREEN_EVIDENCE_SCHEMA_VERSION)),
    );
    request.insert(
        constants::field::SCREEN_QUEUE_JOB_ID.to_string(),
        Value::from(image.queue_job_id.clone()),
    );
    request.insert(
        constants::field::SCREEN_IMAGE_DIGEST.to_string(),
        Value::from(image.image_digest.clone()),
    );
    request.insert(
        SCREEN_SERVICE_ANALYSIS_FIELD_IMAGE_BASE64.to_string(),
        Value::from(BASE64_STANDARD.encode(&image.image_bytes)),
    );
    request.insert(
        constants::field::SCREEN_TEMPLATE_VERSION.to_string(),
        Value::from(SCREEN_SERVICE_ANALYSIS_TEMPLATE_VERSION),
    );
    request.insert(
        constants::field::SCREEN_CAPTURE_REASON.to_string(),
        Value::from(capture_reason(metadata)),
    );
    request.insert(
        constants::field::SCREEN_CAPTURE_SCOPE.to_string(),
        Value::from(capture_scope(metadata)),
    );
    Value::Object(request)
}

fn generation_from_process_output(
    config: &ScreenAiAnalysisRuntimeConfig,
    image: &QueuedScreenImage,
    prompt_char_count: u64,
    duration_ms: u64,
    output: std::process::Output,
) -> LocalAiChatGenerationResult {
    let generation_state = if output.status.success() {
        LocalAiGenerationState::Complete
    } else {
        LocalAiGenerationState::Failed
    };
    LocalAiChatGenerationResult {
        local_ai_result_id: local_ai_result_id(&image.queue_job_id),
        runtime_reference_id: SCREEN_SERVICE_ANALYSIS_RUNTIME_REF.to_string(),
        provider_id: SCREEN_SERVICE_ANALYSIS_PROVIDER_ID.to_string(),
        model_id: SCREEN_SERVICE_ANALYSIS_MODEL_ID.to_string(),
        model_reference: SCREEN_SERVICE_ANALYSIS_MODEL_REFERENCE.to_string(),
        generation_state,
        output_text: String::from_utf8(output.stdout)
            .ok()
            .filter(|value| !value.trim().is_empty()),
        prompt_char_count,
        max_output_tokens: constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
        timeout_ms: config.adapter_timeout_ms,
        duration_ms,
        exit_code: output.status.code(),
        stderr_byte_size: output.stderr.len() as u64,
        unavailable_reason: None,
    }
}

fn unavailable_generation(
    config: &ScreenAiAnalysisRuntimeConfig,
    image: &QueuedScreenImage,
    prompt_char_count: u64,
) -> LocalAiChatGenerationResult {
    LocalAiChatGenerationResult {
        local_ai_result_id: local_ai_result_id(&image.queue_job_id),
        runtime_reference_id: SCREEN_SERVICE_ANALYSIS_RUNTIME_REF.to_string(),
        provider_id: SCREEN_SERVICE_ANALYSIS_PROVIDER_ID.to_string(),
        model_id: SCREEN_SERVICE_ANALYSIS_MODEL_ID.to_string(),
        model_reference: SCREEN_SERVICE_ANALYSIS_MODEL_REFERENCE.to_string(),
        generation_state: LocalAiGenerationState::Unavailable,
        output_text: None,
        prompt_char_count,
        max_output_tokens: constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
        timeout_ms: config.adapter_timeout_ms,
        duration_ms: 0,
        exit_code: None,
        stderr_byte_size: 0,
        unavailable_reason: Some(
            constants::local_ai_runtime::UNAVAILABLE_REASON_RUNTIME_BINARY_UNCONFIGURED.to_string(),
        ),
    }
}

fn failed_generation(
    config: &ScreenAiAnalysisRuntimeConfig,
    image: &QueuedScreenImage,
    prompt_char_count: u64,
    duration_ms: u64,
) -> LocalAiChatGenerationResult {
    failed_generation_with_state(
        config,
        image,
        prompt_char_count,
        duration_ms,
        LocalAiGenerationState::Failed,
        Some(constants::local_ai_runtime::UNAVAILABLE_REASON_RUNTIME_PROCESS_FAILED),
    )
}

fn timed_out_generation(
    config: &ScreenAiAnalysisRuntimeConfig,
    image: &QueuedScreenImage,
    prompt_char_count: u64,
) -> LocalAiChatGenerationResult {
    failed_generation_with_state(
        config,
        image,
        prompt_char_count,
        config.adapter_timeout_ms,
        LocalAiGenerationState::TimedOut,
        Some(constants::local_ai_runtime::UNAVAILABLE_REASON_RUNTIME_PROCESS_TIMEOUT),
    )
}

fn failed_generation_with_state(
    config: &ScreenAiAnalysisRuntimeConfig,
    image: &QueuedScreenImage,
    prompt_char_count: u64,
    duration_ms: u64,
    generation_state: LocalAiGenerationState,
    unavailable_reason: Option<&'static str>,
) -> LocalAiChatGenerationResult {
    LocalAiChatGenerationResult {
        local_ai_result_id: local_ai_result_id(&image.queue_job_id),
        runtime_reference_id: SCREEN_SERVICE_ANALYSIS_RUNTIME_REF.to_string(),
        provider_id: SCREEN_SERVICE_ANALYSIS_PROVIDER_ID.to_string(),
        model_id: SCREEN_SERVICE_ANALYSIS_MODEL_ID.to_string(),
        model_reference: SCREEN_SERVICE_ANALYSIS_MODEL_REFERENCE.to_string(),
        generation_state,
        output_text: None,
        prompt_char_count,
        max_output_tokens: constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
        timeout_ms: config.adapter_timeout_ms,
        duration_ms,
        exit_code: None,
        stderr_byte_size: 0,
        unavailable_reason: unavailable_reason.map(str::to_string),
    }
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

fn required_string(value: &Value, field: &str) -> Option<String> {
    let value = value.get(field)?.as_str()?.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn optional_string(value: &Value, field: &str) -> Option<String> {
    value.get(field).and_then(|field_value| {
        let text = field_value.as_str()?.trim();
        if text.is_empty() {
            None
        } else {
            Some(text.to_string())
        }
    })
}

fn required_f64(value: &Value, field: &str) -> Option<f64> {
    value.get(field)?.as_f64()
}

fn required_bool(value: &Value, field: &str) -> Option<bool> {
    value.get(field)?.as_bool()
}

fn local_ai_result_id(queue_job_id: &str) -> String {
    prefixed_id(constants::local_ai_runtime::RESULT_ID_PREFIX, queue_job_id)
}

fn prefixed_id(prefix: &str, value: &str) -> String {
    let mut id = String::from(prefix);
    id.push_str(value);
    id
}
