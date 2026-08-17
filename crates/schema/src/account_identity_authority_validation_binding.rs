use super::super::{
    AccountIdentityBindingLifecycleState, AccountIdentityBindingRevocationState,
    AccountIdentityBindingValidationError, AccountIdentityCurrentMemberDeviceAuthorityHandoff,
    AccountIdentityInstallState, AccountIdentityMemberAuthorityValidationError,
    AccountIdentityPairingState, ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION,
};

pub(super) fn validate(
    handoff: &AccountIdentityCurrentMemberDeviceAuthorityHandoff,
) -> Result<(), AccountIdentityMemberAuthorityValidationError> {
    (handoff.binding.pairing_state == AccountIdentityPairingState::Paired)
        .then_some(())
        .ok_or(AccountIdentityMemberAuthorityValidationError::PairingNotComplete)?;
    (handoff.binding.install_state == AccountIdentityInstallState::Installed)
        .then_some(())
        .ok_or(AccountIdentityMemberAuthorityValidationError::InstallNotComplete)?;
    (handoff.binding.lifecycle_state == AccountIdentityBindingLifecycleState::Active)
        .then_some(())
        .ok_or(AccountIdentityMemberAuthorityValidationError::LifecycleNotActive)?;
    (handoff.binding.revocation_state == AccountIdentityBindingRevocationState::Active)
        .then_some(())
        .ok_or(AccountIdentityMemberAuthorityValidationError::Revoked)?;
    (handoff.member.authority_generation > 0)
        .then_some(())
        .ok_or(AccountIdentityMemberAuthorityValidationError::ZeroAuthorityGeneration)?;
    (handoff.member.authority_generation <= ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION)
        .then_some(())
        .ok_or(
            AccountIdentityMemberAuthorityValidationError::AuthorityGenerationExceedsSafeInteger,
        )?;
    (handoff.member.authority_generation == handoff.binding.authority_generation)
        .then_some(())
        .ok_or(AccountIdentityMemberAuthorityValidationError::AuthorityGenerationMismatch)?;
    handoff
        .binding
        .validate_shape()
        .map_err(map_binding_validation_error)
}

fn map_binding_validation_error(
    error: AccountIdentityBindingValidationError,
) -> AccountIdentityMemberAuthorityValidationError {
    match error {
        AccountIdentityBindingValidationError::SchemaVersionMismatch => {
            AccountIdentityMemberAuthorityValidationError::SchemaVersionMismatch
        }
        AccountIdentityBindingValidationError::InactiveProviderMapping => {
            AccountIdentityMemberAuthorityValidationError::InactiveProviderMapping
        }
        AccountIdentityBindingValidationError::MappingAccountMismatch => {
            AccountIdentityMemberAuthorityValidationError::MappingAccountMismatch
        }
        AccountIdentityBindingValidationError::ZeroAuthorityGeneration => {
            AccountIdentityMemberAuthorityValidationError::ZeroAuthorityGeneration
        }
        AccountIdentityBindingValidationError::AuthorityGenerationExceedsSafeInteger => {
            AccountIdentityMemberAuthorityValidationError::AuthorityGenerationExceedsSafeInteger
        }
    }
}
