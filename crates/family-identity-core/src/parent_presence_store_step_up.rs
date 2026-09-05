use rusqlite::{params, Transaction};

use crate::household_authority::HouseholdAuthorityAction;
use crate::parent_presence_store::{ParentPresenceStoreError, StoredParentStepUpIntent};
use crate::parent_presence_store_receipt::generate_opaque_receipt_ref;

pub(crate) fn insert_receipt_and_mark(
    transaction: &Transaction<'_>,
    challenge_ref: &str,
    action: &HouseholdAuthorityAction,
    nonce_ref: &str,
    verified_credential: Option<(&str, i32, u32)>,
) -> Result<String, ParentPresenceStoreError> {
    let receipt_ref = generate_opaque_receipt_ref()?;
    if matches!(action, HouseholdAuthorityAction::RegisterLanSignerAnchor) {
        let (credential_id, credential_algorithm, credential_sign_count) =
            verified_credential.ok_or(ParentPresenceStoreError::IntegrityRejected)?;
        let changed = transaction
            .execute(
                "UPDATE parent_step_up_intents
                 SET parent_presence_receipt = ?2,
                     credential_id = ?3,
                     credential_algorithm = ?4,
                     credential_sign_count = ?5
                 WHERE challenge_ref = ?1
                   AND nonce_ref = ?6
                   AND lifecycle_state = 'issued'
                   AND registration_state = 'pending'
                   AND parent_presence_receipt IS NULL
                   AND credential_id IS NULL
                   AND credential_algorithm IS NULL
                   AND credential_sign_count IS NULL",
                params![
                    challenge_ref,
                    receipt_ref,
                    credential_id,
                    credential_algorithm,
                    i64::from(credential_sign_count),
                    nonce_ref,
                ],
            )
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
        if changed != 1 {
            return Err(ParentPresenceStoreError::IntegrityRejected);
        }
    }
    transaction
        .execute(
            "INSERT INTO parent_presence_receipts (challenge_ref, receipt_ref)
             VALUES (?1, ?2)",
            params![challenge_ref, receipt_ref],
        )
        .map_err(|_error| ParentPresenceStoreError::IntegrityRejected)?;
    mark_consumed(transaction, challenge_ref, action, nonce_ref)?;
    Ok(receipt_ref)
}

pub(crate) fn insert_intent(
    transaction: &Transaction<'_>,
    intent: StoredParentStepUpIntent,
) -> Result<(), ParentPresenceStoreError> {
    transaction
        .execute(
            "INSERT INTO parent_step_up_intents (
                challenge_ref, nonce_ref, intent_digest, family_id, trust_subject,
                parent_account_id, parent_device_id, child_device_id, installation_id,
                pairing_id, route_id, signer_public_key, lifecycle_generation,
                installation_binding_generation, authority_generation, correlation_id,
                expires_at, lifecycle_state, registration_state,
                parent_presence_receipt, credential_id, credential_algorithm,
                credential_sign_count
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, 'issued', 'pending', NULL, NULL, NULL, NULL)",
            params![
                intent.challenge_ref,
                intent.nonce_ref,
                intent.intent_digest,
                intent.family_id,
                intent.trust_subject,
                intent.parent_account_id,
                intent.parent_device_id,
                intent.child_device_id,
                intent.installation_id,
                intent.pairing_id,
                intent.route_id,
                intent.signer_public_key,
                intent.lifecycle_generation,
                intent.installation_binding_generation,
                intent.authority_generation,
                intent.correlation_id,
                intent.expires_at,
            ],
        )
        .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
    drop(intent);
    Ok(())
}

pub(crate) fn mark_consumed(
    transaction: &Transaction<'_>,
    challenge_ref: &str,
    action: &HouseholdAuthorityAction,
    nonce_ref: &str,
) -> Result<(), ParentPresenceStoreError> {
    if matches!(action, HouseholdAuthorityAction::RegisterLanSignerAnchor) {
        let changed = transaction
            .execute(
                "UPDATE parent_step_up_intents SET lifecycle_state = 'consumed'
                 WHERE challenge_ref = ?1 AND nonce_ref = ?2 AND lifecycle_state = 'issued'",
                params![challenge_ref, nonce_ref],
            )
            .map_err(|_error| ParentPresenceStoreError::Unavailable)?;
        if changed != 1 {
            return Err(ParentPresenceStoreError::IntegrityRejected);
        }
    }
    Ok(())
}
