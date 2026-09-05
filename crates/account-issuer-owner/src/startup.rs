//! Startup/recovery state for the Account-owned repository.

use crate::repository::{AccountIssuerRepository, AccountIssuerRepositoryError};

pub fn recover(
    repository: &AccountIssuerRepository,
) -> Result<ocentra_family_identity_core::account_identity_authority_issuer_client::AccountIdentityAuthorityIssuerStartupState, AccountIssuerRepositoryError>
{
    repository.recover_startup()
}
