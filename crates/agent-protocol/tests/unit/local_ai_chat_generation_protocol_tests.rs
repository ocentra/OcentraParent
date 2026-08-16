use super::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute, LocalAiChatGenerationResult, LocalAiGenerationState, LogFieldValue,
    LogFields, AGENT_PROTOCOL_SCHEMA_VERSION,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn local_ai_chat_generation_command_serializes_to_typescript_contract_shape() {
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::LOCAL_AI_PROMPT.to_string(),
        LogFieldValue::String(constants::local_ai_runtime::TEST_PROMPT.to_string()),
    );
    payload.insert(
        constants::field::LOCAL_AI_MODEL_ID.to_string(),
        LogFieldValue::String(constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4.to_string()),
    );

    let command = AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: "cmd-local-ai-chat".to_string(),
        sent_at: "2026-05-21T09:18:00Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: "windows".to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentLocalAiChatGenerate,
        payload,
    };

    let serialized = serde_json::to_value(command).expect_value("command serializes");

    assert_eq!(serialized["command"], "agent.local-ai.chat.generate");
    assert_eq!(
        serialized["payload"][constants::field::LOCAL_AI_PROMPT],
        constants::local_ai_runtime::TEST_PROMPT
    );
    assert_eq!(
        serialized["payload"][constants::field::LOCAL_AI_MODEL_ID],
        constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4
    );
}

#[test]
fn local_ai_chat_generation_result_serializes_without_model_paths() {
    let result = LocalAiChatGenerationResult {
        local_ai_result_id: "local-ai-result-cmd-local-ai-chat".to_string(),
        runtime_reference_id: constants::local_ai_runtime::RUNTIME_REFERENCE_LOCAL_LLAMA_CLI
            .to_string(),
        provider_id: constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI.to_string(),
        model_id: constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4.to_string(),
        model_reference: constants::local_ai_runtime::MODEL_REFERENCE_DEFAULT_GEMMA_4.to_string(),
        generation_state: LocalAiGenerationState::Complete,
        output_text: Some("local-ok".to_string()),
        prompt_char_count: constants::local_ai_runtime::TEST_PROMPT.chars().count() as u64,
        max_output_tokens: constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
        timeout_ms: constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
        duration_ms: 1,
        exit_code: Some(0),
        stderr_byte_size: 0,
        unavailable_reason: None,
    };

    let serialized = serde_json::to_value(result).expect_value("generation result serializes");

    assert_eq!(
        serialized["runtimeReferenceId"],
        constants::local_ai_runtime::RUNTIME_REFERENCE_LOCAL_LLAMA_CLI
    );
    assert_eq!(
        serialized["modelId"],
        constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4
    );
    assert_eq!(
        serialized["modelReference"],
        constants::local_ai_runtime::MODEL_REFERENCE_DEFAULT_GEMMA_4
    );
    assert_eq!(
        serialized["generationState"],
        constants::local_ai_runtime::GENERATION_STATE_COMPLETE
    );
    assert_eq!(serialized["outputText"], "local-ok");
    assert_eq!(serialized["unavailableReason"], serde_json::Value::Null);
}
