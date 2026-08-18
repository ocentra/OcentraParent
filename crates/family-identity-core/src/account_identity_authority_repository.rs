use std::path::Path;
use std::time::Duration;

use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};
use rusqlite::Connection;

use crate::account_identity_authority::{
    AccountIdentityAuthorityRepository, AccountIdentityCurrentMemberAuthorityProducer,
    VerifiedAccountIdentityAuthority,
};
use crate::session_lifecycle_custody::SessionLifecyclePolicy;

#[path = "account_identity_authority_repository_cas.rs"]
mod account_identity_authority_repository_cas;
#[path = "account_identity_authority_repository_invariants.rs"]
mod account_identity_authority_repository_invariants;
#[path = "account_identity_authority_repository_read.rs"]
mod account_identity_authority_repository_read;
#[path = "account_identity_authority_service_error.rs"]
mod account_identity_authority_service_error;
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
}

/// Reachable family-owned producer/composition seam. External adapters can
/// resolve an opaque capability by the provider-verified subject, but cannot
/// construct one from a serialized handoff or caller-selected target.
pub struct AccountIdentityAuthorityService {
    repository: SqliteAccountIdentityAuthorityRepository,
}

impl AccountIdentityAuthorityService {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, AccountIdentityAuthorityRepositoryError> {
        Ok(Self {
            repository: SqliteAccountIdentityAuthorityRepository::open(path)?,
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

    pub fn issue_setup_invite(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        purpose: crate::setup_lifecycle::SetupInvitePurpose,
        target_role: crate::setup_lifecycle::SetupInviteTargetRole,
        recipient: &invite_recovery_repository::VerifiedInviteRecipient,
        ttl: Duration,
    ) -> Result<
        invite_recovery_repository::IssuedSetupInvite,
        invite_recovery_repository::InviteRecoveryRepositoryError,
    > {
        self.repository
            .issue_setup_invite(authority, purpose, target_role, recipient, ttl)
    }

    pub fn redeem_setup_invite(
        &mut self,
        recipient: &invite_recovery_repository::VerifiedInviteRecipient,
        code: invite_recovery_repository::SetupInviteCode,
    ) -> Result<
        invite_recovery_repository::RedeemedSetupInvite,
        invite_recovery_repository::InviteRecoveryRepositoryError,
    > {
        self.repository.redeem_setup_invite(recipient, code)
    }

    pub fn revoke_setup_invite(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        invite_id: &crate::family_identity::SetupInviteId,
    ) -> Result<(), invite_recovery_repository::InviteRecoveryRepositoryError> {
        self.repository.revoke_setup_invite(authority, invite_id)
    }

    pub fn begin_recovery(
        &mut self,
        proof: &invite_recovery_repository::VerifiedRecoveryIdentityProof,
        support_authorization: Option<
            &invite_recovery_repository::VerifiedSupportRecoveryAuthorization,
        >,
    ) -> Result<
        crate::family_identity::RecoveryId,
        invite_recovery_repository::InviteRecoveryRepositoryError,
    > {
        self.repository.begin_recovery(proof, support_authorization)
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

    pub fn revoke_recovery(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        recovery_id: &crate::family_identity::RecoveryId,
    ) -> Result<(), invite_recovery_repository::InviteRecoveryRepositoryError> {
        self.repository.revoke_recovery(authority, recovery_id)
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

    pub fn acknowledge_recovery_handoff(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        attempt: &invite_recovery_repository::RecoveryHandoffDeliveryAttempt,
        receipt: &invite_recovery_repository::RecoveryCustodyDeliveryReceipt,
    ) -> Result<(), invite_recovery_repository::InviteRecoveryRepositoryError> {
        self.repository
            .acknowledge_recovery_handoff(authority, attempt, receipt)
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
