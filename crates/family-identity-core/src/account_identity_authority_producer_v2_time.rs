use chrono::{DateTime, Duration, SecondsFormat, Utc};
use ocentra_schema::account_identity_authority_producer_v2::{
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_FUTURE_ISSUED_SKEW_SECONDS,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_LIFETIME_SECONDS,
};

use super::AccountIdentityAuthorityProducerV2Error;

pub(super) fn validate_lifetime(
    issued_at: &str,
    expires_at: &str,
    now: DateTime<Utc>,
) -> Result<(), AccountIdentityAuthorityProducerV2Error> {
    let issued_at = parse_time(issued_at)?;
    let expires_at = parse_time(expires_at)?;
    let latest_issued_at = now
        .checked_add_signed(Duration::seconds(
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_FUTURE_ISSUED_SKEW_SECONDS,
        ))
        .ok_or(AccountIdentityAuthorityProducerV2Error::AuthorityExpired)?;
    let latest_expires_at = issued_at
        .checked_add_signed(Duration::seconds(
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_LIFETIME_SECONDS,
        ))
        .ok_or(AccountIdentityAuthorityProducerV2Error::AuthorityExpired)?;
    if issued_at > latest_issued_at
        || expires_at <= issued_at
        || now >= expires_at
        || expires_at > latest_expires_at
    {
        return Err(AccountIdentityAuthorityProducerV2Error::AuthorityExpired);
    }
    Ok(())
}

fn parse_time(value: &str) -> Result<DateTime<Utc>, AccountIdentityAuthorityProducerV2Error> {
    let parsed = DateTime::parse_from_rfc3339(value)
        .map(|value| value.with_timezone(&Utc))
        .map_err(|_error| AccountIdentityAuthorityProducerV2Error::InvalidWire)?;
    if parsed.to_rfc3339_opts(SecondsFormat::Millis, true) != value {
        return Err(AccountIdentityAuthorityProducerV2Error::InvalidWire);
    }
    Ok(parsed)
}
