use crate::journal::ndjson::NdjsonJournalRecord;
use crate::journal::JournalAppend;
use crate::EventingError;

use super::NdjsonEventJournal;

pub(super) async fn verified(
    journal: &NdjsonEventJournal,
    append: &JournalAppend,
) -> Result<bool, EventingError> {
    let contents = super::idempotent::read_journal(journal).await?;
    let mut completion = false;
    let mut activation = false;
    for (index, line) in contents.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let record = serde_json::from_str::<NdjsonJournalRecord>(line).map_err(|error| {
            EventingError::JournalCorruptLine {
                line: index + 1,
                reason: error.to_string(),
            }
        })?;
        match record {
            NdjsonJournalRecord::SynchronizationCompletion(marker) => {
                completion |= marker_matches(
                    marker.sequence,
                    marker.entry_hash,
                    marker.synchronization_hash,
                    append,
                );
            }
            NdjsonJournalRecord::SynchronizationActivation(marker) => {
                activation |= marker.activation
                    && marker_matches(
                        marker.sequence,
                        marker.entry_hash,
                        marker.synchronization_hash,
                        append,
                    );
            }
            NdjsonJournalRecord::Entry(_) => {}
        }
    }
    Ok(completion && activation)
}

pub(super) async fn ensure_verified(
    journal: &NdjsonEventJournal,
    append: &JournalAppend,
) -> Result<(), EventingError> {
    if verified(journal, append).await? {
        return Ok(());
    }
    journal.write_synchronization_completion(append).await
}

fn marker_matches(
    sequence: u64,
    entry_hash: Option<crate::JournalHash>,
    synchronization_hash: crate::JournalHash,
    append: &JournalAppend,
) -> bool {
    sequence == append.sequence
        && entry_hash == append.current_hash
        && Some(synchronization_hash) == append.synchronization_hash
}
