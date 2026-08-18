use std::fmt;

use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension, Transaction, TransactionBehavior};
use serde::{Deserialize, Deserializer, Serialize};

use super::household_authority_runtime_cas_schema::{self, TargetRecord};
use super::{
    HouseholdAuthorityRuntimeEffectAuthorization, HouseholdAuthorityRuntimeEffectTarget,
    HouseholdAuthorityRuntimeFailure,
};

/// Durable CAS failures are deliberately coarse at the runtime fence boundary. The repository
/// keeps the detailed distinction so recovery callers cannot mistake corruption or ambiguity for
/// an ordinary denial.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HouseholdAuthorityRuntimeCasError {
    AuthorityRejected(HouseholdAuthorityRuntimeFailure),
    ConnectionUnavailable,
    InvalidSchema,
    CorruptRow,
    InvalidTarget,
    InvalidGeneration,
    InvalidTimestamp,
    ClockUnavailable,
    Missing,
    TargetMismatch,
    Conflict,
    AlreadyCommitted,
    AmbiguousRecovery,
    Aborted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HouseholdAuthorityRuntimeEffectStatus {
    Prepared,
    Reserved,
    Committed,
    Aborted,
    Ambiguous,
}

/// An opaque key derived from a consumed effect for Account-owned row bookkeeping. It is not a
/// preparation API and cannot mint, serialize, or rebind an authorization.
pub(super) struct HouseholdAuthorityRuntimeEffectKey {
    pub(super) target: HouseholdAuthorityRuntimeEffectTarget,
    pub(super) consumption_nonce: [u8; 32],
    pub(super) target_digest: [u8; 32],
    pub(super) operation_digest: [u8; 32],
}

impl fmt::Debug for HouseholdAuthorityRuntimeEffectKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HouseholdAuthorityRuntimeEffectKey")
            .field("operation_digest", &"opaque")
            .field("target", &"opaque")
            .finish()
    }
}

/// Durable lookup-only handle derived from an owner-issued receipt. It is safe to serialize for
/// restart correlation because it carries no authority and cannot mint, replay, or rebind a
/// receipt. Deserialization validates the operation binding before a lookup is attempted.
#[derive(Clone, Serialize)]
pub struct HouseholdAuthorityRuntimeEffectRecoveryHandle {
    version: u8,
    pub(super) operation_digest: [u8; 32],
    pub(super) target_digest: [u8; 32],
    pub(super) consumption_nonce: [u8; 32],
}

impl fmt::Debug for HouseholdAuthorityRuntimeEffectRecoveryHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HouseholdAuthorityRuntimeEffectRecoveryHandle")
            .field("version", &self.version)
            .field("operation_digest", &"opaque")
            .field("target_digest", &"opaque")
            .field("consumption_nonce", &"opaque")
            .finish()
    }
}

impl<'de> Deserialize<'de> for HouseholdAuthorityRuntimeEffectRecoveryHandle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Wire {
            version: u8,
            operation_digest: [u8; 32],
            target_digest: [u8; 32],
            consumption_nonce: [u8; 32],
        }

        let wire = Wire::deserialize(deserializer)?;
        if wire.version != 1 {
            return Err(serde::de::Error::custom(
                "unsupported recovery handle version",
            ));
        }
        let operation_digest = wire.operation_digest;
        let target_digest = wire.target_digest;
        let consumption_nonce = wire.consumption_nonce;
        if operation_digest
            != household_authority_runtime_cas_schema::operation_digest(
                &target_digest,
                &consumption_nonce,
            )
        {
            return Err(serde::de::Error::custom("recovery handle binding mismatch"));
        }
        Ok(Self {
            version: 1,
            operation_digest,
            target_digest,
            consumption_nonce,
        })
    }
}

/// Durable bookkeeping for an effect row. This record is deliberately not an authority, a
/// receipt, or proof that a downstream side effect happened. Consumers must never use it to
/// unlock execution or advance state that requires an owner-sealed completion.
pub struct HouseholdAuthorityRuntimeEffectRecord {
    status: HouseholdAuthorityRuntimeEffectStatus,
    operation_digest: [u8; 32],
    handoff_digest: [u8; 32],
}

impl fmt::Debug for HouseholdAuthorityRuntimeEffectRecord {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HouseholdAuthorityRuntimeEffectRecord")
            .field("status", &self.status)
            .field("operation_digest", &"opaque")
            .field("handoff_digest", &"opaque")
            .finish()
    }
}

impl HouseholdAuthorityRuntimeEffectRecord {
    /// Returns durable row state only. It is never an execution proof or authority decision.
    pub fn status(&self) -> HouseholdAuthorityRuntimeEffectStatus {
        self.status
    }

    pub(super) fn from_stored(
        status: HouseholdAuthorityRuntimeEffectStatus,
        operation_digest: [u8; 32],
        handoff_digest: [u8; 32],
    ) -> Self {
        Self {
            status,
            operation_digest,
            handoff_digest,
        }
    }
}

impl HouseholdAuthorityRuntimeEffectAuthorization {
    pub fn recovery_handle(
        &self,
    ) -> Result<HouseholdAuthorityRuntimeEffectRecoveryHandle, HouseholdAuthorityRuntimeCasError>
    {
        let target_digest = household_authority_runtime_cas_schema::target_digest(&self.target)?;
        Ok(HouseholdAuthorityRuntimeEffectRecoveryHandle {
            version: 1,
            operation_digest: household_authority_runtime_cas_schema::operation_digest(
                &target_digest,
                &self.consumption_nonce,
            ),
            target_digest,
            consumption_nonce: self.consumption_nonce,
        })
    }
}

impl super::HouseholdAuthorityRuntimeConsumedEffect {
    pub fn recovery_handle(
        &self,
    ) -> Result<HouseholdAuthorityRuntimeEffectRecoveryHandle, HouseholdAuthorityRuntimeCasError>
    {
        let target_digest = household_authority_runtime_cas_schema::target_digest(&self.target)?;
        Ok(HouseholdAuthorityRuntimeEffectRecoveryHandle {
            version: 1,
            operation_digest: household_authority_runtime_cas_schema::operation_digest(
                &target_digest,
                &self.consumption_nonce,
            ),
            target_digest,
            consumption_nonce: self.consumption_nonce,
        })
    }
}

#[derive(Debug)]
pub(super) struct StoredEffect {
    pub(super) operation_digest: [u8; 32],
    pub(super) consumption_nonce: [u8; 32],
    pub(super) target_digest: [u8; 32],
    pub(super) target_record: TargetRecord,
    pub(super) status: HouseholdAuthorityRuntimeEffectStatus,
    pub(super) handoff_digest: Option<[u8; 32]>,
    pub(super) created_at_epoch_millis: i64,
    pub(super) updated_at_epoch_millis: i64,
    pub(super) recovery_epoch: i64,
}

pub(super) fn initial_epoch_millis() -> Result<i64, HouseholdAuthorityRuntimeCasError> {
    let value = Utc::now().timestamp_millis();
    (value > 0)
        .then_some(value)
        .ok_or(HouseholdAuthorityRuntimeCasError::ClockUnavailable)
}

pub(super) fn next_epoch(
    transaction: &Transaction<'_>,
) -> Result<i64, HouseholdAuthorityRuntimeCasError> {
    let current = transaction
        .query_row(
            "SELECT last_epoch_millis FROM household_authority_runtime_cas_clock WHERE clock_id = 1",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_| HouseholdAuthorityRuntimeCasError::ConnectionUnavailable)?;
    if current <= 0 {
        return Err(HouseholdAuthorityRuntimeCasError::CorruptRow);
    }
    let wall_clock = initial_epoch_millis()?;
    let monotonic = current
        .checked_add(1)
        .ok_or(HouseholdAuthorityRuntimeCasError::ClockUnavailable)?;
    let next = wall_clock.max(monotonic);
    transaction
        .execute(
            "UPDATE household_authority_runtime_cas_clock
             SET last_epoch_millis = ?1 WHERE clock_id = 1",
            params![next],
        )
        .map_err(|_| HouseholdAuthorityRuntimeCasError::ConnectionUnavailable)?;
    Ok(next)
}

pub(super) fn reconcile(
    connection: &mut Connection,
) -> Result<(), HouseholdAuthorityRuntimeCasError> {
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| HouseholdAuthorityRuntimeCasError::ConnectionUnavailable)?;
    let pending = transaction
        .query_row(
            "SELECT count(*) FROM household_authority_runtime_effect
             WHERE status IN ('prepared', 'reserved')",
            [],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_| HouseholdAuthorityRuntimeCasError::ConnectionUnavailable)?;
    if pending == 0 {
        transaction
            .commit()
            .map_err(|_| HouseholdAuthorityRuntimeCasError::ConnectionUnavailable)?;
        return Ok(());
    }
    let overflowing = transaction
        .query_row(
            "SELECT count(*) FROM household_authority_runtime_effect
             WHERE status IN ('prepared', 'reserved') AND recovery_epoch = ?1",
            params![i64::MAX],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_| HouseholdAuthorityRuntimeCasError::ConnectionUnavailable)?;
    if overflowing != 0 {
        return Err(HouseholdAuthorityRuntimeCasError::ClockUnavailable);
    }
    let epoch = next_epoch(&transaction)?;
    transaction
        .execute(
            "UPDATE household_authority_runtime_effect
             SET status = 'ambiguous', updated_at_epoch_millis = ?1,
                 recovery_epoch = recovery_epoch + 1
             WHERE status IN ('prepared', 'reserved')",
            params![epoch],
        )
        .map_err(|_| HouseholdAuthorityRuntimeCasError::ConnectionUnavailable)?;
    transaction
        .commit()
        .map_err(|_| HouseholdAuthorityRuntimeCasError::ConnectionUnavailable)
}

pub(super) fn load_effect(
    connection: &Connection,
    operation_digest: &[u8; 32],
) -> Result<Option<StoredEffect>, HouseholdAuthorityRuntimeCasError> {
    let row = connection
        .query_row(
            "SELECT operation_digest, consumption_nonce, target_digest, action, provider,
                    device_state, account_authority_generation, session_generation,
                    device_authority_generation, capability_authority_generation,
                    capability_expires_at_epoch_millis, capability_revocation_epoch,
                    controller_lease_authority_generation, controller_lease_expires_at_epoch_millis,
                    controller_lease_revocation_epoch, parent_step_up_authority_generation,
                    parent_step_up_expires_at_epoch_millis, parent_step_up_receipt_epoch,
                    status, handoff_digest, created_at_epoch_millis, updated_at_epoch_millis,
                    recovery_epoch
             FROM household_authority_runtime_effect WHERE operation_digest = ?1",
            params![operation_digest.as_slice()],
            decode_row,
        )
        .optional()
        .map_err(|_| HouseholdAuthorityRuntimeCasError::ConnectionUnavailable)?;
    row.map(|row| {
        if row.operation_digest != *operation_digest {
            return Err(HouseholdAuthorityRuntimeCasError::CorruptRow);
        }
        validate_stored_effect(&row)?;
        Ok(row)
    })
    .transpose()
}

pub(super) fn validate_stored_effect(
    row: &StoredEffect,
) -> Result<(), HouseholdAuthorityRuntimeCasError> {
    let optional_valid = |first: Option<i64>, second: Option<i64>, third: Option<i64>| match (
        first, second, third,
    ) {
        (None, None, None) => true,
        (Some(first), Some(second), Some(third)) => first > 0 && second > 0 && third > 0,
        _ => false,
    };
    if row.operation_digest.len() != 32
        || row.consumption_nonce.len() != 32
        || row.target_digest.len() != 32
        || row.operation_digest
            != household_authority_runtime_cas_schema::operation_digest(
                &row.target_digest,
                &row.consumption_nonce,
            )
        || row.created_at_epoch_millis <= 0
        || row.updated_at_epoch_millis < row.created_at_epoch_millis
        || row.recovery_epoch <= 0
        || !(1..=11).contains(&row.target_record.action)
        || !(1..=2).contains(&row.target_record.provider)
        || !(1..=4).contains(&row.target_record.device_state)
        || row.target_record.account_authority_generation <= 0
        || row.target_record.session_generation <= 0
        || row.target_record.device_authority_generation <= 0
        || !optional_valid(
            row.target_record.capability_authority_generation,
            row.target_record.capability_expires_at_epoch_millis,
            row.target_record.capability_revocation_epoch,
        )
        || !optional_valid(
            row.target_record.controller_lease_authority_generation,
            row.target_record.controller_lease_expires_at_epoch_millis,
            row.target_record.controller_lease_revocation_epoch,
        )
        || !optional_valid(
            row.target_record.parent_step_up_authority_generation,
            row.target_record.parent_step_up_expires_at_epoch_millis,
            row.target_record.parent_step_up_receipt_epoch,
        )
    {
        return Err(HouseholdAuthorityRuntimeCasError::CorruptRow);
    }
    match (row.status, row.handoff_digest) {
        (HouseholdAuthorityRuntimeEffectStatus::Committed, Some(handoff))
            if handoff
                == household_authority_runtime_cas_schema::consumption_record_digest(
                    &row.operation_digest,
                ) =>
        {
            Ok(())
        }
        (HouseholdAuthorityRuntimeEffectStatus::Committed, None) | (_, Some(_)) => {
            Err(HouseholdAuthorityRuntimeCasError::CorruptRow)
        }
        _ => Ok(()),
    }
}

pub(super) fn parse_status(
    value: &str,
) -> Result<HouseholdAuthorityRuntimeEffectStatus, HouseholdAuthorityRuntimeCasError> {
    match value {
        "prepared" => Ok(HouseholdAuthorityRuntimeEffectStatus::Prepared),
        "reserved" => Ok(HouseholdAuthorityRuntimeEffectStatus::Reserved),
        "committed" => Ok(HouseholdAuthorityRuntimeEffectStatus::Committed),
        "aborted" => Ok(HouseholdAuthorityRuntimeEffectStatus::Aborted),
        "ambiguous" => Ok(HouseholdAuthorityRuntimeEffectStatus::Ambiguous),
        _ => Err(HouseholdAuthorityRuntimeCasError::CorruptRow),
    }
}

fn decode_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<StoredEffect> {
    let decode = |value: Vec<u8>, index| {
        value.try_into().map_err(|value: Vec<u8>| {
            rusqlite::Error::FromSqlConversionFailure(
                index,
                rusqlite::types::Type::Blob,
                Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("digest length {}", value.len()),
                )),
            )
        })
    };
    let digest = |index| decode(row.get::<_, Vec<u8>>(index)?, index);
    Ok(StoredEffect {
        operation_digest: digest(0)?,
        consumption_nonce: digest(1)?,
        target_digest: digest(2)?,
        target_record: TargetRecord {
            action: row.get(3)?,
            provider: row.get(4)?,
            device_state: row.get(5)?,
            account_authority_generation: row.get(6)?,
            session_generation: row.get(7)?,
            device_authority_generation: row.get(8)?,
            capability_authority_generation: row.get(9)?,
            capability_expires_at_epoch_millis: row.get(10)?,
            capability_revocation_epoch: row.get(11)?,
            controller_lease_authority_generation: row.get(12)?,
            controller_lease_expires_at_epoch_millis: row.get(13)?,
            controller_lease_revocation_epoch: row.get(14)?,
            parent_step_up_authority_generation: row.get(15)?,
            parent_step_up_expires_at_epoch_millis: row.get(16)?,
            parent_step_up_receipt_epoch: row.get(17)?,
        },
        status: parse_status(&row.get::<_, String>(18)?).map_err(|_| {
            rusqlite::Error::FromSqlConversionFailure(
                18,
                rusqlite::types::Type::Text,
                Box::new(std::fmt::Error),
            )
        })?,
        handoff_digest: row
            .get::<_, Option<Vec<u8>>>(19)?
            .map(|value| decode(value, 19))
            .transpose()?,
        created_at_epoch_millis: row.get(20)?,
        updated_at_epoch_millis: row.get(21)?,
        recovery_epoch: row.get(22)?,
    })
}
