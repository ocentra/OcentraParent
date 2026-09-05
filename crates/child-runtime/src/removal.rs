//! Durable parent-authorized trust removal for the child service.
//!
//! This boundary records revocation and reauthorization decisions owned by the
//! child service. Platform package/device-owner removal remains external and
//! therefore stays manual-required in the status surface.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

mod removal_authority;
mod removal_boundary;
mod removal_record;
mod removal_storage;
mod removal_storage_read;
mod removal_transitions;
mod removal_validation;

pub(super) const REMOVAL_STATE_VERSION: u16 = 1;

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
pub enum ChildAgentTamperSignalKind {
    PackageIntegrity,
    EntitlementSnapshot,
    SealedTrustMaterial,
    RuntimeEvidence,
    PlatformIntegrity,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildAgentTamperSignal {
    pub signal_ref: String,
    pub kind: ChildAgentTamperSignalKind,
    pub observed_at_unix_seconds: u64,
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
pub(crate) struct ChildAgentServiceIdentity {
    pub(super) household_id: String,
    pub(super) child_profile_id: String,
    pub(super) target_device_id: String,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChildAgentRemovalStatus {
    pub trust_state: ChildAgentTrustState,
    pub cleanup_state: ChildAgentCleanupState,
    pub latest_audit_ref: Option<String>,
    pub latest_parent_authorization_ref: Option<String>,
    pub audit_entry_count: usize,
    pub latest_tamper_signal_ref: Option<String>,
    pub tamper_signal_count: usize,
}

/// A current-state authority can only be constructed by the family verifier.
/// This wrapper is non-cloneable and consumed by each durable transition.
#[derive(Debug, PartialEq, Eq)]
pub struct VerifiedParentRemovalAuthorization {
    pub(super) reference: String,
    pub(super) action: ChildAgentRemovalAuthorizationAction,
    pub(super) identity: ChildAgentServiceIdentity,
    pub(super) authority_nonce: String,
    pub(super) authority_generation: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChildAgentRemovalBoundary {
    pub(super) path: PathBuf,
    pub(super) identity: Option<ChildAgentServiceIdentity>,
}

impl ChildAgentRemovalBoundary {
    pub fn open(path: impl Into<PathBuf>) -> std::io::Result<Self> {
        Self::open_with_identity(path, None)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
