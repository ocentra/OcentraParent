use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentEventEnvelope};

use super::super::{
    runtime_rejection::pairing_rejection_event, runtime_validation::validate_pairing_proof_target,
    LanPairingRuntime,
};
use crate::lan_pairing_payload::parse_pairing_proof;

pub(super) async fn submit_pairing_proof(
    runtime: LanPairingRuntime,
    origin: LanPairingOptionalText,
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let proof = match parse_pairing_proof(&command.payload) {
        Ok(proof) => proof,
        Err(reason) => return pairing_rejection_event(command, &reason),
    };
    if let Err(reason) = validate_pairing_proof_target(&runtime, &command, &proof, &origin) {
        return pairing_rejection_event(command, &reason);
    }
    // A wire proof carries caller-controlled fields only; it has no
    // dependency-owned device/owner signature. Keep this route contract-only
    // until a trusted owner adapter supplies a non-forgeable authority token.
    // In particular, never derive `Paired` state from the proof fields.
    let event = pairing_rejection_event(
        command,
        &ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason::
            SignedChildAgentContextUnavailable,
    );
    drop(runtime);
    event
}
