#[path = "account_identity_authority_issuer_client_api_issue.rs"]
mod issue;
#[path = "account_identity_authority_issuer_client_api_state.rs"]
mod state;

use rusqlite::{Transaction, TransactionBehavior};

use super::{
    AccountIdentityAuthorityIssuerClient, AccountIdentityAuthorityIssuerClientError,
    AccountIdentityAuthorityIssuerTransaction,
};

impl AccountIdentityAuthorityIssuerClient {
    fn begin_transaction(
        &mut self,
    ) -> Result<
        AccountIdentityAuthorityIssuerTransaction<'_>,
        AccountIdentityAuthorityIssuerClientError,
    > {
        let transaction = Transaction::new_unchecked(
            self.repository.account_issuer_connection(),
            TransactionBehavior::Immediate,
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::Unavailable)?;
        Ok(AccountIdentityAuthorityIssuerTransaction { transaction })
    }
}
