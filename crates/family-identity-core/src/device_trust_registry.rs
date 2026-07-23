//! Durable owner for the parent-device trust lifecycle.
//!
//! A registry mutation requires a consumed, action-bound parent-presence
//! verification. Pairing can only create `PendingSealing`; this module has no
//! transition to `Trusted`, which remains owned by the later platform-key
//! sealing receipt boundary.

use std::fmt;
use std::path::PathBuf;
use std::time::Duration;

use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::device_trust_authority::VerifiedParentDeviceTrustAuthority;
use crate::household_authority::HouseholdAuthorityAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceTrustLifecycleState {
    PendingSealing,
    Trusted,
    Revoked,
    ResetRequired,
}

#[derive(Clone, PartialEq, Eq, Serialize)]
pub struct DeviceTrustRegistryRecord {
    pub device_id: String,
    pub state: DeviceTrustLifecycleState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceTrustRegistryRejection {
    RevokedDeviceCannotRePair,
    OwnershipConflict,
    UnknownDevice,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub enum DeviceTrustRegistryDecision {
    PendingSealing(DeviceTrustRegistryRecord),
    Revoked(DeviceTrustRegistryRecord),
    Rejected(DeviceTrustRegistryRejection),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DeviceTrustRegistryFailure {
    CustodyUnavailable,
    StorageUnavailable,
    StorageIntegrityRejected,
}

pub struct DeviceTrustRegistry {
    path: PathBuf,
}

impl fmt::Debug for DeviceTrustRegistryRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeviceTrustRegistryRecord")
            .field("device_id", &"[redacted]")
            .field("state", &self.state)
            .finish()
    }
}

impl fmt::Debug for DeviceTrustRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeviceTrustRegistry")
            .field("path", &"[redacted]")
            .finish()
    }
}

impl DeviceTrustRegistry {
    pub fn open(path: impl Into<PathBuf>) -> Result<Self, DeviceTrustRegistryFailure> {
        let path = path.into();
        crate::device_trust_registry_storage::validate_custody_path(&path)?;
        let connection = Connection::open(&path)
            .map_err(|_error| DeviceTrustRegistryFailure::StorageUnavailable)?;
        connection
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS device_trust_registry (
                    device_id TEXT PRIMARY KEY NOT NULL,
                    family_id TEXT NOT NULL,
                    parent_account_id TEXT NOT NULL,
                    state TEXT NOT NULL CHECK (state IN ('pending-sealing', 'trusted', 'revoked', 'reset-required'))
                ) STRICT;
                CREATE TABLE IF NOT EXISTS device_trust_registry_journal (
                    operation_id TEXT PRIMARY KEY NOT NULL,
                    correlation_id TEXT NOT NULL,
                    receipt_ref TEXT UNIQUE NOT NULL,
                    device_id TEXT NOT NULL,
                    family_id TEXT NOT NULL,
                    acting_parent_account_id TEXT NOT NULL,
                    action TEXT NOT NULL,
                    outcome TEXT NOT NULL,
                    state TEXT NOT NULL
                ) STRICT;",
            )
            .map_err(|_error| DeviceTrustRegistryFailure::StorageUnavailable)?;
        Ok(Self { path })
    }

    pub fn apply_verified_parent_authority(
        &self,
        authority: VerifiedParentDeviceTrustAuthority,
    ) -> Result<DeviceTrustRegistryDecision, DeviceTrustRegistryFailure> {
        let (
            family_id,
            parent_account_id,
            device_id,
            action,
            _recovery_repair_authorized,
            correlation_id,
            receipt_ref,
        ) = authority.into_registry_parts();
        let mut connection = self.open_connection()?;
        match action {
            HouseholdAuthorityAction::PairChildDevice => {
                crate::device_trust_registry_storage::pair(
                    &mut connection,
                    &family_id,
                    &parent_account_id,
                    &device_id,
                    &correlation_id,
                    &receipt_ref,
                )
            }
            HouseholdAuthorityAction::RevokeChildDevice => {
                crate::device_trust_registry_storage::revoke(
                    &mut connection,
                    &family_id,
                    &parent_account_id,
                    &device_id,
                    &correlation_id,
                    &receipt_ref,
                )
            }
            _ => Err(DeviceTrustRegistryFailure::StorageIntegrityRejected),
        }
    }

    pub fn record(
        &self,
        family_id: &str,
        device_id: &str,
    ) -> Result<Option<DeviceTrustRegistryRecord>, DeviceTrustRegistryFailure> {
        let connection = self.open_connection()?;
        crate::device_trust_registry_storage::record(&connection, family_id, device_id)
    }

    fn open_connection(&self) -> Result<Connection, DeviceTrustRegistryFailure> {
        let connection = Connection::open(&self.path)
            .map_err(|_error| DeviceTrustRegistryFailure::StorageUnavailable)?;
        connection
            .busy_timeout(Duration::from_secs(10))
            .map_err(|_error| DeviceTrustRegistryFailure::StorageUnavailable)?;
        Ok(connection)
    }
}
