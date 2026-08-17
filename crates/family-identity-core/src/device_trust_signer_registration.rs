use std::fmt;

use rusqlite::{params, Connection, OptionalExtension, Transaction};
use serde::Serialize;

use crate::{
    device_trust_lifecycle::{DeviceTrustLifecycleError, DeviceTrustLifecycleState},
    device_trust_lifecycle_authority::ExternalLifecycleAuthority,
    device_trust_lifecycle_current_authority::redacted_signer_binding,
    device_trust_signer_registration_schema,
    device_trust_signer_registration_validation::{
        self, PersistedSignerValidation, ValidatedSignerKey,
    },
};

/// An opaque family-owned authorization for one signer registration.
///
/// The fields are private and the value is consumed by registration.  The
/// constructor is crate-only so a future WP03 ceremony can issue this token
/// after verifying and consuming its one-time parent authorization; callers
/// outside the family-identity crate cannot forge one.
pub(crate) struct SignerRegistrationAuthorization {
    family_id: String,
    trust_subject: String,
    parent_device_id: String,
    child_device_id: String,
    installation_id: String,
    signer_public_key: [u8; 32],
    signer_key_id: String,
    signer_key_sha256: String,
    registration_receipt: String,
    correlation_id: String,
}

impl fmt::Debug for SignerRegistrationAuthorization {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SignerRegistrationAuthorization")
            .field("redaction", &"sensitive-fields-omitted")
            .finish()
    }
}

impl SignerRegistrationAuthorization {
    /// Issue only from a verified family-owned parent ceremony.
    ///
    /// WP03 will become the caller.  Keeping this constructor crate-only and
    /// returning an opaque, single-use token prevents LAN or other dependent
    /// crates from directly minting signer authority.
    pub(crate) fn from_verified_parent_step_up(
        family_id: &str,
        trust_subject: &str,
        parent_device_id: &str,
        child_device_id: &str,
        installation_id: &str,
        signer_public_key: &[u8],
        correlation_id: &str,
    ) -> Result<Self, DeviceTrustLifecycleError> {
        device_trust_signer_registration_validation::validate_canonical_identity(family_id)?;
        device_trust_signer_registration_validation::validate_canonical_identity(trust_subject)?;
        device_trust_signer_registration_validation::validate_canonical_identity(parent_device_id)?;
        device_trust_signer_registration_validation::validate_canonical_identity(child_device_id)?;
        device_trust_signer_registration_validation::validate_canonical_identity(installation_id)?;
        device_trust_signer_registration_validation::validate_canonical_identity(correlation_id)?;
        let ValidatedSignerKey {
            public_key,
            key_id,
            sha256,
        } = device_trust_signer_registration_validation::validate_signer_key(signer_public_key)?;
        Ok(Self {
            family_id: family_id.to_owned(),
            trust_subject: trust_subject.to_owned(),
            parent_device_id: parent_device_id.to_owned(),
            child_device_id: child_device_id.to_owned(),
            installation_id: installation_id.to_owned(),
            signer_public_key: public_key,
            signer_key_id: key_id,
            signer_key_sha256: sha256,
            registration_receipt: device_trust_signer_registration_validation::random_receipt()?,
            correlation_id: correlation_id.to_owned(),
        })
    }

    pub(crate) fn registration_identity(&self) -> (&str, &str, &str, &str, &str, &str) {
        (
            &self.family_id,
            &self.trust_subject,
            &self.parent_device_id,
            &self.child_device_id,
            &self.installation_id,
            &self.correlation_id,
        )
    }

    pub(crate) fn event_binding(&self) -> String {
        redacted_signer_binding(
            &self.family_id,
            &self.trust_subject,
            &self.parent_device_id,
            &self.child_device_id,
            &self.installation_id,
            &self.signer_key_id,
        )
    }
}

/// A read-only point-in-time authority snapshot.
///
/// This value carries no mutation or authorization capability. Callers must
/// resolve it again at each trust decision so lifecycle and sidecar generation
/// checks are performed against current durable state.
#[derive(Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentSignerAuthority {
    pub family_id: String,
    pub trust_subject: String,
    pub parent_device_id: String,
    pub child_device_id: String,
    pub installation_id: String,
    pub signer_public_key: [u8; 32],
    pub signer_key_id: String,
    pub signer_key_sha256: String,
    pub registration_receipt: String,
    pub lifecycle_generation: u64,
    pub installation_binding_generation: u64,
    pub authority_generation: u64,
    pub state: DeviceTrustLifecycleState,
}

impl fmt::Debug for CurrentSignerAuthority {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CurrentSignerAuthority")
            .field("lifecycle_generation", &self.lifecycle_generation)
            .field(
                "installation_binding_generation",
                &self.installation_binding_generation,
            )
            .field("authority_generation", &self.authority_generation)
            .field("state", &self.state)
            .field("redaction", &"sensitive-fields-omitted")
            .finish()
    }
}

pub(crate) fn ensure_schema(connection: &Connection) -> Result<(), DeviceTrustLifecycleError> {
    connection
        .execute_batch(
            "CREATE TABLE IF NOT EXISTS device_trust_signer_registration (
                family_id TEXT NOT NULL,
                trust_subject TEXT NOT NULL,
                parent_device_id TEXT NOT NULL,
                child_device_id TEXT NOT NULL,
                installation_id TEXT NOT NULL,
                signer_public_key BLOB NOT NULL CHECK (length(signer_public_key) = 32),
                signer_key_id TEXT NOT NULL CHECK (length(signer_key_id) = 32),
                signer_key_sha256 TEXT NOT NULL CHECK (length(signer_key_sha256) = 64),
                registration_receipt TEXT NOT NULL UNIQUE CHECK (length(registration_receipt) = 64),
                lifecycle_generation INTEGER NOT NULL CHECK (lifecycle_generation > 0),
                installation_binding_generation INTEGER NOT NULL CHECK (installation_binding_generation > 0),
                authority_generation INTEGER NOT NULL CHECK (authority_generation > 0),
                registration_state TEXT NOT NULL CHECK (registration_state IN ('active','revoked')),
                PRIMARY KEY (family_id, trust_subject, parent_device_id, child_device_id, installation_id, signer_key_id)
            ) STRICT;
            CREATE UNIQUE INDEX IF NOT EXISTS device_trust_signer_registration_active_key
            ON device_trust_signer_registration (family_id, trust_subject, parent_device_id, child_device_id)
            WHERE registration_state = 'active';",
        )
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    device_trust_signer_registration_schema::validate(connection)?;
    validate_persisted_rows(connection)
}

pub(crate) fn register(
    transaction: &Transaction<'_>,
    authorization: SignerRegistrationAuthorization,
    lifecycle_generation: u64,
    installation_binding_generation: u64,
    authority_generation: u64,
) -> Result<(), DeviceTrustLifecycleError> {
    ensure_registration_slot(transaction, &authorization)?;
    insert_registration(
        transaction,
        &authorization,
        lifecycle_generation,
        installation_binding_generation,
        authority_generation,
    )?;
    Ok(())
}

fn ensure_registration_slot(
    transaction: &Transaction<'_>,
    authorization: &SignerRegistrationAuthorization,
) -> Result<(), DeviceTrustLifecycleError> {
    let existing = transaction
        .query_row(
            "SELECT registration_state FROM device_trust_signer_registration
             WHERE family_id = ?1 AND trust_subject = ?2 AND parent_device_id = ?3
               AND child_device_id = ?4 AND installation_id = ?5 AND signer_key_id = ?6",
            params![
                authorization.family_id,
                authorization.trust_subject,
                authorization.parent_device_id,
                authorization.child_device_id,
                authorization.installation_id,
                authorization.signer_key_id
            ],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    if existing.is_some() {
        return Err(DeviceTrustLifecycleError::DuplicateSignerRegistration);
    }
    let active_for_child = transaction
        .query_row(
            "SELECT 1 FROM device_trust_signer_registration
             WHERE family_id = ?1 AND trust_subject = ?2 AND parent_device_id = ?3
               AND child_device_id = ?4 AND registration_state = 'active'",
            params![
                authorization.family_id,
                authorization.trust_subject,
                authorization.parent_device_id,
                authorization.child_device_id
            ],
            |_row| Ok(()),
        )
        .optional()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    active_for_child
        .is_none()
        .then_some(())
        .ok_or(DeviceTrustLifecycleError::SignerRegistrationConflict)
}

fn insert_registration(
    transaction: &Transaction<'_>,
    authorization: &SignerRegistrationAuthorization,
    lifecycle_generation: u64,
    installation_binding_generation: u64,
    authority_generation: u64,
) -> Result<(), DeviceTrustLifecycleError> {
    transaction
        .execute(
            "INSERT INTO device_trust_signer_registration
             (family_id, trust_subject, parent_device_id, child_device_id, installation_id,
              signer_public_key, signer_key_id, signer_key_sha256, registration_receipt,
              lifecycle_generation, installation_binding_generation, authority_generation,
              registration_state)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, 'active')",
            params![
                authorization.family_id,
                authorization.trust_subject,
                authorization.parent_device_id,
                authorization.child_device_id,
                authorization.installation_id,
                authorization.signer_public_key.as_slice(),
                authorization.signer_key_id,
                authorization.signer_key_sha256,
                authorization.registration_receipt,
                to_sql_generation(lifecycle_generation)?,
                to_sql_generation(installation_binding_generation)?,
                to_sql_generation(authority_generation)?,
            ],
        )
        .map_err(|_error| DeviceTrustLifecycleError::SignerRegistrationConflict)
}

pub(crate) fn current_authority(
    connection: &Connection,
    external_authority: &ExternalLifecycleAuthority,
    family_id: &str,
    trust_subject: &str,
    parent_device_id: &str,
    child_device_id: &str,
) -> Result<CurrentSignerAuthority, DeviceTrustLifecycleError> {
    let authority = current(
        connection,
        family_id,
        trust_subject,
        parent_device_id,
        child_device_id,
    )?;
    let (
        state,
        lifecycle_generation,
        installation_id,
        installation_generation,
        authority_generation,
    ) = connection
        .query_row(
            "SELECT lifecycle_state, lifecycle_generation, installation_id,
                    installation_binding_generation, authority_generation
             FROM device_trust_lifecycle
             WHERE family_id = ?1 AND trust_subject = ?2 AND device_ref = ?3",
            params![family_id, trust_subject, parent_device_id],
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
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
        .ok_or(DeviceTrustLifecycleError::RegistrationMissing)?;
    if state != "trusted" {
        return Err(DeviceTrustLifecycleError::InvalidState);
    }
    let lifecycle_generation = from_sql_generation(lifecycle_generation)?;
    let installation_generation = from_sql_generation(installation_generation)?;
    let authority_generation = from_sql_generation(authority_generation)?;
    if !external_authority.matches(
        family_id,
        trust_subject,
        parent_device_id,
        authority_generation,
    ) || authority.lifecycle_generation != lifecycle_generation
        || authority.installation_id != installation_id
        || authority.installation_binding_generation != installation_generation
        || authority.authority_generation != authority_generation
    {
        return Err(DeviceTrustLifecycleError::Unavailable);
    }
    Ok(authority)
}

pub(crate) fn current(
    connection: &Connection,
    family_id: &str,
    trust_subject: &str,
    parent_device_id: &str,
    child_device_id: &str,
) -> Result<CurrentSignerAuthority, DeviceTrustLifecycleError> {
    let mut statement = connection
        .prepare(
            "SELECT family_id, trust_subject, parent_device_id, child_device_id, installation_id,
                    signer_public_key, signer_key_id, signer_key_sha256, registration_receipt,
                    lifecycle_generation, installation_binding_generation, authority_generation,
                    registration_state
             FROM device_trust_signer_registration
             WHERE family_id = ?1 AND trust_subject = ?2 AND parent_device_id = ?3
               AND child_device_id = ?4 AND registration_state = 'active'
             ORDER BY installation_id, signer_key_id",
        )
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let rows: Vec<StoredSignerRow> = statement
        .query_map(
            params![family_id, trust_subject, parent_device_id, child_device_id],
            read_stored_row,
        )
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
        .collect::<Result<_, _>>()
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    if rows.len() != 1 {
        return Err(if rows.is_empty() {
            DeviceTrustLifecycleError::SignerRegistrationMissing
        } else {
            DeviceTrustLifecycleError::SignerRegistrationConflict
        });
    }
    rows.into_iter()
        .next()
        .ok_or(DeviceTrustLifecycleError::SignerRegistrationMissing)?
        .into_authority()
}

struct StoredSignerRow {
    family_id: String,
    trust_subject: String,
    parent_device_id: String,
    child_device_id: String,
    installation_id: String,
    signer_public_key: Vec<u8>,
    signer_key_id: String,
    signer_key_sha256: String,
    registration_receipt: String,
    lifecycle_generation: i64,
    installation_binding_generation: i64,
    authority_generation: i64,
    registration_state: String,
}

impl StoredSignerRow {
    fn validate(&self) -> Result<[u8; 32], DeviceTrustLifecycleError> {
        device_trust_signer_registration_validation::validate_persisted_signer(
            &PersistedSignerValidation {
                family_id: &self.family_id,
                trust_subject: &self.trust_subject,
                parent_device_id: &self.parent_device_id,
                child_device_id: &self.child_device_id,
                installation_id: &self.installation_id,
                signer_public_key: &self.signer_public_key,
                signer_key_id: &self.signer_key_id,
                signer_key_sha256: &self.signer_key_sha256,
                registration_receipt: &self.registration_receipt,
                lifecycle_generation: self.lifecycle_generation,
                installation_binding_generation: self.installation_binding_generation,
                authority_generation: self.authority_generation,
                registration_state: &self.registration_state,
            },
        )
    }

    fn into_authority(self) -> Result<CurrentSignerAuthority, DeviceTrustLifecycleError> {
        let signer_public_key = self.validate()?;
        Ok(CurrentSignerAuthority {
            family_id: self.family_id,
            trust_subject: self.trust_subject,
            parent_device_id: self.parent_device_id,
            child_device_id: self.child_device_id,
            installation_id: self.installation_id,
            signer_public_key,
            signer_key_id: self.signer_key_id,
            signer_key_sha256: self.signer_key_sha256,
            registration_receipt: self.registration_receipt,
            lifecycle_generation: from_sql_generation(self.lifecycle_generation)?,
            installation_binding_generation: from_sql_generation(
                self.installation_binding_generation,
            )?,
            authority_generation: from_sql_generation(self.authority_generation)?,
            state: DeviceTrustLifecycleState::Trusted,
        })
    }
}

pub(crate) fn validate_persisted_rows(
    connection: &Connection,
) -> Result<(), DeviceTrustLifecycleError> {
    let mut statement = connection
        .prepare(
            "SELECT family_id, trust_subject, parent_device_id, child_device_id, installation_id,
                    signer_public_key, signer_key_id, signer_key_sha256, registration_receipt,
                    lifecycle_generation, installation_binding_generation, authority_generation,
                    registration_state
             FROM device_trust_signer_registration
             ORDER BY family_id, trust_subject, parent_device_id, child_device_id,
                      installation_id, signer_key_id",
        )
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    let rows = statement
        .query_map([], read_stored_row)
        .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?;
    for row in rows {
        row.map_err(|_error| DeviceTrustLifecycleError::Unavailable)?
            .validate()?;
    }
    Ok(())
}

fn read_stored_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<StoredSignerRow> {
    Ok(StoredSignerRow {
        family_id: row.get(0)?,
        trust_subject: row.get(1)?,
        parent_device_id: row.get(2)?,
        child_device_id: row.get(3)?,
        installation_id: row.get(4)?,
        signer_public_key: row.get(5)?,
        signer_key_id: row.get(6)?,
        signer_key_sha256: row.get(7)?,
        registration_receipt: row.get(8)?,
        lifecycle_generation: row.get(9)?,
        installation_binding_generation: row.get(10)?,
        authority_generation: row.get(11)?,
        registration_state: row.get(12)?,
    })
}

fn to_sql_generation(generation: u64) -> Result<i64, DeviceTrustLifecycleError> {
    i64::try_from(generation).map_err(|_error| DeviceTrustLifecycleError::InvalidGeneration)
}

fn from_sql_generation(generation: i64) -> Result<u64, DeviceTrustLifecycleError> {
    u64::try_from(generation).map_err(|_error| DeviceTrustLifecycleError::InvalidGeneration)
}
