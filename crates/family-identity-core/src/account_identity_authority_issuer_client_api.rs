#[path = "account_identity_authority_issuer_client_api_issue_legacy.rs"]
mod issue_legacy;
#[path = "account_identity_authority_issuer_client_api_issue_signer_failure.rs"]
mod issue_signer_failure;
#[path = "account_identity_authority_issuer_client_api_issue_signer_flow.rs"]
mod issue_signer_flow;
#[path = "account_identity_authority_issuer_client_api_issue_signer_transitions.rs"]
mod issue_signer_transitions;
#[path = "account_identity_authority_issuer_client_api_state.rs"]
mod state;

use rusqlite::{Transaction, TransactionBehavior};

use super::{
    AccountIdentityAuthorityIssuerClient, AccountIdentityAuthorityIssuerClientError,
    AccountIdentityAuthorityIssuerTransaction,
};

impl AccountIdentityAuthorityIssuerClient {
    fn begin_transaction(
        &self,
    ) -> Result<
        AccountIdentityAuthorityIssuerTransaction<'_>,
        AccountIdentityAuthorityIssuerClientError,
    > {
        let transaction = Transaction::new_unchecked(
            self.repository.account_issuer_connection(),
            TransactionBehavior::Immediate,
        )
        .map_err(|_error| AccountIdentityAuthorityIssuerClientError::Unavailable)?;
        Ok(AccountIdentityAuthorityIssuerTransaction { transaction })
    }
}
