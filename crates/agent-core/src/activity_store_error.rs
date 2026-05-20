use crate::JournalError;

#[derive(Debug)]
pub enum ActivityStoreError {
    Database(duckdb::Error),
    Journal(JournalError),
    Json(serde_json::Error),
}

impl From<duckdb::Error> for ActivityStoreError {
    fn from(error: duckdb::Error) -> Self {
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
