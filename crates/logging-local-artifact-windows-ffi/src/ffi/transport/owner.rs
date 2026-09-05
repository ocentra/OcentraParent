//! Safe owner and lifetime-bound mutation session.

use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::constants::{
    BRIDGE_DIRECTORY, DIRECTORY_ENTRY_NOT_UNICODE, DIRECTORY_ESCAPED_ROOT,
    DIRECTORY_UNSAFE_COMPONENT, MUTATION_LOCK_FILE, MUTATION_OWNER_DIRECTORY, PATH_NOT_DIRECTORY,
    ROOT_DIRECTORY_CHAIN_EMPTY, ROOT_NOT_FILESYSTEM_ROOT,
};
use crate::error::{io_error, ArtifactError};
use crate::owner_journal::io::ensure_metadata_dirs;
use crate::owner_paths::{
    ensure_root_directory, open_directory_chain, validate_relative, DirectoryChain,
};
use crate::owner_types::{
    DirectoryEntry, FileIdentity, FileStat, Mutation, MutationReceipt, ReadSnapshot, ReceiptOutcome,
};
use crate::platform::windows::{verify_metadata, Identity, OwnedFile};

#[path = "owner_current.rs"]
mod current;

pub type LocalArtifactIdentity = FileIdentity;
pub type LocalArtifactStat = FileStat;
pub type LocalArtifactDirectoryEntry = DirectoryEntry;
pub type LocalArtifactMutation = Mutation;
pub type LocalArtifactMutationReceipt = MutationReceipt;
pub type LocalArtifactMutationOutcome = ReceiptOutcome;
pub type LocalArtifactReadSnapshot = crate::owner_types::ReadSnapshot;

/// A retained root and all of its existing filesystem ancestors. This object
/// owns the authority boundary; callers can supply only relative names and
/// request identifiers to a session derived from it.
#[derive(Debug)]
pub struct LocalArtifactMutationOwner {
    pub(super) root_path: PathBuf,
    pub(super) root_chain: DirectoryChain,
    pub(super) root_identities: Vec<Identity>,
    pub(super) root_identity: Identity,
}

impl LocalArtifactMutationOwner {
    /// Establish a Windows owner for one non-root directory.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, ArtifactError> {
        let initial_chain = ensure_root_directory(path.as_ref())?;
        let initial_root = initial_chain
            .paths
            .last()
            .cloned()
            .ok_or(ArtifactError::InvalidPath(ROOT_DIRECTORY_CHAIN_EMPTY))?;
        // `ensure_root_directory` has already opened every existing ancestor
        // with OPEN_REPARSE_POINT and verified its identity/kind.  Retain that
        // normalized, non-reparse path instead of reopening it through the
        // path canonicalizer, whose share mode conflicts with the held root
        // lease on Windows.
        let root_path = initial_root;
        if root_path
            .parent()
            .map(|parent| parent == root_path.as_path())
            .unwrap_or(true)
        {
            return Err(ArtifactError::InvalidPath(ROOT_NOT_FILESYSTEM_ROOT));
        }
        let root_chain = open_directory_chain(&root_path)?;
        let mut root_identities = Vec::with_capacity(root_chain.handles.len());
        for handle in &root_chain.handles {
            root_identities.push(verify_metadata(handle, true)?.identity);
        }
        let root_identity = *root_identities
            .last()
            .ok_or(ArtifactError::InvalidPath(ROOT_DIRECTORY_CHAIN_EMPTY))?;
        Ok(Self {
            root_path,
            root_chain,
            root_identities,
            root_identity,
        })
    }

    pub fn root_path(&self) -> &Path {
        &self.root_path
    }

    pub fn root_identity(&self) -> FileIdentity {
        FileIdentity::from_platform(self.root_identity)
    }

    /// Acquire the root mutation lease and recover all durable intents before
    /// returning a session. The lease and all root/metadata handles remain
    /// alive until the session is dropped.
    pub fn session(&self) -> Result<MutationSession<'_>, ArtifactError> {
        self.verify_current()?;
        let metadata = ensure_metadata_dirs(&self.root_path)?;
        let lock_path = self
            .root_path
            .join(BRIDGE_DIRECTORY)
            .join(MUTATION_OWNER_DIRECTORY)
            .join(MUTATION_LOCK_FILE);
        let lock = match OwnedFile::open_lock_file(&lock_path) {
            Ok(file) => file,
            Err(ArtifactError::AlreadyExists) => OwnedFile::open_existing_lock_file(&lock_path)?,
            Err(error) => return Err(error),
        };
        lock.try_lock_exclusive()?;
        let mut session = MutationSession {
            owner: self,
            metadata,
            lock,
        };
        session.recover()?;
        Ok(session)
    }
}

/// A non-cloneable, root-locked owner lease. All mutation operations are
/// implemented in `owner_mutations`; this type owns their shared guards.
pub struct MutationSession<'a> {
    pub(super) owner: &'a LocalArtifactMutationOwner,
    pub(super) metadata: crate::owner_journal::MetadataDirs,
    pub(super) lock: OwnedFile,
}

#[path = "owner_session_paths.rs"]
mod session_paths;
#[path = "owner_session_reads.rs"]
mod session_reads;

impl<'a> MutationSession<'a> {
    pub fn root_identity(&self) -> FileIdentity {
        self.owner.root_identity()
    }

    pub fn verify_current(&self) -> Result<(), ArtifactError> {
        self.owner.verify_current()
    }
}

impl Drop for MutationSession<'_> {
    fn drop(&mut self) {
        let _ = self.lock.unlock();
    }
}

/// Directory durability is an explicit result, never an alias for merely
/// ensuring a directory exists.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectoryDurability {
    Synced,
}
