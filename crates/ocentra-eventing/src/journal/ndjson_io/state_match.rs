use std::time::SystemTime;

use crate::{EventingError, ExpectValue};

use super::NdjsonEventJournal;

impl NdjsonEventJournal {
    pub(super) async fn state_matches_file(&self) -> Result<bool, EventingError> {
        let (file_len, file_modified) = self.journal_file_state().await?;
        let recovered = self.read_recovered_state().await?;
        let state = self.state.lock().expect_value("journal state lock");
        Ok(state.recovered
            && state.file_len == file_len
            && state.file_modified == file_modified
            && state.next_sequence == recovered.next_sequence
            && state.previous_hash == recovered.previous_hash)
    }

    pub(super) async fn journal_file_state(
        &self,
    ) -> Result<(u64, Option<SystemTime>), EventingError> {
        match tokio::fs::metadata(&self.path).await {
            Ok(metadata) => Ok((metadata.len(), metadata.modified().ok())),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok((0, None)),
            Err(error) => Err(EventingError::journal_io(self.path_string(), &error)),
        }
    }
}
