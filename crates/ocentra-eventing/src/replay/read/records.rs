use crate::{
    journal::{hash_chain::verify_hash_chain_entry, ndjson::NdjsonJournalRecord},
    EventingError, JournalHash, NdjsonEventJournal, NdjsonJournalEntry,
};
use tokio::io::{AsyncBufReadExt, BufReader};

pub(super) async fn collect(
    journal: &NdjsonEventJournal,
) -> Result<
    (
        Vec<NdjsonJournalEntry>,
        Vec<crate::journal::ndjson::NdjsonJournalSynchronizationCompletion>,
        usize,
    ),
    EventingError,
> {
    let file = tokio::fs::File::open(journal.path())
        .await
        .map_err(|error| EventingError::journal_io(journal.path_string(), &error))?;
    let mut lines = BufReader::new(file).lines();
    let mut number = 0;
    let mut entries = Vec::new();
    let mut completions = Vec::new();
    let mut skipped = 0;
    let mut previous: Option<JournalHash> = None;
    while let Some(line) = lines
        .next_line()
        .await
        .map_err(|error| EventingError::journal_io(journal.path_string(), &error))?
    {
        number += 1;
        collect_line(
            &line,
            number,
            &mut previous,
            &mut entries,
            &mut completions,
            &mut skipped,
        )?;
    }
    Ok((entries, completions, skipped))
}

fn collect_line(
    line: &str,
    number: usize,
    previous: &mut Option<JournalHash>,
    entries: &mut Vec<NdjsonJournalEntry>,
    completions: &mut Vec<crate::journal::ndjson::NdjsonJournalSynchronizationCompletion>,
    skipped: &mut usize,
) -> Result<(), EventingError> {
    let record = parse_record(line, number, skipped)?;
    let Some(record) = record else {
        return Ok(());
    };
    match record {
        NdjsonJournalRecord::Entry(entry) => collect_entry(*entry, number, previous, entries)?,
        NdjsonJournalRecord::SynchronizationCompletion(value) => completions.push(value),
    }
    Ok(())
}

fn collect_entry(
    entry: NdjsonJournalEntry,
    number: usize,
    previous: &mut Option<JournalHash>,
    entries: &mut Vec<NdjsonJournalEntry>,
) -> Result<(), EventingError> {
    verify_hash_chain_entry(&entry, previous).map_err(|reason| {
        EventingError::JournalCorruptLine {
            line: number,
            reason,
        }
    })?;
    *previous = entry.append.current_hash.clone();
    entries.push(entry);
    Ok(())
}

fn parse_record(
    line: &str,
    number: usize,
    skipped: &mut usize,
) -> Result<Option<NdjsonJournalRecord>, EventingError> {
    if line.trim().is_empty() {
        *skipped += 1;
        return Ok(None);
    }
    NdjsonJournalRecord::parse(line, number).map(Some)
}
