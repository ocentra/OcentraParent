//! Explicit restart recovery boundary.

use crate::repository::{AccountIssuerRepository, AccountIssuerRepositoryError};

pub fn validate_after_restart(
    repository: &AccountIssuerRepository,
) -> Result<ocentra_family_identity_core::account_identity_authority_issuer_client::AccountIdentityAuthorityIssuerStartupState, AccountIssuerRepositoryError>
{
    repository.recover_startup()
}
