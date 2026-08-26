//! Narrow Account-owned v2 issuer facade.
//!
//! The facade owns the one SQLite connection used for currentness, key
//! registry, receipts, and outbox transitions. The transaction type is
//! intentionally opaque: callers receive capabilities and records, never a
//! raw connection or a caller-mintable authority.

use std::path::Path;

use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};
use rusqlite::{Connection, Transaction, TransactionBehavior};

use crate::account_identity_authority::{
    AccountIdentityCurrentMemberAuthorityProducer, VerifiedAccountIdentityAuthority,
};
use crate::account_identity_authority_producer_v2::AccountIdentityAuthorityProducerV2Error;
use crate::account_identity_authority_repository::{
    AccountIdentityAuthorityRepositoryError, SqliteAccountIdentityAuthorityRepository,
};
use crate::session_lifecycle_custody::SessionLifecyclePolicy;

#[path = "account_identity_authority_issuer_client_reservation.rs"]
pub mod account_identity_authority_issuer_client_reservation;
#[path = "account_identity_authority_issuer_client_types.rs"]
pub mod account_identity_authority_issuer_client_types;
#[path = "account_identity_authority_issuer_client_currentness.rs"]
mod currentness;
#[path = "account_identity_authority_issuer_client_key.rs"]
mod key;

use account_identity_authority_issuer_client_types::{
    AccountIdentityIssuerAccountId, AccountIdentityIssuerHouseholdId, AccountIdentityIssuerV2KeyId,
    AccountIdentityIssuerV2ServiceBindingId,
};
#[path = "account_identity_authority_issuer_client_clock.rs"]
mod clock;
#[path = "account_identity_authority_issuer_client_schema.rs"]
mod schema;
#[path = "account_identity_authority_issuer_client_startup.rs"]
mod startup;
#[path = "account_identity_authority_issuer_client_transaction.rs"]
mod transaction;
#[path = "account_identity_authority_issuer_client_transaction_outbox.rs"]
mod transaction_outbox;

#[derive(Debug)]
pub enum AccountIdentityAuthorityIssuerClientError {
    InvalidPath,
    Unavailable,
    InvalidSchema,
    CurrentnessUnavailable,
    CurrentnessRejected,
    KeyUnavailable,
    InvalidKey,
    InvalidReceipt,
    ReplayDetected,
    ReceiptUnavailable,
    DeliveryUnavailable,
    ClockUnavailable,
    ReservationUnavailable,
    ReservationExpired,
    ManualRequired,
    Producer(AccountIdentityAuthorityProducerV2Error),
}

impl std::fmt::Display for AccountIdentityAuthorityIssuerClientError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("account_identity_authority_issuer_client_error")
    }
}

impl std::error::Error for AccountIdentityAuthorityIssuerClientError {}

impl From<AccountIdentityAuthorityProducerV2Error> for AccountIdentityAuthorityIssuerClientError {
    fn from(error: AccountIdentityAuthorityProducerV2Error) -> Self {
        Self::Producer(error)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccountIdentityAuthorityIssuerStartupState {
    active_key_count: u64,
    pending_outbox_count: u64,
}

pub struct AccountIdentityAuthorityIssuerClient {
    repository: SqliteAccountIdentityAuthorityRepository,
}

pub struct AccountIdentityIssuerCurrentness {
    authority: VerifiedAccountIdentityAuthority,
    account_id: AccountIdentityIssuerAccountId,
    household_id: AccountIdentityIssuerHouseholdId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountIdentityIssuerV2KeyRecord {
    key_id: AccountIdentityIssuerV2KeyId,
    key_generation: u64,
    enrollment_generation: u64,
    public_key: [u8; 65],
    authority_generation: u64,
    service_binding_id: AccountIdentityIssuerV2ServiceBindingId,
}

pub struct AccountIdentityAuthorityIssuerTransaction<'a> {
    transaction: Transaction<'a>,
}

impl AccountIdentityAuthorityIssuerClient {
    pub(crate) fn open(
        path: impl AsRef<Path>,
    ) -> Result<Self, AccountIdentityAuthorityIssuerClientError> {
        let path = path.as_ref();
        validate_path(path)?;
        let repository = SqliteAccountIdentityAuthorityRepository::open_with_session_policy(
            path,
            SessionLifecyclePolicy::production_default(),
        )
        .map_err(map_repository_error)?;
        let client = Self { repository };
        client.initialize_schema()?;
        Ok(client)
    }

    /// Mount the fixed Account-owned database selected by the protected
    /// installer/broker boundary. No caller-supplied path is accepted.
    pub fn mount_account_owned() -> Result<Self, AccountIdentityAuthorityIssuerClientError> {
        // The retained reparse/ACL-verified path and its protected handle are
        // supplied by the broker/Windows custody packet. Until that packet is
        // composed, opening a guessed path would turn an untrusted caller
        // into the authority owner, so this owner mount remains fail-closed.
        Err(AccountIdentityAuthorityIssuerClientError::Unavailable)
    }

    pub fn recover_startup(
        &self,
    ) -> Result<AccountIdentityAuthorityIssuerStartupState, AccountIdentityAuthorityIssuerClientError>
    {
        self.initialize_schema()?;
        let transaction = Transaction::new_unchecked(
            self.repository.account_issuer_connection(),
            TransactionBehavior::Immediate,
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::Unavailable)?;
        let (now, _) = clock::now(&transaction)?;
        transaction::reconcile_issue_reservations(&transaction, now)?;
        transaction_outbox::reconcile_startup(&transaction, now)?;
        transaction
            .commit()
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::Unavailable)?;
        let connection = self.repository.account_issuer_connection();
        let active_key_count = count_connection(
            connection,
            "SELECT COUNT(*) FROM account_identity_issuer_v2_key_registry WHERE key_state = 'active'",
        )?;
        let pending_outbox_count = count_connection(
            connection,
            "SELECT COUNT(*) FROM account_identity_issuer_v2_outbox WHERE delivery_state = 'pending'",
        )?;
        Ok(AccountIdentityAuthorityIssuerStartupState {
            active_key_count,
            pending_outbox_count,
        })
    }

    pub fn resolve_current(
        &self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
    ) -> Result<AccountIdentityIssuerCurrentness, AccountIdentityAuthorityIssuerClientError> {
        let authority = AccountIdentityCurrentMemberAuthorityProducer::new(&self.repository)
            .produce(provider, provider_subject)
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::CurrentnessUnavailable)?;
        let account_id =
            AccountIdentityIssuerAccountId::from_value(authority.account_id().to_string());
        let household_id =
            AccountIdentityIssuerHouseholdId::from_value(authority.household_id().to_string());
        Ok(AccountIdentityIssuerCurrentness {
            authority,
            account_id,
            household_id,
        })
    }

    pub fn begin(
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

    pub fn claim_pending_outbox(
        &mut self,
    ) -> Result<
        Option<account_identity_authority_issuer_client_types::AccountIdentityIssuerOutboxClaim>,
        AccountIdentityAuthorityIssuerClientError,
    > {
        let transaction = Transaction::new_unchecked(
            self.repository.account_issuer_connection(),
            TransactionBehavior::Immediate,
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::Unavailable)?;
        let (now, _) = clock::now(&transaction)?;
        let claim = transaction_outbox::claim_pending(&transaction, now)?;
        transaction
            .commit()
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::Unavailable)?;
        Ok(claim)
    }

    pub fn record_outbox_failure(
        &mut self,
        claim: &account_identity_authority_issuer_client_types::AccountIdentityIssuerOutboxClaim,
        error_code: &str,
        error_digest: Option<&str>,
    ) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
        let transaction = self
            .repository
            .begin_account_issuer_transaction()
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::Unavailable)?;
        let (now, _) = clock::now(&transaction)?;
        transaction_outbox::record_failure(&transaction, claim, error_code, error_digest, now)?;
        transaction
            .commit()
            .map_err(|_| AccountIdentityAuthorityIssuerClientError::Unavailable)
    }

    fn initialize_schema(&self) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
        schema::initialize(self.repository.account_issuer_connection())
    }
}

fn validate_path(path: &Path) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let text = path.to_string_lossy().to_ascii_lowercase();
    if !path.is_absolute()
        || text == ":memory:"
        || text.starts_with("file:")
        || text.contains("mode=memory")
        || text.contains("cache=shared")
        || path.is_dir()
    {
        return Err(AccountIdentityAuthorityIssuerClientError::InvalidPath);
    }
    Ok(())
}

fn count_connection(
    connection: &Connection,
    query: &str,
) -> Result<u64, AccountIdentityAuthorityIssuerClientError> {
    let value: i64 = connection
        .query_row(query, [], |row| row.get(0))
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::Unavailable)?;
    u64::try_from(value).map_err(|_| AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}

fn map_repository_error(
    error: AccountIdentityAuthorityRepositoryError,
) -> AccountIdentityAuthorityIssuerClientError {
    match error {
        AccountIdentityAuthorityRepositoryError::Unavailable
        | AccountIdentityAuthorityRepositoryError::CurrentnessConflict => {
            AccountIdentityAuthorityIssuerClientError::Unavailable
        }
        AccountIdentityAuthorityRepositoryError::InvalidGeneration
        | AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority => {
            AccountIdentityAuthorityIssuerClientError::InvalidSchema
        }
    }
}
