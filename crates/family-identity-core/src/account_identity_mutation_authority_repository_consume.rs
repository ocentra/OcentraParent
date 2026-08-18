use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension, Transaction, TransactionBehavior};

use crate::account_identity_mutation_authority::parse::parse_wire;
use crate::account_identity_mutation_authority::{
    expected_key_id, payload_digest, AccountIdentityMutationAction,
    AccountIdentityMutationAuthorityCustody, VerifiedAccountIdentityMutationAuthority,
};
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;

pub(super) fn consume(
    connection: &mut Connection,
    wire: &[u8],
    custody: &dyn AccountIdentityMutationAuthorityCustody,
) -> Result<VerifiedAccountIdentityMutationAuthority, AccountIdentityMutationAuthorityError> {
    let parsed = parse_wire(wire)?;
    let verifying_key = custody.verification_key(&parsed.envelope.key_id)?;
    if expected_key_id(&verifying_key) != parsed.envelope.key_id {
        return Err(AccountIdentityMutationAuthorityError::VerificationKeyUnavailable);
    }
    verifying_key
        .verify_strict(
            &parsed.payload,
            &ed25519_dalek::Signature::from_bytes(&parsed.signature),
        )
        .map_err(|_| AccountIdentityMutationAuthorityError::SignatureInvalid)?;
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| AccountIdentityMutationAuthorityError::RepositoryUnavailable)?;
    let (now, _) = super::super::invite_recovery_repository::authority::trusted_now_in_transaction(
        &transaction,
    )
    .map_err(|_| AccountIdentityMutationAuthorityError::ClockUnavailable)?;
    validate_times(&parsed.envelope.issued_at, &parsed.envelope.expires_at, now)?;
    super::current::validate_consumed_current(&transaction, &parsed.envelope, now)?;
    super::target::validate_consumed(&transaction, &parsed.envelope, now)?;
    let digest = payload_digest(&parsed.payload);
    reserve_replay(
        &transaction,
        &digest,
        &parsed.envelope.idempotency_key,
        &parsed.envelope.key_id,
        now,
    )?;
    transaction
        .commit()
        .map_err(|_| AccountIdentityMutationAuthorityError::RepositoryUnavailable)?;
    let action = AccountIdentityMutationAction::parse(&parsed.envelope.action)
        .ok_or(AccountIdentityMutationAuthorityError::InvalidEnvelope)?;
    Ok(VerifiedAccountIdentityMutationAuthority::new(
        action,
        parsed.envelope.target_id,
        parsed.envelope.idempotency_key,
        digest,
    ))
}

fn validate_times(
    issued_at: &str,
    expires_at: &str,
    now: i64,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    let issued_at = DateTime::parse_from_rfc3339(issued_at)
        .map_err(|_| AccountIdentityMutationAuthorityError::InvalidEnvelope)?
        .with_timezone(&Utc);
    let expires_at = DateTime::parse_from_rfc3339(expires_at)
        .map_err(|_| AccountIdentityMutationAuthorityError::InvalidEnvelope)?
        .with_timezone(&Utc);
    crate::account_identity_mutation_authority::validation::validate_lifetime(
        issued_at, expires_at, now,
    )
}

fn reserve_replay(
    transaction: &Transaction<'_>,
    digest: &str,
    idempotency_key: &str,
    key_id: &str,
    now: i64,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    let existing = transaction
        .query_row(
            "SELECT payload_digest, idempotency_key
             FROM account_identity_mutation_authority_replay
             WHERE payload_digest = ?1 OR idempotency_key = ?2 LIMIT 1",
            params![digest, idempotency_key],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
        .optional()
        .map_err(|_| AccountIdentityMutationAuthorityError::RepositoryUnavailable)?;
    if let Some((existing_digest, _)) = existing {
        return Err(if existing_digest == digest {
            AccountIdentityMutationAuthorityError::ReplayDetected
        } else {
            AccountIdentityMutationAuthorityError::IdempotencyConflict
        });
    }
    transaction
        .execute(
            "INSERT INTO account_identity_mutation_authority_replay
             (payload_digest, idempotency_key, key_id, consumed_at_epoch_millis)
             VALUES (?1, ?2, ?3, ?4)",
            params![digest, idempotency_key, key_id, now],
        )
        .map_err(|_| AccountIdentityMutationAuthorityError::RepositoryUnavailable)?;
    Ok(())
}
