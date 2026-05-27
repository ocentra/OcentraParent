use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LocalAiGenerationState,
    LocalAiProviderSchedulerJobClass, LogLevel,
};

use crate::{
    event_builder::build_event, local_ai_chat_generation_request::parse_generation_request,
    local_ai_chat_generation_runner::run_local_ai_chat_generation,
    local_ai_generation_payload::local_ai_chat_generation_payload,
    local_ai_provider_scheduler::local_ai_provider_scheduler,
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_status::local_ai_runtime_status_for_model_from_config, time::timestamp_now,
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
                timestamp_now(),
                &config,
                Some(&request.model_id),
            );
            local_ai_provider_scheduler()
                .run_generation_job(
                    LocalAiProviderSchedulerJobClass::ParentAssistant,
                    runtime,
                    || run_local_ai_chat_generation(&command.message_id, request, &config),
                )
                .await
        }
        Err(reason) => crate::local_ai_chat_generation_runner::unavailable_result_for_command(
            &command.message_id,
            &config,
            reason,
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
