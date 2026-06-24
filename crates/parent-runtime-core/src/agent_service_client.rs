use std::time::{SystemTime, UNIX_EPOCH};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentEventName, AgentMessageTarget,
    AgentPeer, AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_schema::parent_ui_bridge::{ParentRouteContext, ParentRouteEventSnapshot};
use serde::Serialize;
use serde_json::Value;
use tungstenite::{
    client::IntoClientRequest,
    connect,
    http::{header::ORIGIN, HeaderValue},
    Message, WebSocket,
};

pub(crate) struct LanAgentServiceSnapshot {
    pub(crate) event: ParentRouteEventSnapshot,
    pub(crate) read_model: LanBrowserAddDeviceReadModel,
}

pub(crate) fn load_lan_route_snapshot(
    context: Option<&ParentRouteContext>,
) -> Result<LanAgentServiceSnapshot, String> {
    send_lan_command(
        AgentCommandName::AgentLanPairingBrowserDiscoveryScan,
        LogFields::new(),
        context,
    )
}

pub(crate) fn request_lan_browser_discovery_scan(
    context: Option<&ParentRouteContext>,
) -> Result<LanAgentServiceSnapshot, String> {
    send_lan_command(
        AgentCommandName::AgentLanPairingBrowserDiscoveryScan,
        LogFields::new(),
        context,
    )
}

pub(crate) fn dispatch_lan_agent_command(
    command_name: &str,
    payload: &Value,
    context: Option<&ParentRouteContext>,
) -> Result<LanAgentServiceSnapshot, String> {
    let command =
        serde_json::from_value::<AgentCommandName>(Value::String(command_name.to_string()))
            .map_err(|error| {
                format!(
                "parent Rust facade rejected unsupported LAN agent command {command_name}: {error}"
            )
            })?;
    let payload = log_fields_from_json(payload)?;
    send_lan_command(command, payload, context)
}

fn send_lan_command(
    command: AgentCommandName,
    payload: LogFields,
    context: Option<&ParentRouteContext>,
) -> Result<LanAgentServiceSnapshot, String> {
    let command_origin = resolve_command_origin(&payload);
    let url = agent_ws_url();
    let mut request = url.as_str().into_client_request().map_err(|error| {
        format!("agent-service WebSocket request build failed at {url}: {error}")
    })?;
    request
        .headers_mut()
        .insert(ORIGIN, header_value(&command_origin)?);
    let (mut socket, _) = connect(request)
        .map_err(|error| format!("agent-service WebSocket connect failed at {url}: {error}"))?;
    let ready_event = read_agent_event(&mut socket)?;
    if ready_event.event != AgentEventName::AgentConnectionReady {
        return Err(format!(
            "agent-service expected connection ready event, received {}",
            serialized_enum_label(&ready_event.event)
        ));
    }

    let command_envelope = lan_command_envelope(command, payload, context);
    let body = serde_json::to_string(&command_envelope)
        .map_err(|error| format!("agent-service command serialization failed: {error}"))?;
    socket
        .send(Message::Text(body))
        .map_err(|error| format!("agent-service WebSocket send failed: {error}"))?;

    let event = read_agent_event(&mut socket)?;
    lan_snapshot_from_event(event)
}

fn agent_ws_url() -> String {
    let agent_addr = std::env::var(constants::env_var::AGENT_ADDR)
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| constants::bind::DEFAULT_AGENT_ADDR.to_string());
    format!("ws://{agent_addr}{}", constants::endpoint::DEV_WS)
}

fn lan_snapshot_from_event(event: AgentEventEnvelope) -> Result<LanAgentServiceSnapshot, String> {
    if event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(&event));
    }

    let read_model_json = event
        .payload
        .get(constants::field::LAN_ADD_DEVICE_READ_MODEL)
        .and_then(log_field_string)
        .ok_or_else(|| {
            format!(
                "agent-service {} did not include {}",
                serialized_enum_label(&event.event),
                constants::field::LAN_ADD_DEVICE_READ_MODEL
            )
        })?;
    let read_model = serde_json::from_str::<LanBrowserAddDeviceReadModel>(read_model_json)
        .map_err(|error| format!("agent-service LAN read model parse failed: {error}"))?;

    Ok(LanAgentServiceSnapshot {
        event: ParentRouteEventSnapshot {
            event: Some(serialized_enum_label(&event.event)),
            event_id: Some(event.event_id),
            sent_at: Some(event.sent_at),
            severity: Some(serialized_enum_label(&event.severity)),
            payload: serde_json::to_value(event.payload).ok(),
        },
        read_model,
    })
}

fn read_agent_event(
    socket: &mut WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>,
) -> Result<AgentEventEnvelope, String> {
    loop {
        let message = socket
            .read()
            .map_err(|error| format!("agent-service WebSocket read failed: {error}"))?;
        match message {
            Message::Text(text) => {
                return serde_json::from_str::<AgentEventEnvelope>(&text)
                    .map_err(|error| format!("agent-service event parse failed: {error}"));
            }
            Message::Ping(bytes) => {
                socket
                    .send(Message::Pong(bytes))
                    .map_err(|error| format!("agent-service WebSocket pong failed: {error}"))?;
            }
            Message::Binary(_) | Message::Pong(_) | Message::Frame(_) => {}
            Message::Close(frame) => {
                return Err(format!(
                    "agent-service WebSocket closed before response: {}",
                    frame
                        .and_then(|value| value.reason.to_string().into())
                        .unwrap_or_else(|| "no close reason".to_string())
                ));
            }
        }
    }
}

fn lan_command_envelope(
    command: AgentCommandName,
    payload: LogFields,
    context: Option<&ParentRouteContext>,
) -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: command_message_id(&command),
        sent_at: String::new(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: lan_target_child_device_id(context),
            platform: host_platform().to_string(),
            route: AgentRoute::LocalNetwork,
        },
        command,
        payload,
    }
}

fn log_fields_from_json(value: &Value) -> Result<LogFields, String> {
    let payload = value.as_object().ok_or_else(|| {
        "parent Rust facade expected an object payload for LAN command dispatch".to_string()
    })?;
    let mut fields = LogFields::new();
    for (key, value) in payload {
        fields.insert(
            key.clone(),
            log_field_value_from_json(value).map_err(|error| {
                format!("parent Rust facade rejected LAN command payload field {key}: {error}")
            })?,
        );
    }
    Ok(fields)
}

fn log_field_value_from_json(value: &Value) -> Result<LogFieldValue, &'static str> {
    match value {
        Value::String(value) => Ok(LogFieldValue::String(value.clone())),
        Value::Number(value) => value
            .as_f64()
            .map(LogFieldValue::Number)
            .ok_or("numbers must be finite f64-compatible values"),
        Value::Bool(value) => Ok(LogFieldValue::Boolean(*value)),
        Value::Null => Ok(LogFieldValue::Null(())),
        Value::Array(_) | Value::Object(_) => {
            Err("nested objects and arrays are not supported on the LAN bridge payload")
        }
    }
}

fn resolve_command_origin(payload: &LogFields) -> String {
    payload
        .get(constants::field::ORIGIN)
        .and_then(log_field_string)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .or_else(read_allowed_origin_from_env)
        .unwrap_or_else(|| {
            constants::bind::DEFAULT_ALLOWED_ORIGINS
                .first()
                .copied()
                .unwrap_or(constants::lan_pairing::ALLOWED_ORIGIN)
                .to_string()
        })
}

fn read_allowed_origin_from_env() -> Option<String> {
    std::env::var(constants::env_var::AGENT_ALLOWED_ORIGINS)
        .ok()
        .and_then(|value| {
            value
                .split(constants::delimiter::LIST)
                .map(str::trim)
                .find(|origin| !origin.is_empty())
                .map(ToOwned::to_owned)
        })
}

fn header_value(value: &str) -> Result<HeaderValue, String> {
    HeaderValue::from_str(value)
        .map_err(|error| format!("agent-service origin header is invalid for {value}: {error}"))
}

fn lan_target_child_device_id(context: Option<&ParentRouteContext>) -> String {
    context
        .and_then(|value| value.selected_child_device_id.as_ref())
        .filter(|value| !value.is_empty())
        .cloned()
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

fn command_message_id(command: &AgentCommandName) -> String {
    let command_name = serialized_enum_label(command);
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|value| value.as_millis())
        .unwrap_or_default();
    format!("parent-ui-bridge-{command_name}-{millis}")
}

fn rejection_message(event: &AgentEventEnvelope) -> String {
    let reason = event
        .payload
        .get(constants::field::REASON)
        .and_then(log_field_string)
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| "unknown reason".to_string());
    format!(
        "agent-service rejected {}: {reason}",
        serialized_enum_label(&event.event)
    )
}

fn log_field_string(value: &LogFieldValue) -> Option<&str> {
    match value {
        LogFieldValue::String(value) => Some(value.as_str()),
        _ => None,
    }
}

fn serialized_enum_label<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|json| json.as_str().map(ToOwned::to_owned))
        .unwrap_or_default()
}
