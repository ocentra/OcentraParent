use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentEventEnvelope};

use crate::lan_pairing::{
    extend_log_fields, pairing_status_event, revoked_route_audit_fields,
    validate_selection_intent_result, LanPairingRuntime,
};
use crate::lan_pairing_payload::parse_intent;

use super::{rejection_event, validate_command_target};

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
