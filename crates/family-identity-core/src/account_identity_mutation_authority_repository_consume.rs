use chrono::{DateTime, Utc};
use rusqlite::{Connection, TransactionBehavior};

use crate::account_identity_mutation_authority::parse::parse_wire;
use crate::account_identity_mutation_authority::{
    expected_key_id, payload_digest, AccountIdentityMutationAction,
    AccountIdentityMutationAuthorityCustody, AccountIdentityMutationOutcome,
};
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;

pub(super) fn consume_and_apply(
    connection: &mut Connection,
    wire: &[u8],
    custody: &dyn AccountIdentityMutationAuthorityCustody,
) -> Result<AccountIdentityMutationOutcome, AccountIdentityMutationAuthorityError> {
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| AccountIdentityMutationAuthorityError::RepositoryUnavailable)?;
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
    let (now, _) = super::super::invite_recovery_repository::authority::trusted_now_in_transaction(
        &transaction,
    )
    .map_err(|_| AccountIdentityMutationAuthorityError::ClockUnavailable)?;
    super::effect::purge_expired(&transaction, now)?;
    let digest = payload_digest(&parsed.payload);
    if parsed.envelope.action == AccountIdentityMutationAction::RevokeChildDevice.as_str() {
        return Err(AccountIdentityMutationAuthorityError::StepUpUnavailable);
    }
    if let Some(result) = super::effect::recorded_result(&transaction, &parsed.envelope, &digest)? {
        transaction
            .commit()
            .map_err(|_| AccountIdentityMutationAuthorityError::RepositoryUnavailable)?;
        return Ok(AccountIdentityMutationOutcome::recorded(result));
    }
    let token_expires_at =
        validate_times(&parsed.envelope.issued_at, &parsed.envelope.expires_at, now)?;
    super::current::validate_consumed_current(&transaction, &parsed.envelope, now)?;
    super::target::validate_consumed(&transaction, &parsed.envelope, now)?;
    super::effect::reserve(
        &transaction,
        &parsed.envelope,
        &digest,
        token_expires_at,
        now,
    )?;
    let result = super::apply::apply(&transaction, &parsed.envelope, now)?;
    super::effect::complete(&transaction, &parsed.envelope, result, now)?;
    transaction
        .commit()
        .map_err(|_| AccountIdentityMutationAuthorityError::RepositoryUnavailable)?;
    Ok(AccountIdentityMutationOutcome::committed(result))
}

fn validate_times(
    issued_at: &str,
    expires_at: &str,
    now: i64,
) -> Result<i64, AccountIdentityMutationAuthorityError> {
    let issued_at = DateTime::parse_from_rfc3339(issued_at)
        .map_err(|_| AccountIdentityMutationAuthorityError::InvalidEnvelope)?
        .with_timezone(&Utc);
    let expires_at = DateTime::parse_from_rfc3339(expires_at)
        .map_err(|_| AccountIdentityMutationAuthorityError::InvalidEnvelope)?
        .with_timezone(&Utc);
    crate::account_identity_mutation_authority::validation::validate_lifetime(
        issued_at, expires_at, now,
    )?;
    Ok(expires_at.timestamp_millis())
}
