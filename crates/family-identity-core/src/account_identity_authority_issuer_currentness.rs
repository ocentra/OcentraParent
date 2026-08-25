use chrono::{DateTime, Utc};
use ocentra_schema::account_identity_authority::{
    AccountIdentityCurrentMemberDeviceAuthorityHandoff, AccountIdentityMappingStatus,
    AccountIdentityProvider, AccountIdentityRole, AccountIdentitySessionFreshnessState,
    AccountIdentitySupportReceiptRevocationState,
};
use rusqlite::{params, OptionalExtension, Transaction};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_issuer::AccountIdentityIssuerError;

use super::service_binding::{
    AccountIdentityIssuerAuthenticatedBinding, AccountIdentityIssuerService,
    AccountIdentityIssuerServiceBinding, AccountIdentityIssuerServiceBindingAuthenticator,
};

/// Resolve the durable Account row while the caller holds the same SQLite
/// `BEGIN IMMEDIATE` transaction used for the issuer transition. Exact handoff
/// equality covers provider subject, member/device/session identity, support
/// receipt, generations, and target binding; a caller-held snapshot is never
/// accepted merely because account and household ids still match.
pub(crate) fn ensure_exact_current(
    transaction: &Transaction<'_>,
    observed: &VerifiedAccountIdentityAuthority,
    now: DateTime<Utc>,
) -> Result<(), AccountIdentityIssuerError> {
    let row = transaction
        .query_row(
            "SELECT mapping_status, authority_generation, session_id,
                    session_generation, authority_json
             FROM account_identity_current_authority
             WHERE provider = ?1 AND provider_subject = ?2 LIMIT 1",
            params![
                provider_label(observed.provider()),
                observed.provider_subject().as_str()
            ],
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
        .map_err(|_| AccountIdentityIssuerError::CurrentAuthorityUnavailable)?
        .ok_or(AccountIdentityIssuerError::CurrentAuthorityUnavailable)?;
    let handoff: AccountIdentityCurrentMemberDeviceAuthorityHandoff = serde_json::from_str(&row.4)
        .map_err(|_| AccountIdentityIssuerError::CurrentAuthorityRejected)?;
    handoff
        .validate_shape()
        .map_err(|_| AccountIdentityIssuerError::CurrentAuthorityRejected)?;
    if row.0 != "active"
        || handoff.mapping.status != AccountIdentityMappingStatus::Active
        || row.1 != to_sql_generation(observed.authority_generation())?
        || row.2 != observed.session_id().as_str()
        || row.3 != to_sql_generation(observed.session_generation())?
        || observed.handoff() != &handoff
    {
        return Err(AccountIdentityIssuerError::CurrentAuthorityRejected);
    }
    validate_temporal_currentness(&handoff, now)
}

pub(crate) fn binding_for_current(
    authority: &VerifiedAccountIdentityAuthority,
    service: AccountIdentityIssuerService,
) -> Result<AccountIdentityIssuerServiceBinding, AccountIdentityIssuerError> {
    AccountIdentityIssuerServiceBinding::from_authority(authority, service)
}

pub(crate) fn authenticate_binding(
    authenticator: Option<&dyn AccountIdentityIssuerServiceBindingAuthenticator>,
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
) -> Result<AccountIdentityIssuerAuthenticatedBinding, AccountIdentityIssuerError> {
    if !binding.matches_authority(authority) {
        return Err(AccountIdentityIssuerError::BindingMismatch);
    }
    let authenticator =
        authenticator.ok_or(AccountIdentityIssuerError::ServiceBindingUnavailable)?;
    let authenticated = authenticator.authenticate(binding)?;
    (authenticated.binding_id() == binding.binding_id())
        .then_some(authenticated)
        .ok_or(AccountIdentityIssuerError::ServiceBindingRejected)
}

fn validate_temporal_currentness(
    handoff: &AccountIdentityCurrentMemberDeviceAuthorityHandoff,
    now: DateTime<Utc>,
) -> Result<(), AccountIdentityIssuerError> {
    if handoff.member.session_freshness_state != AccountIdentitySessionFreshnessState::Fresh {
        return Err(AccountIdentityIssuerError::CurrentAuthorityRejected);
    }
    let session_expires_at = DateTime::parse_from_rfc3339(&handoff.member.session_expires_at)
        .map_err(|_| AccountIdentityIssuerError::CurrentAuthorityRejected)?
        .with_timezone(&Utc);
    if session_expires_at <= now {
        return Err(AccountIdentityIssuerError::CurrentAuthorityRejected);
    }
    let Some(receipt) = handoff.member.support_receipt.as_ref() else {
        return (handoff.member.role != AccountIdentityRole::SupportAdmin)
            .then_some(())
            .ok_or(AccountIdentityIssuerError::CurrentAuthorityRejected);
    };
    let issued_at = DateTime::parse_from_rfc3339(&receipt.issued_at)
        .map_err(|_| AccountIdentityIssuerError::CurrentAuthorityRejected)?
        .with_timezone(&Utc);
    let expires_at = DateTime::parse_from_rfc3339(&receipt.expires_at)
        .map_err(|_| AccountIdentityIssuerError::CurrentAuthorityRejected)?
        .with_timezone(&Utc);
    if receipt.revocation_state != AccountIdentitySupportReceiptRevocationState::Active
        || issued_at > now
        || now >= expires_at
    {
        return Err(AccountIdentityIssuerError::CurrentAuthorityRejected);
    }
    Ok(())
}

fn provider_label(provider: &AccountIdentityProvider) -> &'static str {
    match provider {
        AccountIdentityProvider::Authjs => "authjs",
        AccountIdentityProvider::Firebase => "firebase",
    }
}

fn to_sql_generation(value: u64) -> Result<i64, AccountIdentityIssuerError> {
    i64::try_from(value).map_err(|_| AccountIdentityIssuerError::CurrentAuthorityRejected)
}
