use crate::journal_error::JournalError;

#[derive(Debug)]
pub enum ActivityStoreError {
    Database(rusqlite::Error),
    Journal(JournalError),
    Json(serde_json::Error),
    InvalidNetworkField { field: &'static str },
}

impl From<rusqlite::Error> for ActivityStoreError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Database(error)
    }
}

impl From<JournalError> for ActivityStoreError {
    fn from(error: JournalError) -> Self {
        Self::Journal(error)
    }
}

impl From<serde_json::Error> for ActivityStoreError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}
