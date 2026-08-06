use crate::journal::JournalAppend;
use crate::{EventingError, JournalDispatchPhase, StoredEventEnvelope};

use super::NdjsonJournalEntry;

pub(super) fn matching_append(
    entry: NdjsonJournalEntry,
    envelope: &StoredEventEnvelope,
    expected_phase: JournalDispatchPhase,
) -> Option<Result<JournalAppend, EventingError>> {
    let event_id_matches = entry.envelope.event_id == envelope.event_id;
    let idempotency_key_matches = entry.envelope.idempotency_key == envelope.idempotency_key;
    match (
        event_id_matches,
        idempotency_key_matches,
        entry.envelope == *envelope && entry.phase == expected_phase,
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

pub(super) fn matching_append_by_event_id(
    entries: Vec<NdjsonJournalEntry>,
    envelope: &StoredEventEnvelope,
    expected_phase: JournalDispatchPhase,
) -> Result<Option<JournalAppend>, EventingError> {
    let Some(entry) = entries
        .into_iter()
        .find(|entry| entry.envelope.event_id == envelope.event_id)
    else {
        return Ok(None);
    };
    if entry.phase != expected_phase
        || entry.envelope.contract != envelope.contract
        || entry.envelope.correlation_id != envelope.correlation_id
    {
        return Err(EventingError::DuplicateEventId {
            event_id: envelope.event_id.clone(),
        });
    }
    Ok(Some(entry.append))
}

pub(super) fn is_legacy_idempotent_candidate(
    entry: &NdjsonJournalEntry,
    envelope: &StoredEventEnvelope,
) -> bool {
    entry.envelope != *envelope || entry.phase == JournalDispatchPhase::AfterDispatch
}
