use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute, LogFieldValue, LogFields, AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{
    local_ai_chat_generation_request::parse_generation_request,
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

#[test]
fn parse_generation_request_uses_default_gemma_model_when_payload_omits_model_id() {
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::LOCAL_AI_PROMPT.to_string(),
        LogFieldValue::String(constants::local_ai_runtime::TEST_PROMPT.to_string()),
    );
    let command = command_with_payload(payload);
    let config = LocalAiRuntimeConfigSnapshot::unconfigured();

    let request = parse_generation_request(&command, &config)
        .expect(constants::error::LOCAL_AI_CHAT_REQUEST_PARSES);

    assert_eq!(
        request.model_id,
        constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4
    );
}

#[test]
fn parse_generation_request_rejects_invalid_model_id_field() {
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::LOCAL_AI_PROMPT.to_string(),
        LogFieldValue::String(constants::local_ai_runtime::TEST_PROMPT.to_string()),
    );
    payload.insert(
        constants::field::LOCAL_AI_MODEL_ID.to_string(),
        LogFieldValue::String(String::new()),
    );
    let command = command_with_payload(payload);
    let config = LocalAiRuntimeConfigSnapshot::unconfigured();

    let error = parse_generation_request(&command, &config)
        .expect_err(constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_ID_INVALID);

    assert_eq!(
        error,
        constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_ID_INVALID
    );
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
