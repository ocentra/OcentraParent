use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute, LocalAiGenerationState, LogFieldValue, LogFields,
    AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{
    local_ai_chat_generation_request::{parse_generation_request, LocalAiChatGenerationRequest},
    local_ai_chat_generation_runner::run_local_ai_chat_generation,
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
};

#[test]
fn parse_generation_request_rejects_missing_prompt() {
    let command = command_with_payload(LogFields::new());
    let config = LocalAiRuntimeConfigSnapshot::unconfigured();

    let error = parse_generation_request(&command, &config)
        .expect_err(constants::local_ai_runtime::UNAVAILABLE_REASON_COMMAND_PAYLOAD_INVALID);

    assert_eq!(
        error,
        constants::local_ai_runtime::UNAVAILABLE_REASON_COMMAND_PAYLOAD_INVALID
    );
}

#[test]
fn parse_generation_request_rejects_oversized_prompt() {
    let mut payload = LogFields::new();
    let prompt = constants::local_ai_runtime::TEST_PROMPT
        .repeat(constants::local_ai_runtime::MAX_PROMPT_CHARS);
    payload.insert(
        constants::field::LOCAL_AI_PROMPT.to_string(),
        LogFieldValue::String(prompt),
    );
    let command = command_with_payload(payload);
    let config = LocalAiRuntimeConfigSnapshot::unconfigured();

    let error = parse_generation_request(&command, &config)
        .expect_err(constants::local_ai_runtime::UNAVAILABLE_REASON_PROMPT_TOO_LARGE);

    assert_eq!(
        error,
        constants::local_ai_runtime::UNAVAILABLE_REASON_PROMPT_TOO_LARGE
    );
}

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

fn command_with_payload(payload: LogFields) -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
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
        payload,
    }
}

fn write_temp_file(prefix: &str) -> PathBuf {
    let path = unique_temp_path(prefix);
    fs::write(&path, constants::local_ai_runtime::TEST_CHECKED_AT)
        .expect(constants::error::LOCAL_AI_RUNTIME_SPAWNS);
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
        .expect(constants::error::AGENT_EVENT_SERIALIZES)
        .as_nanos()
}

fn remove_temp_file(path: PathBuf) {
    let _ = fs::remove_file(path);
}
