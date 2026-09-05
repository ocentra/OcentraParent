use chrono::{DateTime, Duration, SecondsFormat, Utc};
use sha2::{Digest, Sha256};

use super::super::AccountIdentityIssuerError;
use super::ParsedTransport;
use ocentra_schema::account_identity_authority_producer::{
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_FUTURE_ISSUED_SKEW_SECONDS,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_LIFETIME_SECONDS,
    ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_WIRE_BYTES,
};

pub(super) const DOMAIN_SEPARATOR: &[u8] = b"ocentra.account-issuer.transport.v1\0";
const VERSION: &str = "v1";
const AUDIENCE: &str = "ocentra.account-authority-producer.cloudflare";
pub(super) const NONCE_BYTES: usize = 32;
pub(super) const SIGNATURE_BYTES: usize = 64;
const MAX_FIELD_BYTES: usize = 1024;
const OUTER_STRING_FIELDS: usize = 11;
pub(super) const MAX_OUTER_WIRE_BYTES: usize = DOMAIN_SEPARATOR.len()
    + OUTER_STRING_FIELDS * (4 + MAX_FIELD_BYTES)
    + NONCE_BYTES
    + 4
    + ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_WIRE_BYTES
    + SIGNATURE_BYTES;

#[derive(Clone, Copy)]
struct TransportReceiptParts<'a> {
    binding_id: &'a str,
    account_id: &'a str,
    household_id: &'a str,
    authority_generation: u64,
    key_id: &'a str,
    key_version: u64,
    nonce: &'a [u8; NONCE_BYTES],
    signing_bytes: &'a [u8],
}

pub(super) fn parse(
    wire: &[u8],
    now: DateTime<Utc>,
) -> Result<ParsedTransport, AccountIdentityIssuerError> {
    if wire.len() <= SIGNATURE_BYTES || wire.len() > MAX_OUTER_WIRE_BYTES {
        return Err(AccountIdentityIssuerError::InvalidTransport);
    }
    let signing_length = wire.len() - SIGNATURE_BYTES;
    let signing_bytes = wire[..signing_length].to_vec();
    let signature = <[u8; SIGNATURE_BYTES]>::try_from(&wire[signing_length..])
        .map_err(|_error| AccountIdentityIssuerError::InvalidTransport)?;
    if !signing_bytes.starts_with(DOMAIN_SEPARATOR) {
        return Err(AccountIdentityIssuerError::InvalidTransport);
    }
    let mut cursor = Cursor::new(&signing_bytes[DOMAIN_SEPARATOR.len()..]);
    let version = cursor.read_string()?;
    let audience = cursor.read_string()?;
    let service_label = cursor.read_string()?;
    let binding_id = cursor.read_string()?;
    let account_id = cursor.read_string()?;
    let household_id = cursor.read_string()?;
    let authority_generation = cursor.read_u64()?;
    let key_id = cursor.read_string()?;
    let key_version = cursor.read_u64()?;
    let issued_at = cursor.read_string()?;
    let expires_at = cursor.read_string()?;
    let nonce = cursor.read_fixed::<NONCE_BYTES>()?;
    let inner_wire = cursor.read_bytes()?;
    cursor.finish()?;
    validate_inner_wire(&inner_wire)?;
    if version != VERSION || audience != AUDIENCE {
        return Err(AccountIdentityIssuerError::InvalidTransport);
    }
    let issued_at = parse_timestamp(&issued_at)?;
    let expires_at = parse_timestamp(&expires_at)?;
    let max_future_issued = now
        .checked_add_signed(Duration::seconds(
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_FUTURE_ISSUED_SKEW_SECONDS,
        ))
        .ok_or(AccountIdentityIssuerError::InvalidClock)?;
    let max_expires = issued_at
        .checked_add_signed(Duration::seconds(
            ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_LIFETIME_SECONDS,
        ))
        .ok_or(AccountIdentityIssuerError::InvalidClock)?;
    if issued_at >= expires_at
        || issued_at > max_future_issued
        || expires_at <= now
        || expires_at > max_expires
    {
        return Err(AccountIdentityIssuerError::TransportExpired);
    }
    let receipt_id = receipt_id_from_parts(TransportReceiptParts {
        binding_id: &binding_id,
        account_id: &account_id,
        household_id: &household_id,
        authority_generation,
        key_id: &key_id,
        key_version,
        nonce: &nonce,
        signing_bytes: &signing_bytes,
    });
    Ok(ParsedTransport {
        signing_bytes,
        signature,
        service_label,
        binding_id,
        account_id,
        household_id,
        authority_generation,
        key_id,
        key_version,
        issued_at,
        expires_at,
        receipt_id,
        inner_wire,
    })
}

fn validate_inner_wire(inner_wire: &[u8]) -> Result<(), AccountIdentityIssuerError> {
    (inner_wire.len() <= ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_MAX_WIRE_BYTES)
        .then_some(())
        .ok_or(AccountIdentityIssuerError::InvalidTransport)
}

fn receipt_id_from_parts(parts: TransportReceiptParts<'_>) -> String {
    let mut digest = Sha256::new();
    digest.update(b"ocentra.account-issuer.transport-receipt.v1\0");
    for value in [
        parts.binding_id.as_bytes(),
        parts.account_id.as_bytes(),
        parts.household_id.as_bytes(),
        parts.key_id.as_bytes(),
    ] {
        digest.update((value.len() as u32).to_be_bytes());
        digest.update(value);
    }
    digest.update(parts.authority_generation.to_be_bytes());
    digest.update(parts.key_version.to_be_bytes());
    digest.update(parts.nonce);
    digest.update(Sha256::digest(parts.signing_bytes));
    format!("sha256:{:x}", digest.finalize())
}

fn parse_timestamp(value: &str) -> Result<DateTime<Utc>, AccountIdentityIssuerError> {
    let parsed = DateTime::parse_from_rfc3339(value)
        .map(|value| value.with_timezone(&Utc))
        .map_err(|_error| AccountIdentityIssuerError::InvalidTransport)?;
    if parsed.to_rfc3339_opts(SecondsFormat::Millis, true) != value {
        return Err(AccountIdentityIssuerError::InvalidTransport);
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

    fn read_string(&mut self) -> Result<String, AccountIdentityIssuerError> {
        let bytes = self.read_bytes()?;
        let value = String::from_utf8(bytes)
            .map_err(|_error| AccountIdentityIssuerError::InvalidTransport)?;
        if value.is_empty() || value.len() > MAX_FIELD_BYTES {
            return Err(AccountIdentityIssuerError::InvalidTransport);
        }
        Ok(value)
    }

    fn read_u64(&mut self) -> Result<u64, AccountIdentityIssuerError> {
        let value = self.read_string()?;
        let parsed = value
            .parse::<u64>()
            .map_err(|_error| AccountIdentityIssuerError::InvalidTransport)?;
        if parsed.to_string() != value {
            return Err(AccountIdentityIssuerError::InvalidTransport);
        }
        Ok(parsed)
    }

    fn read_fixed<const N: usize>(&mut self) -> Result<[u8; N], AccountIdentityIssuerError> {
        let end = self
            .offset
            .checked_add(N)
            .ok_or(AccountIdentityIssuerError::InvalidTransport)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(AccountIdentityIssuerError::InvalidTransport)?;
        self.offset = end;
        value
            .try_into()
            .map_err(|_error| AccountIdentityIssuerError::InvalidTransport)
    }

    fn read_bytes(&mut self) -> Result<Vec<u8>, AccountIdentityIssuerError> {
        let end = self
            .offset
            .checked_add(4)
            .ok_or(AccountIdentityIssuerError::InvalidTransport)?;
        let length = self
            .bytes
            .get(self.offset..end)
            .and_then(|value| <[u8; 4]>::try_from(value).ok())
            .map(u32::from_be_bytes)
            .and_then(|value| usize::try_from(value).ok())
            .ok_or(AccountIdentityIssuerError::InvalidTransport)?;
        self.offset = end;
        let end = self
            .offset
            .checked_add(length)
            .ok_or(AccountIdentityIssuerError::InvalidTransport)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(AccountIdentityIssuerError::InvalidTransport)?
            .to_vec();
        self.offset = end;
        Ok(value)
    }

    fn finish(self) -> Result<(), AccountIdentityIssuerError> {
        (self.offset == self.bytes.len())
            .then_some(())
            .ok_or(AccountIdentityIssuerError::InvalidTransport)
    }
}
