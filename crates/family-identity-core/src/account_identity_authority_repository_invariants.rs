use ocentra_schema::account_identity_authority::AccountIdentityProvider;

use super::AccountIdentityAuthorityRepositoryError;

pub(super) fn provider_label(provider: &AccountIdentityProvider) -> &'static str {
    match provider {
        AccountIdentityProvider::Authjs => "authjs",
        AccountIdentityProvider::Firebase => "firebase",
    }
}

pub(super) fn from_sql_generation(
    value: i64,
) -> Result<u64, AccountIdentityAuthorityRepositoryError> {
    u64::try_from(value)
        .map_err(|_error| AccountIdentityAuthorityRepositoryError::InvalidGeneration)
}
