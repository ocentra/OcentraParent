use chrono::{DateTime, Utc};
use ocentra_schema::account_identity_authority::{
    AccountIdentityCurrentMemberDeviceAuthorityHandoff, AccountIdentityMappingStatus,
    AccountIdentitySupportReceiptRevocationState,
};
use rusqlite::{params, OptionalExtension, Transaction};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_mutation_authority::envelope::CanonicalMutationEnvelope;
use crate::account_identity_mutation_authority::protocol::{
    provider_label as canonical_provider_label, role_label, support_revocation_label,
    support_scope_label,
};
use crate::account_identity_mutation_authority::AccountIdentityMutationAuthorityRequest;
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;

pub(super) fn validate_issue_current(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    request: &AccountIdentityMutationAuthorityRequest,
    now: i64,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    super::current_validation::validate_against_current_authority(authority, request, now)?;
    super::super::invite_recovery_repository::authority::ensure_current_authority(
        transaction,
        authority,
        now,
    )
    .map_err(|_error| AccountIdentityMutationAuthorityError::InvalidAuthority)?;
    let authority_json = transaction
        .query_row(
            "SELECT authority_json FROM account_identity_current_authority
             WHERE provider = ?1 AND provider_subject = ?2 LIMIT 1",
            params![
                issue_provider_label(authority),
                authority.provider_subject().as_str()
            ],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|_error| AccountIdentityMutationAuthorityError::RepositoryUnavailable)?
        .ok_or(AccountIdentityMutationAuthorityError::InvalidAuthority)?;
    let current: AccountIdentityCurrentMemberDeviceAuthorityHandoff =
        serde_json::from_str(&authority_json)
            .map_err(|_error| AccountIdentityMutationAuthorityError::InvalidAuthority)?;
    if current.member.support_receipt.as_ref() != authority.support_receipt() {
        return Err(AccountIdentityMutationAuthorityError::InvalidAuthority);
    }
    Ok(())
}

pub(super) fn validate_consumed_current(
    transaction: &Transaction<'_>,
    envelope: &CanonicalMutationEnvelope,
    now: i64,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    let (status, generation, session_id, session_generation, handoff) =
        load_handoff(transaction, envelope)?;
    handoff
        .validate_shape()
        .map_err(|_error| AccountIdentityMutationAuthorityError::InvalidAuthority)?;
    if status != "active"
        || handoff.mapping.status != AccountIdentityMappingStatus::Active
        || generation != envelope.authority_generation
        || session_id != envelope.session_id
        || session_generation != envelope.session_generation
        || !handoff_matches_envelope(&handoff, envelope)
    {
        return Err(AccountIdentityMutationAuthorityError::InvalidAuthority);
    }
    validate_temporal_authority(&handoff, envelope, now)
}

fn load_handoff(
    transaction: &Transaction<'_>,
    envelope: &CanonicalMutationEnvelope,
) -> Result<
    (
        String,
        u64,
        String,
        u64,
        AccountIdentityCurrentMemberDeviceAuthorityHandoff,
    ),
    AccountIdentityMutationAuthorityError,
> {
    let provider = provider_label(&envelope.provider)?;
    let row = transaction
        .query_row(
            "SELECT mapping_status, authority_generation, session_id,
                    session_generation, authority_json
             FROM account_identity_current_authority
             WHERE provider = ?1 AND provider_subject = ?2 LIMIT 1",
            params![provider, envelope.provider_subject],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, String>(4)?,
                ))
            },
        )
        .optional()
        .map_err(|_error| AccountIdentityMutationAuthorityError::RepositoryUnavailable)?
        .ok_or(AccountIdentityMutationAuthorityError::InvalidAuthority)?;
    let handoff = serde_json::from_str(&row.4)
        .map_err(|_error| AccountIdentityMutationAuthorityError::InvalidAuthority)?;
    let generation = u64::try_from(row.1)
        .map_err(|_error| AccountIdentityMutationAuthorityError::InvalidAuthority)?;
    let session_generation = u64::try_from(row.3)
        .map_err(|_error| AccountIdentityMutationAuthorityError::InvalidAuthority)?;
    Ok((row.0, generation, row.2, session_generation, handoff))
}

fn handoff_matches_envelope(
    handoff: &AccountIdentityCurrentMemberDeviceAuthorityHandoff,
    envelope: &CanonicalMutationEnvelope,
) -> bool {
    canonical_provider_label(&handoff.mapping.provider) == envelope.provider
        && handoff.mapping.provider_subject.as_str() == envelope.provider_subject
        && handoff.member.account_id.to_string() == envelope.account_id
        && handoff.member.household_id.to_string() == envelope.household_id
        && handoff.member.member_id.as_str() == envelope.member_id
        && role_label(handoff.member.role) == envelope.role
        && handoff.member.device_id.as_str() == envelope.device_id
        && handoff.binding.child_profile_id.to_string() == envelope.child_profile_id
        && handoff.binding.child_device_id.as_str() == envelope.child_device_id
        && handoff.member.session_id.as_str() == envelope.session_id
        && handoff.member.authority_generation == envelope.authority_generation
        && handoff.binding.authority_generation == envelope.binding_generation
        && support_matches(handoff, envelope)
}

fn support_matches(
    handoff: &AccountIdentityCurrentMemberDeviceAuthorityHandoff,
    envelope: &CanonicalMutationEnvelope,
) -> bool {
    let Some(receipt) = handoff.member.support_receipt.as_ref() else {
        return envelope.support_receipt_id.is_empty();
    };
    receipt.receipt_id.as_str() == envelope.support_receipt_id
        && receipt.provider_subject.as_str() == envelope.support_provider_subject
        && receipt.account_id.to_string() == envelope.support_account_id
        && receipt.member_id.as_str() == envelope.support_member_id
        && receipt.household_id.to_string() == envelope.support_household_id
        && receipt.device_id.as_str() == envelope.support_device_id
        && receipt.child_profile_id.to_string() == envelope.support_child_profile_id
        && receipt.child_device_id.as_str() == envelope.support_child_device_id
        && support_scope_label(receipt.scope) == envelope.support_scope
        && receipt.issuer.as_str() == envelope.support_issuer
        && receipt.issued_at == envelope.support_issued_at
        && receipt.expires_at == envelope.support_expires_at
        && support_revocation_label(receipt.revocation_state) == envelope.support_revocation_state
        && receipt.audit_identity.as_str() == envelope.support_audit_identity
}

fn validate_temporal_authority(
    handoff: &AccountIdentityCurrentMemberDeviceAuthorityHandoff,
    envelope: &CanonicalMutationEnvelope,
    now: i64,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    let session_expires_at = parse_epoch_millis(&handoff.member.session_expires_at)?;
    if session_expires_at <= now {
        return Err(AccountIdentityMutationAuthorityError::AuthorityExpired);
    }
    let Some(receipt) = handoff.member.support_receipt.as_ref() else {
        return Ok(());
    };
    let issued_at = parse_epoch_millis(&receipt.issued_at)?;
    let expires_at = parse_epoch_millis(&receipt.expires_at)?;
    if receipt.revocation_state != AccountIdentitySupportReceiptRevocationState::Active
        || envelope.support_receipt_id.is_empty()
        || issued_at > now
        || expires_at <= now
    {
        return Err(AccountIdentityMutationAuthorityError::InvalidAuthority);
    }
    Ok(())
}

fn parse_epoch_millis(value: &str) -> Result<i64, AccountIdentityMutationAuthorityError> {
    DateTime::parse_from_rfc3339(value)
        .map(|value| value.with_timezone(&Utc).timestamp_millis())
        .map_err(|_error| AccountIdentityMutationAuthorityError::InvalidAuthority)
}

fn provider_label(value: &str) -> Result<&'static str, AccountIdentityMutationAuthorityError> {
    match value {
        "authjs" => Ok("authjs"),
        "firebase" => Ok("firebase"),
        _ => Err(AccountIdentityMutationAuthorityError::InvalidEnvelope),
    }
}

fn issue_provider_label(authority: &VerifiedAccountIdentityAuthority) -> &'static str {
    canonical_provider_label(authority.provider())
}
