use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingChallengeRequest;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::lan_pairing::LanSelectedRouteTarget;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{
    event_builder::build_event,
    fields::fields_from_pairs,
    lan_pairing::{validate_local_child_target, LanPairingChallengeState, LanPairingRuntime},
    lan_pairing_audit::{challenge_issued_audit_fields, rejected_control_audit_fields},
    lan_pairing_browser_add_device_state::browser_add_device_pairs,
    lan_pairing_payload::parse_challenge_request,
    time::timestamp_now,
};

mod selection;

#[derive(Clone, Debug)]
struct LanPairingStatus {
    pairing_state: &'static str,
    authentication_state: &'static str,
    trusted_device_count: usize,
    selected_target: Option<LanSelectedRouteTarget>,
    trusted_device_ids: Vec<String>,
    revoked_device_ids: Vec<String>,
    has_revoked_pairing: bool,
    active_challenge_count: usize,
}

pub(crate) fn pairing_status_event(
    runtime: &LanPairingRuntime,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let status = pairing_status(runtime);
    let mut pairs = support_surface_pairs(runtime);
    pairs.extend(browser_add_device_pairs(
        runtime,
        &command,
        discovery_state(&status),
    ));
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

pub(crate) fn pairing_challenge_status_event(
    runtime: &LanPairingRuntime,
    origin: Option<&str>,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match parse_challenge_request(&command.payload) {
        Ok(request) => match validate_challenge_request(runtime, origin, &command, &request) {
            Ok(()) => {
                let challenge = challenge_state_for_request(&command, &request);
                runtime.remember_challenge(challenge.clone());
                let mut event = pairing_status_event(runtime, command);
                event
                    .payload
                    .extend(challenge_issued_audit_fields(&challenge));
                event
            }
            Err(reason) => challenge_rejection_event(command, &reason, origin),
        },
        Err(reason) => challenge_rejection_event(command, &reason, origin),
    }
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
        active_challenge_count: active_challenge_count(runtime),
    };
    status.pairing_state = pairing_state(&status);
    status
}

fn validate_challenge_request(
    runtime: &LanPairingRuntime,
    origin: Option<&str>,
    command: &AgentCommandEnvelope,
    request: &LanPairingChallengeRequest,
) -> Result<(), LanPairingRejectionReason> {
    validate_local_child_target(runtime, command)?;
    if command.target.device_id.as_str() != request.child_device_id.as_str() {
        return Err(LanPairingRejectionReason::WrongDevice);
    }
    if origin != Some(request.origin.as_str()) {
        return Err(LanPairingRejectionReason::WrongOrigin);
    }
    if timestamp_now().as_str() > request.expires_at.as_str() {
        return Err(LanPairingRejectionReason::Stale);
    }
    Ok(())
}

fn challenge_state_for_request(
    command: &AgentCommandEnvelope,
    request: &LanPairingChallengeRequest,
) -> LanPairingChallengeState {
    let mut challenge_id = String::from(constants::lan_pairing::CHALLENGE_ID_PREFIX);
    challenge_id.push_str(&request.child_device_id);
    challenge_id.push(constants::delimiter::HYPHEN);
    challenge_id.push_str(&command.source.peer_id);
    let mut proof_digest = String::from(constants::lan_pairing::PROOF_DIGEST_PREVIEW_PREFIX);
    proof_digest.push_str(&request.child_device_id);
    proof_digest.push(constants::delimiter::HYPHEN);
    proof_digest.push_str(&request.parent_device_id);
    proof_digest.push(constants::delimiter::HYPHEN);
    proof_digest.push_str(&request.route_id);
    LanPairingChallengeState {
        challenge_id,
        child_device_id: request.child_device_id.clone(),
        parent_device_id: request.parent_device_id.clone(),
        route_id: request.route_id.clone(),
        origin: request.origin.clone(),
        proof_digest,
        issued_at: request.issued_at.clone(),
        expires_at: request.expires_at.clone(),
        accepted: false,
    }
}

fn challenge_rejection_event(
    command: AgentCommandEnvelope,
    reason: &LanPairingRejectionReason,
    origin: Option<&str>,
) -> AgentEventEnvelope {
    let payload = rejected_control_audit_fields(&command, reason, None, origin);
    build_event(
        constants::event_id::COMMAND_REJECTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentCommandRejected,
        LogLevel::Warn,
        payload,
        None,
    )
}

fn active_challenge_count(runtime: &LanPairingRuntime) -> usize {
    runtime
        .challenges
        .lock()
        .map(|challenges| {
            challenges
                .iter()
                .filter(|challenge| !challenge.accepted)
                .count()
        })
        .unwrap_or(0)
}

fn support_surface_pairs(runtime: &LanPairingRuntime) -> Vec<(&'static str, LogFieldValue)> {
    let status = pairing_status(runtime);
    let mut pairs = vec![
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
            LogFieldValue::String(constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT.to_string()),
        ),
        (
            constants::field::LAN_DISCOVERY_STATE,
            LogFieldValue::String(discovery_state(&status).to_string()),
        ),
        (
            constants::field::LAN_CHALLENGE_STATUS,
            LogFieldValue::String(constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT.to_string()),
        ),
        (
            constants::field::LAN_PROOF_PREVIEW_STATUS,
            LogFieldValue::String(constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT.to_string()),
        ),
        (
            constants::field::LAN_AI_JOB_STATUS,
            LogFieldValue::String(constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT.to_string()),
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
    ];
    pairs.extend(lan_ai_provider_support_pairs());
    pairs
}

fn lan_ai_provider_support_pairs() -> Vec<(&'static str, LogFieldValue)> {
    vec![
        (
            constants::field::LAN_AI_PROVIDER_STATUS,
            LogFieldValue::String(constants::lan_pairing::SUPPORT_WEBSOCKET_DIRECT.to_string()),
        ),
        (
            constants::field::LAN_AI_PROVIDER_ROUTING_STATE,
            LogFieldValue::String(
                constants::value::LAN_AI_PROVIDER_ROUTING_UNAVAILABLE.to_string(),
            ),
        ),
        (
            constants::field::LAN_AI_PROVIDER_CUSTODY_LABEL,
            LogFieldValue::String(
                constants::value::LAN_PROVIDER_CUSTODY_LOCAL_NETWORK_AI_PROVIDER.to_string(),
            ),
        ),
    ]
}

fn discovery_state(status: &LanPairingStatus) -> &'static str {
    match status
        .selected_target
        .as_ref()
        .map(|target| &target.reachability)
    {
        Some(LanPairingDeviceReachability::Offline) => {
            constants::value::LAN_DISCOVERY_STATE_OFFLINE
        }
        Some(LanPairingDeviceReachability::Stale) => constants::value::LAN_DISCOVERY_STATE_STALE,
        Some(LanPairingDeviceReachability::Online) => constants::value::LAN_DISCOVERY_STATE_PAIRED,
        None if status.trusted_device_count > 0 => constants::value::LAN_DISCOVERY_STATE_PAIRED,
        None if status.active_challenge_count > 0 => constants::value::LAN_DISCOVERY_STATE_PENDING,
        None if status.has_revoked_pairing => constants::value::LAN_DISCOVERY_STATE_REVOKED,
        None => constants::value::LAN_DISCOVERY_STATE_DISCOVERED,
    }
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
            LogFieldValue::String(selection::child_device_id(status.selected_target.as_ref())),
        ),
        (
            constants::field::LAN_SELECTED_PAIRING_ID,
            LogFieldValue::String(selection::pairing_id(status.selected_target.as_ref())),
        ),
        (
            constants::field::LAN_SELECTED_ROUTE_TRUST_STATE,
            LogFieldValue::String(
                selection::route_trust_state(status.selected_target.as_ref()).to_string(),
            ),
        ),
        (
            constants::field::LAN_SELECTED_DEVICE_REACHABILITY,
            LogFieldValue::String(
                selection::reachability(status.selected_target.as_ref()).to_string(),
            ),
        ),
        (
            constants::field::LAN_SELECTED_DEVICE_STALE_AT,
            LogFieldValue::String(selection::stale_at(status.selected_target.as_ref())),
        ),
        (
            constants::field::LAN_SELECTED_ROUTE_ID,
            LogFieldValue::String(selection::route_id(status.selected_target.as_ref())),
        ),
        (
            constants::field::LAN_SELECTED_ROUTE_STALE_AT,
            LogFieldValue::String(selection::stale_at(status.selected_target.as_ref())),
        ),
        (
            constants::field::LAN_SELECTED_ROUTE_OFFLINE_AT,
            LogFieldValue::String(selection::offline_at(status.selected_target.as_ref())),
        ),
    ]
}

fn pairing_state(status: &LanPairingStatus) -> &'static str {
    if status.trusted_device_count > 0 {
        constants::value::LAN_PAIRING_PAIRED
    } else if status.active_challenge_count > 0 {
        constants::value::LAN_PAIRING_PAIRING
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
