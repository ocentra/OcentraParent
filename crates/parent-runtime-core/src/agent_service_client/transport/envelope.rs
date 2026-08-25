use chrono::{SecondsFormat, Utc};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentMessageTarget, AgentPeer, AgentPeerRole,
    AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_schema::parent_ui_bridge::{ParentChildDeviceId, ParentRouteContext};
use tungstenite::http::HeaderValue;

use super::super::payload_fields::serialized_enum_label;

pub(super) fn lan_command_envelope(
    command: AgentCommandName,
    mut payload: LogFields,
    context: Option<&ParentRouteContext>,
    route: AgentRoute,
) -> Result<(AgentCommandEnvelope, String), String> {
    let request_nonce = request_nonce()?;
    payload.insert(
        constants::field::REQUEST_NONCE.to_string(),
        ocentra_parent_agent_protocol::logging::LogFieldValue::String(request_nonce.clone()),
    );
    let sent_at = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
    let envelope = AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: command_message_id(&command, &request_nonce),
        sent_at,
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: lan_target_child_device_id(context),
            platform: host_platform().to_string(),
            route,
        },
        command,
        payload,
    };
    Ok((envelope, request_nonce))
}

pub(super) fn header_value(value: &str) -> Result<HeaderValue, String> {
    HeaderValue::from_str(value)
        .map_err(|error| format!("agent-service origin header is invalid for {value}: {error}"))
}

fn lan_target_child_device_id(context: Option<&ParentRouteContext>) -> String {
    context
        .and_then(|value| value.selected_child_device_id.as_ref())
        .map(ParentChildDeviceId::as_str)
        .map(str::to_string)
        .or_else(|| {
            std::env::var(constants::lan_pairing::LOCAL_CHILD_DEVICE_ID_ENV)
                .ok()
                .filter(|value| !value.is_empty())
        })
        .unwrap_or_else(|| constants::lan_pairing::CHILD_DEVICE_ID.to_string())
}

fn host_platform() -> &'static str {
    match std::env::consts::OS {
        "windows" => constants::local_ai_runtime::PLATFORM_OS_WINDOWS,
        "linux" => constants::local_ai_runtime::PLATFORM_OS_LINUX,
        "macos" => constants::local_ai_runtime::PLATFORM_OS_MACOS,
        "android" => constants::enforcement::PLATFORM_ANDROID,
        "ios" => constants::enforcement::PLATFORM_IOS,
        _ => constants::lan_pairing::PLATFORM_UNKNOWN,
    }
}

fn request_nonce() -> Result<String, String> {
    let mut bytes = [0_u8; 32];
    getrandom::fill(&mut bytes)
        .map_err(|error| format!("parent Rust facade request nonce generation failed: {error}"))?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}

fn command_message_id(command: &AgentCommandName, request_nonce: &str) -> String {
    let command_name = serialized_enum_label(command);
    format!(
        "parent-ui-bridge-{command_name}-{}",
        super::request_nonce_digest(request_nonce)
    )
}
