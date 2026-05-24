use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
    LanPairingDeviceReachability, LanSelectedRouteTarget, LogFieldValue, LogLevel,
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
    revoked_device_ids: Vec<String>,
    has_revoked_pairing: bool,
}

pub(crate) fn pairing_status_event(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let status = pairing_status(runtime);
    let mut pairs = support_surface_pairs(runtime);
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
    let mut status = LanPairingStatus {
        pairing_state: constants::value::LAN_PAIRING_UNPAIRED,
        authentication_state: authentication_state(&selected_target),
        trusted_device_count,
        selected_target,
        trusted_device_ids: runtime.trusted_device_ids(),
        revoked_device_ids: runtime.revoked_device_ids(),
        has_revoked_pairing: runtime.has_revoked_pairing(),
    };
    status.pairing_state = pairing_state(&status);
    status
}

fn support_surface_pairs(runtime: &LanPairingRuntime) -> Vec<(&'static str, LogFieldValue)> {
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
            constants::field::LAN_DISCOVERY_STATUS,
            LogFieldValue::String(constants::lan_pairing::SUPPORT_PLANNED_UNSUPPORTED.to_string()),
        ),
        (
            constants::field::LAN_CHALLENGE_STATUS,
            LogFieldValue::String(constants::lan_pairing::SUPPORT_PLANNED_UNSUPPORTED.to_string()),
        ),
        (
            constants::field::LAN_PROOF_PREVIEW_STATUS,
            LogFieldValue::String(constants::lan_pairing::SUPPORT_PLANNED_UNSUPPORTED.to_string()),
        ),
        (
            constants::field::LAN_PERSISTENCE_MODE,
            LogFieldValue::String(runtime.persistence_mode().to_string()),
        ),
        (
            constants::field::LAN_RESTART_BEHAVIOR,
            LogFieldValue::String(runtime.restart_behavior().to_string()),
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
            constants::field::LAN_REVOKED_DEVICE_IDS,
            LogFieldValue::String(
                status
                    .revoked_device_ids
                    .join(&constants::delimiter::LIST.to_string()),
            ),
        ),
        (
            constants::field::LAN_SELECTED_CHILD_DEVICE_ID,
            LogFieldValue::String(selected_child_device_id(status)),
        ),
        (
            constants::field::LAN_SELECTED_DEVICE_REACHABILITY,
            LogFieldValue::String(selected_device_reachability(status).to_string()),
        ),
        (
            constants::field::LAN_SELECTED_DEVICE_STALE_AT,
            LogFieldValue::String(selected_device_stale_at(status)),
        ),
        (
            constants::field::LAN_SELECTED_ROUTE_ID,
            LogFieldValue::String(selected_route_id(status)),
        ),
    ]
}

fn pairing_state(status: &LanPairingStatus) -> &'static str {
    if status.trusted_device_count > 0 {
        constants::value::LAN_PAIRING_PAIRED
    } else if status.has_revoked_pairing {
        constants::value::LAN_PAIRING_REVOKED
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

fn selected_device_reachability(status: &LanPairingStatus) -> &'static str {
    match status
        .selected_target
        .as_ref()
        .map(|target| &target.reachability)
    {
        Some(LanPairingDeviceReachability::Online) => constants::value::LAN_REACHABILITY_ONLINE,
        Some(LanPairingDeviceReachability::Offline) => constants::value::LAN_REACHABILITY_OFFLINE,
        Some(LanPairingDeviceReachability::Stale) => constants::value::LAN_REACHABILITY_STALE,
        None => constants::value::EMPTY,
    }
}

fn selected_device_stale_at(status: &LanPairingStatus) -> String {
    status
        .selected_target
        .as_ref()
        .and_then(|target| target.stale_at.clone())
        .unwrap_or_default()
}
