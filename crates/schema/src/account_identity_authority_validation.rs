use super::{
    AccountIdentityCurrentMemberDeviceAuthorityHandoff,
    AccountIdentityMemberAuthorityValidationError,
};

#[path = "account_identity_authority_validation_binding.rs"]
mod binding;
#[path = "account_identity_authority_validation_identity.rs"]
mod identity;
#[path = "account_identity_authority_validation_state.rs"]
mod state;

pub(super) fn validate_shape(
    handoff: &AccountIdentityCurrentMemberDeviceAuthorityHandoff,
) -> Result<(), AccountIdentityMemberAuthorityValidationError> {
    identity::validate(handoff)?;
    state::validate(handoff)?;
    binding::validate(handoff)
}
