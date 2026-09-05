use std::path::PathBuf as TestPathBuf;
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiGenerationState;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};

use crate::{
    local_ai_chat_generation::build_local_ai_chat_generation_report,
    local_ai_chat_generation_request_input::LocalAiChatGenerationRequest,
    local_ai_chat_generation_runner::run_local_ai_chat_generation,
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_values::LocalAiRuntimePath, test_require_ok::require_ok,
};

#[tokio::test]
async fn disabled_generation_returns_unavailable_without_spawning_runtime() {
    let binary = write_temp_file(constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI);
    let model = write_temp_file(constants::local_ai_runtime::MODEL_ID_LOCAL_GGUF_CONFIGURED);
    let config = LocalAiRuntimeConfigSnapshot::from_parts(
        Some(LocalAiRuntimePath(binary.clone())),
        Some(LocalAiRuntimePath(model.clone())),
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
        Some(LocalAiRuntimePath(binary.clone())),
        Some(LocalAiRuntimePath(model.clone())),
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

#[tokio::test]
async fn local_ai_chat_generation_report_links_request_and_payload_helpers() {
    let event = build_local_ai_chat_generation_report(generation_command()).await;

    assert_eq!(
        event.event,
        AgentEventName::AgentLocalAiChatGenerationReported
    );
    assert!(event
        .event_id
        .starts_with(constants::event_id::LOCAL_AI_CHAT_GENERATION_REPORTED));
    assert_eq!(
        event
            .payload
            .get(constants::field::LOCAL_AI_UNAVAILABLE_REASON),
        Some(
            &ocentra_parent_agent_protocol::logging::LogFieldValue::String(
                constants::local_ai_runtime::UNAVAILABLE_REASON_COMMAND_PAYLOAD_INVALID.to_string()
            )
        )
    );
}

fn write_temp_file(prefix: impl std::fmt::Display) -> TestPathBuf {
    let path = unique_temp_path(prefix);
    require_ok(
        fs::write(&path, constants::local_ai_runtime::TEST_CHECKED_AT),
        constants::error::LOCAL_AI_RUNTIME_SPAWNS,
    );
    path
}

fn unique_temp_path(prefix: impl std::fmt::Display) -> TestPathBuf {
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

fn remove_temp_file(path: TestPathBuf) {
    let _ = fs::remove_file(path);
}

fn generation_command() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::LOCAL_AI_CHAT_GENERATION_REPORTED.to_string(),
        sent_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: constants::local_ai_runtime::RESOURCE_CPU.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentLocalAiChatGenerate,
        payload: LogFields::new(),
    }
}
