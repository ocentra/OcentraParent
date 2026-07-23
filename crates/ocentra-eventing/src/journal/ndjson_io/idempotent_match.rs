use crate::journal::JournalAppend;
use crate::{EventingError, StoredEventEnvelope};

use super::NdjsonJournalEntry;

pub(super) fn matching_append(
    entry: NdjsonJournalEntry,
    envelope: &StoredEventEnvelope,
) -> Option<Result<JournalAppend, EventingError>> {
    let event_id_matches = entry.envelope.event_id == envelope.event_id;
    let idempotency_key_matches = entry.envelope.idempotency_key == envelope.idempotency_key;
    match (
        event_id_matches,
        idempotency_key_matches,
        entry.envelope == *envelope,
    ) {
        (false, false, _) => None,
        (_, _, true) => Some(Ok(entry.append)),
        (true, _, false) => Some(Err(EventingError::DuplicateEventId {
            event_id: envelope.event_id.clone(),
        })),
        (false, true, false) => Some(Err(EventingError::DuplicateIdempotencyKey {
            idempotency_key: envelope.idempotency_key.clone(),
        })),
    }
}
