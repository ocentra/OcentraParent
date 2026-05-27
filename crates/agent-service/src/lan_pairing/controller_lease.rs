use ocentra_parent_agent_protocol::{LanPairingRejectionReason, LanParentIntentEnvelope};

use crate::lan_pairing::LanPairingRuntime;

#[derive(Clone, Debug)]
pub(crate) struct LanControllerLeaseState {
    pub(crate) controller_lease_id: String,
    pub(crate) controller_device_id: String,
    pub(crate) parent_actor_id: String,
    pub(crate) expires_at: String,
}

impl LanPairingRuntime {
    pub(crate) fn validate_controller_lease(
        &self,
        intent: &LanParentIntentEnvelope,
        observed_at: &str,
    ) -> Result<(), LanPairingRejectionReason> {
        let lease = LanControllerLeaseState::from_intent(intent);
        if observed_at > lease.expires_at.as_str() {
            return Err(LanPairingRejectionReason::ControllerLeaseExpired);
        }

        let mut active_lease = self
            .controller_lease
            .lock()
            .map_err(|_| LanPairingRejectionReason::Malformed)?;
        if active_lease
            .as_ref()
            .is_some_and(|active| observed_at > active.expires_at.as_str())
        {
            *active_lease = None;
        }

        match active_lease.as_ref() {
            Some(active) if active.matches(&lease) => Ok(()),
            Some(_) => Err(LanPairingRejectionReason::WrongController),
            None => {
                *active_lease = Some(lease);
                Ok(())
            }
        }
    }
}

impl LanControllerLeaseState {
    fn from_intent(intent: &LanParentIntentEnvelope) -> Self {
        Self {
            controller_lease_id: intent.controller_lease_id.clone(),
            controller_device_id: intent.controller_device_id.clone(),
            parent_actor_id: intent.parent_actor_id.clone(),
            expires_at: intent.controller_lease_expires_at.clone(),
        }
    }

    fn matches(&self, other: &Self) -> bool {
        self.controller_lease_id == other.controller_lease_id
            && self.controller_device_id == other.controller_device_id
            && self.parent_actor_id == other.parent_actor_id
    }
}
