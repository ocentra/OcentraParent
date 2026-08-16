use std::{process::Stdio, time::Duration};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::generation::LocalAiChatGenerationResult;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiGenerationState;
use tokio::{process::Command, time::Instant};

use crate::{
    local_ai_chat_generation_args::llama_acceleration_args,
    local_ai_chat_generation_request::LocalAiChatGenerationRequest,
    local_ai_chat_generation_result::{failed_result, unavailable_result, LocalAiFailedGeneration},
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_values::{
        LocalAiRuntimePath, LocalAiRuntimeText, LocalAiUnavailableReason,
    },
    local_ai_runtime_model_selection::{
        model_reference_for_request, requested_model_unavailable_reason,
    },
    local_ai_runtime_status::local_ai_runtime_is_executable,
};

#[derive(Clone, Debug)]
pub(crate) struct LocalAiGenerationMessageId(String);

impl<T> From<T> for LocalAiGenerationMessageId
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

pub(crate) async fn run_local_ai_chat_generation(
    message_id: impl Into<LocalAiGenerationMessageId>,
    request: LocalAiChatGenerationRequest,
    config: &LocalAiRuntimeConfigSnapshot,
) -> LocalAiChatGenerationResult {
    let message_id = message_id.into();
    let requested_model_id = LocalAiRuntimeText(request.model_id.clone());
    if let Some(reason) = requested_model_unavailable_reason(config, &requested_model_id) {
        return unavailable_result(&message_id.0, config, &request, reason);
    }

    if !local_ai_runtime_is_executable(config) {
        return unavailable_result(
            &message_id.0,
            config,
            &request,
            LocalAiUnavailableReason(
                constants::local_ai_runtime::UNAVAILABLE_REASON_EXECUTION_DISABLED,
            ),
        );
    }

    let Some(runtime_binary) = config
        .runtime_binary()
        .path()
        .map(|path| LocalAiRuntimePath(path.to_path_buf()))
    else {
        return unavailable_result(
            &message_id.0,
            config,
            &request,
            LocalAiUnavailableReason(
                constants::local_ai_runtime::UNAVAILABLE_REASON_RUNTIME_BINARY_MISSING,
            ),
        );
    };
    let Some(model_file) = config
        .model_file()
        .path()
        .map(|path| LocalAiRuntimePath(path.to_path_buf()))
    else {
        return unavailable_result(
            &message_id.0,
            config,
            &request,
            LocalAiUnavailableReason(
                constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_FILE_MISSING,
            ),
        );
    };

    execute_llama_cli(&message_id, request, config, &runtime_binary, &model_file).await
}

pub(crate) fn unavailable_result_for_command(
    message_id: impl Into<LocalAiGenerationMessageId>,
    config: &LocalAiRuntimeConfigSnapshot,
    reason: LocalAiUnavailableReason,
) -> LocalAiChatGenerationResult {
    let message_id = message_id.into();
    let request = LocalAiChatGenerationRequest {
        model_id: config.model_id().0,
        prompt: String::new(),
        max_output_tokens: config.generation_max_tokens(),
        timeout_ms: config.generation_timeout_ms(),
    };
    unavailable_result(&message_id.0, config, &request, reason)
}

async fn execute_llama_cli(
    message_id: &LocalAiGenerationMessageId,
    request: LocalAiChatGenerationRequest,
    config: &LocalAiRuntimeConfigSnapshot,
    runtime_binary: &LocalAiRuntimePath,
    model_file: &LocalAiRuntimePath,
) -> LocalAiChatGenerationResult {
    let started_at = Instant::now();
    let mut command = Command::new(&runtime_binary.0);
    command
        .arg(constants::local_ai_runtime::LLAMA_ARG_MODEL)
        .arg(&model_file.0)
        .arg(constants::local_ai_runtime::LLAMA_ARG_PROMPT)
        .arg(&request.prompt)
        .arg(constants::local_ai_runtime::LLAMA_ARG_MAX_TOKENS)
        .arg(request.max_output_tokens.to_string())
        .arg(constants::local_ai_runtime::LLAMA_ARG_TEMPERATURE)
        .arg(constants::local_ai_runtime::LLAMA_TEMPERATURE_DETERMINISTIC)
        .arg(constants::local_ai_runtime::LLAMA_ARG_NO_DISPLAY_PROMPT)
        .arg(constants::local_ai_runtime::LLAMA_ARG_SINGLE_TURN);
    append_acceleration_args(&mut command, config);
    command.stdin(Stdio::null()).kill_on_drop(true);

    let output =
        match tokio::time::timeout(Duration::from_millis(request.timeout_ms), command.output())
            .await
        {
            Ok(Ok(output)) => output,
            Ok(Err(_)) => {
                return failed_result(
                    &message_id.0,
                    config,
                    &request,
                    &LocalAiFailedGeneration {
                        duration_ms: elapsed_ms(started_at),
                        exit_code: None,
                        stderr_byte_size: 0,
                        generation_state: LocalAiGenerationState::Failed,
                        reason:
                            constants::local_ai_runtime::UNAVAILABLE_REASON_RUNTIME_PROCESS_FAILED,
                    },
                );
            }
            Err(_) => {
                return failed_result(
                    &message_id.0,
                    config,
                    &request,
                    &LocalAiFailedGeneration {
                        duration_ms: elapsed_ms(started_at),
                        exit_code: None,
                        stderr_byte_size: 0,
                        generation_state: LocalAiGenerationState::TimedOut,
                        reason:
                            constants::local_ai_runtime::UNAVAILABLE_REASON_RUNTIME_PROCESS_TIMEOUT,
                    },
                );
            }
        };

    complete_or_failed_result(message_id, &request, config, started_at, &output)
}

fn complete_or_failed_result(
    message_id: &LocalAiGenerationMessageId,
    request: &LocalAiChatGenerationRequest,
    config: &LocalAiRuntimeConfigSnapshot,
    started_at: Instant,
    output: &std::process::Output,
) -> LocalAiChatGenerationResult {
    let exit_code = output.status.code();
    let stderr_byte_size = output.stderr.len() as u64;
    if !output.status.success() {
        return failed_result(
            &message_id.0,
            config,
            request,
            &LocalAiFailedGeneration {
                duration_ms: elapsed_ms(started_at),
                exit_code,
                stderr_byte_size,
                generation_state: LocalAiGenerationState::Failed,
                reason: constants::local_ai_runtime::UNAVAILABLE_REASON_RUNTIME_PROCESS_FAILED,
            },
        );
    }

    let output_text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if output_text.is_empty() {
        return failed_result(
            &message_id.0,
            config,
            request,
            &LocalAiFailedGeneration {
                duration_ms: elapsed_ms(started_at),
                exit_code,
                stderr_byte_size,
                generation_state: LocalAiGenerationState::Failed,
                reason:
                    constants::local_ai_runtime::UNAVAILABLE_REASON_RUNTIME_PROCESS_EMPTY_OUTPUT,
            },
        );
    }

    LocalAiChatGenerationResult {
        local_ai_result_id: crate::local_ai_chat_generation_result::result_id(&message_id.0).0,
        runtime_reference_id: constants::local_ai_runtime::RUNTIME_REFERENCE_LOCAL_LLAMA_CLI
            .to_string(),
        provider_id: constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI.to_string(),
        model_id: request.model_id.clone(),
        model_reference: model_reference_for_request(
            config,
            &LocalAiRuntimeText(request.model_id.clone()),
        )
        .0,
        generation_state: LocalAiGenerationState::Complete,
        output_text: Some(output_text),
        prompt_char_count: request.prompt.chars().count() as u64,
        max_output_tokens: request.max_output_tokens,
        timeout_ms: request.timeout_ms,
        duration_ms: elapsed_ms(started_at),
        exit_code,
        stderr_byte_size,
        unavailable_reason: None,
    }
}

fn elapsed_ms(started_at: Instant) -> u64 {
    started_at.elapsed().as_millis() as u64
}

fn append_acceleration_args(command: &mut Command, config: &LocalAiRuntimeConfigSnapshot) {
    for arg in llama_acceleration_args(config) {
        command.arg(arg.0);
    }
}
