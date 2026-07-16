use ocentra_parent_agent_protocol::lan_pairing::LanPairingProof;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use std::fmt::Display;

use crate::lan_pairing::LanPairingRuntime;

impl LanPairingRuntime {
    pub(crate) fn validate_challenge_proof(
        &self,
        proof: &LanPairingProof,
        observed_at: &impl Display,
    ) -> Result<(), LanPairingRejectionReason> {
        let observed_at = LanPairingText(observed_at.to_string());
        let mut challenges = self.challenges.lock().map_err(|error| {
            let _ = error;
            LanPairingRejectionReason::Malformed
        })?;
        if challenges.is_empty() {
            return Ok(());
        }

        let challenge = challenges
            .iter_mut()
            .find(|candidate| candidate.challenge_id == proof.challenge_id)
            .ok_or(LanPairingRejectionReason::Malformed)?;
        if challenge.accepted {
            return Err(LanPairingRejectionReason::Replayed);
        }
        if challenge.child_device_id != proof.child_device_id {
            return Err(LanPairingRejectionReason::WrongDevice);
        }
        if challenge.parent_device_id != proof.parent_device_id {
            return Err(LanPairingRejectionReason::Malformed);
        }
        if challenge.route_id != proof.route_id {
            return Err(LanPairingRejectionReason::UnsupportedRoute);
        }
        if challenge.origin != proof.origin {
            return Err(LanPairingRejectionReason::WrongOrigin);
        }
        if challenge.proof_digest != proof.proof_digest {
            return Err(LanPairingRejectionReason::Malformed);
        }
        if observed_at.0.as_str() > challenge.expires_at.as_str()
            || observed_at.0.as_str() > proof.expires_at.as_str()
        {
            return Err(LanPairingRejectionReason::Stale);
        }

        challenge.accepted = true;
        Ok(())
    }
}
