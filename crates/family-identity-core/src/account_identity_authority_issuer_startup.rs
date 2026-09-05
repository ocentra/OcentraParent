use rusqlite::Connection;

use super::key_registry as account_identity_authority_issuer_key_registry;
use crate::account_identity_authority_issuer::AccountIdentityIssuerError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccountIdentityIssuerStartupState {
    active_key_count: u64,
}

impl AccountIdentityIssuerStartupState {
    pub fn active_key_count(&self) -> u64 {
        self.active_key_count
    }
}

pub(crate) fn recover(
    connection: &Connection,
) -> Result<AccountIdentityIssuerStartupState, AccountIdentityIssuerError> {
    account_identity_authority_issuer_key_registry::ensure_schema(connection)?;
    let active_key_count =
        account_identity_authority_issuer_key_registry::validate_durable_state(connection)?;
    Ok(AccountIdentityIssuerStartupState { active_key_count })
}
