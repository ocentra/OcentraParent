use ocentra_parent_agent_protocol::lan_pairing::LanPairingAuditEventType;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;

use crate::lan_pairing::{
    authority::validate_observer_read_intent, extend_log_fields,
    runtime_validation::validate_command_target, LanPairingRuntime,
};
use crate::lan_pairing_audit::controller_lease_audit_fields;
use crate::lan_pairing_payload::parse_intent;

use super::fields::lan_ai_provider_fields;
use super::job_submit::lan_ai_rejection_event;

pub(crate) fn lan_ai_provider_status_get(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let origin = LanPairingOptionalText(origin.0);
    let observed_origin = origin.0.as_deref();
    let event = match parse_intent(&command.payload) {
        Ok(intent) => match validate_command_target(&runtime, &command, &intent)
            .and_then(|()| validate_observer_read_intent(&runtime, observed_origin, &intent))
        {
            Ok(()) => {
                let audit_fields = controller_lease_audit_fields(
                    &command,
                    &intent,
                    &origin,
                    LanPairingAuditEventType::LanAiProviderAdvertised,
                    None,
                );
                let mut event = crate::lan_pairing_status::pairing_status_event(&runtime, command);
                extend_log_fields(&mut event.payload, audit_fields);
                extend_log_fields(&mut event.payload, lan_ai_provider_fields(&runtime));
                event
            }
            Err(reason) => lan_ai_rejection_event(
                &runtime,
                command,
                &reason,
                Some(&intent),
                &origin,
                LanPairingAuditEventType::LanAiJobRejected,
            ),
        },
        Err(reason) => lan_ai_rejection_event(
            &runtime,
            command,
            &reason,
            None,
            &origin,
            LanPairingAuditEventType::LanAiJobRejected,
        ),
    };
    drop(runtime);
    event
}
