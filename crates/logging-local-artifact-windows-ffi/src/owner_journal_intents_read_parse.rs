use std::fmt::Display;
use std::path::Path;

use crate::constants::{
    APPEND_OPERATION, REMOVE_OPERATION, REMOVE_TREE_OPERATION, REPLACE_OPERATION,
    TRANSACTION_OPERATION,
};
use crate::error::ArtifactError;
use crate::owner_paths::{validate_directory_relative, validate_relative};
use crate::owner_types::MAX_TRANSACTION_MUTATIONS;
use crate::platform::windows::MAX_ARTIFACT_BYTES;

use super::super::*;

const SHA256_HEX_CHARS: usize = 64;
const TREE_QUARANTINE_SUFFIX: &str = "tree-quarantine";
const PATH_SEPARATOR: &str = "/";

pub(super) fn parse(
    directory: &Path,
    entries: &super::super::IntentEntryNames,
) -> Result<Vec<IntentRecord>, ArtifactError> {
    let mut records = Vec::new();
    for name in entries.0.iter() {
        if !name.ends_with(JSON_SUFFIX) || name.len() <= JSON_SUFFIX.len() {
            continue;
        }
        let request_id = &name[..name.len() - JSON_SUFFIX.len()];
        validate_request_id(request_id)?;
        let path = directory.join(name);
        let mut file = OwnedFile::open_existing_file(&path)?;
        let bytes = file.read_bounded(MAX_INTENT_BYTES)?;
        let record: IntentRecord =
            serde_json::from_slice(&bytes).map_err(|_| ArtifactError::RecoveryRequired)?;
        record.validate()?;
        if record.request_id().to_string() != request_id {
            return Err(ArtifactError::RecoveryRequired);
        }
        records.push(record);
    }
    Ok(records)
}

impl IntentRecord {
    pub(crate) fn validate(&self) -> Result<(), ArtifactError> {
        match self {
            Self::Append { .. } => validate_append(self),
            Self::Replace { .. } => validate_replace(self),
            Self::Remove { .. } => validate_remove(self),
            Self::Transaction { .. } => validate_transaction(self),
            Self::RemoveTree { .. } => validate_remove_tree(self),
        }
    }
}

fn validate_append(record: &IntentRecord) -> Result<(), ArtifactError> {
    let IntentRecord::Append {
        schema,
        request_id,
        relative_path,
        descriptor,
        payload_digest,
        payload_length,
        prior_length,
        created,
        target_identity,
        temp_name,
        phase,
    } = record
    else {
        return Err(invalid_record());
    };
    let expected_temp = format!("{request_id}.{APPEND_OPERATION}{TEMP_SUFFIX}");
    let state_valid = (*created
        && matches!(phase, AppendPhase::Prepared)
        && temp_name
            .as_deref()
            .is_some_and(|name| name == expected_temp))
        || (temp_name.is_none()
            && (!*created
                || matches!(
                    phase,
                    AppendPhase::Created | AppendPhase::Writing | AppendPhase::Written
                )));
    validate_common(*schema, request_id, descriptor)
        .and(validate_file_path(relative_path))
        .and(validate_digest(payload_digest))
        .and(require(
            target_identity.is_some()
                && *payload_length <= MAX_ARTIFACT_BYTES
                && *prior_length <= MAX_ARTIFACT_BYTES
                && prior_length
                    .checked_add(*payload_length)
                    .is_some_and(|length| length <= MAX_ARTIFACT_BYTES),
        ))
        .and(require(state_valid))
}

fn validate_replace(record: &IntentRecord) -> Result<(), ArtifactError> {
    let IntentRecord::Replace {
        schema,
        request_id,
        relative_path,
        descriptor,
        payload_digest,
        temp_name,
        quarantine_name,
        target_identity,
        staged_identity,
        phase,
    } = record
    else {
        return Err(invalid_record());
    };
    let expected_temp = format!("{request_id}.{REPLACE_OPERATION}{TEMP_SUFFIX}");
    let expected_quarantine = format!("{request_id}.{REPLACE_OPERATION}.quarantine");
    validate_common(*schema, request_id, descriptor)
        .and(validate_file_path(relative_path))
        .and(validate_digest(payload_digest))
        .and(require(
            temp_name == &expected_temp
                && quarantine_name == &expected_quarantine
                && (!matches!(phase, ReplacePhase::Quarantined) || target_identity.is_some())
                && (!matches!(phase, ReplacePhase::Quarantined | ReplacePhase::Installed)
                    || staged_identity.is_some()),
        ))
}

fn validate_remove(record: &IntentRecord) -> Result<(), ArtifactError> {
    let IntentRecord::Remove {
        schema,
        request_id,
        relative_path,
        descriptor,
        target_identity,
        phase,
    } = record
    else {
        return Err(invalid_record());
    };
    validate_common(*schema, request_id, descriptor)
        .and(validate_file_path(relative_path))
        .and(validate_no_payload_descriptor(
            &JournalText(REMOVE_OPERATION),
            relative_path,
            descriptor,
        ))
        .and(require(
            !matches!(phase, RemovePhase::Deleted) || target_identity.is_some(),
        ))
}

fn validate_transaction(record: &IntentRecord) -> Result<(), ArtifactError> {
    let IntentRecord::Transaction {
        schema,
        request_id,
        relative_paths,
        descriptor,
        staged,
        phase,
    } = record
    else {
        return Err(invalid_record());
    };
    let items_valid = staged.iter().enumerate().all(|(index, item)| {
        relative_paths
            .get(index)
            .is_some_and(|path| path == &item.relative_path)
            && relative_paths[..index]
                .iter()
                .all(|previous| !paths_overlap(previous, &item.relative_path))
            && validate_staged_item(request_id, index, item, *phase).is_ok()
    });
    validate_common(*schema, request_id, descriptor).and(require(
        !relative_paths.is_empty()
            && relative_paths.len() == staged.len()
            && staged.len() <= MAX_TRANSACTION_MUTATIONS
            && items_valid,
    ))
}

fn validate_staged_item<R>(
    request_id: &R,
    index: usize,
    item: &StagedMutation,
    phase: TransactionPhase,
) -> Result<(), ArtifactError>
where
    R: Display + ?Sized,
{
    let replace = item.operation == REPLACE_OPERATION;
    let remove = item.operation == REMOVE_OPERATION;
    let remove_tree = item.operation == REMOVE_TREE_OPERATION;
    let progressed = matches!(
        phase,
        TransactionPhase::Quarantined | TransactionPhase::Installed
    );
    let expected_stage = format!("{request_id}{STAGE_SEPARATOR}{index}");
    let expected_quarantine = item
        .target_identity
        .as_ref()
        .map(|_| format!("{request_id}.quarantine-{index}"));
    let path_valid = (remove_tree && validate_tree_path(&item.relative_path).is_ok())
        || (!remove_tree && validate_file_path(&item.relative_path).is_ok());
    let digest_valid = (replace
        && validate_optional_digest(item.payload_digest.as_ref(), true).is_ok())
        || ((!replace) && validate_optional_digest(item.payload_digest.as_ref(), false).is_ok());
    let replace_state = !replace
        || (item
            .staged_name
            .as_ref()
            .map_or(!progressed, |name| name == &expected_stage)
            && (!progressed || item.installed_identity.is_some())
            && (item.installed_identity.is_none() || item.staged_name.is_some()));
    let non_replace_state =
        replace || (item.staged_name.is_none() && item.installed_identity.is_none());
    require(
        (replace || remove || remove_tree)
            && path_valid
            && digest_valid
            && replace_state
            && non_replace_state
            && item.quarantine_name.as_ref() == expected_quarantine.as_ref(),
    )
}

fn validate_remove_tree(record: &IntentRecord) -> Result<(), ArtifactError> {
    let IntentRecord::RemoveTree {
        schema,
        request_id,
        relative_path,
        descriptor,
        quarantine_name,
        receipt_operation,
        receipt_relative_path,
        ..
    } = record
    else {
        return Err(invalid_record());
    };
    let expected_quarantine = format!("{request_id}.{TREE_QUARANTINE_SUFFIX}");
    let receipt_operation_text = receipt_operation.to_string();
    let receipt_path_text = receipt_relative_path.to_string();
    validate_common(*schema, request_id, descriptor)
        .and(validate_tree_path(relative_path))
        .and(require(
            quarantine_name
                .as_deref()
                .is_some_and(|name| name == expected_quarantine)
                && ((receipt_operation_text == REMOVE_TREE_OPERATION
                    && receipt_relative_path == relative_path
                    && validate_no_payload_descriptor(
                        &JournalText(REMOVE_TREE_OPERATION),
                        relative_path,
                        descriptor,
                    )
                    .is_ok())
                    || (receipt_operation_text == TRANSACTION_OPERATION
                        && receipt_path_text == TRANSACTION_OPERATION)),
        ))
}

fn validate_common<R, D>(schema: u32, request_id: &R, descriptor: &D) -> Result<(), ArtifactError>
where
    R: Display + ?Sized,
    D: Display + ?Sized,
{
    require(
        schema == JOURNAL_SCHEMA
            && validate_request_id(&request_id.to_string()).is_ok()
            && validate_digest(descriptor).is_ok(),
    )
}

fn validate_file_path<T>(value: &T) -> Result<(), ArtifactError>
where
    T: Display + ?Sized,
{
    require(validate_relative(&value.to_string()).is_ok())
}

fn validate_no_payload_descriptor<P, D>(
    operation: &JournalText<'_>,
    relative_path: &P,
    descriptor: &D,
) -> Result<(), ArtifactError>
where
    P: Display + ?Sized,
    D: Display + ?Sized,
{
    let expected: String = request_descriptor(operation, relative_path, None);
    require(descriptor.to_string() == expected)
}

fn validate_tree_path<T>(value: &T) -> Result<(), ArtifactError>
where
    T: Display + ?Sized,
{
    let text = value.to_string();
    require(!text.is_empty() && validate_directory_relative(&text).is_ok())
}

fn validate_optional_digest<T>(value: Option<&T>, required: bool) -> Result<(), ArtifactError>
where
    T: Display + ?Sized,
{
    require(
        required == value.is_some() && value.is_none_or(|digest| validate_digest(digest).is_ok()),
    )
}

fn validate_digest<T>(value: &T) -> Result<(), ArtifactError>
where
    T: Display + ?Sized,
{
    let text = value.to_string();
    require(
        text.len() == SHA256_HEX_CHARS
            && text
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
    )
}

fn paths_overlap<L, R>(left: &L, right: &R) -> bool
where
    L: Display + ?Sized,
    R: Display + ?Sized,
{
    let left = left
        .to_string()
        .replace('\\', PATH_SEPARATOR)
        .to_lowercase();
    let right = right
        .to_string()
        .replace('\\', PATH_SEPARATOR)
        .to_lowercase();
    let left_prefix = [left.clone(), PATH_SEPARATOR.to_owned()].concat();
    let right_prefix = [right.clone(), PATH_SEPARATOR.to_owned()].concat();
    left == right || left.starts_with(&right_prefix) || right.starts_with(&left_prefix)
}

fn require(condition: bool) -> Result<(), ArtifactError> {
    condition.then_some(()).ok_or_else(invalid_record)
}

fn invalid_record() -> ArtifactError {
    ArtifactError::RecoveryRequired
}
