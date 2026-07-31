use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::journal::hash_chain::verify_hash_chain_entry;
use crate::{EventingError, ExpectValue};

use super::{NdjsonEventJournal, NdjsonJournalEntry};

impl NdjsonEventJournal {
    /// Keeps the authenticated tail in memory while this process still owns an
    /// unchanged journal. A changed file is recovered under the append lock,
    /// so a restarted or competing writer cannot advance the hash chain from a
    /// stale sequence.
    pub(crate) async fn prepare_append_state(&self) -> Result<(), EventingError> {
        if self.state_matches_file().await? {
            return Ok(());
        }
        self.repair_incomplete_trailing_record().await?;
        self.refresh_state().await
    }

    pub(crate) async fn refresh_state(&self) -> Result<(), EventingError> {
        let mut recovered = self.read_recovered_state().await?;
        let (file_len, file_modified) = self.journal_file_state().await?;
        recovered.file_len = file_len;
        recovered.file_modified = file_modified;
        *self.state.lock().expect_value("journal state lock") = recovered;
        #[cfg(debug_assertions)]
        self.recovery_count_for_debug
            .fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        Ok(())
    }

    #[cfg(debug_assertions)]
    pub(crate) fn recovery_count_for_debug(&self) -> u64 {
        self.recovery_count_for_debug
            .load(std::sync::atomic::Ordering::Acquire)
    }

    pub(crate) async fn read_recovered_state(
        &self,
    ) -> Result<super::super::ndjson_state::NdjsonJournalState, EventingError> {
        let file = match File::open(&self.path).await {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(super::super::ndjson_state::NdjsonJournalState::recovered_empty());
            }
            Err(error) => return Err(EventingError::journal_io(self.path_string(), &error)),
        };
        let mut lines = BufReader::new(file).lines();
        let mut line_number = 0_usize;
        let mut state = super::super::ndjson_state::NdjsonJournalState::recovered_empty();
        while let Some(line) = lines
            .next_line()
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), &error))?
        {
            line_number += 1;
            if let Some(entry) = read_recovered_entry(&line, line_number, &state.previous_hash)? {
                state.next_sequence = entry.append.sequence;
                state.previous_hash = entry.append.current_hash;
            }
        }
        Ok(state)
    }
}

fn read_recovered_entry(
    line: &str,
    line_number: usize,
    expected_previous_hash: &Option<crate::JournalHash>,
) -> Result<Option<NdjsonJournalEntry>, EventingError> {
    if line.trim().is_empty() {
        return Ok(None);
    }
    let entry: NdjsonJournalEntry =
        serde_json::from_str(line).map_err(|error| EventingError::JournalCorruptLine {
            line: line_number,
            reason: error.to_string(),
        })?;
    verify_hash_chain_entry(&entry, expected_previous_hash).map_err(|reason| {
        EventingError::JournalCorruptLine {
            line: line_number,
            reason,
        }
    })?;
    Ok(Some(entry))
}
