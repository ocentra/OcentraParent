use super::{AiJournalAppendResult, AiJournalCursor};
use crate::ai_contracts::AiDurabilityState;

impl AiJournalCursor {
    pub(crate) fn new(
        stream_id: super::AiJournalStreamId,
        after_sequence: u64,
        after_entry_id: Option<super::AiJournalEntryId>,
        durable: AiDurabilityState,
    ) -> Result<Self, &'static str> {
        if !durable.is_durable()
            || (after_sequence == 0 && after_entry_id.is_some())
            || (after_sequence > 0 && after_entry_id.is_none())
        {
            return Err("AI journal cursor is not a durable, self-consistent position");
        }
        Ok(Self {
            stream_id,
            after_sequence,
            after_entry_id,
            durable,
        })
    }

    pub fn stream_id(&self) -> &super::AiJournalStreamId {
        &self.stream_id
    }

    pub fn after_sequence(&self) -> u64 {
        self.after_sequence
    }
}

impl AiJournalAppendResult {
    pub(crate) fn new(
        entry: super::AiJournalEntry,
        accepted: bool,
        next_sequence: u64,
        durability: AiDurabilityState,
    ) -> Result<Self, &'static str> {
        if !matches!(durability, AiDurabilityState::Durable)
            || next_sequence
                != if accepted {
                    entry.sequence().checked_add(1)
                } else {
                    Some(entry.sequence())
                }
                .unwrap_or(u64::MAX)
        {
            return Err("AI journal append result does not describe a durable sequence");
        }
        Ok(Self {
            entry,
            accepted,
            next_sequence,
            durability,
        })
    }

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
