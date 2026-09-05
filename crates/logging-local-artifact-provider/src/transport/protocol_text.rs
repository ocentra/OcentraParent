//! Stable provider wire and error vocabulary.

use serde_json::{Map, Value};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(usize)]
pub(crate) enum TextId {
    Empty,
    InvalidArguments,
    ProtocolFrame,
    ProtocolLimit,
    SizeLimit,
    UnsupportedProvider,
    ContainmentFailure,
    RootIdentityChanged,
    OwnershipChanged,
    LinkOrReparse,
    LockConflict,
    NotFound,
    AlreadyExists,
    DurabilityFailure,
    RecoveryUncertainty,
    Io,
    ProviderAuthority,
    AtomicMutationFailure,
    ValidationFailure,
    ProtocolVersionUnsupported,
    RelativePathInvalid,
    IdentifierInvalid,
    ReadBoundInvalid,
    MutationCountInvalid,
    PayloadBoundOverflow,
    PayloadBound,
    NativeProviderUnsupported,
    ArtifactPathNotContained,
    MutationRequestIdentifierInvalid,
    MutationRequestConflict,
    ArtifactRootIdentityChanged,
    AncestorIdentityChanged,
    LinkOrReparseDetected,
    UnexpectedHardLink,
    ArtifactOwnershipChanged,
    ArtifactOwnerLeaseBusy,
    ArtifactNotFound,
    ArtifactAlreadyExists,
    ArtifactOperationBound,
    NativeOwnerDurability,
    NativeOwnerRecovery,
    NativeOperationUnsupported,
    NativeArtifactFailed,
    ProviderLeaseAlreadyActive,
    LeaseNotCurrent,
    OperationRequiresLease,
    LeaseBeginNoSupplied,
    RequestedLeaseNotCurrent,
    ShutdownLeaseActive,
    RecoveredReceiptCount,
    AppendNewline,
    DirectoryEntryCount,
    DuplicateTransactionTarget,
    PayloadNotBase64,
    PayloadNotCanonical,
    ArtifactSizeUnrepresentable,
    ArtifactTimeUnrepresentable,
    ArtifactModificationTimeInvalid,
    SnapshotNotRegular,
    SnapshotLengthUnrepresentable,
    SnapshotIdentityMismatch,
    InvalidDirectoryEntry,
    UnexpectedReceipt,
    UnexpectedMutationOutcome,
    NativeUnsupportedOutcome,
    SecureIdentifierFailure,
    RootNotDirectory,
    BeginLeaseWire,
    EndLeaseWire,
    RecoverWire,
    EnsureDirectoryWire,
    SyncDirectoryWire,
    StatWire,
    ReadSnapshotWire,
    AppendWire,
    ReplaceWire,
    RemoveWire,
    ListWire,
    RemoveTreeWire,
    ApplyTransactionWire,
    ShutdownWire,
    ReceiptRemoveTree,
    ReceiptTransaction,
    CliPipeName,
    CliRoot,
    CliParentPid,
    Dot,
    DotDot,
    PipePrefix,
    LeaseIdKey,
    ReleasedKey,
    RecoveredKey,
    ReadyKey,
    SyncedKey,
    SizeKey,
    ModifiedMsKey,
    IsDirectoryKey,
    IdentityKey,
    DeviceKey,
    InodeKey,
    ContentBase64Key,
    StatKey,
    NameKey,
    WrittenKey,
    ReplayedKey,
    RemovedKey,
    AppliedKey,
    ShutdownKey,
}

const TEXT_VALUES: [&str; 108] = [
    TEXT_EMPTY,
    TEXT_INVALID_ARGUMENTS,
    TEXT_PROTOCOL_FRAME,
    TEXT_PROTOCOL_LIMIT,
    TEXT_SIZE_LIMIT,
    TEXT_UNSUPPORTED_PROVIDER,
    TEXT_CONTAINMENT_FAILURE,
    TEXT_ROOT_IDENTITY_CHANGED,
    TEXT_OWNERSHIP_CHANGED,
    TEXT_LINK_OR_REPARSE,
    TEXT_LOCK_CONFLICT,
    TEXT_NOT_FOUND,
    TEXT_ALREADY_EXISTS,
    TEXT_DURABILITY_FAILURE,
    TEXT_RECOVERY_UNCERTAINTY,
    TEXT_IO,
    TEXT_PROVIDER_AUTHORITY,
    TEXT_ATOMIC_MUTATION_FAILURE,
    TEXT_VALIDATION_FAILURE,
    TEXT_PROTOCOL_VERSION_UNSUPPORTED,
    TEXT_RELATIVE_PATH_INVALID,
    TEXT_IDENTIFIER_INVALID,
    TEXT_READ_BOUND_INVALID,
    TEXT_MUTATION_COUNT_INVALID,
    TEXT_PAYLOAD_BOUND_OVERFLOW,
    TEXT_PAYLOAD_BOUND,
    TEXT_NATIVE_PROVIDER_UNSUPPORTED,
    TEXT_ARTIFACT_PATH_NOT_CONTAINED,
    TEXT_MUTATION_REQUEST_IDENTIFIER_INVALID,
    TEXT_MUTATION_REQUEST_CONFLICT,
    TEXT_ARTIFACT_ROOT_IDENTITY_CHANGED,
    TEXT_ANCESTOR_IDENTITY_CHANGED,
    TEXT_LINK_OR_REPARSE_DETECTED,
    TEXT_UNEXPECTED_HARD_LINK,
    TEXT_ARTIFACT_OWNERSHIP_CHANGED,
    TEXT_ARTIFACT_OWNER_LEASE_BUSY,
    TEXT_ARTIFACT_NOT_FOUND,
    TEXT_ARTIFACT_ALREADY_EXISTS,
    TEXT_ARTIFACT_OPERATION_BOUND,
    TEXT_NATIVE_OWNER_DURABILITY,
    TEXT_NATIVE_OWNER_RECOVERY,
    TEXT_NATIVE_OPERATION_UNSUPPORTED,
    TEXT_NATIVE_ARTIFACT_FAILED,
    TEXT_PROVIDER_LEASE_ALREADY_ACTIVE,
    TEXT_LEASE_NOT_CURRENT,
    TEXT_OPERATION_REQUIRES_LEASE,
    TEXT_LEASE_BEGIN_NO_SUPPLIED,
    TEXT_REQUESTED_LEASE_NOT_CURRENT,
    TEXT_SHUTDOWN_LEASE_ACTIVE,
    TEXT_RECOVERED_RECEIPT_COUNT,
    TEXT_APPEND_NEWLINE,
    TEXT_DIRECTORY_ENTRY_COUNT,
    TEXT_DUPLICATE_TRANSACTION_TARGET,
    TEXT_PAYLOAD_NOT_BASE64,
    TEXT_PAYLOAD_NOT_CANONICAL,
    TEXT_ARTIFACT_SIZE_UNREPRESENTABLE,
    TEXT_ARTIFACT_TIME_UNREPRESENTABLE,
    TEXT_ARTIFACT_MODIFICATION_TIME_INVALID,
    TEXT_SNAPSHOT_NOT_REGULAR,
    TEXT_SNAPSHOT_LENGTH_UNREPRESENTABLE,
    TEXT_SNAPSHOT_IDENTITY_MISMATCH,
    TEXT_INVALID_DIRECTORY_ENTRY,
    TEXT_UNEXPECTED_RECEIPT,
    TEXT_UNEXPECTED_MUTATION_OUTCOME,
    TEXT_NATIVE_UNSUPPORTED_OUTCOME,
    TEXT_SECURE_IDENTIFIER_FAILURE,
    TEXT_ROOT_NOT_DIRECTORY,
    TEXT_BEGIN_LEASE_WIRE,
    TEXT_END_LEASE_WIRE,
    TEXT_RECOVER_WIRE,
    TEXT_ENSURE_DIRECTORY_WIRE,
    TEXT_SYNC_DIRECTORY_WIRE,
    TEXT_STAT_WIRE,
    TEXT_READ_SNAPSHOT_WIRE,
    TEXT_APPEND_WIRE,
    TEXT_REPLACE_WIRE,
    TEXT_REMOVE_WIRE,
    TEXT_LIST_WIRE,
    TEXT_REMOVE_TREE_WIRE,
    TEXT_APPLY_TRANSACTION_WIRE,
    TEXT_SHUTDOWN_WIRE,
    TEXT_RECEIPT_REMOVE_TREE,
    TEXT_RECEIPT_TRANSACTION,
    TEXT_CLI_PIPE_NAME,
    TEXT_CLI_ROOT,
    TEXT_CLI_PARENT_PID,
    TEXT_DOT,
    TEXT_DOT_DOT,
    TEXT_PIPE_PREFIX,
    TEXT_LEASE_ID_KEY,
    TEXT_RELEASED_KEY,
    TEXT_RECOVERED_KEY,
    TEXT_READY_KEY,
    TEXT_SYNCED_KEY,
    TEXT_SIZE_KEY,
    TEXT_MODIFIED_MS_KEY,
    TEXT_IS_DIRECTORY_KEY,
    TEXT_IDENTITY_KEY,
    TEXT_DEVICE_KEY,
    TEXT_INODE_KEY,
    TEXT_CONTENT_BASE64_KEY,
    TEXT_STAT_KEY,
    TEXT_NAME_KEY,
    TEXT_WRITTEN_KEY,
    TEXT_REPLAYED_KEY,
    TEXT_REMOVED_KEY,
    TEXT_APPLIED_KEY,
    TEXT_SHUTDOWN_KEY,
];

const TEXT_EMPTY: &str = "";
const TEXT_INVALID_ARGUMENTS: &str = "invalid-arguments";
const TEXT_PROTOCOL_FRAME: &str = "protocol-frame";
const TEXT_PROTOCOL_LIMIT: &str = "protocol-limit";
const TEXT_SIZE_LIMIT: &str = "size-limit";
const TEXT_UNSUPPORTED_PROVIDER: &str = "unsupported-provider";
const TEXT_CONTAINMENT_FAILURE: &str = "containment-failure";
const TEXT_ROOT_IDENTITY_CHANGED: &str = "root-identity-changed";
const TEXT_OWNERSHIP_CHANGED: &str = "ownership-changed";
const TEXT_LINK_OR_REPARSE: &str = "link-or-reparse";
const TEXT_LOCK_CONFLICT: &str = "lock-conflict";
const TEXT_NOT_FOUND: &str = "not-found";
const TEXT_ALREADY_EXISTS: &str = "already-exists";
const TEXT_DURABILITY_FAILURE: &str = "durability-failure";
const TEXT_RECOVERY_UNCERTAINTY: &str = "recovery-uncertainty";
const TEXT_IO: &str = "io";
const TEXT_PROVIDER_AUTHORITY: &str = "provider-authority";
const TEXT_ATOMIC_MUTATION_FAILURE: &str = "atomic-mutation-failure";
const TEXT_VALIDATION_FAILURE: &str = "the provider request failed frame validation";
const TEXT_PROTOCOL_VERSION_UNSUPPORTED: &str = "request protocol version is unsupported";
const TEXT_RELATIVE_PATH_INVALID: &str = "relative artifact path is invalid";
const TEXT_IDENTIFIER_INVALID: &str = "provider identifiers must use lowercase hexadecimal";
const TEXT_READ_BOUND_INVALID: &str = "read bound is invalid";
const TEXT_MUTATION_COUNT_INVALID: &str = "transaction mutation count is invalid";
const TEXT_PAYLOAD_BOUND_OVERFLOW: &str = "payload bound overflow";
const TEXT_PAYLOAD_BOUND: &str = "the payload exceeded its operation bound";
const TEXT_NATIVE_PROVIDER_UNSUPPORTED: &str = "the native provider is unsupported";
const TEXT_ARTIFACT_PATH_NOT_CONTAINED: &str = "the artifact path is not contained";
const TEXT_MUTATION_REQUEST_IDENTIFIER_INVALID: &str = "the mutation request identifier is invalid";
const TEXT_MUTATION_REQUEST_CONFLICT: &str = "the mutation request conflicts with retained \
     provider state";
const TEXT_ARTIFACT_ROOT_IDENTITY_CHANGED: &str = "the artifact root identity changed";
const TEXT_ANCESTOR_IDENTITY_CHANGED: &str = "an artifact ancestor identity changed";
const TEXT_LINK_OR_REPARSE_DETECTED: &str = "a link or reparse point was detected";
const TEXT_UNEXPECTED_HARD_LINK: &str = "an unexpected hard link was detected";
const TEXT_ARTIFACT_OWNERSHIP_CHANGED: &str = "artifact ownership changed";
const TEXT_ARTIFACT_OWNER_LEASE_BUSY: &str = "the artifact owner lease is busy";
const TEXT_ARTIFACT_NOT_FOUND: &str = "the artifact was not found";
const TEXT_ARTIFACT_ALREADY_EXISTS: &str = "the artifact already exists";
const TEXT_ARTIFACT_OPERATION_BOUND: &str = "the artifact operation exceeded its bound";
const TEXT_NATIVE_OWNER_DURABILITY: &str = "the native owner could not establish durable artifact \
     state";
const TEXT_NATIVE_OWNER_RECOVERY: &str = "the native owner requires artifact recovery";
const TEXT_NATIVE_OPERATION_UNSUPPORTED: &str = "the native operation is unsupported";
const TEXT_NATIVE_ARTIFACT_FAILED: &str = "the native artifact operation failed";
const TEXT_PROVIDER_LEASE_ALREADY_ACTIVE: &str = "a provider lease is already active";
const TEXT_LEASE_NOT_CURRENT: &str = "the provider lease is not current";
const TEXT_OPERATION_REQUIRES_LEASE: &str = "the operation requires a provider lease";
const TEXT_LEASE_BEGIN_NO_SUPPLIED: &str = "a lease cannot be supplied while beginning a lease";
const TEXT_REQUESTED_LEASE_NOT_CURRENT: &str = "the requested lease is not current";
const TEXT_SHUTDOWN_LEASE_ACTIVE: &str = "the provider cannot shut down while a lease is active";
const TEXT_RECOVERED_RECEIPT_COUNT: &str = "the recovered receipt count exceeded its bound";
const TEXT_APPEND_NEWLINE: &str = "append payload must be newline terminated";
const TEXT_DIRECTORY_ENTRY_COUNT: &str = "the directory entry count exceeded its bound";
const TEXT_DUPLICATE_TRANSACTION_TARGET: &str = "the transaction contains duplicate targets";
const TEXT_PAYLOAD_NOT_BASE64: &str = "the payload is not valid base64";
const TEXT_PAYLOAD_NOT_CANONICAL: &str = "the payload is not canonical base64";
const TEXT_ARTIFACT_SIZE_UNREPRESENTABLE: &str = "the artifact size cannot be represented";
const TEXT_ARTIFACT_TIME_UNREPRESENTABLE: &str = "the artifact time cannot be represented";
const TEXT_ARTIFACT_MODIFICATION_TIME_INVALID: &str = "the artifact modification time is invalid";
const TEXT_SNAPSHOT_NOT_REGULAR: &str = "the snapshot target is not a regular file";
const TEXT_SNAPSHOT_LENGTH_UNREPRESENTABLE: &str = "the snapshot length is not representable";
const TEXT_SNAPSHOT_IDENTITY_MISMATCH: &str = "the snapshot identity does not match its bytes";
const TEXT_INVALID_DIRECTORY_ENTRY: &str = "the native owner returned an invalid directory entry";
const TEXT_UNEXPECTED_RECEIPT: &str = "the native owner returned an unexpected mutation receipt";
const TEXT_UNEXPECTED_MUTATION_OUTCOME: &str = "the native owner returned an unexpected mutation \
     outcome";
const TEXT_NATIVE_UNSUPPORTED_OUTCOME: &str = "the native owner reported an unsupported operation";
const TEXT_SECURE_IDENTIFIER_FAILURE: &str = "the provider could not create a secure identifier";
const TEXT_ROOT_NOT_DIRECTORY: &str = "the artifact owner root is not a directory";
const TEXT_BEGIN_LEASE_WIRE: &str = "beginLease";
const TEXT_END_LEASE_WIRE: &str = "endLease";
const TEXT_RECOVER_WIRE: &str = "recover";
const TEXT_ENSURE_DIRECTORY_WIRE: &str = "ensureDirectory";
const TEXT_SYNC_DIRECTORY_WIRE: &str = "syncDirectory";
const TEXT_STAT_WIRE: &str = "stat";
const TEXT_READ_SNAPSHOT_WIRE: &str = "readSnapshot";
const TEXT_APPEND_WIRE: &str = "append";
const TEXT_REPLACE_WIRE: &str = "replace";
const TEXT_REMOVE_WIRE: &str = "remove";
const TEXT_LIST_WIRE: &str = "list";
const TEXT_REMOVE_TREE_WIRE: &str = "removeTree";
const TEXT_APPLY_TRANSACTION_WIRE: &str = "applyTransaction";
const TEXT_SHUTDOWN_WIRE: &str = "shutdown";
const TEXT_RECEIPT_REMOVE_TREE: &str = "remove-tree";
const TEXT_RECEIPT_TRANSACTION: &str = "transaction";
const TEXT_CLI_PIPE_NAME: &str = "--pipe-name";
const TEXT_CLI_ROOT: &str = "--root";
const TEXT_CLI_PARENT_PID: &str = "--parent-pid";
const TEXT_DOT: &str = ".";
const TEXT_DOT_DOT: &str = "..";
const TEXT_PIPE_PREFIX: &str = "\\\\.\\pipe\\";
const TEXT_LEASE_ID_KEY: &str = "lease_id";
const TEXT_RELEASED_KEY: &str = "released";
const TEXT_RECOVERED_KEY: &str = "recovered";
const TEXT_READY_KEY: &str = "ready";
const TEXT_SYNCED_KEY: &str = "synced";
const TEXT_SIZE_KEY: &str = "size";
const TEXT_MODIFIED_MS_KEY: &str = "modified_ms";
const TEXT_IS_DIRECTORY_KEY: &str = "is_directory";
const TEXT_IDENTITY_KEY: &str = "identity";
const TEXT_DEVICE_KEY: &str = "device";
const TEXT_INODE_KEY: &str = "inode";
const TEXT_CONTENT_BASE64_KEY: &str = "content_base64";
const TEXT_STAT_KEY: &str = "stat";
const TEXT_NAME_KEY: &str = "name";
const TEXT_WRITTEN_KEY: &str = "written";
const TEXT_REPLAYED_KEY: &str = "replayed";
const TEXT_REMOVED_KEY: &str = "removed";
const TEXT_APPLIED_KEY: &str = "applied";
const TEXT_SHUTDOWN_KEY: &str = "shutdown";

impl TextId {
    pub(crate) fn text(self) -> String {
        TEXT_VALUES[self as usize].to_owned()
    }

    pub(crate) fn argument(value: &str) -> Option<Self> {
        [Self::CliPipeName, Self::CliRoot, Self::CliParentPid]
            .into_iter()
            .find(|candidate| candidate.text() == value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ErrorText {
    code: TextId,
    message: TextId,
}

impl ErrorText {
    pub(crate) const fn new(code: TextId, message: TextId) -> Self {
        Self { code, message }
    }
    pub(crate) fn code(self) -> String {
        self.code.text()
    }
    pub(crate) fn message(self) -> String {
        self.message.text()
    }
}

pub(crate) fn object(fields: Vec<(TextId, Value)>) -> super::types::ResponseResult {
    let mut object = Map::new();
    for (key, value) in fields {
        object.insert(key.text(), value);
    }
    super::types::ResponseResult::from(Value::Object(object))
}

pub(crate) fn array(values: Vec<super::types::ResponseResult>) -> super::types::ResponseResult {
    Value::Array(
        values
            .into_iter()
            .map(super::types::ResponseResult::into_value)
            .collect(),
    )
    .into()
}

pub(crate) fn null() -> super::types::ResponseResult {
    Value::Null.into()
}

pub(crate) const VALIDATION_FAILURE: ErrorText =
    ErrorText::new(TextId::InvalidArguments, TextId::ValidationFailure);
pub(crate) const VALIDATION_PROTOCOL_FRAME: ErrorText =
    ErrorText::new(TextId::ProtocolFrame, TextId::ValidationFailure);
pub(crate) const VALIDATION_PROTOCOL_LIMIT: ErrorText =
    ErrorText::new(TextId::ProtocolLimit, TextId::ValidationFailure);
pub(crate) const VALIDATION_SIZE_LIMIT: ErrorText =
    ErrorText::new(TextId::SizeLimit, TextId::ValidationFailure);
pub(crate) const PROTOCOL_FRAME: ErrorText =
    ErrorText::new(TextId::ProtocolFrame, TextId::ProtocolFrame);
pub(crate) const PROTOCOL_VERSION_UNSUPPORTED: ErrorText =
    ErrorText::new(TextId::ProtocolFrame, TextId::ProtocolVersionUnsupported);
pub(crate) const PAYLOAD_BOUND: ErrorText = ErrorText::new(TextId::SizeLimit, TextId::PayloadBound);
pub(crate) const RELATIVE_PATH_INVALID: ErrorText =
    ErrorText::new(TextId::InvalidArguments, TextId::RelativePathInvalid);
pub(crate) const IDENTIFIER_INVALID: ErrorText =
    ErrorText::new(TextId::InvalidArguments, TextId::IdentifierInvalid);
pub(crate) const READ_BOUND_INVALID: ErrorText =
    ErrorText::new(TextId::SizeLimit, TextId::ReadBoundInvalid);
pub(crate) const MUTATION_COUNT_INVALID: ErrorText =
    ErrorText::new(TextId::ProtocolLimit, TextId::MutationCountInvalid);
pub(crate) const PAYLOAD_BOUND_OVERFLOW: ErrorText =
    ErrorText::new(TextId::ProtocolLimit, TextId::PayloadBoundOverflow);
pub(crate) const UNSUPPORTED_PROVIDER: ErrorText = ErrorText::new(
    TextId::UnsupportedProvider,
    TextId::NativeProviderUnsupported,
);
pub(crate) const CONTAINMENT_FAILURE: ErrorText =
    ErrorText::new(TextId::ContainmentFailure, TextId::ArtifactPathNotContained);
pub(crate) const MUTATION_REQUEST_ID_INVALID: ErrorText = ErrorText::new(
    TextId::InvalidArguments,
    TextId::MutationRequestIdentifierInvalid,
);
pub(crate) const RETAINED_REQUEST_CONFLICT: ErrorText =
    ErrorText::new(TextId::ProviderAuthority, TextId::MutationRequestConflict);
pub(crate) const ROOT_IDENTITY_CHANGED: ErrorText = ErrorText::new(
    TextId::RootIdentityChanged,
    TextId::ArtifactRootIdentityChanged,
);
pub(crate) const ANCESTOR_IDENTITY_CHANGED: ErrorText =
    ErrorText::new(TextId::OwnershipChanged, TextId::AncestorIdentityChanged);
pub(crate) const LINK_OR_REPARSE: ErrorText =
    ErrorText::new(TextId::LinkOrReparse, TextId::LinkOrReparseDetected);
pub(crate) const HARDLINK_CHANGED: ErrorText =
    ErrorText::new(TextId::OwnershipChanged, TextId::UnexpectedHardLink);
pub(crate) const OWNERSHIP_CHANGED: ErrorText =
    ErrorText::new(TextId::OwnershipChanged, TextId::ArtifactOwnershipChanged);
pub(crate) const LOCK_CONFLICT: ErrorText =
    ErrorText::new(TextId::LockConflict, TextId::ArtifactOwnerLeaseBusy);
pub(crate) const ARTIFACT_NOT_FOUND: ErrorText =
    ErrorText::new(TextId::NotFound, TextId::ArtifactNotFound);
pub(crate) const ARTIFACT_ALREADY_EXISTS: ErrorText =
    ErrorText::new(TextId::AlreadyExists, TextId::ArtifactAlreadyExists);
pub(crate) const ARTIFACT_SIZE_LIMIT: ErrorText =
    ErrorText::new(TextId::SizeLimit, TextId::ArtifactOperationBound);
pub(crate) const DURABILITY_FAILURE: ErrorText =
    ErrorText::new(TextId::DurabilityFailure, TextId::NativeOwnerDurability);
pub(crate) const RECOVERY_UNCERTAINTY: ErrorText =
    ErrorText::new(TextId::RecoveryUncertainty, TextId::NativeOwnerRecovery);
pub(crate) const UNSUPPORTED_OPERATION: ErrorText = ErrorText::new(
    TextId::UnsupportedProvider,
    TextId::NativeOperationUnsupported,
);
pub(crate) const NATIVE_FAILURE: ErrorText =
    ErrorText::new(TextId::Io, TextId::NativeArtifactFailed);
pub(crate) const LEASE_ALREADY_ACTIVE: ErrorText = ErrorText::new(
    TextId::ProviderAuthority,
    TextId::ProviderLeaseAlreadyActive,
);
pub(crate) const LEASE_NOT_CURRENT: ErrorText =
    ErrorText::new(TextId::ProviderAuthority, TextId::LeaseNotCurrent);
pub(crate) const LEASE_REQUIRED: ErrorText =
    ErrorText::new(TextId::ProviderAuthority, TextId::OperationRequiresLease);
pub(crate) const LEASE_BEGIN_ARGUMENT: ErrorText =
    ErrorText::new(TextId::ProviderAuthority, TextId::LeaseBeginNoSupplied);
pub(crate) const REQUESTED_LEASE_NOT_CURRENT: ErrorText =
    ErrorText::new(TextId::ProviderAuthority, TextId::RequestedLeaseNotCurrent);
pub(crate) const SHUTDOWN_LEASE_ACTIVE: ErrorText =
    ErrorText::new(TextId::ProviderAuthority, TextId::ShutdownLeaseActive);
pub(crate) const RECOVERED_RECEIPT_LIMIT: ErrorText =
    ErrorText::new(TextId::ProtocolLimit, TextId::RecoveredReceiptCount);
pub(crate) const APPEND_NOT_NEWLINE_TERMINATED: ErrorText =
    ErrorText::new(TextId::SizeLimit, TextId::AppendNewline);
pub(crate) const DIRECTORY_ENTRY_LIMIT: ErrorText =
    ErrorText::new(TextId::ProtocolLimit, TextId::DirectoryEntryCount);
pub(crate) const DUPLICATE_TRANSACTION_TARGET: ErrorText = ErrorText::new(
    TextId::ProviderAuthority,
    TextId::DuplicateTransactionTarget,
);
pub(crate) const PAYLOAD_NOT_BASE64: ErrorText =
    ErrorText::new(TextId::InvalidArguments, TextId::PayloadNotBase64);
pub(crate) const PAYLOAD_NOT_CANONICAL: ErrorText =
    ErrorText::new(TextId::InvalidArguments, TextId::PayloadNotCanonical);
pub(crate) const JSON_SIZE_UNREPRESENTABLE: ErrorText =
    ErrorText::new(TextId::ProtocolLimit, TextId::ArtifactSizeUnrepresentable);
pub(crate) const JSON_TIME_UNREPRESENTABLE: ErrorText =
    ErrorText::new(TextId::ProtocolLimit, TextId::ArtifactTimeUnrepresentable);
pub(crate) const MODIFIED_TIME_INVALID: ErrorText =
    ErrorText::new(TextId::Io, TextId::ArtifactModificationTimeInvalid);
pub(crate) const SNAPSHOT_NOT_REGULAR: ErrorText =
    ErrorText::new(TextId::OwnershipChanged, TextId::SnapshotNotRegular);
pub(crate) const SNAPSHOT_LENGTH_UNREPRESENTABLE: ErrorText =
    ErrorText::new(TextId::SizeLimit, TextId::SnapshotLengthUnrepresentable);
pub(crate) const SNAPSHOT_IDENTITY_MISMATCH: ErrorText =
    ErrorText::new(TextId::OwnershipChanged, TextId::SnapshotIdentityMismatch);
pub(crate) const INVALID_DIRECTORY_ENTRY: ErrorText =
    ErrorText::new(TextId::ProtocolFrame, TextId::InvalidDirectoryEntry);
pub(crate) const UNEXPECTED_RECEIPT: ErrorText =
    ErrorText::new(TextId::ProviderAuthority, TextId::UnexpectedReceipt);
pub(crate) const UNEXPECTED_OUTCOME: ErrorText = ErrorText::new(
    TextId::AtomicMutationFailure,
    TextId::UnexpectedMutationOutcome,
);
pub(crate) const UNSUPPORTED_OUTCOME: ErrorText = ErrorText::new(
    TextId::UnsupportedProvider,
    TextId::NativeUnsupportedOutcome,
);
pub(crate) const SECURE_IDENTIFIER_FAILURE: ErrorText =
    ErrorText::new(TextId::ProviderAuthority, TextId::SecureIdentifierFailure);
pub(crate) const ROOT_NOT_DIRECTORY: ErrorText =
    ErrorText::new(TextId::ProviderAuthority, TextId::RootNotDirectory);

pub(crate) const RECEIPT_REMOVE_TREE: TextId = TextId::ReceiptRemoveTree;
pub(crate) const RECEIPT_TRANSACTION: TextId = TextId::ReceiptTransaction;

pub(crate) const OPERATION_TEXT: [TextId; 14] = [
    TextId::BeginLeaseWire,
    TextId::EndLeaseWire,
    TextId::RecoverWire,
    TextId::EnsureDirectoryWire,
    TextId::SyncDirectoryWire,
    TextId::StatWire,
    TextId::ReadSnapshotWire,
    TextId::AppendWire,
    TextId::ReplaceWire,
    TextId::RemoveWire,
    TextId::ListWire,
    TextId::RemoveTreeWire,
    TextId::ApplyTransactionWire,
    TextId::ShutdownWire,
];
