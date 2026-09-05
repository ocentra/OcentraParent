use chrono::{DateTime, Duration, Utc};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingChallengeRequest;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::lan_pairing::LanSelectedRouteTarget;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::{
    event_builder::build_event,
    lan_pairing::{
        extend_log_fields, validate_local_child_target, LanPairingChallengeState, LanPairingRuntime,
    },
    lan_pairing_audit::{challenge_issued_audit_fields, rejected_control_audit_fields},
    lan_pairing_browser_add_device_state::browser_add_device_fields,
    lan_pairing_payload::parse_challenge_request,
};

#[path = "lan_pairing_status/selection.rs"]
pub(crate) mod selection;
#[path = "lan_pairing_status/state_projection.rs"]
mod state_projection;
#[path = "lan_pairing_status/support_fields.rs"]
mod support_fields;

#[derive(Clone, Debug)]
struct LanPairingStatus {
    pairing_state: LanPairingText,
    authentication_state: LanPairingText,
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
    let mut fields = support_surface_fields(runtime);
    extend_log_fields(
        &mut fields,
        browser_add_device_fields(
            runtime,
            &command,
            &state_projection::discovery_state(&status),
        ),
    );
    extend_log_fields(&mut fields, state_fields(&status));
    build_event(
        constants::lan_pairing::EVENT_STATUS_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentLanPairingStatusReported,
        LogLevel::Info,
        fields,
        None,
    )
}

pub(crate) fn pairing_challenge_status_event(
    runtime: &LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let origin = LanPairingOptionalText(origin.0);
    match parse_challenge_request(&command.payload) {
        Ok(request) => {
            validated_pairing_challenge_status_event(runtime, &origin, command, &request)
        }
        Err(reason) => challenge_rejection_event(command, &reason, &origin),
    }
}

fn validated_pairing_challenge_status_event(
    runtime: &LanPairingRuntime,
    origin: &LanPairingOptionalText,
    command: AgentCommandEnvelope,
    request: &LanPairingChallengeRequest,
) -> AgentEventEnvelope {
    if let Err(reason) = validate_challenge_request(runtime, origin, &command, request) {
        return challenge_rejection_event(command, &reason, origin);
    }
    let challenge = challenge_state_for_request(&command, request);
    if let Err(reason) = runtime.remember_challenge(challenge.clone()) {
        return challenge_rejection_event(command, &reason, origin);
    }
    let mut event = pairing_status_event(runtime, command);
    extend_log_fields(
        &mut event.payload,
        challenge_issued_audit_fields(&challenge),
    );
    event
}

fn pairing_status(runtime: &LanPairingRuntime) -> LanPairingStatus {
    let trusted_device_count = runtime.trusted_device_count();
    let selected_target = runtime.selected_target();
    let mut status = LanPairingStatus {
        pairing_state: constants::value::LAN_PAIRING_UNPAIRED.to_string().into(),
        authentication_state: state_projection::authentication_state(&selected_target),
        trusted_device_count,
        selected_target,
        trusted_device_ids: runtime
            .trusted_device_ids()
            .into_iter()
            .map(|value| value.0)
            .collect(),
        revoked_device_ids: runtime
            .revoked_device_ids()
            .into_iter()
            .map(|value| value.0)
            .collect(),
        has_revoked_pairing: runtime.has_revoked_pairing(),
        active_challenge_count: active_challenge_count(runtime),
    };
    status.pairing_state = state_projection::pairing_state(&status);
    status
}

fn validate_challenge_request(
    runtime: &LanPairingRuntime,
    origin: &LanPairingOptionalText,
    command: &AgentCommandEnvelope,
    request: &LanPairingChallengeRequest,
) -> Result<(), LanPairingRejectionReason> {
    validate_local_child_target(runtime, command)?;
    if command.target.device_id.as_str() != request.child_device_id.as_str() {
        return Err(LanPairingRejectionReason::WrongDevice);
    }
    if origin.0.as_deref() != Some(request.origin.as_str()) {
        return Err(LanPairingRejectionReason::WrongOrigin);
    }
    if request.child_device_id.trim().is_empty()
        || request.parent_device_id.trim().is_empty()
        || request.route_id.trim().is_empty()
        || request.origin.trim().is_empty()
        || request.issued_at.trim().is_empty()
        || request.expires_at.trim().is_empty()
        || command.message_id.trim().is_empty()
        || command.source.peer_id.trim().is_empty()
    {
        return Err(LanPairingRejectionReason::Anonymous);
    }
    let Some(issued_at) = strict_timestamp(&LanPairingText(request.issued_at.clone())) else {
        return Err(LanPairingRejectionReason::Malformed);
    };
    let Some(expires_at) = strict_timestamp(&LanPairingText(request.expires_at.clone())) else {
        return Err(LanPairingRejectionReason::Malformed);
    };
    let now = Utc::now();
    if issued_at > now
        || expires_at <= now
        || expires_at <= issued_at
        || expires_at - issued_at
            > Duration::seconds(constants::lan_pairing::LAN_PAIRING_CHALLENGE_MAX_LIFETIME_SECONDS)
    {
        return Err(LanPairingRejectionReason::Stale);
    }
    Ok(())
}

fn strict_timestamp(value: &LanPairingText) -> Option<DateTime<Utc>> {
    let parsed = DateTime::parse_from_rfc3339(value.0.as_str())
        .ok()?
        .with_timezone(&Utc);
    (parsed.to_rfc3339_opts(chrono::SecondsFormat::Millis, true) == value.0).then_some(parsed)
}

fn challenge_state_for_request(
    command: &AgentCommandEnvelope,
    request: &LanPairingChallengeRequest,
) -> LanPairingChallengeState {
    let mut challenge_id = String::from(constants::lan_pairing::CHALLENGE_ID_PREFIX);
    challenge_id.push_str(&request.child_device_id);
    challenge_id.push(constants::delimiter::HYPHEN);
    challenge_id.push_str(&command.source.peer_id);
    challenge_id.push(constants::delimiter::HYPHEN);
    challenge_id.push_str(&command.message_id);
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
    origin: &LanPairingOptionalText,
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

fn support_surface_fields(
    runtime: &LanPairingRuntime,
) -> ocentra_parent_agent_protocol::logging::LogFields {
    support_fields::support_surface_fields(runtime)
}

pub(crate) fn discovery_state_for_runtime(runtime: &LanPairingRuntime) -> LanPairingText {
    state_projection::discovery_state(&pairing_status(runtime))
}

pub fn route_trust_state_for_selected_target(
    selected: Option<&LanSelectedRouteTarget>,
) -> LanPairingText {
    selection::route_trust_state(selected)
}

fn state_fields(status: &LanPairingStatus) -> ocentra_parent_agent_protocol::logging::LogFields {
    state_projection::state_fields(status)
}
