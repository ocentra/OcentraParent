use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentEventEnvelope};

use super::super::runtime_validation::validate_command_target;
use super::super::{
    extend_log_fields,
    runtime_rejection::rejection_event,
    runtime_validation::{select_pairing_result, validate_selection_intent_result},
    LanPairingRuntime,
};
use crate::lan_pairing_audit::selected_route_audit_fields;
use crate::lan_pairing_payload::parse_intent;
use crate::lan_pairing_status::pairing_status_event;

pub(super) fn lan_pairing_route_select(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let intent = match parse_intent(&command.payload) {
        Ok(intent) => intent,
        Err(reason) => return rejection_event(command, &reason, None, &origin),
    };
    if let Err(reason) = validate_command_target(&runtime, &command, &intent)
        .and_then(|()| validate_selection_intent_result(&runtime, &origin, &intent))
    {
        return rejection_event(command, &reason, Some(&intent), &origin);
    }
    if let Err(reason) = select_pairing_result(&runtime, &intent) {
        return rejection_event(command, &reason, Some(&intent), &origin);
    }
    let audit_fields = selected_route_audit_fields(&command, &intent, &origin);
    let mut event = pairing_status_event(&runtime, command);
    extend_log_fields(&mut event.payload, audit_fields);
    drop(origin);
    drop(runtime);
    event
}
