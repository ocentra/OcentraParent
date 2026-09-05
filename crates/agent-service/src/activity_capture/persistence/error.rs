use ocentra_parent_agent_core::{
    activity_store_error::ActivityStoreError, journal_error::JournalError,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ActivityCapturePersistenceError {
    Store,
    Journal,
    Io,
    InvalidKeyLength,
}

impl From<ActivityStoreError> for ActivityCapturePersistenceError {
    fn from(_: ActivityStoreError) -> Self {
        Self::Store
    }
}

impl From<JournalError> for ActivityCapturePersistenceError {
    fn from(_: JournalError) -> Self {
        Self::Journal
    }
}

impl From<std::io::Error> for ActivityCapturePersistenceError {
    fn from(_: std::io::Error) -> Self {
        Self::Io
    }
}
