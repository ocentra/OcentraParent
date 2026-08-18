use std::fmt;

use super::{
    ConsumedParentStepUp, CurrentChildDeviceTrustBinding, CurrentHouseholdCapability,
    CurrentHouseholdControllerLease, HouseholdAuthorityRuntimeAuthorization,
};

impl fmt::Debug for HouseholdAuthorityRuntimeAuthorization {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HouseholdAuthorityRuntimeAuthorization")
            .field("action", &self.action)
            .field(
                "account_authority_generation",
                &self.account_authority_generation,
            )
            .field("session_generation", &self.session_generation)
            .field("capability", &self.capability.is_some())
            .field("controller_lease", &self.controller_lease.is_some())
            .field("parent_step_up", &self.parent_step_up.is_some())
            .field("device_binding", &"opaque")
            .finish()
    }
}

impl HouseholdAuthorityRuntimeAuthorization {
    pub fn action(&self) -> super::HouseholdAuthorityAction {
        self.action
    }

    pub fn account_authority_generation(&self) -> u64 {
        self.account_authority_generation
    }

    pub fn session_generation(&self) -> u64 {
        self.session_generation
    }

    pub(crate) fn device_binding(&self) -> &CurrentChildDeviceTrustBinding {
        &self.device_binding
    }

    pub(crate) fn capability(&self) -> Option<&CurrentHouseholdCapability> {
        self.capability.as_ref()
    }

    pub(crate) fn controller_lease(&self) -> Option<&CurrentHouseholdControllerLease> {
        self.controller_lease.as_ref()
    }

    pub(crate) fn parent_step_up(&self) -> Option<&ConsumedParentStepUp> {
        self.parent_step_up.as_ref()
    }
}
