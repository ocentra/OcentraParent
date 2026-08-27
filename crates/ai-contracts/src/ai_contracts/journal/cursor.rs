use super::{AiJournalAppendResult, AiJournalCursor};
impl AiJournalCursor {
    pub fn stream_id(&self) -> &super::AiJournalStreamId {
        &self.stream_id
    }

    pub fn after_sequence(&self) -> u64 {
        self.after_sequence
    }
}

impl AiJournalAppendResult {
    pub fn entry(&self) -> &super::AiJournalEntry {
        &self.entry
    }

    pub fn accepted(&self) -> bool {
        self.accepted
    }

    pub fn next_sequence(&self) -> u64 {
        self.next_sequence
    }
}
