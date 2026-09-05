//! Public, handle-free values exchanged with the safe owner.

use crate::platform::windows::{Identity, Metadata};

/// Maximum request identifier length accepted by the owner.
pub const MAX_REQUEST_ID_BYTES: usize = 128;
/// Maximum number of mutations in one owner transaction.
pub const MAX_TRANSACTION_MUTATIONS: usize = 256;

#[path = "owner_types_directory.rs"]
mod directory;
#[path = "owner_types_identity_display.rs"]
mod identity_display;
#[path = "owner_types_mutation.rs"]
mod mutation;
#[path = "owner_types_receipt.rs"]
mod receipt;

/// A Windows volume plus the complete 128-bit file identifier observed from a
/// retained handle. It is an observation, not a caller-supplied capability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FileIdentity {
    volume_serial_number: u64,
    file_id: [u8; 16],
}

impl FileIdentity {
    pub(crate) fn from_platform(identity: Identity) -> Self {
        Self {
            volume_serial_number: identity.volume_serial_number,
            file_id: identity.file_id,
        }
    }

    /// The volume serial number reported by Windows.
    pub fn volume_serial_number(&self) -> u64 {
        self.volume_serial_number
    }

    /// The complete 128-bit Windows file identifier.
    pub fn file_id(&self) -> [u8; 16] {
        self.file_id
    }
}

/// A handle-observed regular-file or directory snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FileStat {
    identity: FileIdentity,
    length: u64,
    links: u32,
    directory: bool,
    modified_ms: i64,
}

impl FileStat {
    pub(crate) fn from_platform(metadata: Metadata) -> Self {
        Self {
            identity: FileIdentity::from_platform(metadata.identity),
            length: metadata.length,
            links: metadata.links,
            directory: metadata.directory,
            modified_ms: metadata.modified_ms,
        }
    }

    pub fn identity(&self) -> FileIdentity {
        self.identity
    }

    pub fn length(&self) -> u64 {
        self.length
    }

    pub fn links(&self) -> u32 {
        self.links
    }

    pub fn is_directory(&self) -> bool {
        self.directory
    }

    pub fn modified_ms(&self) -> i64 {
        self.modified_ms
    }
}

/// Bytes and the stat captured from the same retained target handle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadSnapshot {
    stat: FileStat,
    bytes: Vec<u8>,
}

impl ReadSnapshot {
    pub(crate) fn new(stat: FileStat, bytes: Vec<u8>) -> Self {
        Self { stat, bytes }
    }

    pub fn stat(&self) -> FileStat {
        self.stat
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// One directory entry observed while its directory handle is retained.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectoryEntry {
    name: String,
    stat: FileStat,
}

/// One caller-requested mutation. Every path remains relative to the owner
/// root; this value contains no handle, identity, or authority token.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Mutation {
    Append {
        relative_path: String,
        payload: Vec<u8>,
    },
    Replace {
        relative_path: String,
        payload: Vec<u8>,
    },
    Remove {
        relative_path: String,
    },
    RemoveTree {
        relative_path: String,
    },
}

/// The terminal result persisted for a stable request identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReceiptOutcome {
    Appended { offset: u64, length: u64 },
    Replaced,
    Removed { existed: bool },
    TransactionCommitted { count: u32 },
    Unsupported { operation: String },
}

/// A provider-replayable terminal receipt. The owner writes this value before
/// clearing the intent, so resending a request identifier cannot duplicate a
/// committed mutation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MutationReceipt {
    request_id: String,
    operation: String,
    relative_path: String,
    outcome: ReceiptOutcome,
    replayed: bool,
}
