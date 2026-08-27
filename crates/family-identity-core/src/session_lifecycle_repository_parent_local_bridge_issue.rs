#![forbid(unsafe_code)]

//! Issuance operations for Account-owned parent-local bridge sessions.

use ocentra_schema::account_identity_parent_local_bridge::{
    AccountIdentityParentLocalBridgeAudience, AccountIdentityParentLocalBridgeHandshake,
    ACCOUNT_IDENTITY_PARENT_LOCAL_BRIDGE_SCHEMA_VERSION,
};
use rusqlite::{params, Transaction, TransactionBehavior};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::session_lifecycle_custody::parent_local_bridge::{
    connection_nonce_digest, issue_connection_nonce, IssuedParentLocalBridgeSession,
    ParentLocalBridgeSessionCapability, CAPABILITY_DIGEST_DOMAIN,
};

use super::super::{authority, clock, codec, labels, SessionLifecycleRepositoryError};
use super::{audit, ParentLocalBridgeState, StoredParentLocalBridgeSession, DIGEST_ALGORITHM};

impl super::super::SqliteAccountIdentityAuthorityRepository {
    /// Issue one short-lived, one-connection parent-local capability from an
    /// exact current Account authority. No provider or caller identity fields
    /// enter the row; all binding values come from the repository re-read.
    pub fn issue_parent_local_bridge_session(
        &mut self,
        current_authority: &VerifiedAccountIdentityAuthority,
    ) -> Result<IssuedParentLocalBridgeSession, SessionLifecycleRepositoryError> {
        let capability = ParentLocalBridgeSessionCapability::issue()
            .map_err(|_| SessionLifecycleRepositoryError::EntropyUnavailable)?;
        let connection_nonce = issue_connection_nonce()
            .map_err(|_| SessionLifecycleRepositoryError::EntropyUnavailable)?;
        let bridge_ttl_millis = self.session_policy.freshness_ttl_millis;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        let now_epoch_millis = clock::trusted_now_in_transaction(&transaction)?;
        let binding = authority::parent_local_bridge_binding_from_verified(
            &transaction,
            current_authority,
            now_epoch_millis,
        )?;
        let current_epoch =
            super::storage::current_bridge_revoke_epoch(&transaction, &binding.account_id)?;
        let expires_at_epoch_millis = now_epoch_millis
            .checked_add(bridge_ttl_millis)
            .ok_or(SessionLifecycleRepositoryError::InvalidTransition)?
            .min(binding.authority_expires_at_epoch_millis);
        if expires_at_epoch_millis <= now_epoch_millis {
            return Err(SessionLifecycleRepositoryError::AuthorityExpired);
        }
        let connection_nonce_digest = connection_nonce_digest(&connection_nonce)
            .ok_or(SessionLifecycleRepositoryError::InvalidParentLocalBridgeHandshake)?;
        let capability_wire = capability.expose_secret().to_owned();
        let record = StoredParentLocalBridgeSession {
            capability_digest: capability.digest(),
            audience: AccountIdentityParentLocalBridgeAudience::fixed(),
            connection_nonce_digest,
            binding,
            issued_at_epoch_millis: now_epoch_millis,
            expires_at_epoch_millis,
            bridge_revoke_epoch: current_epoch,
            state: ParentLocalBridgeState::Active,
            last_transition_at_epoch_millis: now_epoch_millis,
        };
        insert_record(&transaction, &record)?;
        audit::insert_session_event(&transaction, &record, "issued", now_epoch_millis)?;
        audit::cleanup(&transaction, &record.binding.account_id, now_epoch_millis)?;
        transaction
            .commit()
            .map_err(|_| SessionLifecycleRepositoryError::Unavailable)?;
        Ok(IssuedParentLocalBridgeSession::new(
            capability,
            AccountIdentityParentLocalBridgeHandshake {
                schema_version: ACCOUNT_IDENTITY_PARENT_LOCAL_BRIDGE_SCHEMA_VERSION,
                capability: capability_wire,
                audience: AccountIdentityParentLocalBridgeAudience::fixed(),
                connection_nonce,
            },
            expires_at_epoch_millis,
        ))
    }
}

pub(super) fn insert_record(
    transaction: &Transaction<'_>,
    record: &StoredParentLocalBridgeSession,
) -> Result<(), SessionLifecycleRepositoryError> {
    let changed = transaction
        .execute(
            "INSERT INTO account_identity_parent_local_bridge_session (
                 capability_digest, digest_algorithm, capability_digest_domain,
                 audience, connection_nonce_digest, account_id, provider,
                 provider_subject, household_id, member_id, device_id,
                 authority_session_id, authority_session_generation,
                 authority_generation, authority_expires_at_epoch_millis,
                 issued_at_epoch_millis, expires_at_epoch_millis,
                 bridge_revoke_epoch, state, last_transition_at_epoch_millis
             ) VALUES (
                 ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                 ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20
             )",
            params![
                record.capability_digest,
                DIGEST_ALGORITHM,
                CAPABILITY_DIGEST_DOMAIN,
                record.audience.as_str(),
                record.connection_nonce_digest,
                record.binding.account_id.to_string(),
                labels::provider_label(&record.binding.provider).0,
                record.binding.provider_subject.as_str(),
                record.binding.household_id.to_string(),
                record.binding.member_id.as_str(),
                record.binding.device_id.as_str(),
                record.binding.authority_session_id.as_str(),
                codec::to_sql_generation(record.binding.authority_session_generation)?,
                codec::to_sql_generation(record.binding.authority_generation)?,
                record.binding.authority_expires_at_epoch_millis,
                record.issued_at_epoch_millis,
                record.expires_at_epoch_millis,
                codec::to_sql_generation(record.bridge_revoke_epoch)?,
                state_label(record.state),
                record.last_transition_at_epoch_millis,
            ],
        )
        .map_err(|_| SessionLifecycleRepositoryError::CurrentnessConflict)?;
    (changed == 1)
        .then_some(())
        .ok_or(SessionLifecycleRepositoryError::CurrentnessConflict)
}

pub(super) fn state_label(state: ParentLocalBridgeState) -> &'static str {
    match state {
        ParentLocalBridgeState::Active => "active",
        ParentLocalBridgeState::Consumed => "consumed",
        ParentLocalBridgeState::Revoked => "revoked",
    }
}
