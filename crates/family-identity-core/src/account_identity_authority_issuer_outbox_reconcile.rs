use chrono::{DateTime, Utc};
use rusqlite::{params, Transaction};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

use super::super::service_binding::AccountIdentityIssuerServiceBinding;
use super::super::AccountIdentityIssuerError;
use super::to_sql_generation;

/// Terminalize every undelivered row made obsolete by an Account-owned key
/// rotation or revocation in the same transaction as that key transition.
pub(crate) fn supersede_for_rotation(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
    now: DateTime<Utc>,
) -> Result<(), AccountIdentityIssuerError> {
    ensure_scope(authority, binding)?;
    transaction
        .execute(
            "UPDATE account_identity_issuer_transport_outbox
                SET delivery_state = 'superseded', claim_id = NULL,
                    claim_expires_at_millis = NULL, terminal_at_millis = ?1
              WHERE account_id = ?2 AND household_id = ?3 AND service_label = ?4
                AND delivery_state IN ('pending','claimed')",
            params![
                now.timestamp_millis(),
                authority.account_id().to_string(),
                authority.household_id().to_string(),
                binding.service().label(),
            ],
        )
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
    Ok(())
}

/// Reconcile expiry, authority rotation, and key rotation before a claim or
/// acknowledgement can advance. Terminal rows remain inspectable instead of
/// being stranded indefinitely or silently retried with stale authority.
pub(crate) fn reconcile_for_current(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
    now: DateTime<Utc>,
) -> Result<(), AccountIdentityIssuerError> {
    ensure_scope(authority, binding)?;
    expire_rows(transaction, authority, binding, now)?;
    supersede_stale_rows(transaction, authority, binding, now)
}

pub(crate) fn supersede_receipt(
    transaction: &Transaction<'_>,
    receipt_id: &str,
    now: DateTime<Utc>,
) -> Result<(), AccountIdentityIssuerError> {
    let changed = transaction
        .execute(
            "UPDATE account_identity_issuer_transport_outbox
                SET delivery_state = 'superseded', claim_id = NULL,
                    claim_expires_at_millis = NULL, terminal_at_millis = ?1
              WHERE receipt_id = ?2 AND delivery_state IN ('pending','claimed')",
            params![now.timestamp_millis(), receipt_id],
        )
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
    (changed == 1)
        .then_some(())
        .ok_or(AccountIdentityIssuerError::DeliveryUnavailable)
}

fn expire_rows(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
    now: DateTime<Utc>,
) -> Result<(), AccountIdentityIssuerError> {
    transaction
        .execute(
            "UPDATE account_identity_issuer_transport_outbox AS outbox
                SET delivery_state = 'expired', claim_id = NULL,
                    claim_expires_at_millis = NULL, terminal_at_millis = ?1
              WHERE outbox.account_id = ?2 AND outbox.service_label = ?3
                AND outbox.delivery_state IN ('pending','claimed')
                AND EXISTS (
                    SELECT 1 FROM account_identity_issuer_transport_receipt AS receipt
                     WHERE receipt.receipt_id = outbox.receipt_id
                       AND receipt.expires_at_millis <= ?1
                )",
            params![
                now.timestamp_millis(),
                authority.account_id().to_string(),
                binding.service().label(),
            ],
        )
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
    Ok(())
}

fn supersede_stale_rows(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
    now: DateTime<Utc>,
) -> Result<(), AccountIdentityIssuerError> {
    transaction
        .execute(
            "UPDATE account_identity_issuer_transport_outbox AS outbox
                SET delivery_state = 'superseded', claim_id = NULL,
                    claim_expires_at_millis = NULL, terminal_at_millis = ?1
              WHERE outbox.account_id = ?2 AND outbox.service_label = ?3
                AND outbox.delivery_state IN ('pending','claimed')
                AND (
                    outbox.household_id != ?4
                    OR outbox.service_binding_id != ?5
                    OR outbox.authority_generation != ?6
                    OR NOT EXISTS (
                        SELECT 1 FROM account_identity_issuer_key_registry AS active_key
                         WHERE active_key.account_id = outbox.account_id
                           AND active_key.household_id = outbox.household_id
                           AND active_key.service_binding_id = outbox.service_binding_id
                           AND active_key.service_label = outbox.service_label
                           AND active_key.key_id = outbox.key_id
                           AND active_key.key_version = outbox.key_version
                           AND active_key.authority_generation = outbox.authority_generation
                           AND active_key.key_state = 'active'
                           AND NOT EXISTS (
                               SELECT 1 FROM account_identity_issuer_key_registry AS newer
                                WHERE newer.account_id = active_key.account_id
                                  AND newer.household_id = active_key.household_id
                                  AND newer.service_label = active_key.service_label
                                  AND newer.key_version > active_key.key_version
                           )
                    )
                )",
            params![
                now.timestamp_millis(),
                authority.account_id().to_string(),
                binding.service().label(),
                authority.household_id().to_string(),
                binding.binding_id(),
                to_sql_generation(authority.authority_generation())?,
            ],
        )
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
    Ok(())
}

fn ensure_scope(
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
) -> Result<(), AccountIdentityIssuerError> {
    (binding.account_id() == authority.account_id().to_string()
        && binding.household_id() == authority.household_id().to_string()
        && binding.authority_generation() == authority.authority_generation())
    .then_some(())
    .ok_or(AccountIdentityIssuerError::BindingMismatch)
}
