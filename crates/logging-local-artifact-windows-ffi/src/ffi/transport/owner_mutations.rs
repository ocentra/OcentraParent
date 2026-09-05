//! Handle-bound mutation and recovery algorithms.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::error::ArtifactError;
use crate::owner::MutationSession;
use crate::owner_journal::io::{
    read_intents, read_receipt, reconcile_receipt_temps, remove_intent, replace_intent,
    write_intent, write_receipt,
};
use crate::owner_journal::{
    intent_temp_name, mutation_records, payload_digest, request_descriptor, transaction_descriptor,
    validate_request_id, AppendPhase, IdentityRecord, IntentRecord, RemovePhase, RemoveTreePhase,
    ReplacePhase, StagedMutation, TransactionPhase,
};
use crate::owner_paths::{
    parent_and_leaf, validate_directory_relative, validate_relative, DirectoryChain,
};
use crate::owner_types::{Mutation, MutationReceipt, ReceiptOutcome};
use crate::platform::windows::{verify_metadata, Identity, OwnedFile, MAX_ARTIFACT_BYTES};

#[path = "owner_mutations_identity.rs"]
mod identity;
#[path = "owner_mutations_optional.rs"]
mod optional;
#[path = "owner_mutations_siblings.rs"]
mod siblings;
#[path = "owner_mutations_tree_cleanup.rs"]
mod tree_cleanup;

#[path = "owner_mutations_append.rs"]
mod append;
#[path = "owner_mutations_recovery.rs"]
mod recovery;
#[path = "owner_mutations_remove.rs"]
mod remove;
#[path = "owner_mutations_replace.rs"]
mod replace;
#[path = "owner_mutations_transaction.rs"]
mod transaction;
#[path = "owner_mutations_tree.rs"]
mod tree;

fn bounded_payload(payload: &[u8]) -> Result<(), ArtifactError> {
    let length = u64::try_from(payload.len()).map_err(|_| ArtifactError::SizeLimit)?;
    if length > MAX_ARTIFACT_BYTES {
        return Err(ArtifactError::SizeLimit);
    }
    Ok(())
}

fn identity_record(identity: Identity) -> IdentityRecord {
    IdentityRecord {
        volume_serial_number: identity.volume_serial_number,
        file_id: identity.file_id,
    }
}

fn verify_expected_identity(
    file: &OwnedFile,
    expected: Option<&IdentityRecord>,
) -> Result<(), ArtifactError> {
    identity::expected_file(file, expected)
}

fn verify_expected_directory_identity(
    file: &OwnedFile,
    expected: &IdentityRecord,
) -> Result<(), ArtifactError> {
    identity::expected_directory(file, expected)
}

fn verify_expected_new_file(
    file: &mut OwnedFile,
    expected_identity: &IdentityRecord,
    expected_digest: &str,
) -> Result<(), ArtifactError> {
    identity::expected_new_file(file, expected_identity, expected_digest)
}

fn optional_directory(path: &Path) -> Result<Option<OwnedFile>, ArtifactError> {
    optional::directory(path)
}

fn remove_tree_contents(path: &Path, directory: &OwnedFile) -> Result<(), ArtifactError> {
    tree_cleanup::remove_tree_contents(path, directory)
}

fn optional_mutation_file(path: &std::path::Path) -> Result<Option<OwnedFile>, ArtifactError> {
    optional::file(path)
}

fn reject_existing_sibling(
    root: &std::path::Path,
    chain: &DirectoryChain,
    name: &str,
) -> Result<(), ArtifactError> {
    siblings::reject_existing_sibling(root, chain, name)
}
