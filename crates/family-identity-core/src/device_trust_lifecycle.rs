//! Durable, runtime-owned lifecycle authority for trusted parent devices.
//!
//! This is deliberately separate from an unsealing credential.  A credential
//! identifies what was sealed; this repository answers whether that exact
//! registration is still authorised *now*.

use std::path::Path;

use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use serde::{Deserialize, Serialize};

use crate::trust_bootstrap::current_authority::{
    CurrentParentDeviceTrustAuthority, CurrentParentDeviceTrustAuthorityError,
    CurrentParentDeviceTrustAuthoritySource,
};

const TRUSTED: &str = "trusted";
const REVOKED: &str = "revoked";
const RESET_REQUIRED: &str = "reset-required";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceTrustLifecycleState {
    Trusted,
    Revoked,
    ResetRequired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTrustLifecycleEvent {
    pub correlation_id: String,
    pub kind: DeviceTrustLifecycleEventKind,
    pub lifecycle_generation: u64,
    pub installation_binding_generation: u64,
    pub redaction: DeviceTrustLifecycleRedaction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceTrustLifecycleEventKind {
    Registered,
    Revoked,
    ResetRequired,
    Repaired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceTrustLifecycleRedaction {
    SensitiveIdentifiersOmitted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceTrustLifecycleError {
    Unavailable,
    DuplicateRegistration,
    RegistrationMissing,
    InvalidIdentity,
    InvalidGeneration,
}

/// The canonical local authority owner.  It persists both the lifecycle epoch
/// and a non-restored installation epoch.  Any revoke, reset, or re-pair bumps
/// the lifecycle epoch, making a copied/stale sealed record fail closed.
pub struct DeviceTrustLifecycleRepository {
    connection: Connection,
}

impl DeviceTrustLifecycleRepository {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DeviceTrustLifecycleError> {
        let connection =
            Connection::open(path).map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        connection.execute_batch(
            "PRAGMA foreign_keys = ON; PRAGMA journal_mode = DELETE; PRAGMA synchronous = FULL;
             CREATE TABLE IF NOT EXISTS device_trust_lifecycle (
                trust_subject TEXT NOT NULL,
                device_ref TEXT NOT NULL,
                lifecycle_state TEXT NOT NULL CHECK (lifecycle_state IN ('trusted','revoked','reset-required')),
                lifecycle_generation INTEGER NOT NULL CHECK (lifecycle_generation > 0),
                installation_binding_generation INTEGER NOT NULL CHECK (installation_binding_generation > 0),
                PRIMARY KEY (trust_subject, device_ref)
             ) STRICT;
             CREATE TABLE IF NOT EXISTS device_trust_lifecycle_outbox (
                sequence INTEGER PRIMARY KEY AUTOINCREMENT,
                correlation_id TEXT NOT NULL UNIQUE,
                event_json TEXT NOT NULL,
                delivery_state TEXT NOT NULL CHECK (delivery_state IN ('pending','delivered'))
             ) STRICT;",
        ).map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        Ok(Self { connection })
    }

    pub fn register_parent_device(
        &mut self,
        trust_subject: &str,
        device_ref: &str,
        installation_binding_generation: u64,
        correlation_id: &str,
    ) -> Result<(), DeviceTrustLifecycleError> {
        self.validate_identifiers(trust_subject, device_ref, correlation_id)?;
        self.require_generation(installation_binding_generation)?;
        let transaction = self.transaction()?;
        let existing = transaction.query_row(
            "SELECT lifecycle_state FROM device_trust_lifecycle WHERE trust_subject = ?1 AND device_ref = ?2",
            params![trust_subject, device_ref], |row| row.get::<_, String>(0),
        ).optional().map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        if existing.is_some() {
            return Err(DeviceTrustLifecycleError::DuplicateRegistration);
        }
        transaction.execute(
            "INSERT INTO device_trust_lifecycle (trust_subject, device_ref, lifecycle_state, lifecycle_generation, installation_binding_generation) VALUES (?1, ?2, ?3, 1, ?4)",
            params![
                trust_subject,
                device_ref,
                TRUSTED,
                to_sql_generation(installation_binding_generation)?
            ],
        ).map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        Self::insert_event(
            &transaction,
            correlation_id,
            DeviceTrustLifecycleEventKind::Registered,
            1,
            installation_binding_generation,
        )?;
        transaction
            .commit()
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)
    }

    pub fn revoke_or_reset(
        &mut self,
        trust_subject: &str,
        device_ref: &str,
        reset_required: bool,
        correlation_id: &str,
    ) -> Result<(), DeviceTrustLifecycleError> {
        self.validate_identifiers(trust_subject, device_ref, correlation_id)?;
        let transaction = self.transaction()?;
        let row = Self::row(&transaction, trust_subject, device_ref)?;
        let Some((generation, installation_generation)) = row else {
            return Err(DeviceTrustLifecycleError::RegistrationMissing);
        };
        let next_generation = generation
            .checked_add(1)
            .ok_or(DeviceTrustLifecycleError::InvalidGeneration)?;
        let state = if reset_required {
            RESET_REQUIRED
        } else {
            REVOKED
        };
        transaction.execute(
            "UPDATE device_trust_lifecycle SET lifecycle_state = ?3, lifecycle_generation = ?4 WHERE trust_subject = ?1 AND device_ref = ?2",
            params![
                trust_subject,
                device_ref,
                state,
                to_sql_generation(next_generation)?
            ],
        ).map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        let kind = if reset_required {
            DeviceTrustLifecycleEventKind::ResetRequired
        } else {
            DeviceTrustLifecycleEventKind::Revoked
        };
        Self::insert_event(
            &transaction,
            correlation_id,
            kind,
            next_generation,
            installation_generation,
        )?;
        transaction
            .commit()
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)
    }

    /// Re-pair is a new lifecycle authority, never a restoration of an old
    /// install binding.  Callers must supply a strictly newer local install epoch.
    pub fn repair_with_new_installation(
        &mut self,
        trust_subject: &str,
        device_ref: &str,
        installation_binding_generation: u64,
        correlation_id: &str,
    ) -> Result<(), DeviceTrustLifecycleError> {
        self.validate_identifiers(trust_subject, device_ref, correlation_id)?;
        self.require_generation(installation_binding_generation)?;
        let transaction = self.transaction()?;
        let Some((generation, prior_installation)) =
            Self::row(&transaction, trust_subject, device_ref)?
        else {
            return Err(DeviceTrustLifecycleError::RegistrationMissing);
        };
        if installation_binding_generation <= prior_installation {
            return Err(DeviceTrustLifecycleError::InvalidGeneration);
        }
        let next_generation = generation
            .checked_add(1)
            .ok_or(DeviceTrustLifecycleError::InvalidGeneration)?;
        transaction.execute(
            "UPDATE device_trust_lifecycle SET lifecycle_state = ?3, lifecycle_generation = ?4, installation_binding_generation = ?5 WHERE trust_subject = ?1 AND device_ref = ?2",
            params![
                trust_subject,
                device_ref,
                TRUSTED,
                to_sql_generation(next_generation)?,
                to_sql_generation(installation_binding_generation)?
            ],
        ).map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        Self::insert_event(
            &transaction,
            correlation_id,
            DeviceTrustLifecycleEventKind::Repaired,
            next_generation,
            installation_binding_generation,
        )?;
        transaction
            .commit()
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)
    }

    pub fn pending_events(
        &self,
    ) -> Result<Vec<DeviceTrustLifecycleEvent>, DeviceTrustLifecycleError> {
        let mut statement = self.connection.prepare("SELECT event_json FROM device_trust_lifecycle_outbox WHERE delivery_state = 'pending' ORDER BY sequence")
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        let events = statement
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
            .map(|row| {
                row.map_err(|_error| DeviceTrustLifecycleError::Unavailable)
                    .and_then(|json| {
                        serde_json::from_str(&json)
                            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)
                    })
            })
            .collect();
        events
    }

    pub fn mark_delivered(
        &mut self,
        correlation_id: &str,
    ) -> Result<(), DeviceTrustLifecycleError> {
        let changed = self.connection.execute("UPDATE device_trust_lifecycle_outbox SET delivery_state = 'delivered' WHERE correlation_id = ?1 AND delivery_state = 'pending'", [correlation_id])
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        if changed == 1 {
            Ok(())
        } else {
            Err(DeviceTrustLifecycleError::RegistrationMissing)
        }
    }

    fn transaction(&mut self) -> Result<rusqlite::Transaction<'_>, DeviceTrustLifecycleError> {
        self.connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)
    }

    fn row(
        transaction: &rusqlite::Transaction<'_>,
        subject: &str,
        device: &str,
    ) -> Result<Option<(u64, u64)>, DeviceTrustLifecycleError> {
        let row = transaction
            .query_row(
                "SELECT lifecycle_generation, installation_binding_generation FROM device_trust_lifecycle WHERE trust_subject = ?1 AND device_ref = ?2",
                params![subject, device],
                |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)),
            )
            .optional()
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        row.map(|(generation, installation)| {
            Ok((
                from_sql_generation(generation)?,
                from_sql_generation(installation)?,
            ))
        })
        .transpose()
    }

    fn insert_event(
        transaction: &rusqlite::Transaction<'_>,
        correlation_id: &str,
        kind: DeviceTrustLifecycleEventKind,
        lifecycle_generation: u64,
        installation_binding_generation: u64,
    ) -> Result<(), DeviceTrustLifecycleError> {
        let event = DeviceTrustLifecycleEvent {
            correlation_id: correlation_id.to_owned(),
            kind,
            lifecycle_generation,
            installation_binding_generation,
            redaction: DeviceTrustLifecycleRedaction::SensitiveIdentifiersOmitted,
        };
        let json = serde_json::to_string(&event)
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        transaction.execute("INSERT INTO device_trust_lifecycle_outbox (correlation_id, event_json, delivery_state) VALUES (?1, ?2, 'pending')", params![correlation_id, json])
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        Ok(())
    }

    fn validate_identifiers(
        &self,
        subject: &str,
        device: &str,
        correlation: &str,
    ) -> Result<(), DeviceTrustLifecycleError> {
        (!subject.trim().is_empty() && !device.trim().is_empty() && !correlation.trim().is_empty())
            .then_some(())
            .ok_or(DeviceTrustLifecycleError::InvalidIdentity)
    }

    fn require_generation(&self, generation: u64) -> Result<(), DeviceTrustLifecycleError> {
        (generation > 0)
            .then_some(())
            .ok_or(DeviceTrustLifecycleError::InvalidGeneration)
    }
}

fn to_sql_generation(generation: u64) -> Result<i64, DeviceTrustLifecycleError> {
    i64::try_from(generation).map_err(|_error| DeviceTrustLifecycleError::InvalidGeneration)
}

fn from_sql_generation(generation: i64) -> Result<u64, DeviceTrustLifecycleError> {
    u64::try_from(generation).map_err(|_error| DeviceTrustLifecycleError::InvalidGeneration)
}

impl CurrentParentDeviceTrustAuthoritySource for DeviceTrustLifecycleRepository {
    fn current_authorized_parent_device(
        &self,
        trust_subject: &str,
        device_ref: &str,
    ) -> Result<CurrentParentDeviceTrustAuthority, CurrentParentDeviceTrustAuthorityError> {
        let row = self.connection.query_row("SELECT lifecycle_state, lifecycle_generation, installation_binding_generation FROM device_trust_lifecycle WHERE trust_subject = ?1 AND device_ref = ?2", params![trust_subject, device_ref], |row| Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?, row.get::<_, i64>(2)?))).optional().ok().flatten();
        let Some((state, lifecycle_generation, installation_binding_generation)) = row else {
            return Err(CurrentParentDeviceTrustAuthorityError::NotTrusted);
        };
        if state != TRUSTED {
            return Err(CurrentParentDeviceTrustAuthorityError::NotTrusted);
        }
        Ok(CurrentParentDeviceTrustAuthority {
            lifecycle_generation: u64::try_from(lifecycle_generation)
                .map_err(|_error| CurrentParentDeviceTrustAuthorityError::NotTrusted)?,
            installation_binding_generation: u64::try_from(installation_binding_generation)
                .map_err(|_error| CurrentParentDeviceTrustAuthorityError::NotTrusted)?,
        })
    }
}
