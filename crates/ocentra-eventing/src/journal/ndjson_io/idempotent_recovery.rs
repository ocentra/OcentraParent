use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

use crate::EventingError;

use super::{NdjsonEventJournal, NdjsonJournalEntry};

impl NdjsonEventJournal {
    pub(crate) async fn repair_incomplete_trailing_record(&self) -> Result<(), EventingError> {
        let contents = match tokio::fs::read(&self.path).await {
            Ok(contents) => contents,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(error) => return Err(EventingError::journal_io(self.path_string(), &error)),
        };
        if contents.is_empty() || contents.last() == Some(&b'\n') {
            return Ok(());
        }
        let complete_len = contents
            .iter()
            .rposition(|byte| *byte == b'\n')
            .map_or(0, |index| index + 1);
        let line_number = contents[..complete_len]
            .iter()
            .filter(|byte| **byte == b'\n')
            .count()
            + 1;
        if trailing_record_is_complete(&contents[complete_len..], line_number)? {
            self.read_recovered_state().await?;
            return append_missing_newline(self).await;
        }
        let file = OpenOptions::new()
            .write(true)
            .open(&self.path)
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), &error))?;
        file.set_len(complete_len as u64)
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), &error))?;
        file.sync_all()
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), &error))
    }
}

fn trailing_record_is_complete(bytes: &[u8], line_number: usize) -> Result<bool, EventingError> {
    match serde_json::from_slice::<NdjsonJournalEntry>(bytes) {
        Ok(_entry) => Ok(true),
        Err(error) if error.is_eof() => Ok(false),
        Err(error) => Err(EventingError::JournalCorruptLine {
            line: line_number,
            reason: error.to_string(),
        }),
    }
}

async fn append_missing_newline(journal: &NdjsonEventJournal) -> Result<(), EventingError> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(&journal.path)
        .await
        .map_err(|error| EventingError::journal_io(journal.path_string(), &error))?;
    file.write_all(b"\n")
        .await
        .map_err(|error| EventingError::journal_io(journal.path_string(), &error))?;
    file.sync_all()
        .await
        .map_err(|error| EventingError::journal_io(journal.path_string(), &error))
}
