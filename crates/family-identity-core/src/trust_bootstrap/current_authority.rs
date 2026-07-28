use std::fmt;

use serde::{Deserialize, Serialize};

use crate::household_authority::{
    authorize_household_action, HouseholdAuthorityAction, HouseholdAuthorityInput,
    HouseholdAuthorizationState,
};

/// Fresh authority presented at the local platform-key boundary.
///
/// The identity fields are deliberately separate from the persisted credential:
/// the caller must obtain them again from current authority state before each
/// unseal, and they must match the subject/device recorded when the key was
/// sealed.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentParentDeviceTrustAuthorityInput {
    pub household_authority_input: HouseholdAuthorityInput,
    pub trust_subject: String,
    pub device_ref: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CurrentParentDeviceTrustAuthorityError {
    NotTrusted,
    DeviceBindingMismatch,
}

impl fmt::Debug for CurrentParentDeviceTrustAuthorityInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CurrentParentDeviceTrustAuthorityInput")
            .field("household_authority_input", &self.household_authority_input)
            .field("trust_subject", &"[redacted]")
            .field("device_ref", &"[redacted]")
            .finish()
    }
}

pub fn current_parent_device_trust_authority(
    input: HouseholdAuthorityInput,
) -> Result<(), CurrentParentDeviceTrustAuthorityError> {
    let is_sealing_action = input.action == HouseholdAuthorityAction::SealParentDeviceTrust;
    let decision = authorize_household_action(input);
    (is_sealing_action && decision.authorization_state == HouseholdAuthorizationState::Authorized)
        .then_some(())
        .ok_or(CurrentParentDeviceTrustAuthorityError::NotTrusted)
}

pub fn current_parent_device_trust_authority_for_sealed_device(
    input: &CurrentParentDeviceTrustAuthorityInput,
    sealed_trust_subject: &str,
    sealed_device_ref: &str,
) -> Result<(), CurrentParentDeviceTrustAuthorityError> {
    current_parent_device_trust_authority(input.household_authority_input)?;
    [
        non_empty_identity(&input.trust_subject),
        non_empty_identity(&input.device_ref),
        input.trust_subject == sealed_trust_subject,
        input.device_ref == sealed_device_ref,
    ]
    .into_iter()
    .all(std::convert::identity)
    .then_some(())
    .ok_or(CurrentParentDeviceTrustAuthorityError::DeviceBindingMismatch)
}

fn non_empty_identity(value: &str) -> bool {
    !value.trim().is_empty()
}
