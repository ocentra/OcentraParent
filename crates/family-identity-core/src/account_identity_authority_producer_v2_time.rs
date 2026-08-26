use chrono::{DateTime, Duration, Utc};
use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_FUTURE_ISSUED_SKEW_SECONDS;

use super::AccountIdentityAuthorityProducerV2Error;

pub(super) fn validate_lifetime(
    issued_at: &str,
    expires_at: &str,
    now: DateTime<Utc>,
) -> Result<(), AccountIdentityAuthorityProducerV2Error> {
    let issued_at = parse_time(issued_at)?;
    let expires_at = parse_time(expires_at)?;
    if issued_at
        > now
            + Duration::seconds(
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_MAX_FUTURE_ISSUED_SKEW_SECONDS,
            )
        || expires_at <= issued_at
        || now >= expires_at
    {
        return Err(AccountIdentityAuthorityProducerV2Error::AuthorityExpired);
    }
    Ok(())
}

fn parse_time(value: &str) -> Result<DateTime<Utc>, AccountIdentityAuthorityProducerV2Error> {
    DateTime::parse_from_rfc3339(value)
        .map(|value| value.with_timezone(&Utc))
        .map_err(|_| AccountIdentityAuthorityProducerV2Error::InvalidWire)
}
