use std::time::SystemTime;

use crate::JournalHash;

#[derive(Default, Debug)]
pub(crate) struct NdjsonJournalState {
    pub(super) next_sequence: u64,
    pub(super) previous_hash: Option<JournalHash>,
    pub(super) recovered: bool,
    pub(super) file_len: u64,
    pub(super) file_modified: Option<SystemTime>,
}

impl NdjsonJournalState {
    pub(super) fn recovered_empty() -> Self {
        Self {
            next_sequence: 0,
            previous_hash: None,
            recovered: true,
            file_len: 0,
            file_modified: None,
        }
    }
}
