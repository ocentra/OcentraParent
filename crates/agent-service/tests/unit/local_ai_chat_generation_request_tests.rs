use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentCommandName;
use ocentra_parent_agent_protocol::transport::AgentMessageTarget;
use ocentra_parent_agent_protocol::transport::AgentPeer;
use ocentra_parent_agent_protocol::transport::AgentPeerRole;
use ocentra_parent_agent_protocol::transport::AgentRoute;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use std::primitive::str as TestStr;
use std::string::String as TestString;

use crate::{
    local_ai_chat_generation_request::parse_generation_request,
    local_ai_runtime_config::LocalAiRuntimeConfigSnapshot,
    local_ai_runtime_config_values::LocalAiUnavailableReason,
};

type TestResult = Result<(), TestString>;

#[test]
fn parse_generation_request_rejects_missing_prompt() {
    let command = command_with_payload(LogFields::new());
    let config = LocalAiRuntimeConfigSnapshot::unconfigured();

    assert_request_error(
        &parse_generation_request(&command, &config),
        constants::local_ai_runtime::UNAVAILABLE_REASON_COMMAND_PAYLOAD_INVALID,
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

    assert_request_error(
        &parse_generation_request(&command, &config),
        constants::local_ai_runtime::UNAVAILABLE_REASON_PROMPT_TOO_LARGE,
    );
}

#[test]
fn parse_generation_request_uses_default_gemma_model_when_payload_omits_model_id() -> TestResult {
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::LOCAL_AI_PROMPT.to_string(),
        LogFieldValue::String(constants::local_ai_runtime::TEST_PROMPT.to_string()),
    );
    let command = command_with_payload(payload);
    let config = LocalAiRuntimeConfigSnapshot::unconfigured();

    let request = ok(
        parse_generation_request(&command, &config),
        constants::error::LOCAL_AI_CHAT_REQUEST_PARSES,
    )?;

    assert_eq!(
        request.model_id,
        constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4
    );

    Ok(())
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
        LogFieldValue::String(TestString::new()),
    );
    let command = command_with_payload(payload);
    let config = LocalAiRuntimeConfigSnapshot::unconfigured();

    assert_request_error(
        &parse_generation_request(&command, &config),
        constants::local_ai_runtime::UNAVAILABLE_REASON_MODEL_ID_INVALID,
    );
}

fn assert_request_error<T>(result: &Result<T, LocalAiUnavailableReason>, expected: &TestStr) {
    assert!(
        result.is_err(),
        "{}",
        constants::error::LOCAL_AI_CHAT_REQUEST_PARSES
    );
    if let Err(error) = result {
        assert_eq!(error.0, expected);
    }
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

fn ok<T, E: std::fmt::Debug>(result: Result<T, E>, context: &TestStr) -> Result<T, TestString> {
    result.map_err(|error| format!("{context}: {error:?}"))
}
