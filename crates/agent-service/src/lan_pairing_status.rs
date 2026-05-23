use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LanSelectedRouteTarget,
    LogFieldValue, LogLevel,
};

use crate::{
    event_builder::build_event, fields::fields_from_pairs, lan_pairing::LanPairingRuntime,
};

#[derive(Clone, Debug)]
struct LanPairingStatus {
    pairing_state: &'static str,
    authentication_state: &'static str,
    trusted_device_count: usize,
    selected_target: Option<LanSelectedRouteTarget>,
    trusted_device_ids: Vec<String>,
}

pub(crate) fn pairing_status_event(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let status = pairing_status(runtime);
    let mut pairs = support_surface_pairs();
    pairs.extend(state_pairs(&status));
    build_event(
        constants::lan_pairing::EVENT_STATUS_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentLanPairingStatusReported,
        LogLevel::Info,
        fields_from_pairs(pairs),
        None,
    )
}

fn pairing_status(runtime: &LanPairingRuntime) -> LanPairingStatus {
    let trusted_device_count = runtime.trusted_device_count();
    let selected_target = runtime.selected_target();
    LanPairingStatus {
        pairing_state: pairing_state(trusted_device_count),
        authentication_state: authentication_state(&selected_target),
        trusted_device_count,
        selected_target,
        trusted_device_ids: runtime.trusted_device_ids(),
    }
}

fn support_surface_pairs() -> Vec<(&'static str, LogFieldValue)> {
    vec![
        (
            constants::field::TRANSPORT,
            LogFieldValue::String(constants::value::TRANSPORT_WEBSOCKET.to_string()),
        ),
        (
            constants::field::LAN_SUPPORTED_WEBSOCKET_COMMANDS,
            LogFieldValue::String(
                constants::lan_pairing::SUPPORTED_WEBSOCKET_COMMANDS
                    .join(&constants::delimiter::LIST.to_string()),
            ),
        ),
        (
            constants::field::LAN_UNSUPPORTED_HTTP_ENDPOINTS,
            LogFieldValue::String(
                constants::lan_pairing::PLANNED_HTTP_ENDPOINT_PATHS
                    .join(&constants::delimiter::LIST.to_string()),
            ),
        ),
        (
            constants::field::LAN_PERSISTENCE_MODE,
            LogFieldValue::String(
                constants::value::LAN_PERSISTENCE_IN_MEMORY_FAIL_CLOSED.to_string(),
            ),
        ),
        (
            constants::field::LAN_PROOF_MODE,
            LogFieldValue::String(constants::value::LAN_PROOF_DIRECT_PROOF_SUBMIT.to_string()),
        ),
        (
            constants::field::LAN_ROUTE_REQUIREMENTS,
            LogFieldValue::String(
                constants::lan_pairing::ROUTE_REQUIREMENTS
                    .join(&constants::delimiter::LIST.to_string()),
            ),
        ),
        (
            constants::field::LAN_MANUAL_PROOF_GAPS,
            LogFieldValue::String(
                constants::lan_pairing::MANUAL_PROOF_GAPS
                    .join(&constants::delimiter::LIST.to_string()),
            ),
        ),
    ]
}

fn state_pairs(status: &LanPairingStatus) -> Vec<(&'static str, LogFieldValue)> {
    vec![
        (
            constants::field::LAN_PAIRING_STATE,
            LogFieldValue::String(status.pairing_state.to_string()),
        ),
        (
            constants::field::LAN_AUTHENTICATION_STATE,
            LogFieldValue::String(status.authentication_state.to_string()),
        ),
        (
            constants::field::LAN_TRUSTED_DEVICE_COUNT,
            LogFieldValue::Number(status.trusted_device_count as f64),
        ),
        (
            constants::field::LAN_TRUSTED_DEVICE_IDS,
            LogFieldValue::String(
                status
                    .trusted_device_ids
                    .join(&constants::delimiter::LIST.to_string()),
            ),
        ),
        (
            constants::field::LAN_SELECTED_CHILD_DEVICE_ID,
            LogFieldValue::String(selected_child_device_id(status)),
        ),
        (
            constants::field::LAN_SELECTED_ROUTE_ID,
            LogFieldValue::String(selected_route_id(status)),
        ),
    ]
}

fn pairing_state(count: usize) -> &'static str {
    if count > 0 {
        constants::value::LAN_PAIRING_PAIRED
    } else {
        constants::value::LAN_PAIRING_UNPAIRED
    }
}

fn authentication_state(selected: &Option<LanSelectedRouteTarget>) -> &'static str {
    if selected.is_some() {
        constants::value::LAN_AUTH_PAIRED
    } else {
        constants::value::LAN_AUTH_UNPAIRED
    }
}

fn selected_child_device_id(status: &LanPairingStatus) -> String {
    status
        .selected_target
        .as_ref()
        .map(|target| target.selected_child_device_id.clone())
        .unwrap_or_default()
}

fn selected_route_id(status: &LanPairingStatus) -> String {
    status
        .selected_target
        .as_ref()
        .map(|target| target.route_id.clone())
        .unwrap_or_default()
}
