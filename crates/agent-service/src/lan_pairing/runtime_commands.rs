#[path = "runtime_commands/proof_submit.rs"]
mod proof_submit;
#[path = "runtime_commands/proof_submit_persistence.rs"]
mod proof_submit_persistence;
#[path = "runtime_commands/route_revoke.rs"]
mod route_revoke;
#[path = "runtime_commands/route_select.rs"]
mod route_select;
#[path = "runtime_commands/status_get.rs"]
mod status_get;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{LanPairingOptionalText, LanParentIntentEnvelope};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};

use super::runtime_rejection::rejection_event;
use super::runtime_validation::{validate_command_target, validate_intent_result};
use super::{extend_log_fields, LanPairingRuntime};
use crate::lan_pairing_audit::accepted_control_audit_fields;
use crate::lan_pairing_payload::parse_signed_child_agent_envelope;
use crate::lan_pairing_status::pairing_status_event;
use crate::time::timestamp_now;

pub(super) fn lan_pairing_route_select(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    route_select::lan_pairing_route_select(runtime, origin, command)
}

pub(super) fn lan_pairing_status_get(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    status_get::lan_pairing_status_get(runtime, origin, command)
}

pub(super) fn lan_pairing_route_revoke(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    route_revoke::lan_pairing_route_revoke(runtime, origin, command)
}

pub(super) fn signed_child_agent_observed(
    runtime: &LanPairingRuntime,
    origin: &LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let envelope = match parse_signed_child_agent_envelope(&command.payload) {
        Ok(envelope) => envelope,
        Err(reason) => return rejection_event(command, &reason, None, origin),
    };
    let claim =
        match runtime.observe_signed_child_agent_envelope(&envelope, &timestamp_now::<String>()) {
            Ok(claim) => claim,
            Err(reason) => return rejection_event(command, &reason, None, origin),
        };
    let mut event = pairing_status_event(runtime, command);
    event.event_id = constants::lan_pairing::EVENT_SIGNED_CHILD_AGENT_REPORTED.to_string();
    event.event = AgentEventName::AgentLanPairingSignedChildAgentReported;
    extend_log_fields(
        &mut event.payload,
        crate::lan_pairing_audit::signed_child_agent_audit_fields(&claim),
    );
    event
}

pub(super) async fn submit_pairing_proof(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    proof_submit::submit_pairing_proof(runtime, origin, command).await
}

pub(super) fn validate_control_intent(
    runtime: LanPairingRuntime,
    origin: &LanPairingOptionalText,
    command: AgentCommandEnvelope,
    intent: LanParentIntentEnvelope,
) -> super::LanCommandDecision {
    let decision = match validate_command_target(&runtime, &command, &intent)
        .and_then(|()| validate_intent_result(&runtime, origin, &intent))
    {
        Ok(()) => super::LanCommandDecision::Continue {
            audit_fields: Some(accepted_control_audit_fields(&command, &intent, origin)),
            command,
        },
        Err(reason) => super::LanCommandDecision::Respond(rejection_event(
            command,
            &reason,
            Some(&intent),
            origin,
        )),
    };
    drop(intent);
    drop(runtime);
    decision
}
