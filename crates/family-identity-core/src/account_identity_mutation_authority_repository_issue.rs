use chrono::{Duration, SecondsFormat, TimeZone, Utc};
use rusqlite::{Connection, TransactionBehavior};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_mutation_authority::envelope::{encode, from_resolved};
use crate::account_identity_mutation_authority::{
    expected_key_id, AccountIdentityMutationAuthority, AccountIdentityMutationAuthorityCustody,
    AccountIdentityMutationAuthorityRequest,
};
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;

pub(super) fn issue(
    connection: &mut Connection,
    authority: &VerifiedAccountIdentityAuthority,
    request: &AccountIdentityMutationAuthorityRequest,
    custody: &dyn AccountIdentityMutationAuthorityCustody,
) -> Result<AccountIdentityMutationAuthority, AccountIdentityMutationAuthorityError> {
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_error| AccountIdentityMutationAuthorityError::RepositoryUnavailable)?;
    let (now, issued_at) =
        super::super::invite_recovery_repository::authority::trusted_now_in_transaction(
            &transaction,
        )
        .map_err(|_error| AccountIdentityMutationAuthorityError::ClockUnavailable)?;
    super::current::validate_issue_current(&transaction, authority, request, now)?;
    let target = super::target::resolve_request(&transaction, authority, request, now)?;
    let key_id = custody.signing_key_id();
    let verifying_key = custody.verification_key(key_id)?;
    if expected_key_id(&verifying_key) != key_id {
        return Err(AccountIdentityMutationAuthorityError::VerificationKeyUnavailable);
    }
    let expires_at = Utc
        .timestamp_millis_opt(now)
        .single()
        .ok_or(AccountIdentityMutationAuthorityError::ClockUnavailable)?
        + Duration::minutes(5);
    let expires_at = expires_at.to_rfc3339_opts(SecondsFormat::Millis, true);
    let envelope = from_resolved(key_id, authority, request, &target, &issued_at, &expires_at);
    let payload = encode(&envelope)?;
    let signature = custody.sign(&payload)?;
    verifying_key
        .verify_strict(&payload, &ed25519_dalek::Signature::from_bytes(&signature))
        .map_err(|_error| AccountIdentityMutationAuthorityError::SignatureInvalid)?;
    transaction
        .commit()
        .map_err(|_error| AccountIdentityMutationAuthorityError::RepositoryUnavailable)?;
    AccountIdentityMutationAuthority::from_signed_parts(payload, signature)
}
