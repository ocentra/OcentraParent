use chrono::{DateTime, Utc};
use ocentra_schema::account_identity_authority::{
    AccountIdentityCurrentMemberDeviceAuthorityHandoff, AccountIdentityProvider,
    AccountIdentityProviderSubject, AccountIdentityRole, AccountIdentitySessionFreshnessState,
    AccountIdentitySupportReceiptRevocationState, ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION,
};

use super::AccountIdentityAuthorityRepositoryError;

pub(super) fn validate_next_handoff(
    provider: &AccountIdentityProvider,
    provider_subject: &AccountIdentityProviderSubject,
    handoff: &AccountIdentityCurrentMemberDeviceAuthorityHandoff,
) -> Result<(), AccountIdentityAuthorityRepositoryError> {
    if &handoff.mapping.provider != provider
        || &handoff.mapping.provider_subject != provider_subject
    {
        return Err(AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority);
    }
    handoff
        .validate_shape()
        .map_err(|_| AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority)?;
    if handoff.member.authority_generation > ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION
        || handoff.member.session_generation > ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION
    {
        return Err(AccountIdentityAuthorityRepositoryError::InvalidGeneration);
    }
    validate_durable_currentness(provider_subject, handoff)?;
    Ok(())
}

/// CAS writes are a trust boundary as well as reads. Validate temporal and
/// receipt currentness against one trusted clock sample before serializing the
/// row, so an expired or revoked handoff can never become the current row.
fn validate_durable_currentness(
    provider_subject: &AccountIdentityProviderSubject,
    handoff: &AccountIdentityCurrentMemberDeviceAuthorityHandoff,
) -> Result<(), AccountIdentityAuthorityRepositoryError> {
    if handoff.member.session_freshness_state != AccountIdentitySessionFreshnessState::Fresh
        || handoff.member.session_generation == 0
    {
        return Err(AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority);
    }

    let now = Utc::now();
    let session_expires_at = DateTime::parse_from_rfc3339(&handoff.member.session_expires_at)
        .map_err(|_| AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority)?
        .with_timezone(&Utc);
    if session_expires_at <= now {
        return Err(AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority);
    }

    let Some(receipt) = handoff.member.support_receipt.as_ref() else {
        if handoff.member.role == AccountIdentityRole::SupportAdmin {
            return Err(AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority);
        }
        return Ok(());
    };

    if receipt.provider_subject != *provider_subject
        || receipt.account_id != handoff.member.account_id
        || receipt.member_id != handoff.member.member_id
        || receipt.household_id != handoff.member.household_id
        || receipt.device_id != handoff.member.device_id
        || receipt.child_profile_id != handoff.binding.child_profile_id
        || receipt.child_device_id != handoff.binding.child_device_id
        || receipt.revocation_state != AccountIdentitySupportReceiptRevocationState::Active
    {
        return Err(AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority);
    }

    let issued_at = DateTime::parse_from_rfc3339(&receipt.issued_at)
        .map_err(|_| AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority)?
        .with_timezone(&Utc);
    let expires_at = DateTime::parse_from_rfc3339(&receipt.expires_at)
        .map_err(|_| AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority)?
        .with_timezone(&Utc);
    if issued_at > now || now >= expires_at {
        return Err(AccountIdentityAuthorityRepositoryError::InvalidStoredAuthority);
    }

    Ok(())
}

pub(super) fn provider_label(provider: &AccountIdentityProvider) -> &'static str {
    match provider {
        AccountIdentityProvider::Authjs => "authjs",
        AccountIdentityProvider::Firebase => "firebase",
    }
}

pub(super) fn to_sql_generation(
    value: u64,
) -> Result<i64, AccountIdentityAuthorityRepositoryError> {
    i64::try_from(value).map_err(|_| AccountIdentityAuthorityRepositoryError::InvalidGeneration)
}

pub(super) fn from_sql_generation(
    value: i64,
) -> Result<u64, AccountIdentityAuthorityRepositoryError> {
    u64::try_from(value).map_err(|_| AccountIdentityAuthorityRepositoryError::InvalidGeneration)
}
