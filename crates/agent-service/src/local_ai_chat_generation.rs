use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiGenerationState;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerJobClass;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{
    event_builder::build_event,
    local_ai_chat_generation_request::parse_generation_request,
    local_ai_chat_generation_request_input::LocalAiChatGenerationRequest,
    local_ai_chat_generation_result::unavailable_result,
    local_ai_chat_generation_runner::run_local_ai_chat_generation,
    local_ai_generation_payload::local_ai_chat_generation_payload,
    local_ai_provider_scheduler::local_ai_provider_scheduler,
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_values::{LocalAiRuntimeText, LocalAiUnavailableReason},
    local_ai_runtime_status::local_ai_runtime_status_for_model_from_config,
    time::timestamp_now,
};

pub async fn build_local_ai_chat_generation_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let config = tokio::task::spawn_blocking(LocalAiRuntimeConfigSnapshot::from_environment)
        .await
        .unwrap_or_else(|_| LocalAiRuntimeConfigSnapshot::unconfigured());
    let result = match parse_generation_request(&command, &config) {
        Ok(request) => {
            let (runtime, _, _) = local_ai_runtime_status_for_model_from_config(
                timestamp_now::<String>(),
                &config,
                Some(crate::local_ai_runtime_config_values::LocalAiRuntimeText(
                    request.model_id.clone(),
                )),
            );
            local_ai_provider_scheduler()
                .run_generation_job(
                    LocalAiProviderSchedulerJobClass::ParentAssistant,
                    runtime,
                    || run_local_ai_chat_generation(command.message_id.as_str(), request, &config),
                )
                .await
        }
        Err(reason) => unavailable_result_for_command(
            LocalAiRuntimeText(command.message_id.clone()),
            &config,
            LocalAiUnavailableReason(reason.0),
        ),
    };
    let severity = if result.generation_state == LocalAiGenerationState::Complete {
        LogLevel::Info
    } else {
        LogLevel::Warn
    };

    build_event(
        constants::event_id::LOCAL_AI_CHAT_GENERATION_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentLocalAiChatGenerationReported,
        severity,
        local_ai_chat_generation_payload(&result),
        None,
    )
}

fn unavailable_result_for_command(
    message_id: LocalAiRuntimeText,
    config: &LocalAiRuntimeConfigSnapshot,
    reason: LocalAiUnavailableReason,
) -> ocentra_parent_agent_protocol::local_ai_runtime::generation::LocalAiChatGenerationResult {
    let request = LocalAiChatGenerationRequest {
        model_id: config.model_id().0,
        prompt: String::new(),
        max_output_tokens: config.generation_max_tokens(),
        timeout_ms: config.generation_timeout_ms(),
    };
    unavailable_result(message_id, config, &request, reason)
}
