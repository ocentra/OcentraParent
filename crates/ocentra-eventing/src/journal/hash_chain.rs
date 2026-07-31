use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::{EventingError, JournalDispatchPhase, JournalHash, StoredEventEnvelope};

use super::{
    ndjson::{NdjsonJournalEntry, NdjsonJournalSynchronizationCompletion},
    JournalAppend, JournalAppendDurability, JournalHashVersion,
};

const JOURNAL_HASH_PREFIX: &str = "journal-hash:";
const JOURNAL_HASH_VERSION: u8 = 2;
const JOURNAL_HASH_VERSION_V3: u8 = 3;
const JOURNAL_SYNCHRONIZATION_RECEIPT_VERSION: u8 = 1;

#[derive(Serialize)]
struct JournalHashInputV2<'a> {
    version: u8,
    sequence: u64,
    previous_hash: Option<&'a JournalHash>,
    phase: JournalDispatchPhase,
    durability: JournalAppendDurability,
    envelope: &'a StoredEventEnvelope,
}

pub(crate) fn verify_synchronization_completion(
    entry: &NdjsonJournalEntry,
    completion: &NdjsonJournalSynchronizationCompletion,
) -> bool {
    let mut acknowledged = entry.append.clone();
    acknowledged.durability = JournalAppendDurability::Synchronized;
    acknowledged.synchronization_hash = None;
    entry.append.hash_version == JournalHashVersion::V3
        && entry.append.current_hash.as_ref() == Some(&completion.entry_hash)
        && synchronization_receipt_hash(&acknowledged)
            .is_ok_and(|expected| expected == completion.synchronization_hash)
}

#[derive(Serialize)]
struct JournalHashInputV3<'a> {
    version: u8,
    sequence: u64,
    previous_hash: Option<&'a JournalHash>,
    phase: JournalDispatchPhase,
    requested_durability: JournalAppendDurability,
    achieved_durability: JournalAppendDurability,
    envelope: &'a StoredEventEnvelope,
}

#[derive(Serialize)]
struct JournalHashInputV1<'a> {
    sequence: u64,
    previous_hash: Option<&'a JournalHash>,
    phase: JournalDispatchPhase,
    envelope: &'a StoredEventEnvelope,
}

#[derive(Serialize)]
struct JournalSynchronizationReceiptInput<'a> {
    version: u8,
    sequence: u64,
    entry_hash: Option<&'a JournalHash>,
    requested_durability: JournalAppendDurability,
    achieved_durability: JournalAppendDurability,
}

pub(super) fn synchronization_receipt_hash(
    append: &JournalAppend,
) -> Result<JournalHash, EventingError> {
    let input = JournalSynchronizationReceiptInput {
        version: JOURNAL_SYNCHRONIZATION_RECEIPT_VERSION,
        sequence: append.sequence,
        entry_hash: append.current_hash.as_ref(),
        requested_durability: append.requested_durability,
        achieved_durability: JournalAppendDurability::Synchronized,
    };
    let bytes =
        serde_json::to_vec(&input).map_err(|error| EventingError::journal_encode(&error))?;
    let digest = Sha256::digest(&bytes);
    JournalHash::parse(format!("{JOURNAL_HASH_PREFIX}{:x}", digest))
}

pub(super) fn verify_synchronization_receipt(
    append: &JournalAppend,
    receipt: &JournalHash,
) -> bool {
    synchronization_receipt_hash(append).is_ok_and(|expected| expected == *receipt)
}

pub(super) fn hash_entry_v3(
    sequence: u64,
    previous_hash: Option<&JournalHash>,
    envelope: &StoredEventEnvelope,
    phase: JournalDispatchPhase,
    requested_durability: JournalAppendDurability,
    achieved_durability: JournalAppendDurability,
) -> Result<JournalHash, EventingError> {
    let input = JournalHashInputV3 {
        version: JOURNAL_HASH_VERSION_V3,
        sequence,
        previous_hash,
        phase,
        requested_durability,
        achieved_durability,
        envelope,
    };
    let bytes =
        serde_json::to_vec(&input).map_err(|error| EventingError::journal_encode(&error))?;
    let digest = Sha256::digest(&bytes);
    JournalHash::parse(format!("{JOURNAL_HASH_PREFIX}{:x}", digest))
}

fn hash_v2_entry(
    sequence: u64,
    previous_hash: Option<&JournalHash>,
    envelope: &StoredEventEnvelope,
    phase: JournalDispatchPhase,
    durability: JournalAppendDurability,
) -> Result<JournalHash, EventingError> {
    let input = JournalHashInputV2 {
        version: JOURNAL_HASH_VERSION,
        sequence,
        previous_hash,
        phase,
        durability,
        envelope,
    };
    let bytes =
        serde_json::to_vec(&input).map_err(|error| EventingError::journal_encode(&error))?;
    let digest = Sha256::digest(&bytes);
    JournalHash::parse(format!("{JOURNAL_HASH_PREFIX}{:x}", digest))
}

fn hash_legacy_entry(
    sequence: u64,
    previous_hash: Option<&JournalHash>,
    envelope: &StoredEventEnvelope,
    phase: JournalDispatchPhase,
) -> Result<JournalHash, EventingError> {
    let input = JournalHashInputV1 {
        sequence,
        previous_hash,
        phase,
        envelope,
    };
    let bytes =
        serde_json::to_vec(&input).map_err(|error| EventingError::journal_encode(&error))?;
    let digest = Sha256::digest(&bytes);
    JournalHash::parse(format!("{JOURNAL_HASH_PREFIX}{:x}", digest))
}

pub(crate) fn verify_hash_chain_entry(
    entry: &NdjsonJournalEntry,
    expected_previous_hash: &Option<JournalHash>,
) -> Result<(), String> {
    if entry.append.current_hash.is_none() && entry.append.previous_hash.is_none() {
        return if expected_previous_hash.is_none() {
            Ok(())
        } else {
            Err(format!(
                "journal hash-chain missing current hash at sequence {}",
                entry.append.sequence
            ))
        };
    }
    if &entry.append.previous_hash != expected_previous_hash {
        return Err(format!(
            "journal hash-chain previous hash mismatch at sequence {}",
            entry.append.sequence
        ));
    }
    let expected_current = match entry.append.hash_version {
        JournalHashVersion::LegacyV1 => hash_legacy_entry(
            entry.append.sequence,
            entry.append.previous_hash.as_ref(),
            &entry.envelope,
            entry.phase,
        ),
        JournalHashVersion::V2 => hash_v2_entry(
            entry.append.sequence,
            entry.append.previous_hash.as_ref(),
            &entry.envelope,
            entry.phase,
            entry.append.durability,
        ),
        JournalHashVersion::V3 => hash_entry_v3(
            entry.append.sequence,
            entry.append.previous_hash.as_ref(),
            &entry.envelope,
            entry.phase,
            entry.append.requested_durability,
            entry.append.durability,
        ),
    }
    .map_err(|error| error.to_string())?;
    match &entry.append.current_hash {
        Some(current_hash) if current_hash == &expected_current => Ok(()),
        Some(current_hash) => Err(format!(
            "journal hash-chain current hash mismatch at sequence {}: expected {}, received {}",
            entry.append.sequence,
            expected_current.as_str(),
            current_hash.as_str()
        )),
        None => Err(format!(
            "journal hash-chain missing current hash at sequence {}",
            entry.append.sequence
        )),
    }
}
