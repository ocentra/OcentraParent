use std::path::Path;

use base64::prelude::{Engine as _, BASE64_STANDARD};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::generation::LocalAiChatGenerationResult;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiGenerationState;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus;
use ocentra_parent_agent_protocol::screen_evidence::ScreenAnalysisResult;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_FIELD_IMAGE_BASE64;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_MODEL_ID;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_MODEL_REFERENCE;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_PROVIDER_ID;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_RUNTIME_REF;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_TEMPLATE_VERSION;
use ocentra_parent_agent_protocol::SCREEN_EVIDENCE_SCHEMA_VERSION;
use serde_json::{Map, Value};

use super::{
    config::ScreenOcrRedactionPolicy, queue::QueuedScreenImage, ScreenAiAnalysisRuntimeConfig,
};

#[path = "adapter_execution.rs"]
mod execution;
#[path = "adapter_parsing.rs"]
mod parsing;
#[path = "adapter_status.rs"]
mod status;

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
    execution::run_adapter(config, image, metadata).await
}

pub(super) fn runtime_status(command: Option<&Path>, timestamp: &str) -> LocalModelRuntimeStatus {
    status::runtime_status(
        status::AdapterRuntimeCommand(command),
        status::AdapterRuntimeTimestamp(timestamp),
    )
}

pub(super) fn parsed_generation_output_with_policy(
    generation: &LocalAiChatGenerationResult,
    policy: &ScreenOcrRedactionPolicy,
) -> Option<ScreenAiAnalysisAdapterOutput> {
    parsing::parsed_generation_output_with_policy(generation, policy)
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

fn local_ai_result_id(queue_job_id: &str) -> String {
    prefixed_id(constants::local_ai_runtime::RESULT_ID_PREFIX, queue_job_id)
}

fn prefixed_id(prefix: &str, value: &str) -> String {
    let mut id = String::from(prefix);
    id.push_str(value);
    id
}
