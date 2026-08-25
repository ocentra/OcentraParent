use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingRejectionReason, LanParentIntentEnvelope,
};
use serde::{Deserialize, Serialize};

use super::TrustedDeviceRegistry;

mod validation;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LanControllerLeaseMutation {
    Ensure,
    Renew,
    Release,
    Takeover,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LanTrustedControllerLease {
    pub controller_lease_id: String,
    pub controller_device_id: String,
    pub parent_actor_id: String,
    pub issued_at: String,
    pub expires_at: String,
}

impl TrustedDeviceRegistry {
    pub fn active_controller_lease(&self) -> Option<&LanTrustedControllerLease> {
        self.controller_lease.as_ref()
    }

    pub fn apply_controller_lease(
        &mut self,
        intent: &LanParentIntentEnvelope,
        observed_at: &str,
        mutation: LanControllerLeaseMutation,
    ) -> Result<(), LanPairingRejectionReason> {
        let mut candidate = self.clone();
        candidate.apply_controller_lease_candidate(intent, observed_at, mutation)?;
        *self = candidate;
        Ok(())
    }

    fn apply_controller_lease_candidate(
        &mut self,
        intent: &LanParentIntentEnvelope,
        observed_at: &str,
        mutation: LanControllerLeaseMutation,
    ) -> Result<(), LanPairingRejectionReason> {
        let validated = validation::validate_candidate(intent, observed_at)?;
        validation::clear_expired_or_invalid(self, validated.observed_at)?;
        let relationship = match self.controller_lease.as_ref() {
            None => LeaseOwnerRelationship::Missing,
            Some(active) if active.same_owner(&validated.candidate) => LeaseOwnerRelationship::Same,
            Some(_) => LeaseOwnerRelationship::Other,
        };

        match (mutation, relationship) {
            (LanControllerLeaseMutation::Ensure, LeaseOwnerRelationship::Same) => Ok(()),
            (LanControllerLeaseMutation::Release, LeaseOwnerRelationship::Same) => {
                self.controller_lease = None;
                Ok(())
            }
            (
                LanControllerLeaseMutation::Ensure
                | LanControllerLeaseMutation::Renew
                | LanControllerLeaseMutation::Takeover,
                LeaseOwnerRelationship::Missing | LeaseOwnerRelationship::Same,
            ) => {
                self.controller_lease = Some(validated.candidate);
                Ok(())
            }
            (LanControllerLeaseMutation::Release, LeaseOwnerRelationship::Missing) => {
                Err(LanPairingRejectionReason::ControllerLeaseMissing)
            }
            (LanControllerLeaseMutation::Takeover, LeaseOwnerRelationship::Other) => {
                Err(LanPairingRejectionReason::TakeoverDenied)
            }
            (_, LeaseOwnerRelationship::Other) => Err(LanPairingRejectionReason::WrongController),
        }
    }

    pub(super) fn validate_controller_lease_state(&self) -> Result<(), ()> {
        validation::validate_state(self)
    }
}

impl LanTrustedControllerLease {
    fn same_owner(&self, other: &Self) -> bool {
        self.controller_lease_id == other.controller_lease_id
            && self.controller_device_id == other.controller_device_id
            && self.parent_actor_id == other.parent_actor_id
    }
}

enum LeaseOwnerRelationship {
    Missing,
    Same,
    Other,
}
