use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingAuditEventType;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingProof;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::lan_pairing::LanParentIntentEnvelope;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use crate::event_builder::build_event;
use crate::lan_pairing::runtime_device_ref::device_ref;
use crate::lan_pairing::runtime_rejection::{pairing_rejection_event, rejection_event};
use crate::lan_pairing::runtime_validation::{
    select_pairing_result, validate_command_target, validate_intent_result,
    validate_pairing_proof_target, validate_selection_intent_result,
};
use crate::lan_pairing::{
    extend_log_fields, pairing_challenge_status_event, pairing_status_event,
    rejected_control_audit_fields, rejected_pairing_audit_fields, selected_route_audit_fields,
    LanPairingRuntime,
};
use crate::lan_pairing_audit::{
    accepted_control_audit_fields, accepted_pairing_audit_fields, revoked_route_audit_fields,
    signed_child_agent_audit_fields,
};
use crate::lan_pairing_payload::{
    is_challenge_request, parse_intent, parse_pairing_proof, parse_signed_child_agent_envelope,
};
use crate::time::timestamp_now;

pub(super) fn lan_pairing_route_select(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let event = match parse_intent(&command.payload) {
        Ok(intent) => match validate_command_target(&runtime, &command, &intent)
            .and_then(|()| validate_selection_intent_result(&runtime, &origin, &intent))
        {
            Ok(()) => match select_pairing_result(&runtime, &intent) {
                Ok(()) => {
                    let audit_fields = selected_route_audit_fields(&command, &intent, &origin);
                    let mut event = pairing_status_event(&runtime, command);
                    extend_log_fields(&mut event.payload, audit_fields);
                    event
                }
                Err(reason) => rejection_event(command, &reason, Some(&intent), &origin),
            },
            Err(reason) => rejection_event(command, &reason, Some(&intent), &origin),
        },
        Err(reason) => rejection_event(command, &reason, None, &origin),
    };
    drop(origin);
    drop(runtime);
    event
}

pub(super) fn lan_pairing_status_get(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    if is_challenge_request(&command.payload) {
        return pairing_challenge_status_event(&runtime, origin.clone(), command);
    }
    if crate::lan_pairing::log_fields_is_empty(&command.payload) {
        return pairing_status_event(&runtime, command);
    }
    let event = match parse_intent(&command.payload) {
        Ok(intent) => match validate_command_target(&runtime, &command, &intent)
            .and_then(|()| validate_selection_intent_result(&runtime, &origin, &intent))
        {
            Ok(()) => {
                let audit_fields = accepted_control_audit_fields(&command, &intent, &origin);
                let mut event = pairing_status_event(&runtime, command);
                extend_log_fields(&mut event.payload, audit_fields);
                event
            }
            Err(reason) => rejection_event(command, &reason, Some(&intent), &origin),
        },
        Err(reason) => rejection_event(command, &reason, None, &origin),
    };
    drop(origin);
    drop(runtime);
    event
}

pub(super) fn lan_pairing_route_revoke(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let event = match parse_intent(&command.payload) {
        Ok(intent) => match validate_command_target(&runtime, &command, &intent)
            .and_then(|()| validate_selection_intent_result(&runtime, &origin, &intent))
        {
            Ok(()) => {
                crate::lan_pairing::runtime_validation::revoke_pairing(&runtime, &intent);
                let audit_fields = revoked_route_audit_fields(&command, &intent, &origin);
                let mut event = pairing_status_event(&runtime, command);
                extend_log_fields(&mut event.payload, audit_fields);
                event
            }
            Err(reason) => rejection_event(command, &reason, Some(&intent), &origin),
        },
        Err(reason) => rejection_event(command, &reason, None, &origin),
    };
    drop(origin);
    drop(runtime);
    event
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
    let claim = match runtime.observe_signed_child_agent_envelope(&envelope, &timestamp_now()) {
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
    let proof = match parse_pairing_proof(&command.payload) {
        Ok(proof) => proof,
        Err(reason) => return pairing_rejection_event(command, &reason),
    };
    let observed_origin = origin.0.as_deref();
    let accepted_audit_fields = accepted_pairing_audit_fields(&command, &proof);
    let event = match validate_pairing_proof_target(&runtime, &command, &proof, &origin)
        .and_then(|()| runtime.validate_challenge_proof(&proof, &timestamp_now()))
    {
        Ok(()) => {
            let mut event = pairing_status_event(&runtime, command);
            extend_log_fields(&mut event.payload, accepted_audit_fields);
            let _ = observed_origin;
            event
        }
        Err(reason) => pairing_rejection_event(command, &reason),
    };
    drop(runtime);
    event
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
