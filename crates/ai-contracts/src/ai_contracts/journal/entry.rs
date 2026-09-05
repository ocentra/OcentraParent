use super::AiJournalEntry;
use crate::ai_contracts::identity::{AiDigest, AiJournalEntryId, AiJournalStreamId};
use crate::ai_contracts::AiDurabilityState;

impl AiJournalEntry {
    pub fn journal_entry_id(&self) -> &AiJournalEntryId {
        &self.journal_entry_id
    }

    pub fn stream_id(&self) -> &AiJournalStreamId {
        &self.stream_id
    }

    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn payload(&self) -> &super::AiJournalPayloadReference {
        &self.payload
    }

    pub fn durability(&self) -> AiDurabilityState {
        self.durability
    }

    pub fn digest(&self) -> &AiDigest {
        &self.digest
    }
}
