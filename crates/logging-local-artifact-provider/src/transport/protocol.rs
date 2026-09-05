//! Provider wire DTOs and the single raw-to-validated conversion boundary.

use serde::{Deserialize, Serialize};

#[path = "mutation_validation.rs"]
mod mutation_validation;
#[path = "operation_names.rs"]
mod operation_names;
#[path = "operation_validation.rs"]
mod operation_validation;
#[path = "protocol_text.rs"]
pub(crate) mod text;
#[path = "protocol_types.rs"]
pub(crate) mod types;
#[path = "protocol_validation.rs"]
mod validation;

pub(crate) const PROTOCOL_VERSION: u32 = 1;
pub(crate) const FRAME_PREFIX_BYTES: usize = 4;
pub(crate) const MAXIMUM_FRAME_BYTES: usize = 96 * 1024 * 1024;
pub(crate) const MAXIMUM_APPEND_BYTES: usize = 1024 * 1024;
pub(crate) const MAXIMUM_REPLACE_BYTES: usize = 64 * 1024 * 1024;
pub(crate) const MAXIMUM_READ_BYTES: u64 = 64 * 1024 * 1024;
pub(crate) const MAXIMUM_TRANSACTION_MUTATIONS: usize = 256;
pub(crate) const MAXIMUM_RELATIVE_PATH_BYTES: usize = 4_096;
pub(crate) const MAXIMUM_REQUEST_ID_BYTES: usize = 128;
pub(crate) const MAXIMUM_NONCE_BYTES: usize = 128;
pub(crate) const MAXIMUM_LEASE_ID_BYTES: usize = 128;

/// A request identifier validated at the protocol boundary.
///
/// BRAND-INVARIANT: the value is non-empty, lowercase hexadecimal, and no
/// longer than the bound for the identifier role that created it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProviderIdentifier(String);

/// A validated provider-relative path.
///
/// BRAND-INVARIANT: the value is UTF-8, uses forward slashes, stays below the
/// protocol byte bound, and contains no rooted, empty, dot, or parent segment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProviderRelativePath(String);

/// Base64 text whose encoded length is within one operation's bound.
///
/// BRAND-INVARIANT: the value is the exact wire payload text and was checked
/// against the operation's encoded-size bound before entering operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProviderPayload(String);

/// A validated read-size bound.
///
/// BRAND-INVARIANT: this value is non-zero and no larger than the provider
/// maximum read size.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ReadMaximum(u64);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(usize)]
pub(crate) enum ProtocolValidationError {
    ProtocolVersion,
    Identifier,
    RelativePath,
    ReadBound,
    PayloadBound,
    PayloadBoundOverflow,
    MutationCount,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Request {
    pub(crate) protocol_version: u32,
    pub(crate) request_id: String,
    pub(crate) nonce: String,
    pub(crate) lease_id: Option<String>,
    pub(crate) operation: Operation,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub(crate) enum Operation {
    BeginLease,
    EndLease {
        lease_id: String,
    },
    Recover,
    EnsureDirectory {
        relative_path: String,
    },
    SyncDirectory {
        relative_path: String,
    },
    Stat {
        relative_path: String,
    },
    ReadSnapshot {
        relative_path: String,
        maximum_bytes: u64,
    },
    Append {
        relative_path: String,
        payload_base64: String,
    },
    Replace {
        relative_path: String,
        payload_base64: String,
    },
    Remove {
        relative_path: String,
    },
    List {
        relative_path: String,
    },
    RemoveTree {
        relative_path: String,
    },
    ApplyTransaction {
        mutations: Vec<TransactionMutation>,
    },
    Shutdown,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub(crate) enum TransactionMutation {
    Replace {
        relative_path: String,
        payload_base64: String,
    },
    Remove {
        relative_path: String,
    },
    RemoveTree {
        relative_path: String,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(usize)]
pub(crate) enum OperationName {
    BeginLease,
    EndLease,
    Recover,
    EnsureDirectory,
    SyncDirectory,
    Stat,
    ReadSnapshot,
    Append,
    Replace,
    Remove,
    List,
    RemoveTree,
    ApplyTransaction,
    Shutdown,
}

#[derive(Debug)]
pub(crate) struct ValidatedRequest {
    request_id: ProviderIdentifier,
    nonce: ProviderIdentifier,
    lease_id: Option<ProviderIdentifier>,
    operation: ValidatedOperation,
}

impl ValidatedRequest {
    pub(crate) fn request_id(&self) -> &ProviderIdentifier {
        &self.request_id
    }

    pub(crate) fn nonce(&self) -> &ProviderIdentifier {
        &self.nonce
    }

    pub(crate) fn lease_id(&self) -> Option<&ProviderIdentifier> {
        self.lease_id.as_ref()
    }

    pub(crate) fn operation(&self) -> &ValidatedOperation {
        &self.operation
    }
}

#[derive(Debug)]
pub(crate) enum ValidatedOperation {
    BeginLease,
    EndLease {
        lease_id: ProviderIdentifier,
    },
    Recover,
    EnsureDirectory {
        relative_path: ProviderRelativePath,
    },
    SyncDirectory {
        relative_path: ProviderRelativePath,
    },
    Stat {
        relative_path: ProviderRelativePath,
    },
    ReadSnapshot {
        relative_path: ProviderRelativePath,
        maximum_bytes: ReadMaximum,
    },
    Append {
        relative_path: ProviderRelativePath,
        payload_base64: ProviderPayload,
    },
    Replace {
        relative_path: ProviderRelativePath,
        payload_base64: ProviderPayload,
    },
    Remove {
        relative_path: ProviderRelativePath,
    },
    List {
        relative_path: ProviderRelativePath,
    },
    RemoveTree {
        relative_path: ProviderRelativePath,
    },
    ApplyTransaction {
        mutations: Vec<ValidatedMutation>,
    },
    Shutdown,
}

#[derive(Debug)]
pub(crate) enum ValidatedMutation {
    Replace {
        relative_path: ProviderRelativePath,
        payload_base64: ProviderPayload,
    },
    Remove {
        relative_path: ProviderRelativePath,
    },
    RemoveTree {
        relative_path: ProviderRelativePath,
    },
}

#[derive(Debug, Serialize)]
pub(crate) struct ReadyFrame {
    pub(crate) protocol_version: u32,
    pub(crate) provider_instance_id: String,
    pub(crate) binary_sha256: String,
    pub(crate) root_identity: WireIdentity,
}

#[derive(Debug, Serialize)]
pub(crate) struct WireIdentity {
    pub(crate) device: String,
    pub(crate) inode: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct ErrorBody {
    pub(crate) code: String,
    pub(crate) message: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct Response {
    pub(crate) protocol_version: u32,
    pub(crate) request_id: String,
    pub(crate) operation: String,
    pub(crate) nonce: String,
    pub(crate) ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) result: Option<types::ResponseResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) error: Option<ErrorBody>,
}

pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &str = "0123456789abcdef";
    let hex = HEX.as_bytes();
    let mut encoded = String::with_capacity(bytes.len().saturating_mul(2));
    for &byte in bytes {
        encoded.push(char::from(hex[usize::from(byte >> 4)]));
        encoded.push(char::from(hex[usize::from(byte & 0x0f)]));
    }
    encoded
}
