pub const CURRENT_COMPONENT: &str = ".";
pub const PARENT_COMPONENT: &str = "..";
pub const ROOT_SEPARATOR: &str = "\\";

pub const BRIDGE_DIRECTORY: &str = ".bridge";
pub const MUTATION_OWNER_DIRECTORY: &str = ".mutation-owner";
pub const INTENTS_DIRECTORY: &str = "intents";
pub const MUTATION_LOCK_FILE: &str = "mutation.lock";

pub const APPEND_OPERATION: &str = "append";
pub const APPEND_CREATED_PHASE: &str = "append-created";
pub const APPEND_WRITING_PHASE: &str = "append-writing";
pub const APPEND_WRITTEN_PHASE: &str = "append-written";
pub const REPLACE_OPERATION: &str = "replace";
pub const REPLACE_STAGED_PHASE: &str = "replace-staged";
pub const REPLACE_QUARANTINED_PHASE: &str = "replace-quarantined";
pub const REPLACE_INSTALLED_PHASE: &str = "replace-installed";
pub const REMOVE_OPERATION: &str = "remove";
pub const REMOVE_DELETED_PHASE: &str = "remove-deleted";
pub const REMOVE_TREE_OPERATION: &str = "remove-tree";
pub const REMOVE_TREE_QUARANTINED_PHASE: &str = "remove-tree-quarantined";
pub const REMOVE_TREE_RECOVERY_QUARANTINED_PHASE: &str = "remove-tree-recovery-quarantined";
pub const TRANSACTION_OPERATION: &str = "transaction";
pub const TRANSACTION_STAGED_PHASE: &str = "transaction-staged";
pub const TRANSACTION_QUARANTINED_PHASE: &str = "transaction-quarantined";
pub const TRANSACTION_INSTALLED_PHASE: &str = "transaction-installed";

pub const ROOT_DIRECTORY_CHAIN_EMPTY: &str = "root directory chain is empty";
pub const ROOT_NOT_FILESYSTEM_ROOT: &str = "local-artifact root must not be a filesystem root";
pub const ROOT_NOT_DIRECTORY: &str = "local-artifact root is not a directory";
pub const ROOT_PATH_NOT_UNICODE: &str = "root path is not valid Unicode";
pub const ROOT_HAS_DOT_COMPONENT: &str = "root path must not contain dot components";
pub const DIRECTORY_ESCAPED_ROOT: &str = "directory escaped owner root";
pub const DIRECTORY_UNSAFE_COMPONENT: &str = "directory contains unsafe component";
pub const PATH_NOT_DIRECTORY: &str = "path is not a directory";
pub const DIRECTORY_ENTRY_NOT_UNICODE: &str = "directory entry is not valid Unicode";
pub const RELATIVE_PATH_EMPTY_OR_LONG: &str = "relative artifact path is empty or too long";
pub const PATH_NOT_RELATIVE: &str = "artifact path must be relative";
pub const PATH_NOT_UNICODE: &str = "artifact path is not valid Unicode";
pub const PATH_UNSAFE_COMPONENT: &str = "artifact path contains an unsafe component";
pub const PATH_EMPTY: &str = "artifact path is empty";
pub const METADATA_NOT_TARGET: &str = "the owner metadata directory is not an artifact target";
pub const TARGET_NO_PARENT: &str = "artifact target has no parent";
pub const TARGET_NOT_UNICODE: &str = "artifact target name is not valid Unicode";
pub const COMPONENT_TOO_LONG: &str = "artifact path component is too long";
pub const COMPONENT_ALIASING_SUFFIX: &str = "artifact path component has a Windows aliasing suffix";
pub const PROCESS_IMAGE_NOT_FILE: &str = "local-artifact process image must be a file";
pub const PATH_MUST_BE_DIRECTORY: &str = "local-artifact path must be a directory";
pub const PATH_MUST_BE_FILE: &str = "local-artifact path must be a file";
pub const DESTINATION_NOT_COMPONENT: &str = "destination name must be one path component";
pub const DESTINATION_UNSAFE: &str = "destination name is not a safe Windows name";

pub const PARENT_PID_NONZERO: &str = "parent PID must be nonzero";
pub const TRANSPORT_NOT_NAMED_PIPE: &str = "transport stream is not a named pipe";
pub const NEGATIVE_FILE_LENGTH: &str = "Windows returned a negative file length";
pub const NEGATIVE_LAST_WRITE_TIME: &str = "Windows returned a negative last-write time";
pub const FILE_ID_INFO_FAILURE: &str = "GetFileInformationByHandleEx(FileIdInfo): {}";
pub const FILE_STANDARD_INFO_FAILURE: &str = "GetFileInformationByHandleEx(FileStandardInfo): {}";
const FILE_ATTRIBUTE_TAG_ERROR: &str = "GetFileInformationByHandleEx(FileAttributeTagInfo): {}";
pub const FILE_ATTRIBUTE_TAG_INFO_FAILURE: &str = FILE_ATTRIBUTE_TAG_ERROR;
pub const FILE_BASIC_INFO_FAILURE: &str = "GetFileInformationByHandleEx(FileBasicInfo): {}";
pub const FILE_RENAME_INFO_FAILURE: &str = "SetFileInformationByHandle(FileRenameInfo): {}";
const FILE_DISPOSITION_ERROR: &str = "SetFileInformationByHandle(FileDispositionInfo): {}";
pub const FILE_DISPOSITION_INFO_FAILURE: &str = FILE_DISPOSITION_ERROR;

pub const IDENTITY_PREFIX: &str = "{}:";
pub const HEX_DIGITS: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
];

const TX: &str = "transaction supports coordinated replace/remove/remove-tree; append is separate";
pub const TRANSACTION_MUTATION_ERROR: &str = TX;
pub const REMOVE_TREE_CHILD_REQUIRED: &str = "remove-tree requires a child directory";
