#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActivityJournalPath(pub std::path::PathBuf);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActivityJournalKeyPath(pub std::path::PathBuf);

impl AsRef<std::path::Path> for ActivityJournalPath {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_path()
    }
}

impl AsRef<std::path::Path> for ActivityJournalKeyPath {
    fn as_ref(&self) -> &std::path::Path {
        self.0.as_path()
    }
}

impl From<ActivityJournalPath> for std::path::PathBuf {
    fn from(value: ActivityJournalPath) -> Self {
        value.0
    }
}

impl From<ActivityJournalKeyPath> for std::path::PathBuf {
    fn from(value: ActivityJournalKeyPath) -> Self {
        value.0
    }
}

pub fn activity_journal_path() -> ActivityJournalPath {
    ActivityJournalPath(
        std::env::var(ocentra_parent_agent_protocol::constants::env_var::ACTIVITY_JOURNAL_PATH)
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| {
                let mut path = std::env::temp_dir();
                path.push(
                ocentra_parent_agent_protocol::constants::activity_store::DEFAULT_JOURNAL_FILE_NAME,
            );
                path
            }),
    )
}

pub fn activity_journal_key_path() -> ActivityJournalKeyPath {
    ActivityJournalKeyPath(
        std::env::var(
            ocentra_parent_agent_protocol::constants::env_var::ACTIVITY_JOURNAL_KEY_PATH,
        )
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            let mut path = std::env::temp_dir();
            path.push(
                ocentra_parent_agent_protocol::constants::activity_store::DEFAULT_JOURNAL_KEY_FILE_NAME,
            );
            path
        }),
    )
}
