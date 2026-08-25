use std::path::Path;
use std::time::Duration;

use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};
use rusqlite::{Connection, Transaction, TransactionBehavior};

use crate::account_identity_authority::{
    AccountIdentityCurrentMemberAuthorityProducer, VerifiedAccountIdentityAuthority,
};
use crate::account_identity_authority_producer::AccountIdentityAuthorityProducerCustody;
use crate::account_identity_authority_producer::AccountIdentityAuthorityProducerTransport;
use crate::account_identity_authority_producer_error::AccountIdentityAuthorityProducerError;
use crate::account_identity_mutation_authority::{
    AccountIdentityMutationAuthority, AccountIdentityMutationAuthorityCustody,
    AccountIdentityMutationAuthorityRequest, AccountIdentityMutationOutcome,
};
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;
use crate::session_lifecycle_custody::SessionLifecyclePolicy;

#[path = "account_identity_authority_repository_cas.rs"]
mod account_identity_authority_repository_cas;
#[path = "account_identity_authority_repository_invariants.rs"]
mod account_identity_authority_repository_invariants;
#[path = "account_identity_authority_repository_read.rs"]
mod account_identity_authority_repository_read;
#[path = "account_identity_authority_repository_schema.rs"]
mod account_identity_authority_repository_schema;
#[path = "account_identity_authority_service_error.rs"]
mod account_identity_authority_service_error;
#[path = "account_identity_mutation_authority_repository.rs"]
mod account_identity_mutation_authority_repository;
#[path = "invite_recovery_repository.rs"]
pub mod invite_recovery_repository;
#[path = "session_lifecycle_repository.rs"]
pub mod session_lifecycle_repository;

#[derive(Debug)]
pub enum AccountIdentityAuthorityRepositoryError {
    Unavailable,
    InvalidGeneration,
    InvalidStoredAuthority,
    CurrentnessConflict,
}

/// Durable Account-owned currentness store. The JSON column is retained as a
/// canonical DTO/evidence snapshot, while the key, authority generation,
/// session identity, and session generation are independently guarded by SQL.
pub struct SqliteAccountIdentityAuthorityRepository {
    connection: Connection,
    session_policy: SessionLifecyclePolicy,
}

impl SqliteAccountIdentityAuthorityRepository {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, AccountIdentityAuthorityRepositoryError> {
        Self::open_with_session_policy(path, SessionLifecyclePolicy::production_default())
    }

    pub fn open_with_session_policy(
        path: impl AsRef<Path>,
        session_policy: SessionLifecyclePolicy,
    ) -> Result<Self, AccountIdentityAuthorityRepositoryError> {
        let connection = Connection::open(path)
            .map_err(|_| AccountIdentityAuthorityRepositoryError::Unavailable)?;
        Self::from_owned_connection(connection, session_policy)
    }

    /// Finish Account repository initialization on a connection opened by an
    /// Account-owned protected store boundary. This keeps authority state and
    /// issuer state in one SQLite locking domain without reopening the path.
    pub(crate) fn from_owned_connection(
        connection: Connection,
        session_policy: SessionLifecyclePolicy,
    ) -> Result<Self, AccountIdentityAuthorityRepositoryError> {
        connection
            .busy_timeout(Duration::from_secs(5))
            .map_err(|_| AccountIdentityAuthorityRepositoryError::Unavailable)?;
        connection
            .execute_batch(
                "PRAGMA foreign_keys = ON;
                 PRAGMA journal_mode = DELETE;
                 PRAGMA synchronous = FULL;
                 CREATE TABLE IF NOT EXISTS account_identity_current_authority (
                    provider TEXT NOT NULL CHECK (provider IN ('authjs','firebase')),
                    provider_subject TEXT NOT NULL,
                    mapping_status TEXT NOT NULL CHECK (mapping_status IN ('active','revoked')),
                    authority_generation INTEGER NOT NULL CHECK (
                        authority_generation > 0 AND authority_generation <= 9007199254740991
                    ),
                    session_id TEXT NOT NULL CHECK (length(session_id) > 0),
                    session_generation INTEGER NOT NULL CHECK (
                        session_generation > 0 AND session_generation <= 9007199254740991
                    ),
                    authority_json TEXT NOT NULL CHECK (length(authority_json) > 0),
                    PRIMARY KEY (provider, provider_subject)
                 ) STRICT;",
            )
            .map_err(|_| AccountIdentityAuthorityRepositoryError::Unavailable)?;
        account_identity_authority_repository_schema::validate(&connection)?;
        connection
            .execute_batch(session_lifecycle_repository::SESSION_SCHEMA_SQL)
            .map_err(|_| AccountIdentityAuthorityRepositoryError::Unavailable)?;
        connection
            .execute_batch(invite_recovery_repository::INVITE_RECOVERY_SCHEMA_SQL)
            .map_err(|_| AccountIdentityAuthorityRepositoryError::Unavailable)?;
        invite_recovery_repository::validate_schema(&connection)
            .map_err(|_| AccountIdentityAuthorityRepositoryError::Unavailable)?;
        Ok(Self {
            connection,
            session_policy,
        })
    }

    /// Begin the Account-owned issuer transition on the same connection that
    /// owns current authority. `BEGIN IMMEDIATE` prevents a competing Account
    /// CAS from committing between exact currentness resolution and the issuer
    /// mutation/receipt commit.
    pub(crate) fn begin_account_issuer_transaction(
        &mut self,
    ) -> Result<Transaction<'_>, AccountIdentityAuthorityRepositoryError> {
        self.connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| AccountIdentityAuthorityRepositoryError::Unavailable)
    }

    pub(crate) fn account_issuer_connection(&self) -> &Connection {
        &self.connection
    }
}

/// Reachable family-owned producer/composition seam. External adapters can
/// resolve an opaque capability by the provider-verified subject, but cannot
/// construct one from a serialized handoff or caller-selected target.
pub struct AccountIdentityAuthorityService {
    repository: SqliteAccountIdentityAuthorityRepository,
    mutation_custody: Option<Box<dyn AccountIdentityMutationAuthorityCustody>>,
    authority_producer_custody: Option<Box<dyn AccountIdentityAuthorityProducerCustody>>,
}

impl AccountIdentityAuthorityService {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, AccountIdentityAuthorityRepositoryError> {
        Ok(Self {
            repository: SqliteAccountIdentityAuthorityRepository::open(path)?,
            mutation_custody: None,
            authority_producer_custody: None,
        })
    }

    pub fn open_with_session_policy(
        path: impl AsRef<Path>,
        session_policy: SessionLifecyclePolicy,
    ) -> Result<Self, AccountIdentityAuthorityRepositoryError> {
        Ok(Self {
            repository: SqliteAccountIdentityAuthorityRepository::open_with_session_policy(
                path,
                session_policy,
            )?,
            mutation_custody: None,
            authority_producer_custody: None,
        })
    }

    pub fn resolve_current(
        &self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
    ) -> Result<VerifiedAccountIdentityAuthority, AccountIdentityAuthorityServiceError> {
        AccountIdentityCurrentMemberAuthorityProducer::new(&self.repository)
            .produce(provider, provider_subject)
            .map_err(AccountIdentityAuthorityServiceError::from)
    }

    /// Issue a short-lived Account-owned current-authority transport. The
    /// provider subject is only a lookup key; every signed authority field is
    /// copied from the opaque, repository-validated capability.
    pub(crate) fn issue_current_authority_producer(
        &self,
        provider: &AccountIdentityProvider,
        provider_subject: &AccountIdentityProviderSubject,
    ) -> Result<AccountIdentityAuthorityProducerTransport, AccountIdentityAuthorityProducerError>
    {
        let custody = self
            .authority_producer_custody
            .as_deref()
            .ok_or(AccountIdentityAuthorityProducerError::SignerCustodyUnavailable)?;
        let authority = self
            .resolve_current(provider, provider_subject)
            .map_err(AccountIdentityAuthorityProducerError::Authority)?;
        crate::account_identity_authority_producer::issue(&authority, custody)
    }

    /// Crate-private installation seam for the future durable signer/key
    /// registry. No public constructor can inject process or caller custody.
    pub(crate) fn set_authority_producer_custody(
        &mut self,
        custody: Box<dyn AccountIdentityAuthorityProducerCustody>,
    ) {
        self.authority_producer_custody = Some(custody);
    }

    /// Issue a short-lived mutation transport only after the provider subject
    /// resolves to the current durable Account authority. The request carries
    /// a target and idempotency key, never authority facts; the issuer binds
    /// both to the opaque current binding before signing.
    pub fn issue_mutation_authority(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        request: &AccountIdentityMutationAuthorityRequest,
    ) -> Result<AccountIdentityMutationAuthority, AccountIdentityMutationAuthorityServiceError>
    {
        let custody = self.mutation_custody.as_deref().ok_or(
            AccountIdentityMutationAuthorityServiceError::Mutation(
                AccountIdentityMutationAuthorityError::SignerCustodyUnavailable,
            ),
        )?;
        self.repository
            .issue_mutation_authority(authority, request, custody)
            .map_err(AccountIdentityMutationAuthorityServiceError::Mutation)
    }

    /// Verify and apply an Account-owned mutation atomically. The method never
    /// returns a detachable authority token: it returns only the durable
    /// result committed with the mutation and idempotency record.
    pub fn consume_and_apply_mutation_authority(
        &mut self,
        wire: &[u8],
    ) -> Result<AccountIdentityMutationOutcome, AccountIdentityMutationAuthorityServiceError> {
        let custody = self.mutation_custody.as_deref().ok_or(
            AccountIdentityMutationAuthorityServiceError::Mutation(
                AccountIdentityMutationAuthorityError::VerificationKeyUnavailable,
            ),
        )?;
        self.repository
            .consume_and_apply_mutation_authority(wire, custody)
            .map_err(AccountIdentityMutationAuthorityServiceError::Mutation)
    }

    pub fn approve_recovery(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        recovery_id: &crate::family_identity::RecoveryId,
    ) -> Result<(), invite_recovery_repository::InviteRecoveryRepositoryError> {
        self.repository.approve_recovery(authority, recovery_id)
    }

    pub fn complete_recovery(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        recovery_id: &crate::family_identity::RecoveryId,
    ) -> Result<
        invite_recovery_repository::RecoveryCompletion,
        invite_recovery_repository::InviteRecoveryRepositoryError,
    > {
        self.repository.complete_recovery(authority, recovery_id)
    }

    pub fn claim_recovery_handoff(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
    ) -> Result<
        Option<invite_recovery_repository::RecoveryHandoffDeliveryAttempt>,
        invite_recovery_repository::InviteRecoveryRepositoryError,
    > {
        self.repository.claim_recovery_handoff(authority)
    }

    pub fn release_recovery_handoff(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        attempt: &invite_recovery_repository::RecoveryHandoffDeliveryAttempt,
    ) -> Result<(), invite_recovery_repository::InviteRecoveryRepositoryError> {
        self.repository.release_recovery_handoff(authority, attempt)
    }
}

#[derive(Debug)]
pub enum AccountIdentityAuthorityServiceError {
    Repository(AccountIdentityAuthorityRepositoryError),
    Missing,
    InvalidAuthority,
}

#[derive(Debug)]
pub enum AccountIdentityMutationAuthorityServiceError {
    Authority(AccountIdentityAuthorityServiceError),
    Mutation(AccountIdentityMutationAuthorityError),
}
