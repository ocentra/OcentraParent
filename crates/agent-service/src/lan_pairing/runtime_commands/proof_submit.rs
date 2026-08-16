use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentEventEnvelope};

use super::super::{
    device_ref, extend_log_fields, runtime_rejection::pairing_rejection_event,
    runtime_validation::validate_pairing_proof_target, LanPairingRuntime,
};
use crate::lan_pairing_audit::accepted_pairing_audit_fields;
use crate::lan_pairing_payload::parse_pairing_proof;
use crate::lan_pairing_status::pairing_status_event;
use crate::time::timestamp_now;

pub(super) async fn submit_pairing_proof(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let proof = match parse_pairing_proof(&command.payload) {
        Ok(proof) => proof,
        Err(reason) => return pairing_rejection_event(command, &reason),
    };
    let accepted_audit_fields = accepted_pairing_audit_fields(&command, &proof);
    let event = match validate_pairing_proof_target(&runtime, &command, &proof, &origin) {
        Ok(()) => {
            let platform = LanPairingText(command.target.platform.clone());
            let child_device = device_ref(
                LanPairingText(proof.child_device_id.clone()),
                platform.clone(),
            );
            let parent_device =
                device_ref(LanPairingText(proof.parent_device_id.clone()), platform);
            let trusted_at: String = timestamp_now();
            let accepted = runtime
                .registry
                .lock()
                .map(|mut registry| {
                    registry.accept_pairing_proof(&proof, child_device, parent_device, &trusted_at);
                    runtime.persist_registry(&registry);
                })
                .is_ok();
            if accepted {
                let mut event = pairing_status_event(&runtime, command);
                extend_log_fields(&mut event.payload, accepted_audit_fields);
                event
            } else {
                pairing_rejection_event(
                    command,
                    &ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason::Malformed,
                )
            }
        }
        Err(reason) => pairing_rejection_event(command, &reason),
    };
    drop(runtime);
    event
}
