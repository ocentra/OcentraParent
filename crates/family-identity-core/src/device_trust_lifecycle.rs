//! Durable, runtime-owned lifecycle authority for trusted parent devices.
//!
//! This is deliberately separate from an unsealing credential.  A credential
//! identifies what was sealed; this repository answers whether that exact
//! registration is still authorised *now*.

use std::path::Path;

use crate::device_trust_lifecycle_authority::{redacted_binding, ExternalLifecycleAuthority};
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use serde::{Deserialize, Serialize};

const TRUSTED: &str = "trusted";
const PENDING: &str = "pending";
const REVOKED: &str = "revoked";
const RESET_REQUIRED: &str = "reset-required";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceTrustLifecycleState {
    Pending,
    Trusted,
    Revoked,
    ResetRequired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceTrustLifecycleEvent {
    pub event_id: String,
    pub household_binding: String,
    pub correlation_id: String,
    pub device_binding: String,
    pub kind: DeviceTrustLifecycleEventKind,
    pub state: DeviceTrustLifecycleState,
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
    Activated,
}

struct LifecycleEventInput<'a> {
    family_id: &'a str,
    trust_subject: &'a str,
    device_ref: &'a str,
    correlation_id: &'a str,
    kind: DeviceTrustLifecycleEventKind,
    state: DeviceTrustLifecycleState,
    lifecycle_generation: u64,
    installation_binding_generation: u64,
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
    RevokedDevice,
    InvalidState,
}

/// The canonical local authority owner. Lifecycle rows are restorable data, so
/// each row is additionally tied to the platform-owned authority sidecar. Any
/// revoke, reset, or re-pair bumps both generations, making a copied/stale
/// database fail closed when it cannot present the current sidecar authority.
pub struct DeviceTrustLifecycleRepository {
    pub(crate) connection: Connection,
    pub(crate) external_authority: ExternalLifecycleAuthority,
}

impl DeviceTrustLifecycleRepository {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DeviceTrustLifecycleError> {
        let path = path.as_ref();
        let external_authority = ExternalLifecycleAuthority::open(path)?;
        let connection =
            Connection::open(path).map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        connection.execute_batch(
            "PRAGMA foreign_keys = ON; PRAGMA journal_mode = DELETE; PRAGMA synchronous = FULL;
             CREATE TABLE IF NOT EXISTS device_trust_lifecycle (
                family_id TEXT NOT NULL,
                trust_subject TEXT NOT NULL,
                device_ref TEXT NOT NULL,
                lifecycle_state TEXT NOT NULL CHECK (lifecycle_state IN ('pending','trusted','revoked','reset-required')),
                lifecycle_generation INTEGER NOT NULL CHECK (lifecycle_generation > 0),
                installation_binding_generation INTEGER NOT NULL CHECK (installation_binding_generation > 0),
                authority_generation INTEGER NOT NULL CHECK (authority_generation > 0),
                PRIMARY KEY (family_id, trust_subject, device_ref)
             ) STRICT;
             CREATE TABLE IF NOT EXISTS device_trust_lifecycle_outbox (
                sequence INTEGER PRIMARY KEY AUTOINCREMENT,
                event_id TEXT NOT NULL UNIQUE,
                correlation_id TEXT NOT NULL,
                event_json TEXT NOT NULL,
                delivery_state TEXT NOT NULL CHECK (delivery_state IN ('pending','delivered'))
             ) STRICT;",
        ).map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        Ok(Self {
            connection,
            external_authority,
        })
    }

    pub fn register_parent_device(
        &mut self,
        family_id: &str,
        trust_subject: &str,
        device_ref: &str,
        installation_binding_generation: u64,
        correlation_id: &str,
    ) -> Result<(), DeviceTrustLifecycleError> {
        self.validate_identifiers(family_id, trust_subject, device_ref, correlation_id)?;
        self.require_generation(installation_binding_generation)?;
        let transaction = self.transaction()?;
        let existing = transaction.query_row(
            "SELECT lifecycle_state FROM device_trust_lifecycle WHERE family_id = ?1 AND trust_subject = ?2 AND device_ref = ?3",
            params![family_id, trust_subject, device_ref], |row| row.get::<_, String>(0),
        ).optional().map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        if existing.is_some() {
            return Err(DeviceTrustLifecycleError::DuplicateRegistration);
        }
        transaction.execute(
            "INSERT INTO device_trust_lifecycle (family_id, trust_subject, device_ref, lifecycle_state, lifecycle_generation, installation_binding_generation, authority_generation) VALUES (?1, ?2, ?3, ?4, 1, ?5, 1)",
            params![
                family_id,
                trust_subject,
                device_ref,
                PENDING,
                to_sql_generation(installation_binding_generation)?
            ],
        ).map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        Self::insert_event(
            &transaction,
            &LifecycleEventInput {
                family_id,
                trust_subject,
                device_ref,
                correlation_id,
                kind: DeviceTrustLifecycleEventKind::Registered,
                state: DeviceTrustLifecycleState::Pending,
                lifecycle_generation: 1,
                installation_binding_generation,
            },
        )?;
        transaction
            .commit()
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        self.set_external_authority(family_id, trust_subject, device_ref, 1)
    }

    pub fn revoke_or_reset(
        &mut self,
        family_id: &str,
        trust_subject: &str,
        device_ref: &str,
        reset_required: bool,
        correlation_id: &str,
    ) -> Result<(), DeviceTrustLifecycleError> {
        self.validate_identifiers(family_id, trust_subject, device_ref, correlation_id)?;
        let transaction = self.transaction()?;
        let row = Self::row(&transaction, family_id, trust_subject, device_ref)?;
        let Some((_current_state, generation, installation_generation, authority_generation)) = row
        else {
            return Err(DeviceTrustLifecycleError::RegistrationMissing);
        };
        let next_generation = generation
            .checked_add(1)
            .ok_or(DeviceTrustLifecycleError::InvalidGeneration)?;
        let next_state = if reset_required {
            RESET_REQUIRED
        } else {
            REVOKED
        };
        transaction.execute(
            "UPDATE device_trust_lifecycle SET lifecycle_state = ?4, lifecycle_generation = ?5, authority_generation = ?6 WHERE family_id = ?1 AND trust_subject = ?2 AND device_ref = ?3",
            params![
                family_id,
                trust_subject,
                device_ref,
                next_state,
                to_sql_generation(next_generation)?,
                to_sql_generation(authority_generation.checked_add(1).ok_or(DeviceTrustLifecycleError::InvalidGeneration)?)?
            ],
        ).map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        let kind = if reset_required {
            DeviceTrustLifecycleEventKind::ResetRequired
        } else {
            DeviceTrustLifecycleEventKind::Revoked
        };
        Self::insert_event(
            &transaction,
            &LifecycleEventInput {
                family_id,
                trust_subject,
                device_ref,
                correlation_id,
                kind,
                state: if reset_required {
                    DeviceTrustLifecycleState::ResetRequired
                } else {
                    DeviceTrustLifecycleState::Revoked
                },
                lifecycle_generation: next_generation,
                installation_binding_generation: installation_generation,
            },
        )?;
        transaction
            .commit()
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        self.set_external_authority(
            family_id,
            trust_subject,
            device_ref,
            authority_generation
                .checked_add(1)
                .ok_or(DeviceTrustLifecycleError::InvalidGeneration)?,
        )
    }

    /// Re-pair is a new lifecycle authority, never a restoration of an old
    /// install binding.  Callers must supply a strictly newer local install epoch.
    pub fn repair_with_new_installation(
        &mut self,
        family_id: &str,
        trust_subject: &str,
        device_ref: &str,
        installation_binding_generation: u64,
        correlation_id: &str,
    ) -> Result<(), DeviceTrustLifecycleError> {
        self.validate_identifiers(family_id, trust_subject, device_ref, correlation_id)?;
        self.require_generation(installation_binding_generation)?;
        let transaction = self.transaction()?;
        let Some((state, generation, prior_installation, authority_generation)) =
            Self::row(&transaction, family_id, trust_subject, device_ref)?
        else {
            return Err(DeviceTrustLifecycleError::RegistrationMissing);
        };
        if state == REVOKED {
            return Err(DeviceTrustLifecycleError::RevokedDevice);
        }
        if state != RESET_REQUIRED {
            return Err(DeviceTrustLifecycleError::InvalidState);
        }
        if installation_binding_generation <= prior_installation {
            return Err(DeviceTrustLifecycleError::InvalidGeneration);
        }
        let next_generation = generation
            .checked_add(1)
            .ok_or(DeviceTrustLifecycleError::InvalidGeneration)?;
        transaction.execute(
            "UPDATE device_trust_lifecycle SET lifecycle_state = ?4, lifecycle_generation = ?5, installation_binding_generation = ?6, authority_generation = ?7 WHERE family_id = ?1 AND trust_subject = ?2 AND device_ref = ?3",
            params![
                family_id,
                trust_subject,
                device_ref,
                TRUSTED,
                to_sql_generation(next_generation)?,
                to_sql_generation(installation_binding_generation)?,
                to_sql_generation(authority_generation.checked_add(1).ok_or(DeviceTrustLifecycleError::InvalidGeneration)?)?
            ],
        ).map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        Self::insert_event(
            &transaction,
            &LifecycleEventInput {
                family_id,
                trust_subject,
                device_ref,
                correlation_id,
                kind: DeviceTrustLifecycleEventKind::Repaired,
                state: DeviceTrustLifecycleState::Trusted,
                lifecycle_generation: next_generation,
                installation_binding_generation,
            },
        )?;
        transaction
            .commit()
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        self.set_external_authority(
            family_id,
            trust_subject,
            device_ref,
            authority_generation
                .checked_add(1)
                .ok_or(DeviceTrustLifecycleError::InvalidGeneration)?,
        )
    }

    /// Mark a pending registration trusted only after platform sealing has durably succeeded.
    pub fn activate_after_sealing(
        &mut self,
        family_id: &str,
        trust_subject: &str,
        device_ref: &str,
        correlation_id: &str,
    ) -> Result<(), DeviceTrustLifecycleError> {
        self.validate_identifiers(family_id, trust_subject, device_ref, correlation_id)?;
        let transaction = self.transaction()?;
        let Some((state, generation, installation_generation, authority_generation)) =
            Self::row(&transaction, family_id, trust_subject, device_ref)?
        else {
            return Err(DeviceTrustLifecycleError::RegistrationMissing);
        };
        if state != PENDING {
            return Err(DeviceTrustLifecycleError::InvalidState);
        }
        let next_generation = generation
            .checked_add(1)
            .ok_or(DeviceTrustLifecycleError::InvalidGeneration)?;
        let next_authority_generation = authority_generation
            .checked_add(1)
            .ok_or(DeviceTrustLifecycleError::InvalidGeneration)?;
        transaction
            .execute(
                "UPDATE device_trust_lifecycle SET lifecycle_state = ?4, lifecycle_generation = ?5, authority_generation = ?6 WHERE family_id = ?1 AND trust_subject = ?2 AND device_ref = ?3",
                params![
                    family_id,
                    trust_subject,
                    device_ref,
                    TRUSTED,
                    to_sql_generation(next_generation)?,
                    to_sql_generation(next_authority_generation)?,
                ],
            )
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        Self::insert_event(
            &transaction,
            &LifecycleEventInput {
                family_id,
                trust_subject,
                device_ref,
                correlation_id,
                kind: DeviceTrustLifecycleEventKind::Activated,
                state: DeviceTrustLifecycleState::Trusted,
                lifecycle_generation: next_generation,
                installation_binding_generation: installation_generation,
            },
        )?;
        transaction
            .commit()
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        self.set_external_authority(
            family_id,
            trust_subject,
            device_ref,
            next_authority_generation,
        )
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

    pub fn mark_delivered(&mut self, event_id: &str) -> Result<(), DeviceTrustLifecycleError> {
        let changed = self.connection.execute("UPDATE device_trust_lifecycle_outbox SET delivery_state = 'delivered' WHERE event_id = ?1 AND delivery_state = 'pending'", [event_id])
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
        family_id: &str,
        subject: &str,
        device: &str,
    ) -> Result<Option<(String, u64, u64, u64)>, DeviceTrustLifecycleError> {
        let row = transaction
            .query_row(
                "SELECT lifecycle_state, lifecycle_generation, installation_binding_generation, authority_generation FROM device_trust_lifecycle WHERE family_id = ?1 AND trust_subject = ?2 AND device_ref = ?3",
                params![family_id, subject, device],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?, row.get::<_, i64>(2)?, row.get::<_, i64>(3)?)),
            )
            .optional()
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        row.map(|(state, generation, installation, authority_generation)| {
            Ok((
                state,
                from_sql_generation(generation)?,
                from_sql_generation(installation)?,
                from_sql_generation(authority_generation)?,
            ))
        })
        .transpose()
    }

    fn insert_event(
        transaction: &rusqlite::Transaction<'_>,
        input: &LifecycleEventInput<'_>,
    ) -> Result<(), DeviceTrustLifecycleError> {
        let LifecycleEventInput {
            family_id,
            trust_subject,
            device_ref,
            correlation_id,
            kind,
            state,
            lifecycle_generation,
            installation_binding_generation,
        } = *input;
        let device_binding = redacted_binding(family_id, trust_subject, device_ref);
        let household_binding = redacted_binding(family_id, "household", "household");
        let event_id = format!("{device_binding}:{correlation_id}:{lifecycle_generation}");
        let event = DeviceTrustLifecycleEvent {
            event_id: event_id.clone(),
            household_binding,
            correlation_id: correlation_id.to_owned(),
            device_binding,
            kind,
            state,
            lifecycle_generation,
            installation_binding_generation,
            redaction: DeviceTrustLifecycleRedaction::SensitiveIdentifiersOmitted,
        };
        let json = serde_json::to_string(&event)
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        transaction.execute("INSERT INTO device_trust_lifecycle_outbox (event_id, correlation_id, event_json, delivery_state) VALUES (?1, ?2, ?3, 'pending')", params![event_id, correlation_id, json])
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        Ok(())
    }

    fn validate_identifiers(
        &self,
        family_id: &str,
        subject: &str,
        device: &str,
        correlation: &str,
    ) -> Result<(), DeviceTrustLifecycleError> {
        (!family_id.trim().is_empty()
            && !subject.trim().is_empty()
            && !device.trim().is_empty()
            && !correlation.trim().is_empty())
        .then_some(())
        .ok_or(DeviceTrustLifecycleError::InvalidIdentity)
    }

    fn require_generation(&self, generation: u64) -> Result<(), DeviceTrustLifecycleError> {
        (generation > 0)
            .then_some(())
            .ok_or(DeviceTrustLifecycleError::InvalidGeneration)
    }

    fn set_external_authority(
        &mut self,
        family_id: &str,
        trust_subject: &str,
        device_ref: &str,
        generation: u64,
    ) -> Result<(), DeviceTrustLifecycleError> {
        self.external_authority
            .set(family_id, trust_subject, device_ref, generation)
    }
}

fn to_sql_generation(generation: u64) -> Result<i64, DeviceTrustLifecycleError> {
    i64::try_from(generation).map_err(|_error| DeviceTrustLifecycleError::InvalidGeneration)
}

fn from_sql_generation(generation: i64) -> Result<u64, DeviceTrustLifecycleError> {
    u64::try_from(generation).map_err(|_error| DeviceTrustLifecycleError::InvalidGeneration)
}
