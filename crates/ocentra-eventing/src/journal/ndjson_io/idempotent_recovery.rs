use tokio::fs::OpenOptions;

use crate::EventingError;

use super::NdjsonEventJournal;

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
