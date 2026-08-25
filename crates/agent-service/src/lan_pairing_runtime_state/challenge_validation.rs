use chrono::{DateTime, Utc};
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
            // A proof is never self-authorizing.  The in-memory challenge ledger is
            // deliberately empty after restart, so an old or caller-minted proof
            // must fail closed instead of being accepted as a fresh pairing.
            return Err(LanPairingRejectionReason::Anonymous);
        }

        let challenge = challenges
            .iter_mut()
            .find(|candidate| candidate.challenge_id == proof.challenge_id)
            .ok_or(LanPairingRejectionReason::Malformed)?;
        if challenge.accepted {
            return Err(LanPairingRejectionReason::Replayed);
        }
        if challenge.child_device_id.is_empty()
            || challenge.parent_device_id.is_empty()
            || challenge.route_id.is_empty()
            || challenge.origin.is_empty()
            || challenge.proof_digest.is_empty()
            || proof.challenge_id.is_empty()
            || proof.pairing_id.is_empty()
            || proof.child_device_id.is_empty()
            || proof.parent_device_id.is_empty()
            || proof.route_id.is_empty()
            || proof.origin.is_empty()
            || proof.proof_digest.is_empty()
        {
            return Err(LanPairingRejectionReason::Malformed);
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
        if challenge.issued_at != proof.issued_at || challenge.expires_at != proof.expires_at {
            return Err(LanPairingRejectionReason::Malformed);
        }
        let Some(observed_at) = strict_timestamp(&observed_at) else {
            return Err(LanPairingRejectionReason::Malformed);
        };
        let Some(issued_at) = strict_timestamp(&LanPairingText(challenge.issued_at.clone())) else {
            return Err(LanPairingRejectionReason::Malformed);
        };
        let Some(expires_at) = strict_timestamp(&LanPairingText(challenge.expires_at.clone()))
        else {
            return Err(LanPairingRejectionReason::Malformed);
        };
        if issued_at >= expires_at || observed_at < issued_at || observed_at >= expires_at {
            return Err(LanPairingRejectionReason::Stale);
        }

        challenge.accepted = true;
        Ok(())
    }
}

fn strict_timestamp(value: &LanPairingText) -> Option<DateTime<Utc>> {
    let parsed = DateTime::parse_from_rfc3339(value.0.as_str())
        .ok()?
        .with_timezone(&Utc);
    (parsed.to_rfc3339_opts(chrono::SecondsFormat::Millis, true) == value.0).then_some(parsed)
}
