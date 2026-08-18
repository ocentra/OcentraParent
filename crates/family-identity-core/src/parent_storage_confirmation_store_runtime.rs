use ocentra_schema::parent_storage_settings_apply_flow::{
    ParentStorageApplyIntentDigest, ParentStoragePreviewId,
};
use rusqlite::{params, Connection, OptionalExtension, Transaction, TransactionBehavior};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::household_authority_runtime_composer::HouseholdAuthorityDeviceTrustSource;

use super::device_failure::map_device_failure;
use super::{
    parse_state, validate_current_binding, validate_input, ConsumedParentStorageConfirmation,
    ParentStorageConfirmationBinding, ParentStorageConfirmationStoreError,
    StagedParentStorageConfirmation, StoredLifecycleState, StoredRow,
};

pub(super) fn stage(
    connection: &mut Connection,
    authority: &VerifiedAccountIdentityAuthority,
    device_trust_source: &impl HouseholdAuthorityDeviceTrustSource,
    binding: ParentStorageConfirmationBinding<'_>,
    preview_id: &ParentStoragePreviewId,
    apply_intent_digest: &ParentStorageApplyIntentDigest,
) -> Result<StagedParentStorageConfirmation, ParentStorageConfirmationStoreError> {
    validate_input(&binding, preview_id, apply_intent_digest)?;
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
    let (now, _now_timestamp) = super::support::trusted_now(&transaction)?;
    super::support::ensure_current_authority(&transaction, authority, now)?;
    let current_device = device_trust_source
        .current_device_trust_binding(authority)
        .map_err(map_device_failure)?;
    validate_current_binding(authority, &current_device, &binding)?;
    let expires_at = now
        .checked_add(super::MAX_CONFIRMATION_TTL_MILLIS)
        .ok_or(ParentStorageConfirmationStoreError::ClockUnavailable)?;
    expire_prior_staged_intent(
        &transaction,
        binding.household_id,
        preview_id.as_str(),
        apply_intent_digest.as_str(),
        now,
    )?;
    let receipt_id = random_hex_id()?;
    let nonce_id = random_hex_id()?;
    let receipt_epoch = super::support::next_receipt_epoch(&transaction, now)?;
    insert_staged_row(
        &transaction,
        &binding,
        preview_id,
        apply_intent_digest,
        &receipt_id,
        &nonce_id,
        receipt_epoch,
        now,
        expires_at,
    )?;
    transaction
        .commit()
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
    Ok(StagedParentStorageConfirmation {
        receipt_id,
        nonce_id,
        receipt_epoch,
        expires_at_epoch_millis: expires_at,
    })
}

fn insert_staged_row(
    transaction: &Transaction<'_>,
    binding: &ParentStorageConfirmationBinding<'_>,
    preview_id: &ParentStoragePreviewId,
    apply_intent_digest: &ParentStorageApplyIntentDigest,
    receipt_id: &str,
    nonce_id: &str,
    receipt_epoch: u64,
    issued_at: i64,
    expires_at: i64,
) -> Result<(), ParentStorageConfirmationStoreError> {
    transaction
        .execute(
            "INSERT INTO account_identity_parent_storage_confirmation (
                receipt_id, nonce_id, provider, provider_subject, household_id, account_id,
                parent_device_id, child_profile_id, child_device_id, installation_id,
                pairing_id, route_id, authority_generation, session_generation,
                device_trust_subject, device_lifecycle_generation,
                device_installation_binding_generation, device_authority_generation,
                preview_id, apply_intent_digest, receipt_epoch, issued_at_epoch_millis,
                expires_at_epoch_millis, consumed_at_epoch_millis, lifecycle_state
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14,
                      ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, NULL, 'staged')",
            params![
                receipt_id,
                nonce_id,
                super::provider_label(binding.provider),
                binding.provider_subject,
                binding.household_id,
                binding.account_id,
                binding.parent_device_id,
                binding.child_profile_id,
                binding.child_device_id,
                binding.installation_id,
                binding.pairing_id,
                binding.route_id,
                sql_generation(binding.authority_generation)?,
                sql_generation(binding.session_generation)?,
                binding.device_trust_subject,
                sql_generation(binding.device_lifecycle_generation)?,
                sql_generation(binding.device_installation_binding_generation)?,
                sql_generation(binding.device_authority_generation)?,
                preview_id.as_str(),
                apply_intent_digest.as_str(),
                sql_generation(receipt_epoch)?,
                issued_at,
                expires_at,
            ],
        )
        .map_err(super::support::map_write_error)?;
    Ok(())
}

fn expire_prior_staged_intent(
    transaction: &Transaction<'_>,
    household_id: &str,
    preview_id: &str,
    apply_intent_digest: &str,
    now: i64,
) -> Result<(), ParentStorageConfirmationStoreError> {
    transaction
        .execute(
            "UPDATE account_identity_parent_storage_confirmation
             SET lifecycle_state = 'expired'
             WHERE household_id = ?1 AND preview_id = ?2 AND apply_intent_digest = ?3
               AND lifecycle_state = 'staged' AND expires_at_epoch_millis <= ?4",
            params![household_id, preview_id, apply_intent_digest, now],
        )
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
    Ok(())
}

pub(super) fn consume(
    connection: &mut Connection,
    authority: &VerifiedAccountIdentityAuthority,
    device_trust_source: &impl HouseholdAuthorityDeviceTrustSource,
    binding: ParentStorageConfirmationBinding<'_>,
    receipt_id: &str,
    nonce_id: &str,
    receipt_epoch: u64,
    preview_id: &ParentStoragePreviewId,
    apply_intent_digest: &ParentStorageApplyIntentDigest,
) -> Result<ConsumedParentStorageConfirmation, ParentStorageConfirmationStoreError> {
    validate_input(&binding, preview_id, apply_intent_digest)?;
    validate_receipt_inputs(receipt_id, nonce_id, receipt_epoch)?;
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
    let (now, _now_timestamp) = super::support::trusted_now(&transaction)?;
    super::support::ensure_current_authority(&transaction, authority, now)?;
    let current_device = device_trust_source
        .current_device_trust_binding(authority)
        .map_err(map_device_failure)?;
    validate_current_binding(authority, &current_device, &binding)?;
    let row = load_row(&transaction, receipt_id)?;
    row.validate()?;
    let state = parse_state(&row.lifecycle_state)?;
    validate_receipt_identity(&row, receipt_id, nonce_id, receipt_epoch)?;
    handle_existing_state(&transaction, &row, state, now, receipt_id, nonce_id)?;
    let expected = super::load_binding(&transaction, receipt_id)?;
    expected
        .matches(&binding, preview_id, apply_intent_digest)
        .then_some(())
        .ok_or(ParentStorageConfirmationStoreError::BindingMismatch)?;
    consume_staged(&transaction, receipt_id, nonce_id, row.receipt_epoch, now)?;
    Ok(ConsumedParentStorageConfirmation {
        receipt_id: receipt_id.to_owned(),
        nonce_id: nonce_id.to_owned(),
        receipt_epoch,
        expires_at_epoch_millis: row.expires_at_epoch_millis,
    })
}

fn validate_receipt_inputs(
    receipt_id: &str,
    nonce_id: &str,
    receipt_epoch: u64,
) -> Result<(), ParentStorageConfirmationStoreError> {
    super::schema::validate_hex_id(receipt_id)?;
    super::schema::validate_hex_id(nonce_id)?;
    (receipt_epoch > 0)
        .then_some(())
        .ok_or(ParentStorageConfirmationStoreError::BindingMismatch)
}

fn load_row(
    transaction: &Transaction<'_>,
    receipt_id: &str,
) -> Result<StoredRow, ParentStorageConfirmationStoreError> {
    transaction
        .query_row(
            "SELECT receipt_id, nonce_id, provider, provider_subject, household_id, account_id,
                    parent_device_id, child_profile_id, child_device_id, installation_id,
                    pairing_id, route_id, authority_generation, session_generation,
                    device_trust_subject, device_lifecycle_generation,
                    device_installation_binding_generation, device_authority_generation,
                    preview_id, apply_intent_digest, receipt_epoch, issued_at_epoch_millis,
                    expires_at_epoch_millis, consumed_at_epoch_millis, lifecycle_state
             FROM account_identity_parent_storage_confirmation WHERE receipt_id = ?1",
            [receipt_id],
            StoredRow::from_row,
        )
        .optional()
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?
        .ok_or(ParentStorageConfirmationStoreError::Missing)
}

fn validate_receipt_identity(
    row: &StoredRow,
    receipt_id: &str,
    nonce_id: &str,
    receipt_epoch: u64,
) -> Result<(), ParentStorageConfirmationStoreError> {
    (row.receipt_id == receipt_id
        && row.nonce_id == nonce_id
        && row.receipt_epoch == sql_generation(receipt_epoch)?)
    .then_some(())
    .ok_or(ParentStorageConfirmationStoreError::BindingMismatch)
}

fn handle_existing_state(
    transaction: &Transaction<'_>,
    row: &StoredRow,
    state: StoredLifecycleState,
    now: i64,
    receipt_id: &str,
    nonce_id: &str,
) -> Result<(), ParentStorageConfirmationStoreError> {
    match state {
        StoredLifecycleState::Consumed => Err(ParentStorageConfirmationStoreError::ReplayRejected),
        StoredLifecycleState::Expired => Err(ParentStorageConfirmationStoreError::Expired),
        StoredLifecycleState::Staged if row.expires_at_epoch_millis <= now => {
            expire_staged(transaction, receipt_id, nonce_id, row.receipt_epoch)?;
            Err(ParentStorageConfirmationStoreError::Expired)
        }
        StoredLifecycleState::Staged => Ok(()),
    }
}

fn expire_staged(
    transaction: &Transaction<'_>,
    receipt_id: &str,
    nonce_id: &str,
    receipt_epoch: i64,
) -> Result<(), ParentStorageConfirmationStoreError> {
    let changed = transaction
        .execute(
            "UPDATE account_identity_parent_storage_confirmation
             SET lifecycle_state = 'expired'
             WHERE receipt_id = ?1 AND nonce_id = ?2 AND receipt_epoch = ?3
               AND lifecycle_state = 'staged'",
            params![receipt_id, nonce_id, receipt_epoch],
        )
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
    (changed == 1)
        .then_some(())
        .ok_or(ParentStorageConfirmationStoreError::Conflict)?;
    transaction
        .commit()
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)
}

fn consume_staged(
    transaction: &Transaction<'_>,
    receipt_id: &str,
    nonce_id: &str,
    receipt_epoch: i64,
    now: i64,
) -> Result<(), ParentStorageConfirmationStoreError> {
    let changed = transaction
        .execute(
            "UPDATE account_identity_parent_storage_confirmation
             SET lifecycle_state = 'consumed', consumed_at_epoch_millis = ?2
            WHERE receipt_id = ?1 AND nonce_id = ?3 AND receipt_epoch = ?4
               AND lifecycle_state = 'staged' AND expires_at_epoch_millis > ?2",
            params![receipt_id, now, nonce_id, receipt_epoch],
        )
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?;
    (changed == 1)
        .then_some(())
        .ok_or(ParentStorageConfirmationStoreError::Conflict)?;
    transaction
        .commit()
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)
}

fn random_hex_id() -> Result<String, ParentStorageConfirmationStoreError> {
    let mut bytes = [0_u8; 32];
    getrandom::fill(&mut bytes)
        .map_err(|_| ParentStorageConfirmationStoreError::EntropyUnavailable)?;
    let mut value = String::with_capacity(64);
    for byte in bytes {
        value.push_str(&format!("{byte:02x}"));
    }
    Ok(value)
}

fn sql_generation(value: u64) -> Result<i64, ParentStorageConfirmationStoreError> {
    i64::try_from(value).map_err(|_| ParentStorageConfirmationStoreError::BindingMismatch)
}
