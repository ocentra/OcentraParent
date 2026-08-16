use std::{
    env,
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::constants;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActivityDbPath(pub PathBuf);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActivityJournalPath(pub PathBuf);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActivityJournalKeyPath(pub PathBuf);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkRuntimeJournalPath(pub PathBuf);

impl AsRef<Path> for ActivityDbPath {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

impl AsRef<Path> for ActivityJournalPath {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

impl AsRef<Path> for ActivityJournalKeyPath {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

impl AsRef<Path> for NetworkRuntimeJournalPath {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

impl From<ActivityDbPath> for PathBuf {
    fn from(value: ActivityDbPath) -> Self {
        value.0
    }
}

impl From<ActivityJournalPath> for PathBuf {
    fn from(value: ActivityJournalPath) -> Self {
        value.0
    }
}

impl From<ActivityJournalKeyPath> for PathBuf {
    fn from(value: ActivityJournalKeyPath) -> Self {
        value.0
    }
}

pub fn activity_db_path() -> ActivityDbPath {
    ActivityDbPath(
        env::var(constants::env_var::ACTIVITY_DB_PATH)
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let mut path = env::temp_dir();
                path.push(constants::activity_store::DEFAULT_FILE_NAME);
                path
            }),
    )
}

pub fn activity_journal_path() -> ActivityJournalPath {
    ActivityJournalPath(
        env::var(constants::env_var::ACTIVITY_JOURNAL_PATH)
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let mut path = env::temp_dir();
                path.push(constants::activity_store::DEFAULT_JOURNAL_FILE_NAME);
                path
            }),
    )
}

pub fn activity_journal_key_path() -> ActivityJournalKeyPath {
    ActivityJournalKeyPath(
        env::var(constants::env_var::ACTIVITY_JOURNAL_KEY_PATH)
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let mut path = env::temp_dir();
                path.push(constants::activity_store::DEFAULT_JOURNAL_KEY_FILE_NAME);
                path
            }),
    )
}

pub fn network_runtime_journal_path() -> NetworkRuntimeJournalPath {
    NetworkRuntimeJournalPath(
        env::var(constants::env_var::NETWORK_RUNTIME_JOURNAL_PATH)
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let mut path = env::temp_dir();
                path.push(constants::activity_store::DEFAULT_NETWORK_RUNTIME_JOURNAL_FILE_NAME);
                path
            }),
    )
}
