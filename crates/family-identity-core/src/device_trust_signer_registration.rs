use std::fmt;

use rusqlite::{params, Connection, OptionalExtension, Transaction};

use crate::{
    device_trust_lifecycle::{DeviceTrustLifecycleError, DeviceTrustLifecycleState},
    device_trust_lifecycle_authority::ExternalLifecycleAuthority,
    device_trust_lifecycle_current_authority::redacted_signer_binding,
    device_trust_signer_registration_schema,
    device_trust_signer_registration_validation::{
        self, PersistedSignerValidation, ValidatedSignerKey,
    },
};

#[path = "device_trust_signer_registration_current_authority.rs"]
mod current_authority;

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
    parent_presence_receipt: String,
    parent_intent_digest: String,
    parent_route_id: String,
    credential_id: String,
    credential_algorithm: i32,
    credential_sign_count: u32,
    lifecycle_generation: u64,
    installation_binding_generation: u64,
    authority_generation: u64,
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
        parent_presence_receipt: &str,
        parent_intent_digest: &str,
        parent_route_id: &str,
        credential_id: &str,
        credential_algorithm: i32,
        credential_sign_count: u32,
        lifecycle_generation: u64,
        installation_binding_generation: u64,
        authority_generation: u64,
    ) -> Result<Self, DeviceTrustLifecycleError> {
        device_trust_signer_registration_validation::validate_canonical_identity(family_id)?;
        device_trust_signer_registration_validation::validate_canonical_identity(trust_subject)?;
        device_trust_signer_registration_validation::validate_canonical_identity(parent_device_id)?;
        device_trust_signer_registration_validation::validate_canonical_identity(child_device_id)?;
        device_trust_signer_registration_validation::validate_canonical_identity(installation_id)?;
        device_trust_signer_registration_validation::validate_canonical_identity(correlation_id)?;
        device_trust_signer_registration_validation::validate_receipt(parent_presence_receipt)?;
        device_trust_signer_registration_validation::validate_digest(parent_intent_digest)?;
        device_trust_signer_registration_validation::validate_canonical_identity(parent_route_id)?;
        device_trust_signer_registration_validation::validate_credential_id(credential_id)?;
        if credential_algorithm != -8 {
            return Err(DeviceTrustLifecycleError::InvalidSignerKey);
        }
        if lifecycle_generation == 0
            || installation_binding_generation == 0
            || authority_generation == 0
        {
            return Err(DeviceTrustLifecycleError::InvalidGeneration);
        }
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
            parent_presence_receipt: parent_presence_receipt.to_owned(),
            parent_intent_digest: parent_intent_digest.to_owned(),
            parent_route_id: parent_route_id.to_owned(),
            credential_id: credential_id.to_owned(),
            credential_algorithm,
            credential_sign_count,
            lifecycle_generation,
            installation_binding_generation,
            authority_generation,
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
            &self.parent_presence_receipt,
            &self.parent_intent_digest,
            &self.parent_route_id,
            self.credential_algorithm,
            self.credential_sign_count,
        )
    }
}

/// A read-only point-in-time authority snapshot.
///
/// This value carries no mutation or authorization capability. Callers must
/// resolve it again at each trust decision so lifecycle and sidecar generation
/// checks are performed against current durable state.
#[derive(PartialEq, Eq)]
pub struct CurrentSignerAuthority {
    family_id: String,
    trust_subject: String,
    parent_device_id: String,
    child_device_id: String,
    installation_id: String,
    signer_public_key: [u8; 32],
    signer_key_id: String,
    signer_key_sha256: String,
    registration_receipt: String,
    parent_presence_receipt: String,
    parent_intent_digest: String,
    parent_route_id: String,
    credential_id: String,
    credential_algorithm: i32,
    credential_sign_count: u32,
    lifecycle_generation: u64,
    installation_binding_generation: u64,
    authority_generation: u64,
    state: DeviceTrustLifecycleState,
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

pub(crate) fn create_schema(connection: &Connection) -> Result<(), DeviceTrustLifecycleError> {
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
                parent_presence_receipt TEXT NOT NULL CHECK (length(parent_presence_receipt) = 64),
                parent_intent_digest TEXT NOT NULL CHECK (length(parent_intent_digest) = 64),
                parent_route_id TEXT NOT NULL CHECK (length(parent_route_id) BETWEEN 1 AND 256),
                credential_id TEXT NOT NULL CHECK (length(credential_id) BETWEEN 1 AND 512),
                credential_algorithm INTEGER NOT NULL CHECK (credential_algorithm = -8),
                credential_sign_count INTEGER NOT NULL CHECK (credential_sign_count >= 0),
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
            .map_err(|_error| DeviceTrustLifecycleError::Unavailable)
}

pub(crate) fn ensure_schema(connection: &Connection) -> Result<(), DeviceTrustLifecycleError> {
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
    if authorization.lifecycle_generation != lifecycle_generation
        || authorization.installation_binding_generation != installation_binding_generation
        || authorization.authority_generation != authority_generation
    {
        return Err(DeviceTrustLifecycleError::ParentReauthorizationRequired);
    }
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
              parent_presence_receipt, parent_intent_digest, parent_route_id, credential_id,
              credential_algorithm, credential_sign_count,
              lifecycle_generation, installation_binding_generation, authority_generation,
              registration_state)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, 'active')",
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
                authorization.parent_presence_receipt,
                authorization.parent_intent_digest,
                authorization.parent_route_id,
                authorization.credential_id,
                authorization.credential_algorithm,
                i64::from(authorization.credential_sign_count),
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
                    parent_presence_receipt, parent_intent_digest, parent_route_id, credential_id,
                    credential_algorithm, credential_sign_count,
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
    parent_presence_receipt: String,
    parent_intent_digest: String,
    parent_route_id: String,
    credential_id: String,
    credential_algorithm: i64,
    credential_sign_count: i64,
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
                parent_presence_receipt: &self.parent_presence_receipt,
                parent_intent_digest: &self.parent_intent_digest,
                parent_route_id: &self.parent_route_id,
                credential_id: &self.credential_id,
                credential_algorithm: self.credential_algorithm,
                credential_sign_count: self.credential_sign_count,
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
            parent_presence_receipt: self.parent_presence_receipt,
            parent_intent_digest: self.parent_intent_digest,
            parent_route_id: self.parent_route_id,
            credential_id: self.credential_id,
            credential_algorithm: i32::try_from(self.credential_algorithm)
                .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?,
            credential_sign_count: u32::try_from(self.credential_sign_count)
                .map_err(|_error| DeviceTrustLifecycleError::Unavailable)?,
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
                    parent_presence_receipt, parent_intent_digest, parent_route_id, credential_id,
                    credential_algorithm, credential_sign_count,
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
        parent_presence_receipt: row.get(9)?,
        parent_intent_digest: row.get(10)?,
        parent_route_id: row.get(11)?,
        credential_id: row.get(12)?,
        credential_algorithm: row.get(13)?,
        credential_sign_count: row.get(14)?,
        lifecycle_generation: row.get(15)?,
        installation_binding_generation: row.get(16)?,
        authority_generation: row.get(17)?,
        registration_state: row.get(18)?,
    })
}

fn to_sql_generation(generation: u64) -> Result<i64, DeviceTrustLifecycleError> {
    i64::try_from(generation).map_err(|_error| DeviceTrustLifecycleError::InvalidGeneration)
}

fn from_sql_generation(generation: i64) -> Result<u64, DeviceTrustLifecycleError> {
    u64::try_from(generation).map_err(|_error| DeviceTrustLifecycleError::InvalidGeneration)
}
