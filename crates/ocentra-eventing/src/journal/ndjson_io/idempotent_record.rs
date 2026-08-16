use crate::{journal::ndjson::NdjsonJournalRecord, EventingError, NdjsonJournalEntry};

pub(super) fn decode_entry(
    line: &str,
    line_number: usize,
) -> Result<Option<NdjsonJournalEntry>, EventingError> {
    NdjsonJournalRecord::parse(line, line_number).map(NdjsonJournalRecord::entry)
}
