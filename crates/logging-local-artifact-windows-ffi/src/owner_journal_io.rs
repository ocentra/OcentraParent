//! Durable metadata, intent, and terminal-receipt I/O.

use std::fmt::Display;
use std::path::Path;

use crate::error::ArtifactError;

use super::*;

#[path = "owner_journal_io_intents.rs"]
mod intent_io;
#[path = "owner_journal_intents.rs"]
mod intents;
#[path = "owner_journal_intents_read.rs"]
mod intents_read;
#[path = "owner_journal_metadata.rs"]
mod metadata;
#[path = "owner_journal_io_names.rs"]
mod names;
#[path = "owner_journal_io_receipts.rs"]
mod receipt_io;
#[path = "owner_journal_receipts.rs"]
mod receipts;

pub(crate) fn ensure_metadata_dirs(root: &Path) -> Result<MetadataDirs, ArtifactError> {
    metadata::ensure_metadata_dirs(root)
}

pub(crate) fn read_receipt<R, O, P, D>(
    root: &Path,
    request_id: R,
    operation: O,
    relative_path: P,
    descriptor: D,
) -> Result<Option<MutationReceipt>, ArtifactError>
where
    R: Display,
    O: Display,
    P: Display,
    D: Display,
{
    receipts::read_receipt(root, request_id, operation, relative_path, descriptor)
}

pub(crate) fn write_receipt<R, O, P, D>(
    root: &Path,
    request_id: R,
    operation: O,
    relative_path: P,
    descriptor: D,
    outcome: ReceiptOutcome,
    receipt_directory: &OwnedFile,
) -> Result<MutationReceipt, ArtifactError>
where
    R: Display + descriptors::names::TempNameInput,
    O: Display,
    P: Display,
    D: Display,
{
    receipts::write_receipt(
        root,
        request_id,
        operation,
        relative_path,
        descriptor,
        outcome,
        receipt_directory,
    )
}

pub(crate) fn write_intent<R>(
    root: &Path,
    request_id: R,
    record: &IntentRecord,
    intent_directory: &OwnedFile,
) -> Result<(), ArtifactError>
where
    R: Display + descriptors::names::TempNameInput,
{
    intent_io::write(root, request_id, record, intent_directory)
}

pub(crate) fn replace_intent<R, S>(
    root: &Path,
    request_id: R,
    record: &IntentRecord,
    intent_directory: &OwnedFile,
    suffix: S,
) -> Result<(), ArtifactError>
where
    R: Display + descriptors::names::TempNameInput,
    S: Display,
{
    intent_io::replace(root, request_id, record, intent_directory, suffix)
}

pub(crate) fn remove_intent<R>(
    root: &Path,
    request_id: R,
    intent_directory: &OwnedFile,
) -> Result<(), ArtifactError>
where
    R: Display,
{
    intent_io::remove(root, request_id, intent_directory)
}

pub(crate) fn read_intents(
    root: &Path,
    intent_directory: &OwnedFile,
) -> Result<Vec<IntentRecord>, ArtifactError> {
    intents_read::read_intents(root, intent_directory)
}

pub(crate) fn reconcile_receipt_temps(
    root: &Path,
    receipt_directory: &OwnedFile,
) -> Result<(), ArtifactError> {
    receipt_io::reconcile(root, receipt_directory)
}

fn generated_receipt_temp_name<N>(name: &N) -> bool
where
    N: descriptors::generated_names::ReceiptTempNameInput + ?Sized,
{
    names::receipt_temp(name)
}
