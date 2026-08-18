use std::fmt;
use std::path::Path;
use std::time::Duration;

use rusqlite::{params, Connection, TransactionBehavior};

use super::household_authority_runtime_cas_recovery::{
    self as recovery, HouseholdAuthorityRuntimeCasError, HouseholdAuthorityRuntimeEffectKey,
    HouseholdAuthorityRuntimeEffectRecord, HouseholdAuthorityRuntimeEffectRecoveryHandle,
    HouseholdAuthorityRuntimeEffectStatus, StoredEffect,
};
use super::household_authority_runtime_cas_schema;
use super::{
    ConsumedParentStepUp, CurrentChildDeviceTrustBinding, CurrentHouseholdCapability,
    CurrentHouseholdControllerLease, HouseholdAuthorityRuntimeAuthorization,
    HouseholdAuthorityRuntimeCasFence, HouseholdAuthorityRuntimeEffectAuthorization,
    HouseholdAuthorityRuntimeFailure,
};
use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

/// Account-owned durable coordinator for one opaque household effect. The SQLite file is the
/// custody boundary: prepared/reserved rows never become positive authority, and only a committed
/// row can be recovered after restart. Its transaction is atomic for this durable row and nonce;
/// this packet does not claim a same-transaction comparison across external owner stores.
pub struct SqliteHouseholdAuthorityRuntimeCasRepository {
    connection: Connection,
}

impl fmt::Debug for SqliteHouseholdAuthorityRuntimeCasRepository {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SqliteHouseholdAuthorityRuntimeCasRepository")
            .field("storage", &"durable-sqlite-omitted")
            .finish()
    }
}

impl SqliteHouseholdAuthorityRuntimeCasRepository {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, HouseholdAuthorityRuntimeCasError> {
        let path = path.as_ref();
        if path
            .to_str()
            .is_some_and(|value| value == ":memory:" || value.contains(":memory:"))
        {
            return Err(HouseholdAuthorityRuntimeCasError::ConnectionUnavailable);
        }
        let connection = Connection::open(path)
            .map_err(|_| HouseholdAuthorityRuntimeCasError::ConnectionUnavailable)?;
        connection
            .busy_timeout(Duration::from_secs(5))
            .map_err(|_| HouseholdAuthorityRuntimeCasError::ConnectionUnavailable)?;
        connection
            .execute_batch(
                "PRAGMA foreign_keys = ON;
                 PRAGMA journal_mode = DELETE;
                 PRAGMA synchronous = FULL;",
            )
            .map_err(|_| HouseholdAuthorityRuntimeCasError::ConnectionUnavailable)?;
        let initial_epoch = recovery::initial_epoch_millis()?;
        household_authority_runtime_cas_schema::install(&connection, initial_epoch)
            .map_err(|_| HouseholdAuthorityRuntimeCasError::InvalidSchema)?;
        household_authority_runtime_cas_schema::validate(&connection)
            .map_err(|_| HouseholdAuthorityRuntimeCasError::InvalidSchema)?;
        let mut repository = Self { connection };
        recovery::reconcile(&mut repository.connection)?;
        household_authority_runtime_cas_schema::validate(&repository.connection)
            .map_err(|_| HouseholdAuthorityRuntimeCasError::InvalidSchema)?;
        Ok(repository)
    }

    /// Record a terminal Account-owned custody row for one already-consumed effect. The record
    /// is idempotent handoff bookkeeping only; it is not an owner-sealed completion and must never
    /// be accepted as proof that a downstream side effect happened. A future downstream owner
    /// must return its own opaque, owner-sealed completion at a real seam before such completion
    /// can be added here; this packet deliberately has no caller-byte completion API.
    pub fn commit(
        &mut self,
        effect: super::HouseholdAuthorityRuntimeConsumedEffect,
    ) -> Result<HouseholdAuthorityRuntimeEffectRecord, HouseholdAuthorityRuntimeCasError> {
        self.commit_key(consumed_key(effect)?)
    }

    /// Abort one consumed handoff. An ambiguous row remains fail-closed and cannot be aborted
    /// into a caller-selected terminal state.
    pub fn abort(
        &mut self,
        effect: super::HouseholdAuthorityRuntimeConsumedEffect,
    ) -> Result<HouseholdAuthorityRuntimeEffectStatus, HouseholdAuthorityRuntimeCasError> {
        self.abort_key(consumed_key(effect)?)
    }

    pub fn status(
        &self,
        receipt: &HouseholdAuthorityRuntimeEffectAuthorization,
    ) -> Result<HouseholdAuthorityRuntimeEffectStatus, HouseholdAuthorityRuntimeCasError> {
        let handle = receipt.recovery_handle()?;
        recovery::load_effect(&self.connection, &handle.operation_digest)?
            .ok_or(HouseholdAuthorityRuntimeCasError::Missing)
            .and_then(|stored| {
                ensure_receipt_matches(receipt, &stored)?;
                Ok(stored.status)
            })
    }

    /// Recover only a committed bookkeeping record for the same owner-issued receipt. Prepared,
    /// reserved, and ambiguous rows are never retried or converted into a fresh receipt.
    pub fn recover(
        &self,
        handle: &HouseholdAuthorityRuntimeEffectRecoveryHandle,
    ) -> Result<HouseholdAuthorityRuntimeEffectRecord, HouseholdAuthorityRuntimeCasError> {
        let stored = recovery::load_effect(&self.connection, &handle.operation_digest)?
            .ok_or(HouseholdAuthorityRuntimeCasError::Missing)?;
        ensure_handle_matches(handle, &stored)?;
        recover_committed(stored)
    }

    fn commit_key(
        &mut self,
        key: HouseholdAuthorityRuntimeEffectKey,
    ) -> Result<HouseholdAuthorityRuntimeEffectRecord, HouseholdAuthorityRuntimeCasError> {
        let handoff_digest = household_authority_runtime_cas_schema::consumption_record_digest(
            &key.operation_digest,
        );
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| HouseholdAuthorityRuntimeCasError::ConnectionUnavailable)?;
        let stored = recovery::load_effect(&transaction, &key.operation_digest)?
            .ok_or(HouseholdAuthorityRuntimeCasError::Missing)?;
        ensure_key_matches(&key, &stored)?;
        match stored.status {
            HouseholdAuthorityRuntimeEffectStatus::Reserved => {
                let epoch = recovery::next_epoch(&transaction)?;
                let changed = transaction
                    .execute(
                        "UPDATE household_authority_runtime_effect
                         SET status = 'committed', handoff_digest = ?1,
                             updated_at_epoch_millis = ?2,
                             recovery_epoch = recovery_epoch + 1
                         WHERE operation_digest = ?3 AND consumption_nonce = ?4
                           AND target_digest = ?5 AND status = 'reserved'",
                        params![
                            handoff_digest.as_slice(),
                            epoch,
                            key.operation_digest.as_slice(),
                            key.consumption_nonce.as_slice(),
                            key.target_digest.as_slice(),
                        ],
                    )
                    .map_err(|_| HouseholdAuthorityRuntimeCasError::ConnectionUnavailable)?;
                if changed != 1 {
                    return Err(HouseholdAuthorityRuntimeCasError::AmbiguousRecovery);
                }
                transaction
                    .commit()
                    .map_err(|_| HouseholdAuthorityRuntimeCasError::ConnectionUnavailable)?;
                Ok(HouseholdAuthorityRuntimeEffectRecord::from_stored(
                    HouseholdAuthorityRuntimeEffectStatus::Committed,
                    key.operation_digest,
                    handoff_digest,
                ))
            }
            HouseholdAuthorityRuntimeEffectStatus::Committed => {
                if stored.handoff_digest == Some(handoff_digest) {
                    Ok(HouseholdAuthorityRuntimeEffectRecord::from_stored(
                        HouseholdAuthorityRuntimeEffectStatus::Committed,
                        key.operation_digest,
                        handoff_digest,
                    ))
                } else {
                    Err(HouseholdAuthorityRuntimeCasError::CorruptRow)
                }
            }
            HouseholdAuthorityRuntimeEffectStatus::Prepared
            | HouseholdAuthorityRuntimeEffectStatus::Ambiguous => {
                Err(HouseholdAuthorityRuntimeCasError::AmbiguousRecovery)
            }
            HouseholdAuthorityRuntimeEffectStatus::Aborted => {
                Err(HouseholdAuthorityRuntimeCasError::Aborted)
            }
        }
    }

    fn abort_key(
        &mut self,
        key: HouseholdAuthorityRuntimeEffectKey,
    ) -> Result<HouseholdAuthorityRuntimeEffectStatus, HouseholdAuthorityRuntimeCasError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| HouseholdAuthorityRuntimeCasError::ConnectionUnavailable)?;
        let stored = recovery::load_effect(&transaction, &key.operation_digest)?
            .ok_or(HouseholdAuthorityRuntimeCasError::Missing)?;
        ensure_key_matches(&key, &stored)?;
        match stored.status {
            HouseholdAuthorityRuntimeEffectStatus::Prepared
            | HouseholdAuthorityRuntimeEffectStatus::Reserved => {
                let epoch = recovery::next_epoch(&transaction)?;
                let changed = transaction
                    .execute(
                        "UPDATE household_authority_runtime_effect
                         SET status = 'aborted', updated_at_epoch_millis = ?1,
                             recovery_epoch = recovery_epoch + 1
                         WHERE operation_digest = ?2 AND consumption_nonce = ?3
                           AND target_digest = ?4 AND status IN ('prepared', 'reserved')",
                        params![
                            epoch,
                            key.operation_digest.as_slice(),
                            key.consumption_nonce.as_slice(),
                            key.target_digest.as_slice(),
                        ],
                    )
                    .map_err(|_| HouseholdAuthorityRuntimeCasError::ConnectionUnavailable)?;
                if changed != 1 {
                    return Err(HouseholdAuthorityRuntimeCasError::AmbiguousRecovery);
                }
                transaction
                    .commit()
                    .map_err(|_| HouseholdAuthorityRuntimeCasError::ConnectionUnavailable)?;
                Ok(HouseholdAuthorityRuntimeEffectStatus::Aborted)
            }
            HouseholdAuthorityRuntimeEffectStatus::Aborted => {
                Ok(HouseholdAuthorityRuntimeEffectStatus::Aborted)
            }
            HouseholdAuthorityRuntimeEffectStatus::Committed => {
                Err(HouseholdAuthorityRuntimeCasError::AlreadyCommitted)
            }
            HouseholdAuthorityRuntimeEffectStatus::Ambiguous => {
                Err(HouseholdAuthorityRuntimeCasError::AmbiguousRecovery)
            }
        }
    }
}

impl HouseholdAuthorityRuntimeCasFence for SqliteHouseholdAuthorityRuntimeCasRepository {
    fn compare_and_consume(
        &mut self,
        _authorization: HouseholdAuthorityRuntimeAuthorization,
        _current_account_authority: VerifiedAccountIdentityAuthority,
        _current_device_binding: CurrentChildDeviceTrustBinding,
        _current_capability: Option<CurrentHouseholdCapability>,
        _current_controller_lease: Option<CurrentHouseholdControllerLease>,
        _current_parent_step_up: Option<ConsumedParentStepUp>,
        _consumption_nonce: &[u8; 32],
    ) -> Result<HouseholdAuthorityRuntimeEffectAuthorization, HouseholdAuthorityRuntimeFailure>
    {
        // Snapshot-only inputs cannot honestly provide a cross-owner transactional fence. Keep
        // this production seam fail-closed until an owner reservation is composed here.
        Err(HouseholdAuthorityRuntimeFailure::RuntimeFenceUnavailable)
    }
}

fn consumed_key(
    effect: super::HouseholdAuthorityRuntimeConsumedEffect,
) -> Result<HouseholdAuthorityRuntimeEffectKey, HouseholdAuthorityRuntimeCasError> {
    let target_digest = household_authority_runtime_cas_schema::target_digest(&effect.target)?;
    let operation_digest = household_authority_runtime_cas_schema::operation_digest(
        &target_digest,
        &effect.consumption_nonce,
    );
    Ok(HouseholdAuthorityRuntimeEffectKey {
        target: effect.target,
        consumption_nonce: effect.consumption_nonce,
        target_digest,
        operation_digest,
    })
}

fn ensure_key_matches(
    key: &HouseholdAuthorityRuntimeEffectKey,
    stored: &StoredEffect,
) -> Result<(), HouseholdAuthorityRuntimeCasError> {
    if stored.operation_digest != key.operation_digest
        || stored.consumption_nonce != key.consumption_nonce
        || stored.target_digest != key.target_digest
    {
        return Err(HouseholdAuthorityRuntimeCasError::TargetMismatch);
    }
    let record = household_authority_runtime_cas_schema::target_record(&key.target)?;
    (stored.target_record == record)
        .then_some(())
        .ok_or(HouseholdAuthorityRuntimeCasError::TargetMismatch)
}

fn ensure_receipt_matches(
    receipt: &HouseholdAuthorityRuntimeEffectAuthorization,
    stored: &StoredEffect,
) -> Result<(), HouseholdAuthorityRuntimeCasError> {
    let target_digest = household_authority_runtime_cas_schema::target_digest(&receipt.target)?;
    let operation_digest = household_authority_runtime_cas_schema::operation_digest(
        &target_digest,
        &receipt.consumption_nonce,
    );
    if stored.operation_digest != operation_digest
        || stored.consumption_nonce != receipt.consumption_nonce
        || stored.target_digest != target_digest
    {
        return Err(HouseholdAuthorityRuntimeCasError::TargetMismatch);
    }
    let record = household_authority_runtime_cas_schema::target_record(&receipt.target)?;
    (stored.target_record == record)
        .then_some(())
        .ok_or(HouseholdAuthorityRuntimeCasError::TargetMismatch)
}

fn ensure_handle_matches(
    handle: &HouseholdAuthorityRuntimeEffectRecoveryHandle,
    stored: &StoredEffect,
) -> Result<(), HouseholdAuthorityRuntimeCasError> {
    (handle.operation_digest
        == household_authority_runtime_cas_schema::operation_digest(
            &handle.target_digest,
            &handle.consumption_nonce,
        )
        && stored.operation_digest == handle.operation_digest
        && stored.target_digest == handle.target_digest
        && stored.consumption_nonce == handle.consumption_nonce)
        .then_some(())
        .ok_or(HouseholdAuthorityRuntimeCasError::TargetMismatch)
}

fn recover_committed(
    stored: StoredEffect,
) -> Result<HouseholdAuthorityRuntimeEffectRecord, HouseholdAuthorityRuntimeCasError> {
    match (stored.status, stored.handoff_digest) {
        (HouseholdAuthorityRuntimeEffectStatus::Committed, Some(handoff_digest)) => {
            Ok(HouseholdAuthorityRuntimeEffectRecord::from_stored(
                HouseholdAuthorityRuntimeEffectStatus::Committed,
                stored.operation_digest,
                handoff_digest,
            ))
        }
        (HouseholdAuthorityRuntimeEffectStatus::Ambiguous, _)
        | (HouseholdAuthorityRuntimeEffectStatus::Prepared, _)
        | (HouseholdAuthorityRuntimeEffectStatus::Reserved, _) => {
            Err(HouseholdAuthorityRuntimeCasError::AmbiguousRecovery)
        }
        (HouseholdAuthorityRuntimeEffectStatus::Aborted, _) => {
            Err(HouseholdAuthorityRuntimeCasError::Aborted)
        }
        (HouseholdAuthorityRuntimeEffectStatus::Committed, None) => {
            Err(HouseholdAuthorityRuntimeCasError::CorruptRow)
        }
    }
}
