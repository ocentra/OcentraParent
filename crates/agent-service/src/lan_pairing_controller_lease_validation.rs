use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingRejectionReason, LanPairingText, LanParentIntentEnvelope,
};

use crate::lan_pairing::LanPairingRuntime;

use super::LanControllerLeaseState;

impl LanPairingRuntime {
    pub(crate) fn validate_controller_lease(
        &self,
        intent: &LanParentIntentEnvelope,
        observed_at: impl Into<LanPairingText>,
    ) -> Result<(), LanPairingRejectionReason> {
        let observed_at = observed_at.into();
        let lease = LanControllerLeaseState::from_intent(intent);
        if lease_is_expired(&lease, &observed_at) {
            return Err(LanPairingRejectionReason::ControllerLeaseExpired);
        }

        let mut active_lease = self.controller_lease.lock().map_err(|error| {
            let _ = error;
            LanPairingRejectionReason::Malformed
        })?;
        clear_expired_lease(&mut active_lease, &observed_at);

        match active_lease.as_ref() {
            Some(active) if active.matches(&lease) => Ok(()),
            Some(_) => Err(LanPairingRejectionReason::WrongController),
            None => {
                *active_lease = Some(lease);
                Ok(())
            }
        }
    }

    pub(crate) fn release_controller_lease(
        &self,
        intent: &LanParentIntentEnvelope,
        observed_at: impl Into<LanPairingText>,
    ) -> Result<(), LanPairingRejectionReason> {
        let observed_at = observed_at.into();
        let lease = LanControllerLeaseState::from_intent(intent);
        let mut active_lease = self.controller_lease.lock().map_err(|error| {
            let _ = error;
            LanPairingRejectionReason::Malformed
        })?;
        clear_expired_lease(&mut active_lease, &observed_at);

        match active_lease.as_ref() {
            Some(active) if active.matches(&lease) => {
                *active_lease = None;
                Ok(())
            }
            Some(_) => Err(LanPairingRejectionReason::WrongController),
            None => Err(LanPairingRejectionReason::ControllerLeaseMissing),
        }
    }
}

pub(super) fn clear_expired_lease(
    active_lease: &mut Option<LanControllerLeaseState>,
    observed_at: &LanPairingText,
) {
    if active_lease
        .as_ref()
        .is_some_and(|active| lease_is_expired(active, observed_at))
    {
        *active_lease = None;
    }
}

pub(super) fn lease_is_expired(
    lease: &LanControllerLeaseState,
    observed_at: &LanPairingText,
) -> bool {
    observed_at.0.as_str() > lease.expires_at.as_str()
}
