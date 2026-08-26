use chrono::{DateTime, SecondsFormat, Utc};
use ocentra_schema::account_identity_authority_producer_v2::ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE;
use ring::digest::{digest, SHA256};
use rusqlite::{params, OptionalExtension, Transaction};

use super::account_identity_authority_issuer_client_types::AccountIdentityIssuerOutboxClaim;
use super::AccountIdentityAuthorityIssuerClientError;

const RETRY_BASE_MILLIS: i64 = 5 * 1_000;
const RETRY_MAX_MILLIS: i64 = 15 * 60 * 1_000;
const CLAIM_LEASE_MILLIS: i64 = 60 * 1_000;
const MAX_RECLAIMED_CLAIMS: i64 = 64;
const ERROR_CODE_DELIVERY_FAILED: &str = "delivery_failed";
const ERROR_CODE_LEASE_EXPIRED: &str = "lease_expired";

pub(super) fn reconcile_startup(
    transaction: &Transaction<'_>,
    now: i64,
) -> Result<bool, AccountIdentityAuthorityIssuerClientError> {
    let now_text = timestamp(now)?;
    transaction
        .execute(
            "UPDATE account_identity_issuer_v2_outbox
                SET delivery_state = 'failed', claim_id = NULL, claimed_at = NULL,
                    claim_expires_at = NULL, last_error_code = ?1,
                    last_error_digest = NULL, last_result = NULL,
                    next_attempt_at = ?2
              WHERE rowid IN (
                    SELECT rowid FROM account_identity_issuer_v2_outbox
                     WHERE service = ?3 AND delivery_state = 'claimed'
                       AND claim_expires_at IS NOT NULL AND claim_expires_at <= ?4
                     ORDER BY claim_expires_at, receipt_id LIMIT ?5
              )",
            params![
                ERROR_CODE_LEASE_EXPIRED,
                now_text,
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                now_text,
                MAX_RECLAIMED_CLAIMS
            ],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::DeliveryUnavailable)
        .and_then(|_| {
            transaction
                .query_row(
                    "SELECT EXISTS(
                        SELECT 1 FROM account_identity_issuer_v2_outbox
                         WHERE service = ?1 AND delivery_state = 'claimed'
                           AND claim_expires_at IS NOT NULL
                           AND claim_expires_at <= ?2
                    )",
                    params![ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE, now_text],
                    |row| row.get(0),
                )
                .map_err(|_| AccountIdentityAuthorityIssuerClientError::DeliveryUnavailable)
        })
}

pub(super) fn claim_pending(
    transaction: &Transaction<'_>,
    now: i64,
) -> Result<Option<AccountIdentityIssuerOutboxClaim>, AccountIdentityAuthorityIssuerClientError> {
    let _ = reconcile_startup(transaction, now)?;
    let now_text = timestamp(now)?;
    let claim_expires_at = now
        .checked_add(CLAIM_LEASE_MILLIS)
        .ok_or(AccountIdentityAuthorityIssuerClientError::ClockUnavailable)?;
    let claim_expires_at_text = timestamp(claim_expires_at)?;
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
                    claim_expires_at = ?3, attempt_count = ?4,
                    last_error_code = NULL, last_error_digest = NULL,
                    last_result = NULL, next_attempt_at = NULL
              WHERE receipt_id = ?5 AND service = ?6
                AND (delivery_state = 'pending'
                     OR (delivery_state = 'failed'
                         AND (next_attempt_at IS NULL OR next_attempt_at <= ?2)))",
            params![
                claim_id,
                claimed_at,
                claim_expires_at_text,
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
        claim_expires_at: claim_expires_at_text,
        wire,
    }))
}

pub(super) fn record_failure(
    transaction: &Transaction<'_>,
    claim: &AccountIdentityIssuerOutboxClaim,
    error_code: &str,
    error_digest: Option<&str>,
    now: i64,
) -> Result<(), AccountIdentityAuthorityIssuerClientError> {
    if error_code != ERROR_CODE_DELIVERY_FAILED || !valid_error_digest(error_digest) {
        return Err(AccountIdentityAuthorityIssuerClientError::DeliveryUnavailable);
    }
    let next_attempt = now
        .checked_add(retry_delay(transaction, claim)?)
        .ok_or(AccountIdentityAuthorityIssuerClientError::ClockUnavailable)?;
    let next_attempt_text = timestamp(next_attempt)?;
    let now_text = timestamp(now)?;
    let result = format!(
        "sha256:delivery-result:{}",
        digest_hex(
            [
                error_code.as_bytes(),
                error_digest.unwrap_or_default().as_bytes()
            ]
            .concat()
            .as_slice(),
        )
    );
    let changed = transaction
        .execute(
            "UPDATE account_identity_issuer_v2_outbox
                SET delivery_state = 'failed', claim_id = NULL, claimed_at = NULL,
                    claim_expires_at = NULL, last_error_code = ?1,
                    last_error_digest = ?2, last_result = ?3, next_attempt_at = ?4
              WHERE receipt_id = ?5 AND claim_id = ?6 AND claim_expires_at = ?7
                AND service = ?8
                AND claim_expires_at > ?9
                AND delivery_state = 'claimed'",
            params![
                error_code,
                error_digest,
                result,
                next_attempt_text,
                claim.receipt_id(),
                claim.claim_id(),
                claim.claim_expires_at(),
                ACCOUNT_IDENTITY_AUTHORITY_PRODUCER_V2_SERVICE,
                now_text,
            ],
        )
        .map_err(|_| AccountIdentityAuthorityIssuerClientError::DeliveryUnavailable)?;
    if changed != 1 {
        return Err(AccountIdentityAuthorityIssuerClientError::DeliveryUnavailable);
    }
    Ok(())
}

fn valid_error_digest(value: Option<&str>) -> bool {
    let Some(value) = value else {
        return false;
    };
    let Some(hex) = value.strip_prefix("sha256:delivery-detail:") else {
        return false;
    };
    hex.len() == 64 && hex.bytes().all(|byte| byte.is_ascii_hexdigit())
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
