use chrono::{DateTime, SecondsFormat, Utc};
use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE;
use ring::digest::{digest, SHA256};
use rusqlite::{params, OptionalExtension, Transaction};

use super::account_identity_authority_issuer_client_types::AccountIdentityIssuerOutboxClaim;
use super::AccountIdentityAuthorityIssuerClientError;

const RETRY_BASE_MILLIS: i64 = 5 * 1_000;
const RETRY_MAX_MILLIS: i64 = 15 * 60 * 1_000;

pub(super) fn reconcile_startup(
    transaction: &Transaction<'_>,
    now: i64,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    let now_text = timestamp(now)?;
    transaction
        .execute(
            "UPDATE account_identity_issuer_v2_outbox
                SET delivery_state = 'failed', claim_id = NULL, claimed_at = NULL,
                    last_error = 'startup-recovery-lease-released',
                    next_attempt_at = ?1
              WHERE service = ?2 AND delivery_state = 'claimed'",
            params![now_text, ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE],
        )
        .map(|_| ())
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::DeliveryUnavailable)
}

pub(super) fn claim_pending(
    transaction: &Transaction<'_>,
    now: i64,
) -> Result<Option<AccountIdentityIssuerOutboxClaim>, AccountIdentityAuthorityIssuerClientError> {
    let now_text = timestamp(now)?;
    let row: Option<(String, Vec<u8>, i64)> = transaction
        .query_row(
            "SELECT receipt_id, wire, attempt_count
               FROM account_identity_issuer_v2_outbox
              WHERE service = ?1 AND (
                    delivery_state = 'pending'
                    OR (delivery_state = 'failed'
                        AND (next_attempt_at IS NULL OR next_attempt_at <= ?2))
              )
              ORDER BY receipt_id LIMIT 1",
            params![ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE, now_text],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .optional()
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::DeliveryUnavailable)?;
    let Some((receipt_id, wire, attempts)) = row else {
        return Ok(None);
    };
    let next_attempts = attempts
        .checked_add(1)
        .ok_or(AccountIdentityAuthorityIssuerClientError::DeliveryUnavailable)?;
    let claim_id = claim_id(&receipt_id, next_attempts, now);
    let claimed_at = timestamp(now)?;
    let changed = transaction
        .execute(
            "UPDATE account_identity_issuer_v2_outbox
                SET delivery_state = 'claimed', claim_id = ?1, claimed_at = ?2,
                    attempt_count = ?3, last_error = NULL, next_attempt_at = NULL
              WHERE receipt_id = ?4 AND service = ?5
                AND (delivery_state = 'pending'
                     OR (delivery_state = 'failed'
                         AND (next_attempt_at IS NULL OR next_attempt_at <= ?2)))",
            params![
                claim_id,
                claimed_at,
                next_attempts,
                receipt_id,
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
            ],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::DeliveryUnavailable)?;
    if changed != 1 {
        return Err(AccountIdentityAuthorityIssuerClientError::DeliveryUnavailable);
    }
    Ok(Some(AccountIdentityIssuerOutboxClaim {
        receipt_id,
        claim_id,
        wire,
    }))
}

pub(super) fn record_failure(
    transaction: &Transaction<'_>,
    claim: &AccountIdentityIssuerOutboxClaim,
    message: &str,
    now: i64,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    if message.trim().is_empty() || message.len() > 1_024 {
        return Err(AccountIdentityAuthorityIssuerClientError::DeliveryUnavailable);
    }
    let next_attempt = now
        .checked_add(retry_delay(transaction, claim)?)
        .ok_or(AccountIdentityAuthorityIssuerClientError::ClockUnavailable)?;
    let next_attempt_text = timestamp(next_attempt)?;
    let result = format!("sha256:delivery-failure:{}", digest_hex(message.as_bytes()));
    let changed = transaction
        .execute(
            "UPDATE account_identity_issuer_v2_outbox
                SET delivery_state = 'failed', claim_id = NULL, claimed_at = NULL,
                    last_error = ?1, last_result = ?2, next_attempt_at = ?3
              WHERE receipt_id = ?4 AND claim_id = ?5 AND service = ?6
                AND delivery_state = 'claimed'",
            params![
                message,
                result,
                next_attempt_text,
                claim.receipt_id(),
                claim.claim_id(),
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE
            ],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::DeliveryUnavailable)?;
    if changed != 1 {
        return Err(AccountIdentityAuthorityIssuerClientError::DeliveryUnavailable);
    }
    Ok(())
}

fn retry_delay(
    transaction: &Transaction<'_>,
    claim: &AccountIdentityIssuerOutboxClaim,
) -> Result<i64, AccountIdentityAuthorityIssuerClientError> {
    let attempts: i64 = transaction
        .query_row(
            "SELECT attempt_count FROM account_identity_issuer_v2_outbox
              WHERE receipt_id = ?1 AND claim_id = ?2",
            params![claim.receipt_id(), claim.claim_id()],
            |row| row.get(0),
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::DeliveryUnavailable)?;
    let shift = attempts.saturating_sub(1).min(10) as u32;
    Ok(RETRY_BASE_MILLIS
        .saturating_mul(1_i64.checked_shl(shift).unwrap_or(i64::MAX))
        .min(RETRY_MAX_MILLIS))
}

fn claim_id(receipt_id: &str, attempt: i64, now: i64) -> String {
    let mut input = Vec::new();
    input.extend_from_slice(b"ocentra.account-issuer.outbox-claim.v2\0");
    input.extend_from_slice(receipt_id.as_bytes());
    input.extend_from_slice(&attempt.to_be_bytes());
    input.extend_from_slice(&now.to_be_bytes());
    format!("sha256:claim:{}", digest_hex(input.as_slice()))
}

fn digest_hex(value: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let digest = digest(&SHA256, value);
    let mut text = String::with_capacity(digest.as_ref().len() * 2);
    for byte in digest.as_ref() {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }
    text
}

fn timestamp(value: i64) -> Result<String, AccountIdentityAuthorityIssuerClientError> {
    DateTime::<Utc>::from_timestamp_millis(value)
        .map(|time| time.to_rfc3339_opts(SecondsFormat::Millis, true))
        .ok_or(AccountIdentityAuthorityIssuerClientError::ClockUnavailable)
}
