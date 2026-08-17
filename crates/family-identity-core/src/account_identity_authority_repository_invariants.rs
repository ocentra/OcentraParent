use ocentra_schema::account_identity_authority::{
    AccountIdentityCurrentMemberDeviceAuthorityHandoff, AccountIdentityProvider,
    AccountIdentityProviderSubject, ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION,
};

use super::AccountIdentityAuthorityRepositoryError;

pub(super) fn validate_next_handoff(
    provider: &AccountIdentityProvider,
    provider_subject: &AccountIdentityProviderSubject,
    handoff: &AccountIdentityCurrentMemberDeviceAuthorityHandoff,
) -> Result<(), AccountIdentityAuthorityRepositoryError> {
    if &handoff.mapping.provider != provider
        || &handoff.mapping.provider_subject != provider_subject
    {
        return Err(AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority);
    }
    handoff
        .validate_shape()
        .map_err(|_| AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority)?;
    if handoff.member.authority_generation > ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION
        || handoff.member.session_generation > ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION
    {
        return Err(AccountIdentityAuthorityRepositoryError::InvalidGeneration);
    }
    Ok(())
}

pub(super) fn provider_label(provider: &AccountIdentityProvider) -> &'static str {
    match provider {
        AccountIdentityProvider::Authjs => "authjs",
        AccountIdentityProvider::Firebase => "firebase",
    }
}

pub(super) fn to_sql_generation(
    value: u64,
) -> Result<i64, AccountIdentityAuthorityRepositoryError> {
    i64::try_from(value).map_err(|_| AccountIdentityAuthorityRepositoryError::InvalidGeneration)
}

pub(super) fn from_sql_generation(
    value: i64,
) -> Result<u64, AccountIdentityAuthorityRepositoryError> {
    u64::try_from(value).map_err(|_| AccountIdentityAuthorityRepositoryError::InvalidGeneration)
}
