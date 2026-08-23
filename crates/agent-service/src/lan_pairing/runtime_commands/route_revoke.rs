use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentEventEnvelope};

use super::super::runtime_validation::{revoke_pairing, validate_command_target};
use super::super::{
    extend_log_fields, runtime_rejection::rejection_event,
    runtime_validation::validate_selection_intent_result, LanPairingRuntime,
};
use crate::lan_pairing_audit::revoked_route_audit_fields;
use crate::lan_pairing_payload::parse_intent;
use crate::lan_pairing_status::pairing_status_event;

pub(super) fn lan_pairing_route_revoke(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let event = match parse_intent(&command.payload) {
        Ok(intent) => match validate_command_target(&runtime, &command, &intent)
            .and_then(|()| validate_selection_intent_result(&runtime, &origin, &intent))
        {
            Ok(()) => revoke_or_status(runtime, origin, command, intent),
            Err(reason) => rejection_event(command, &reason, Some(&intent), &origin),
        },
        Err(reason) => rejection_event(command, &reason, None, &origin),
    };
    event
}

fn revoke_or_status(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
    intent: ocentra_parent_agent_protocol::lan_pairing::LanParentIntentEnvelope,
) -> AgentEventEnvelope {
    match revoke_pairing(&runtime, &intent) {
        Ok(()) => {
            let audit_fields = revoked_route_audit_fields(&command, &intent, &origin);
            let mut event = pairing_status_event(&runtime, command);
            extend_log_fields(&mut event.payload, audit_fields);
            event
        }
        Err(reason) => rejection_event(command, &reason, Some(&intent), &origin),
    }
}
