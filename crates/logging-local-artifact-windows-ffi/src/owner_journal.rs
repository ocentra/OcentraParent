//! Bounded, typed terminal receipts and recovery intents.

use std::fmt;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::ArtifactError;
use crate::owner_paths::DirectoryChain;
use crate::owner_types::{Mutation, MutationReceipt, ReceiptOutcome};
use crate::platform::windows::OwnedFile;

const JOURNAL_SCHEMA: u32 = 1;
const MAX_RECEIPT_BYTES: u64 = 64 * 1024;
/// Receipt custody is bounded by admission, never by unsafe eviction. Once
/// full, new mutations fail closed so no request can be replayed after its
/// terminal receipt has fallen outside an ambiguous retention window.
const MAX_RETAINED_RECEIPTS: usize = 4096;
const MAX_INTENT_BYTES: u64 = 4 * 1024 * 1024;
const BRIDGE_DIRECTORY: &str = ".bridge";
const MUTATION_OWNER_DIRECTORY: &str = ".mutation-owner";
const RECEIPTS_DIRECTORY: &str = "receipts";
const INTENTS_DIRECTORY: &str = "intents";
const JSON_SUFFIX: &str = ".json";
const TEMP_SUFFIX: &str = ".tmp";
const STAGE_SEPARATOR: &str = ".stage-";
const RECEIPT_TEMP_SUFFIX: &str = ".receipt.tmp";
const INTENT_OPERATION: &str = "intent";
const APPEND_OPERATION: &str = "append";
const APPEND_CREATED_PHASE: &str = "append-created";
const APPEND_WRITING_PHASE: &str = "append-writing";
const APPEND_WRITTEN_PHASE: &str = "append-written";
const REPLACE_OPERATION: &str = "replace";
const REMOVE_OPERATION: &str = "remove";
const TRANSACTION_OPERATION: &str = "transaction";
const RECEIPT_OPERATION: &str = "receipt";
const REPLACE_STAGED_PHASE: &str = "replace-staged";
const REPLACE_QUARANTINED_PHASE: &str = "replace-quarantined";
const REPLACE_INSTALLED_PHASE: &str = "replace-installed";
const REMOVE_DELETED_PHASE: &str = "remove-deleted";
const TRANSACTION_QUARANTINED_PHASE: &str = "transaction-quarantined";
const TRANSACTION_STAGED_PHASE: &str = "transaction-staged";
const TRANSACTION_INSTALLED_PHASE: &str = "transaction-installed";
const REMOVE_TREE_QUARANTINED_PHASE: &str = "remove-tree-quarantined";
const REMOVE_TREE_RECOVERY_QUARANTINED_PHASE: &str = "remove-tree-recovery-quarantined";
const REQUEST_DESCRIPTOR_DOMAIN: &str = "ocentra.local-artifact-request.v1\0";
const TRANSACTION_DESCRIPTOR_DOMAIN: &str = "ocentra.local-artifact-transaction.v1\0";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct JournalText<'a>(&'a str);

impl fmt::Display for JournalText<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}

#[derive(Debug)]
pub(crate) struct JournalPath(PathBuf);

const HEX_DIGITS: &str = "0123456789abcdef";
const GENERATED_INTENT_TEMP_SUFFIXES: &[&str] = &[
    INTENT_OPERATION,
    APPEND_OPERATION,
    APPEND_CREATED_PHASE,
    APPEND_WRITING_PHASE,
    APPEND_WRITTEN_PHASE,
    REPLACE_OPERATION,
    REPLACE_STAGED_PHASE,
    REPLACE_QUARANTINED_PHASE,
    REPLACE_INSTALLED_PHASE,
    REMOVE_DELETED_PHASE,
    TRANSACTION_QUARANTINED_PHASE,
    TRANSACTION_STAGED_PHASE,
    TRANSACTION_INSTALLED_PHASE,
    REMOVE_TREE_QUARANTINED_PHASE,
    REMOVE_TREE_RECOVERY_QUARANTINED_PHASE,
];

/// BRAND-INVARIANT: exactly one SHA-256 descriptor digest produced by the
/// journal domain; the bytes never represent caller-controlled text.
#[derive(Debug)]
pub(crate) struct DescriptorDigest([u8; 32]);

/// BRAND-INVARIANT: lower-case hexadecimal text assembled by the journal
/// encoder from bytes; callers cannot inject an unvalidated representation.
#[derive(Debug)]
pub(crate) struct HexText(String);

/// BRAND-INVARIANT: a generated journal filename assembled from a validated
/// request identifier and a fixed operation suffix.
#[derive(Debug)]
pub(crate) struct NameText(String);

pub(crate) trait DescriptorOutput {
    fn from_descriptor(value: DescriptorDigest) -> Self;
}

pub(crate) trait HexOutput {
    fn from_hex(value: HexText) -> Self;
}

pub(crate) trait NameOutput {
    fn from_name(value: NameText) -> Self;
}

#[path = "owner_journal/boundary/owner_journal_values.rs"]
mod values;

#[path = "owner_journal_descriptors.rs"]
mod descriptors;
#[path = "owner_journal_intent_access.rs"]
mod intent_access;
#[path = "owner_journal_intent_state.rs"]
mod intent_state;
#[path = "owner_journal_io.rs"]
pub(crate) mod io;
#[path = "owner_journal_mutation_records.rs"]
mod mutation_records;
#[path = "owner_journal_receipt_records.rs"]
mod receipt_records;

#[derive(Debug)]
pub(crate) struct MetadataDirs {
    pub(crate) receipts: DirectoryChain,
    pub(crate) intents: DirectoryChain,
}

impl MetadataDirs {
    pub(crate) fn receipt_directory(&self) -> Result<&OwnedFile, ArtifactError> {
        self.receipts.leaf()
    }

    pub(crate) fn intent_directory(&self) -> Result<&OwnedFile, ArtifactError> {
        self.intents.leaf()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReceiptRecord {
    schema: u32,
    request_id: String,
    operation: String,
    relative_path: String,
    descriptor: String,
    outcome: ReceiptRecordOutcome,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum ReceiptRecordOutcome {
    Appended { offset: u64, length: u64 },
    Replaced,
    Removed { existed: bool },
    TransactionCommitted { count: u32 },
    Unsupported { operation: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum IntentRecord {
    Append {
        schema: u32,
        request_id: String,
        relative_path: String,
        descriptor: String,
        payload_digest: String,
        payload_length: u64,
        prior_length: u64,
        created: bool,
        target_identity: Option<IdentityRecord>,
        temp_name: Option<String>,
        phase: AppendPhase,
    },
    Replace {
        schema: u32,
        request_id: String,
        relative_path: String,
        descriptor: String,
        payload_digest: String,
        temp_name: String,
        quarantine_name: String,
        target_identity: Option<IdentityRecord>,
        staged_identity: Option<IdentityRecord>,
        phase: ReplacePhase,
    },
    Remove {
        schema: u32,
        request_id: String,
        relative_path: String,
        descriptor: String,
        target_identity: Option<IdentityRecord>,
        phase: RemovePhase,
    },
    Transaction {
        schema: u32,
        request_id: String,
        relative_paths: Vec<String>,
        descriptor: String,
        staged: Vec<StagedMutation>,
        phase: TransactionPhase,
    },
    RemoveTree {
        schema: u32,
        request_id: String,
        relative_path: String,
        descriptor: String,
        target_identity: IdentityRecord,
        quarantine_name: Option<String>,
        receipt_operation: String,
        receipt_relative_path: String,
        phase: RemoveTreePhase,
    },
}

impl IntentRecord {
    pub(crate) fn descriptor(&self) -> JournalText<'_> {
        match self {
            Self::Append { descriptor, .. }
            | Self::Replace { descriptor, .. }
            | Self::Remove { descriptor, .. }
            | Self::Transaction { descriptor, .. }
            | Self::RemoveTree { descriptor, .. } => JournalText(descriptor),
        }
    }

    pub(crate) fn operation(&self) -> JournalText<'_> {
        match self {
            Self::Append { .. } => JournalText(APPEND_OPERATION),
            Self::Replace { .. } => JournalText(REPLACE_OPERATION),
            Self::Remove { .. } => JournalText(REMOVE_OPERATION),
            Self::Transaction { .. } => JournalText(TRANSACTION_OPERATION),
            Self::RemoveTree {
                receipt_operation, ..
            } => JournalText(receipt_operation),
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum AppendPhase {
    Prepared,
    Created,
    Writing,
    Written,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ReplacePhase {
    Prepared,
    Quarantined,
    Installed,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RemovePhase {
    Prepared,
    Deleted,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum TransactionPhase {
    Prepared,
    Quarantined,
    Installed,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RemoveTreePhase {
    Prepared,
    Quarantined,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IdentityRecord {
    pub(crate) volume_serial_number: u64,
    pub(crate) file_id: [u8; 16],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct StagedMutation {
    pub(crate) relative_path: String,
    pub(crate) operation: String,
    pub(crate) payload_digest: Option<String>,
    pub(crate) staged_name: Option<String>,
    pub(crate) quarantine_name: Option<String>,
    pub(crate) target_identity: Option<IdentityRecord>,
    pub(crate) installed_identity: Option<IdentityRecord>,
}

pub(crate) fn validate_request_id<R>(request_id: &R) -> Result<(), ArtifactError>
where
    R: descriptors::request::RequestIdInput + ?Sized,
{
    descriptors::request::validate_request_id(request_id)
}

pub(crate) fn request_descriptor<O, P, D>(
    operation: &O,
    relative_path: &P,
    payload: Option<&[u8]>,
) -> D
where
    O: descriptors::digests::DescriptorText
        + descriptors::digests::DescriptorOutputInput<D>
        + ?Sized,
    P: std::fmt::Display + ?Sized,
    D: DescriptorOutput,
{
    D::from_descriptor(descriptors::digests::request_descriptor(
        operation,
        relative_path,
        payload,
    ))
}

pub(crate) fn payload_digest<P, D>(payload: &P) -> D
where
    P: descriptors::digests::HexInput + ?Sized,
    P: descriptors::digests::HexOutputInput<D>,
    D: HexOutput,
{
    D::from_hex(descriptors::digests::payload_digest(payload))
}

pub(crate) fn transaction_descriptor<M, D>(mutations: &M) -> D
where
    M: descriptors::transaction::TransactionDescriptorInput
        + descriptors::transaction::TransactionDescriptorOutputInput<D>
        + ?Sized,
    D: DescriptorOutput,
{
    D::from_descriptor(descriptors::transaction::transaction_descriptor(mutations))
}

pub(crate) fn receipt_path<R>(root: &Path, request_id: &R) -> JournalPath
where
    R: std::fmt::Display + ?Sized,
{
    descriptors::names::receipt_path(root, request_id)
}

pub(crate) fn intent_path<R>(root: &Path, request_id: &R) -> JournalPath
where
    R: std::fmt::Display + ?Sized,
{
    descriptors::names::intent_path(root, request_id)
}

pub(crate) fn intent_temp_name<R, S, D>(request_id: &R, suffix: &S) -> D
where
    R: descriptors::names::TempNameInput + descriptors::names::NameOutputInput<D> + ?Sized,
    S: std::fmt::Display + ?Sized,
    D: NameOutput,
{
    D::from_name(descriptors::names::intent_temp_name(request_id, suffix))
}

pub(crate) fn mutation_records(
    mutations: &[Mutation],
) -> Result<Vec<StagedMutation>, ArtifactError> {
    mutation_records::mutation_records(mutations)
}

fn record_outcome(outcome: &ReceiptOutcome) -> ReceiptRecordOutcome {
    receipt_records::record_outcome(outcome)
}
