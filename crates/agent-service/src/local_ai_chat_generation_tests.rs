use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiGenerationState;

use crate::{
    local_ai_chat_generation_request::LocalAiChatGenerationRequest,
    local_ai_chat_generation_runner::run_local_ai_chat_generation,
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
};

#[tokio::test]
async fn disabled_generation_returns_unavailable_without_spawning_runtime() {
    let binary = write_temp_file(constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI);
    let model = write_temp_file(constants::local_ai_runtime::MODEL_ID_LOCAL_GGUF_CONFIGURED);
    let config = LocalAiRuntimeConfigSnapshot::from_parts(
        Some(binary.clone()),
        Some(model.clone()),
        None,
        None,
    );
    let request = LocalAiChatGenerationRequest {
        model_id: constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4.to_string(),
        prompt: constants::local_ai_runtime::TEST_PROMPT.to_string(),
        max_output_tokens: constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
        timeout_ms: constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
    };

    let result = run_local_ai_chat_generation(
        constants::event_id::LOCAL_AI_CHAT_GENERATION_REPORTED,
        request,
        &config,
    )
    .await;

    assert_eq!(result.generation_state, LocalAiGenerationState::Unavailable);
    assert_eq!(result.output_text, None);
    assert_eq!(
        result.unavailable_reason,
        Some(constants::local_ai_runtime::UNAVAILABLE_REASON_EXECUTION_DISABLED.to_string())
    );

    remove_temp_file(binary);
    remove_temp_file(model);
}

#[tokio::test]
async fn unsupported_requested_model_returns_unavailable_without_spawning_runtime() {
    let binary = write_temp_file(constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI);
    let model = write_temp_file(constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4);
    let config = LocalAiRuntimeConfigSnapshot::from_parts_with_execution(
        Some(binary.clone()),
        Some(model.clone()),
        None,
        None,
        true,
        constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
        constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
    );
    let request = LocalAiChatGenerationRequest {
        model_id: constants::local_ai_runtime::TEST_UNSUPPORTED_MODEL_ID.to_string(),
        prompt: constants::local_ai_runtime::TEST_PROMPT.to_string(),
        max_output_tokens: constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
        timeout_ms: constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
    };

    let result = run_local_ai_chat_generation(
        constants::event_id::LOCAL_AI_CHAT_GENERATION_REPORTED,
        request,
        &config,
    )
    .await;

    assert_eq!(result.generation_state, LocalAiGenerationState::Unavailable);
    assert_eq!(
        result.model_id,
        constants::local_ai_runtime::TEST_UNSUPPORTED_MODEL_ID
    );
    assert_eq!(
        result.model_reference,
        constants::local_ai_runtime::MODEL_REFERENCE_UNCONFIGURED
    );
    assert_eq!(
        result.unavailable_reason,
        Some(constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_UNSUPPORTED.to_string())
    );

    remove_temp_file(binary);
    remove_temp_file(model);
}

fn write_temp_file(prefix: &str) -> PathBuf {
    let path = unique_temp_path(prefix);
    fs::write(&path, constants::local_ai_runtime::TEST_CHECKED_AT)
        .unwrap_or_else(|_| panic!("{}", constants::error::LOCAL_AI_RUNTIME_SPAWNS));
    path
}

fn unique_temp_path(prefix: &str) -> PathBuf {
    let mut name = prefix.to_string();
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&nanos_now().to_string());
    let mut path = std::env::temp_dir();
    path.push(name);
    path
}

fn nanos_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}

fn remove_temp_file(path: PathBuf) {
    let _ = fs::remove_file(path);
}
