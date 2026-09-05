//! Durable, runtime-owned lifecycle authority for trusted parent devices.
//!
//! This is deliberately separate from an unsealing credential.  A credential
//! identifies what was sealed; this repository answers whether that exact
//! registration is still authorised *now*.

use std::{fmt, path::Path, time::Duration};

use crate::device_trust_lifecycle_authority::ExternalLifecycleAuthority;
use crate::device_trust_lifecycle_authority_fence::AuthorityTransition;
use crate::device_trust_lifecycle_current_authority::redacted_binding;
use crate::device_trust_lifecycle_schema;
use crate::device_trust_signer_registration::{self, CurrentSignerAuthority};
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use serde::{Deserialize, Serialize};

pub(crate) struct DeviceTrustLifecycleRow {
    pub(crate) state: String,
    pub(crate) lifecycle_generation: u64,
    pub(crate) installation_id: String,
    pub(crate) installation_binding_generation: u64,
    pub(crate) authority_generation: u64,
}

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
    SignerRegistered,
    SignerRevoked,
}

pub(crate) struct LifecycleEventInput<'a> {
    pub(crate) family_id: &'a str,
    pub(crate) trust_subject: &'a str,
    pub(crate) device_ref: &'a str,
    pub(crate) correlation_id: &'a str,
    pub(crate) event_binding: Option<&'a str>,
    pub(crate) kind: DeviceTrustLifecycleEventKind,
    pub(crate) state: DeviceTrustLifecycleState,
    pub(crate) lifecycle_generation: u64,
    pub(crate) installation_binding_generation: u64,
}

/// An opaque, single-use authorization proving that platform custody was
/// durably sealed for one exact pending lifecycle row.
///
/// Its fields are private and its constructor is crate-only. A caller outside
/// the family-identity owner therefore cannot turn caller-supplied strings into
/// a trusted lifecycle transition.
pub struct SealingCustodyAuthorization {
    family_id: String,
    trust_subject: String,
    device_ref: String,
    installation_id: String,
    lifecycle_generation: u64,
    installation_binding_generation: u64,
    authority_generation: u64,
    correlation_id: String,
}

impl fmt::Debug for SealingCustodyAuthorization {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SealingCustodyAuthorization")
            .field("redaction", &"sensitive-fields-omitted")
            .finish()
    }
}

impl SealingCustodyAuthorization {
    pub(crate) fn into_parts(self) -> (String, String, String, String, u64, u64, u64, String) {
        (
            self.family_id,
            self.trust_subject,
            self.device_ref,
            self.installation_id,
            self.lifecycle_generation,
            self.installation_binding_generation,
            self.authority_generation,
            self.correlation_id,
        )
    }
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
    InvalidSignerKey,
    DuplicateSignerRegistration,
    SignerRegistrationConflict,
    SignerRegistrationMissing,
    ParentReauthorizationRequired,
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
    /// Open the canonical lifecycle store.  Only a path that did not exist
    /// before this call may receive the initial schema; an existing database
    /// is validated as-is so missing objects cannot be silently recreated as a
    /// migration or recovery side effect.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DeviceTrustLifecycleError> {
        let path = path.as_ref();
        let initialize_schema = !path.exists();
        let mut external_authority = ExternalLifecycleAuthority::open(path)?;
        let connection =
            Connection::open(path).map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        connection
            .busy_timeout(Duration::from_secs(10))
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        connection
            .execute_batch(
                "PRAGMA foreign_keys = ON; PRAGMA journal_mode = DELETE; PRAGMA synchronous = FULL;",
            )
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        if initialize_schema {
            connection.execute_batch(
                "CREATE TABLE IF NOT EXISTS device_trust_lifecycle (
                family_id TEXT NOT NULL,
                trust_subject TEXT NOT NULL,
                device_ref TEXT NOT NULL,
                installation_id TEXT NOT NULL,
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
             ) STRICT;
             CREATE TABLE IF NOT EXISTS device_trust_authority_transition (
                authority_key TEXT NOT NULL PRIMARY KEY CHECK (length(authority_key) = 64),
                operation_id TEXT NOT NULL CHECK (length(operation_id) = 64),
                from_generation INTEGER CHECK (from_generation IS NULL OR from_generation > 0),
                to_generation INTEGER NOT NULL CHECK (to_generation > 0),
                CHECK ((from_generation IS NULL AND to_generation = 1) OR
                       (from_generation IS NOT NULL AND to_generation = from_generation + 1))
                 ) STRICT;",
            )
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
            device_trust_signer_registration::create_schema(&connection)?;
        }
        device_trust_lifecycle_schema::validate(&connection)?;
        device_trust_signer_registration::ensure_schema(&connection)?;
        external_authority.reconcile(&connection)?;
        Ok(Self {
            connection,
            external_authority,
        })
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

    /// Resolve the current active signer anchor without granting mutation
    /// authority.  The lifecycle row and external sidecar must both be
    /// trusted/current before the signer snapshot is returned.
    pub fn current_signer_authority(
        &self,
        family_id: &str,
        trust_subject: &str,
        parent_device_id: &str,
        child_device_id: &str,
    ) -> Result<CurrentSignerAuthority, DeviceTrustLifecycleError> {
        self.validate_identifiers(family_id, trust_subject, parent_device_id, child_device_id)?;
        device_trust_signer_registration::current_authority(
            &self.connection,
            &self.external_authority,
            family_id,
            trust_subject,
            parent_device_id,
            child_device_id,
        )
    }

    pub(crate) fn transaction(
        &mut self,
    ) -> Result<rusqlite::Transaction<'_>, DeviceTrustLifecycleError> {
        self.connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)
    }

    pub(crate) fn row(
        connection: &Connection,
        family_id: &str,
        subject: &str,
        device: &str,
    ) -> Result<Option<DeviceTrustLifecycleRow>, DeviceTrustLifecycleError> {
        let row = connection
            .query_row(
                "SELECT lifecycle_state, lifecycle_generation, installation_id,
                        installation_binding_generation, authority_generation
                 FROM device_trust_lifecycle
                 WHERE family_id = ?1 AND trust_subject = ?2 AND device_ref = ?3",
                params![family_id, subject, device],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, i64>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, i64>(3)?,
                        row.get::<_, i64>(4)?,
                    ))
                },
            )
            .optional()
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
        row.map(
            |(state, generation, installation_id, installation, authority_generation)| {
                Ok(DeviceTrustLifecycleRow {
                    state,
                    lifecycle_generation: from_sql_generation(generation)?,
                    installation_id,
                    installation_binding_generation: from_sql_generation(installation)?,
                    authority_generation: from_sql_generation(authority_generation)?,
                })
            },
        )
        .transpose()
    }

    pub(crate) fn insert_event(
        transaction: &rusqlite::Transaction<'_>,
        input: &LifecycleEventInput<'_>,
    ) -> Result<(), DeviceTrustLifecycleError> {
        let LifecycleEventInput {
            family_id,
            trust_subject,
            device_ref,
            correlation_id,
            event_binding,
            kind,
            state,
            lifecycle_generation,
            installation_binding_generation,
        } = *input;
        let device_binding = redacted_binding(family_id, trust_subject, device_ref);
        let household_binding = redacted_binding(family_id, "household", "household");
        let event_id = event_binding.map_or_else(
            || format!("{device_binding}:{correlation_id}:{lifecycle_generation}"),
            |event_binding| {
                format!(
                    "{device_binding}:{event_binding}:{correlation_id}:{kind:?}:{lifecycle_generation}"
                )
            },
        );
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

    pub(crate) fn validate_identifiers(
        &self,
        family_id: &str,
        subject: &str,
        device: &str,
        correlation: &str,
    ) -> Result<(), DeviceTrustLifecycleError> {
        for identity in [family_id, subject, device, correlation] {
            crate::device_trust_signer_registration_validation::validate_canonical_identity(
                identity,
            )?;
        }
        Ok(())
    }

    pub(crate) fn finish_authority_transition(
        &mut self,
        transition: AuthorityTransition,
        database_result: Result<(), DeviceTrustLifecycleError>,
    ) -> Result<(), DeviceTrustLifecycleError> {
        match database_result {
            Ok(()) => {
                let values = transition.complete()?;
                self.external_authority.replace_values(values);
                Ok(())
            }
            Err(error) => {
                let values = transition.reconcile_after_database_error(&self.connection)?;
                self.external_authority.replace_values(values);
                Err(error)
            }
        }
    }
}

pub(crate) fn to_sql_generation(generation: u64) -> Result<i64, DeviceTrustLifecycleError> {
    i64::try_from(generation).map_err(|_error| DeviceTrustLifecycleError::InvalidGeneration)
}

fn from_sql_generation(generation: i64) -> Result<u64, DeviceTrustLifecycleError> {
    u64::try_from(generation).map_err(|_error| DeviceTrustLifecycleError::InvalidGeneration)
}
