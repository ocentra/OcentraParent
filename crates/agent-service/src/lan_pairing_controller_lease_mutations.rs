use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingRejectionReason, LanPairingText, LanParentIntentEnvelope,
};

use crate::lan_pairing::LanPairingRuntime;

use super::{
    lease_validation::{clear_expired_lease, lease_is_expired},
    LanControllerLeaseState,
};

impl LanPairingRuntime {
    pub(crate) fn renew_controller_lease(
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
            Some(active) if active.matches(&lease) => {
                *active_lease = Some(lease);
                Ok(())
            }
            Some(_) => Err(LanPairingRejectionReason::WrongController),
            None => {
                *active_lease = Some(lease);
                Ok(())
            }
        }
    }

    pub(crate) fn takeover_controller_lease(
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
            Some(active) if active.matches(&lease) => {
                *active_lease = Some(lease);
                Ok(())
            }
            Some(_) => Err(LanPairingRejectionReason::TakeoverDenied),
            None => {
                *active_lease = Some(lease);
                Ok(())
            }
        }
    }
}
