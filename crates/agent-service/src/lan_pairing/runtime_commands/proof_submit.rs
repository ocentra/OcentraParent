use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentEventEnvelope};

use super::super::{
    device_ref, extend_log_fields, runtime_rejection::pairing_rejection_event,
    runtime_validation::validate_pairing_proof_target, LanPairingRuntime,
};
use super::proof_submit_persistence::persist_accepted_pairing_proof;
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
    if let Err(reason) = validate_pairing_proof_target(&runtime, &command, &proof, &origin) {
        return pairing_rejection_event(command, &reason);
    }
    let platform = LanPairingText(command.target.platform.clone());
    let child_device = device_ref(
        LanPairingText(proof.child_device_id.clone()),
        platform.clone(),
    );
    let parent_device = device_ref(LanPairingText(proof.parent_device_id.clone()), platform);
    let trusted_at = LanPairingText(timestamp_now());
    let accepted =
        persist_accepted_pairing_proof(&runtime, &proof, child_device, parent_device, trusted_at);
    let event = match accepted {
        Ok(()) => {
            let mut event = pairing_status_event(&runtime, command);
            extend_log_fields(&mut event.payload, accepted_audit_fields);
            event
        }
        Err(reason) => pairing_rejection_event(command, &reason),
    };
    drop(runtime);
    event
}
