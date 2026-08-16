//! Durable parent-authorized trust removal for the child service.
//!
//! This boundary records revocation and reauthorization decisions owned by the
//! child service. It deliberately accepts only a parent authorization
//! token constructed from an already verified household-authority proof; the
//! service cannot mint or validate that authority locally.
//! Package-manager removal and device-owner removal remain external platform
//! operations and therefore stay manual-required in the status surface.

use std::{
    fs,
    fs::OpenOptions,
    io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use atomicwrites::{AllowOverwrite, AtomicFile};
use fs2::FileExt;
use ocentra_family_identity_core::{
    household_authority::HouseholdAuthorityAction,
    household_authority_proof::VerifiedHouseholdAuthority,
};
use serde::{Deserialize, Serialize};

const REMOVAL_STATE_VERSION: u16 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildAgentTrustState {
    Active,
    Revoked,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildAgentCleanupState {
    NotRequired,
    ManualPlatformRemovalRequired,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChildAgentRemovalAction {
    Revoked,
    Reauthorized,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildAgentRemovalAuthorizationAction {
    Revoke,
    Reauthorize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChildAgentServiceIdentity {
    pub household_id: String,
    pub child_profile_id: String,
    pub target_device_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildAgentRemovalAuditEntry {
    pub audit_ref: String,
    pub action: ChildAgentRemovalAction,
    pub parent_authorization_ref: String,
    pub household_id: String,
    pub child_profile_id: String,
    pub target_device_id: String,
    pub recorded_at_unix_seconds: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct ChildAgentRemovalRecord {
    version: u16,
    trust_state: ChildAgentTrustState,
    cleanup_state: ChildAgentCleanupState,
    audit: Vec<ChildAgentRemovalAuditEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChildAgentRemovalStatus {
    pub trust_state: ChildAgentTrustState,
    pub cleanup_state: ChildAgentCleanupState,
    pub latest_audit_ref: Option<String>,
    pub latest_parent_authorization_ref: Option<String>,
    pub audit_entry_count: usize,
}

/// A removal authorization can only be created from a proof already accepted
/// by the family-identity verifier. The reference is retained as audit
/// evidence, never treated as the authority itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifiedParentRemovalAuthorization {
    reference: String,
    action: ChildAgentRemovalAuthorizationAction,
    identity: ChildAgentServiceIdentity,
}

impl VerifiedParentRemovalAuthorization {
    pub fn for_revocation(
        authority: &VerifiedHouseholdAuthority,
        reference: impl Into<String>,
    ) -> io::Result<Self> {
        Self::from_verified_authority(
            authority,
            reference,
            HouseholdAuthorityAction::RevokeChildDevice,
            ChildAgentRemovalAuthorizationAction::Revoke,
        )
    }

    pub fn for_reauthorization(
        authority: &VerifiedHouseholdAuthority,
        reference: impl Into<String>,
    ) -> io::Result<Self> {
        Self::from_verified_authority(
            authority,
            reference,
            HouseholdAuthorityAction::PairChildDevice,
            ChildAgentRemovalAuthorizationAction::Reauthorize,
        )
    }

    fn from_verified_authority(
        authority: &VerifiedHouseholdAuthority,
        reference: impl Into<String>,
        required_action: HouseholdAuthorityAction,
        action: ChildAgentRemovalAuthorizationAction,
    ) -> io::Result<Self> {
        if authority.input().action != required_action {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "verified authority is not scoped to child removal or reauthorization",
            ));
        }
        let binding = authority.identity_binding().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::PermissionDenied,
                "bound household authority is required for child removal",
            )
        })?;
        let identity = ChildAgentServiceIdentity {
            household_id: non_empty_ref(&binding.household_id)?,
            child_profile_id: non_empty_ref(&binding.child_profile_id)?,
            target_device_id: non_empty_ref(&binding.target_device_id)?,
        };
        let reference = non_empty_ref(&reference.into())?;
        Ok(Self {
            reference,
            action,
            identity,
        })
    }

    pub fn reference(&self) -> &str {
        &self.reference
    }

    pub fn action(&self) -> ChildAgentRemovalAuthorizationAction {
        self.action
    }

    fn identity(&self) -> &ChildAgentServiceIdentity {
        &self.identity
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChildAgentRemovalBoundary {
    path: PathBuf,
    identity: Option<ChildAgentServiceIdentity>,
}

impl ChildAgentRemovalBoundary {
    pub fn open(path: impl Into<PathBuf>) -> io::Result<Self> {
        Self::open_with_identity(path, None)
    }

    pub fn open_with_identity(
        path: impl Into<PathBuf>,
        identity: Option<ChildAgentServiceIdentity>,
    ) -> io::Result<Self> {
        let path = path.into();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        if let Ok(metadata) = fs::symlink_metadata(&path) {
            if metadata.file_type().is_symlink() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "child removal state must not be a symlink",
                ));
            }
        }
        let boundary = Self { path, identity };
        boundary.status()?;
        Ok(boundary)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn status(&self) -> io::Result<ChildAgentRemovalStatus> {
        self.with_locked_record(|record| Ok(status_from_record(record)))
    }

    /// Records a parent-authorized revocation and leaves platform removal
    /// visible as manual-required. The reference is opaque here: authority is
    /// supplied by the parent/control plane, never self-issued by the child.
    pub fn revoke_with_parent_authorization(
        &self,
        authorization: &VerifiedParentRemovalAuthorization,
    ) -> io::Result<ChildAgentRemovalStatus> {
        self.with_locked_record(|record| {
            self.require_identity(authorization, ChildAgentRemovalAuthorizationAction::Revoke)?;
            if record.trust_state == ChildAgentTrustState::Revoked {
                return Ok(status_from_record(record));
            }
            append_audit(
                record,
                ChildAgentRemovalAction::Revoked,
                authorization.reference().to_owned(),
                authorization.identity().clone(),
            )?;
            record.trust_state = ChildAgentTrustState::Revoked;
            record.cleanup_state = ChildAgentCleanupState::ManualPlatformRemovalRequired;
            Ok(status_from_record(record))
        })
    }

    /// Records parent reauthorization while retaining the prior revocation
    /// audit. This does not reinstall packages or claim platform cleanup.
    pub fn reauthorize_with_parent_authorization(
        &self,
        authorization: &VerifiedParentRemovalAuthorization,
    ) -> io::Result<ChildAgentRemovalStatus> {
        self.with_locked_record(|record| {
            self.require_identity(
                authorization,
                ChildAgentRemovalAuthorizationAction::Reauthorize,
            )?;
            append_audit(
                record,
                ChildAgentRemovalAction::Reauthorized,
                authorization.reference().to_owned(),
                authorization.identity().clone(),
            )?;
            record.trust_state = ChildAgentTrustState::Active;
            record.cleanup_state = ChildAgentCleanupState::NotRequired;
            Ok(status_from_record(record))
        })
    }

    fn require_identity(
        &self,
        authorization: &VerifiedParentRemovalAuthorization,
        expected_action: ChildAgentRemovalAuthorizationAction,
    ) -> io::Result<()> {
        if authorization.action() != expected_action {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "parent removal authorization action does not match the requested transition",
            ));
        }
        let Some(identity) = &self.identity else {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "child service identity is required before parent removal authorization",
            ));
        };
        if identity != authorization.identity() {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "parent removal authorization is bound to a different child identity",
            ));
        }
        Ok(())
    }

    fn with_locked_record<T>(
        &self,
        operation: impl FnOnce(&mut ChildAgentRemovalRecord) -> io::Result<T>,
    ) -> io::Result<T> {
        let lock_path = self.path.with_extension("lock");
        let lock = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(lock_path)?;
        lock.lock_exclusive()?;
        let mut record = self.read_record_unlocked()?;
        let result = match operation(&mut record) {
            Ok(value) => self.write_record(&record).map(|()| value),
            Err(error) => Err(error),
        };
        let unlock_result = FileExt::unlock(&lock);
        match (result, unlock_result) {
            (Ok(value), Ok(())) => Ok(value),
            (Err(error), _) => Err(error),
            (Ok(_), Err(error)) => Err(error),
        }
    }

    fn read_record_unlocked(&self) -> io::Result<ChildAgentRemovalRecord> {
        match fs::read(&self.path) {
            Ok(bytes) => {
                let record: ChildAgentRemovalRecord = serde_json::from_slice(&bytes)
                    .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
                if record.version != REMOVAL_STATE_VERSION {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "unsupported child removal state version",
                    ));
                }
                Ok(record)
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(empty_record()),
            Err(error) => Err(error),
        }
    }

    fn write_record(&self, record: &ChildAgentRemovalRecord) -> io::Result<()> {
        AtomicFile::new(&self.path, AllowOverwrite)
            .write(|file| {
                serde_json::to_writer(&mut *file, record).map_err(io::Error::other)?;
                file.sync_all()
            })
            .map_err(|error| io::Error::other(error.to_string()))?;
        #[cfg(not(windows))]
        if let Some(parent) = self.path.parent() {
            fs::File::open(parent)?.sync_all()?;
        }
        Ok(())
    }
}

fn empty_record() -> ChildAgentRemovalRecord {
    ChildAgentRemovalRecord {
        version: REMOVAL_STATE_VERSION,
        trust_state: ChildAgentTrustState::Active,
        cleanup_state: ChildAgentCleanupState::NotRequired,
        audit: Vec::new(),
    }
}

fn append_audit(
    record: &mut ChildAgentRemovalRecord,
    action: ChildAgentRemovalAction,
    parent_authorization_ref: String,
    authorization_identity: ChildAgentServiceIdentity,
) -> io::Result<()> {
    let sequence = record.audit.len() + 1;
    record.audit.push(ChildAgentRemovalAuditEntry {
        audit_ref: format!("child-removal-audit-{sequence}"),
        action,
        parent_authorization_ref,
        household_id: authorization_identity.household_id,
        child_profile_id: authorization_identity.child_profile_id,
        target_device_id: authorization_identity.target_device_id,
        recorded_at_unix_seconds: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(io::Error::other)?
            .as_secs(),
    });
    Ok(())
}

fn status_from_record(record: &ChildAgentRemovalRecord) -> ChildAgentRemovalStatus {
    let latest = record.audit.last();
    ChildAgentRemovalStatus {
        trust_state: record.trust_state.clone(),
        cleanup_state: record.cleanup_state.clone(),
        latest_audit_ref: latest.map(|entry| entry.audit_ref.clone()),
        latest_parent_authorization_ref: latest.map(|entry| entry.parent_authorization_ref.clone()),
        audit_entry_count: record.audit.len(),
    }
}

fn non_empty_ref(value: &str) -> io::Result<String> {
    let value = value.trim();
    if value.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "parent authorization reference must not be empty",
        ));
    }
    Ok(value.to_owned())
}
