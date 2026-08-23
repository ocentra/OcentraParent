use chrono::{DateTime, Duration, SecondsFormat, Utc};
use ocentra_schema::account_identity_authority::AccountIdentityCurrentMemberDeviceAuthorityHandoff;
use ocentra_schema::account_identity_authority_producer::{
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_AUDIENCE, ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_ENVIRONMENT,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_FIELD_BYTES,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_FUTURE_ISSUED_SKEW_SECONDS,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_LIFETIME_SECONDS,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_PAYLOAD_BYTES,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_WIRE_BYTES,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_SCHEMA_VERSION,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_SIGNATURE_ALGORITHM,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_SIGNATURE_BYTES,
};

use super::envelope::{domain_separator, CanonicalAuthorityProducerEnvelope};
use crate::account_identity_authority_producer_error::AccountIdentityAuthorityProducerError;

pub(crate) struct ParsedAuthorityProducerEnvelope {
    pub(crate) signing_bytes: Vec<u8>,
    pub(crate) signature: [u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_SIGNATURE_BYTES],
    pub(crate) envelope: CanonicalAuthorityProducerEnvelope,
    pub(crate) handoff: AccountIdentityCurrentMemberDeviceAuthorityHandoff,
}

pub(crate) fn parse_wire(
    wire: &[u8],
) -> Result<ParsedAuthorityProducerEnvelope, AccountIdentityAuthorityProducerError> {
    parse_wire_at(wire, Utc::now())
}

pub(crate) fn parse_wire_at(
    wire: &[u8],
    now: DateTime<Utc>,
) -> Result<ParsedAuthorityProducerEnvelope, AccountIdentityAuthorityProducerError> {
    if wire.len() > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_WIRE_BYTES
        || wire.len() <= ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_SIGNATURE_BYTES
    {
        return Err(AccountIdentityAuthorityProducerError::InvalidWire);
    }
    let signing_length = wire.len() - ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_SIGNATURE_BYTES;
    let signing_bytes = wire[..signing_length].to_vec();
    let signature = <[u8; ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_SIGNATURE_BYTES]>::try_from(
        &wire[signing_length..],
    )
    .map_err(|_| AccountIdentityAuthorityProducerError::InvalidWire)?;
    if !signing_bytes.starts_with(domain_separator()) {
        return Err(AccountIdentityAuthorityProducerError::InvalidWire);
    }

    let mut cursor = Cursor::new(&signing_bytes[domain_separator().len()..]);
    let version = cursor.read_string()?;
    let audience = cursor.read_string()?;
    let environment = cursor.read_string()?;
    let algorithm = cursor.read_string()?;
    let key_id = cursor.read_string()?;
    let issued_at = cursor.read_string()?;
    let expires_at = cursor.read_string()?;
    let payload = cursor.read_bytes()?;
    cursor.finish()?;
    if version != ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_SCHEMA_VERSION
        || audience != ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_AUDIENCE
        || environment != ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_ENVIRONMENT
        || algorithm != ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_SIGNATURE_ALGORITHM
    {
        return Err(AccountIdentityAuthorityProducerError::InvalidWire);
    }
    let handoff: AccountIdentityCurrentMemberDeviceAuthorityHandoff =
        serde_json::from_slice(&payload)
            .map_err(|_| AccountIdentityAuthorityProducerError::InvalidWire)?;
    if serde_json::to_vec(&handoff)
        .map_err(|_| AccountIdentityAuthorityProducerError::InvalidWire)?
        != payload
    {
        return Err(AccountIdentityAuthorityProducerError::InvalidWire);
    }
    handoff
        .validate_shape()
        .map_err(|_| AccountIdentityAuthorityProducerError::InvalidWire)?;
    let issued = parse_timestamp(&issued_at)?;
    let expires = parse_timestamp(&expires_at)?;
    let max_future_issued = now
        .checked_add_signed(Duration::seconds(
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_FUTURE_ISSUED_SKEW_SECONDS,
        ))
        .ok_or(AccountIdentityAuthorityProducerError::AuthorityExpired)?;
    let max_expires = issued
        .checked_add_signed(Duration::seconds(
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_LIFETIME_SECONDS,
        ))
        .ok_or(AccountIdentityAuthorityProducerError::AuthorityExpired)?;
    if issued >= expires || issued > max_future_issued || expires <= now || expires > max_expires {
        return Err(AccountIdentityAuthorityProducerError::AuthorityExpired);
    }
    Ok(ParsedAuthorityProducerEnvelope {
        signing_bytes,
        signature,
        envelope: CanonicalAuthorityProducerEnvelope {
            key_id,
            issued_at,
            expires_at,
            payload,
        },
        handoff,
    })
}

fn parse_timestamp(value: &str) -> Result<DateTime<Utc>, AccountIdentityAuthorityProducerError> {
    let parsed = DateTime::parse_from_rfc3339(value)
        .map(|value| value.with_timezone(&Utc))
        .map_err(|_| AccountIdentityAuthorityProducerError::InvalidWire)?;
    if parsed.to_rfc3339_opts(SecondsFormat::Millis, true) != value {
        return Err(AccountIdentityAuthorityProducerError::InvalidWire);
    }
    Ok(parsed)
}

struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn read_string(&mut self) -> Result<String, AccountIdentityAuthorityProducerError> {
        let bytes = self.read_bytes()?;
        let value = String::from_utf8(bytes)
            .map_err(|_| AccountIdentityAuthorityProducerError::InvalidWire)?;
        if value.is_empty() || value.len() > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_FIELD_BYTES {
            return Err(AccountIdentityAuthorityProducerError::InvalidWire);
        }
        Ok(value)
    }

    fn read_bytes(&mut self) -> Result<Vec<u8>, AccountIdentityAuthorityProducerError> {
        let end = self
            .offset
            .checked_add(4)
            .ok_or(AccountIdentityAuthorityProducerError::InvalidWire)?;
        let length = self
            .bytes
            .get(self.offset..end)
            .and_then(|value| <[u8; 4]>::try_from(value).ok())
            .map(u32::from_be_bytes)
            .and_then(|value| usize::try_from(value).ok())
            .ok_or(AccountIdentityAuthorityProducerError::InvalidWire)?;
        self.offset = end;
        if length > ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_PAYLOAD_BYTES {
            return Err(AccountIdentityAuthorityProducerError::InvalidWire);
        }
        let end = self
            .offset
            .checked_add(length)
            .ok_or(AccountIdentityAuthorityProducerError::InvalidWire)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(AccountIdentityAuthorityProducerError::InvalidWire)?
            .to_vec();
        self.offset = end;
        Ok(value)
    }

    fn finish(self) -> Result<(), AccountIdentityAuthorityProducerError> {
        (self.offset == self.bytes.len())
            .then_some(())
            .ok_or(AccountIdentityAuthorityProducerError::InvalidWire)
    }
}
