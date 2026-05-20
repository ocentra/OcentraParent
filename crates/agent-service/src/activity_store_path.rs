use std::{env, path::PathBuf};

use ocentra_parent_agent_protocol::constants;

pub fn activity_db_path() -> PathBuf {
    env::var(constants::env_var::ACTIVITY_DB_PATH)
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let mut path = env::temp_dir();
            path.push(constants::activity_store::DEFAULT_FILE_NAME);
            path
        })
}

pub fn activity_journal_path() -> PathBuf {
    env::var(constants::env_var::ACTIVITY_JOURNAL_PATH)
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let mut path = env::temp_dir();
            path.push(constants::activity_store::DEFAULT_JOURNAL_FILE_NAME);
            path
        })
}

pub fn activity_journal_key_path() -> PathBuf {
    env::var(constants::env_var::ACTIVITY_JOURNAL_KEY_PATH)
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let mut path = env::temp_dir();
            path.push(constants::activity_store::DEFAULT_JOURNAL_KEY_FILE_NAME);
            path
        })
}
