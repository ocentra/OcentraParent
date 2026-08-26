use chrono::{DateTime, Duration, Utc};
use rusqlite::{params, OptionalExtension, Transaction};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

use super::super::key_registry::RegisteredIssuerKey;
use super::super::service_binding::AccountIdentityIssuerServiceBinding;
use super::super::AccountIdentityIssuerError;
use super::{
    opaque_digest, reconcile, to_sql_generation, AccountIdentityIssuerDeliveryAttempt,
    CLAIM_LEASE_SECONDS,
};

pub(crate) fn claim_next(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
    registered: &RegisteredIssuerKey,
    now: DateTime<Utc>,
) -> Result<Option<AccountIdentityIssuerDeliveryAttempt>, AccountIdentityIssuerError> {
    reconcile::reconcile_for_current(transaction, authority, binding, now)?;
    loop {
        let Some((receipt_id, wire)) = select_claimable(transaction, authority, binding, now)?
        else {
            return Ok(None);
        };
        match super::super::transport::verify(&wire, authority, binding, registered, now) {
            Ok(verified) if verified.receipt_id() == receipt_id => {
                return claim_selected(transaction, authority, binding, receipt_id, wire, now);
            }
            _ => reconcile::supersede_receipt(transaction, &receipt_id, now)?,
        }
    }
}

fn claim_selected(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
    receipt_id: String,
    wire: Vec<u8>,
    now: DateTime<Utc>,
) -> Result<Option<AccountIdentityIssuerDeliveryAttempt>, AccountIdentityIssuerError> {
    let claim_id = opaque_digest("delivery-claim")?;
    let claim_expires_at = now
        .checked_add_signed(Duration::seconds(CLAIM_LEASE_SECONDS))
        .ok_or(AccountIdentityIssuerError::InvalidClock)?;
    let changed = transaction
        .execute(
            "UPDATE account_identity_issuer_transport_outbox
                SET delivery_state = 'claimed', claim_id = ?1,
                    claim_expires_at_millis = ?2, attempt_count = attempt_count + 1
              WHERE receipt_id = ?3 AND (
                    delivery_state = 'pending'
                    OR (delivery_state = 'claimed' AND claim_expires_at_millis <= ?4)
              )",
            params![
                claim_id,
                claim_expires_at.timestamp_millis(),
                receipt_id,
                now.timestamp_millis()
            ],
        )
        .map_err(|_| AccountIdentityIssuerError::Unavailable)?;
    if changed != 1 {
        return Err(AccountIdentityIssuerError::DeliveryUnavailable);
    }
    Ok(Some(AccountIdentityIssuerDeliveryAttempt {
        receipt_id,
        claim_id,
        service: binding.service(),
        binding_id: binding.binding_id().to_owned(),
        account_id: authority.account_id().to_string(),
        household_id: authority.household_id().to_string(),
        authority_generation: authority.authority_generation(),
        wire,
    }))
}

fn select_claimable(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    binding: &AccountIdentityIssuerServiceBinding,
    now: DateTime<Utc>,
) -> Result<Option<(String, Vec<u8>)>, AccountIdentityIssuerError> {
    transaction
        .query_row(
            "SELECT outbox.receipt_id, outbox.wire
             FROM account_identity_issuer_transport_outbox AS outbox
             WHERE outbox.account_id = ?1 AND outbox.household_id = ?2
               AND outbox.service_binding_id = ?3 AND outbox.service_label = ?4
               AND outbox.authority_generation = ?5
               AND (outbox.delivery_state = 'pending'
                    OR (outbox.delivery_state = 'claimed'
                        AND outbox.claim_expires_at_millis <= ?6))
               AND EXISTS (
                    SELECT 1 FROM account_identity_issuer_transport_receipt AS receipt
                     WHERE receipt.receipt_id = outbox.receipt_id
                       AND receipt.expires_at_millis > ?6
               )
               AND EXISTS (
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
                              AND newer.service_binding_id = active_key.service_binding_id
                              AND newer.service_label = active_key.service_label
                              AND newer.key_version > active_key.key_version
                       )
               )
             ORDER BY outbox.created_at_millis ASC, outbox.receipt_id ASC LIMIT 1",
            params![
                authority.account_id().to_string(),
                authority.household_id().to_string(),
                binding.binding_id(),
                binding.service().label(),
                to_sql_generation(authority.authority_generation())?,
                now.timestamp_millis(),
            ],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(|_| AccountIdentityIssuerError::Unavailable)
}
