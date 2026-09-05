#[path = "../../src/activity_store_path/activity_db.rs"]
mod activity_db;
#[path = "../../src/activity_store_path/activity_journal.rs"]
mod activity_journal;

pub type ActivityDbPath = activity_db::ActivityDbPath;
pub type ActivityJournalPath = activity_journal::ActivityJournalPath;
pub type ActivityJournalKeyPath = activity_journal::ActivityJournalKeyPath;

pub fn activity_db_path() -> ActivityDbPath {
    activity_db::activity_db_path()
}

pub fn activity_journal_path() -> ActivityJournalPath {
    activity_journal::activity_journal_path()
}

pub fn activity_journal_key_path() -> ActivityJournalKeyPath {
    activity_journal::activity_journal_key_path()
}
