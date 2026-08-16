use ocentra_parent_agent_protocol::lan_pairing::LanPairingOptionalText;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingProof;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::lan_pairing::LanParentIntentEnvelope;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::LanPairingParentAuthority;

use crate::lan_pairing::authority::{validate_registry_selection_intent, validate_write_authority};
use crate::lan_pairing::LanPairingRuntime;
use crate::time::timestamp_now;

pub(super) fn validate_pairing_proof_target(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
    proof: &LanPairingProof,
    origin: &LanPairingOptionalText,
) -> Result<(), LanPairingRejectionReason> {
    validate_local_child_target(runtime, command)?;
    if origin.0.as_deref() != Some(proof.origin.as_str()) {
        return Err(LanPairingRejectionReason::WrongOrigin);
    }
    if command.target.device_id.as_str() == proof.child_device_id.as_str() {
        runtime.validate_challenge_proof(proof, &timestamp_now::<String>())
    } else {
        Err(LanPairingRejectionReason::WrongDevice)
    }
}

pub(super) fn validate_command_target(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    validate_local_child_target(runtime, command)?;
    if command.target.device_id.as_str() == intent.target_child_device_id.as_str() {
        Ok(())
    } else {
        Err(LanPairingRejectionReason::WrongDevice)
    }
}

pub(super) fn validate_local_child_target(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    match runtime.local_child_device_id.as_deref() {
        Some(local_child_device_id)
            if command.target.device_id.as_str() != local_child_device_id =>
        {
            Err(LanPairingRejectionReason::WrongDevice)
        }
        _ => Ok(()),
    }
}

pub(super) fn validate_intent_result(
    runtime: &LanPairingRuntime,
    origin: &LanPairingOptionalText,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    let observed_at: String = timestamp_now();
    if crate::lan_pairing::authority::is_write_intent(intent) {
        validate_write_authority(intent)?;
    }
    if intent.parent_authority == LanPairingParentAuthority::ActiveController {
        runtime.validate_controller_lease(intent, &*observed_at)?;
    }
    runtime
        .registry
        .lock()
        .map(|mut registry| registry.validate_intent(intent, origin.0.as_deref(), &observed_at))
        .unwrap_or(Err(LanPairingRejectionReason::Malformed))
}

pub(super) fn validate_selection_intent_result(
    runtime: &LanPairingRuntime,
    origin: &LanPairingOptionalText,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    let observed_at: String = timestamp_now();
    validate_write_authority(intent)?;
    runtime.validate_controller_lease(intent, &*observed_at)?;
    validate_registry_selection_intent(runtime, origin.0.as_deref(), intent)
}

pub(super) fn select_pairing_result(
    runtime: &LanPairingRuntime,
    intent: &LanParentIntentEnvelope,
) -> Result<(), LanPairingRejectionReason> {
    runtime
        .registry
        .lock()
        .map(|mut registry| {
            let selected = registry.select_pairing(
                &intent.pairing_id,
                &intent.target_child_device_id,
                &intent.route_id,
                &intent.expires_at,
            );
            if selected.is_ok() {
                let _ = registry.clear_selected_route_reachability();
                runtime.persist_registry(&registry);
            }
            selected
        })
        .unwrap_or(Err(LanPairingRejectionReason::Malformed))
        .map(|_| ())
}

pub(super) fn revoke_pairing(
    runtime: &LanPairingRuntime,
    intent: &LanParentIntentEnvelope,
) -> bool {
    let revoked_at: String = timestamp_now();
    runtime
        .registry
        .lock()
        .map(|mut registry| {
            let revoked = registry.revoke_pairing(&intent.pairing_id, &revoked_at);
            if revoked {
                runtime.persist_registry(&registry);
            }
            revoked
        })
        .unwrap_or(false)
}
