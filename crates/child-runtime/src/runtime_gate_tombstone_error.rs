use ocentra_eventing::error::EventingError;

pub(super) fn is_retryable_journal_error(error: &EventingError) -> bool {
    // Journal I/O can be a transient flush/lock/device failure. Identity
    // collisions and decoded/corrupt records are durable contradictions and
    // must reach the caller instead of being retried as if the append failed
    // transiently.
    matches!(error, EventingError::JournalIo { .. })
}
