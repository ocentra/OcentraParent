use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentEventEnvelope};

use super::super::runtime_validation::validate_command_target;
use super::super::{
    extend_log_fields, runtime_rejection::rejection_event,
    runtime_validation::validate_selection_intent_result, LanPairingRuntime,
};
use crate::lan_pairing_audit::accepted_control_audit_fields;
use crate::lan_pairing_payload::{is_challenge_request, parse_intent};
use crate::lan_pairing_status::{pairing_challenge_status_event, pairing_status_event};

pub(super) fn lan_pairing_status_get(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    if is_challenge_request(&command.payload) {
        return pairing_challenge_status_event(&runtime, origin, command);
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
