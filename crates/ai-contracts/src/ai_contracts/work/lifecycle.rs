use std::collections::HashSet;

use super::{AiDurableWorkLifecycle, AiWorkLifecycleRecord};
use crate::ai_contracts::identity::{AiJournalEntryId, AiRequestId, AiTimestamp, AiWorkItemId};
use crate::ai_contracts::{AiDurabilityState, AI_INITIAL_LIFECYCLE_SEQUENCE};

fn follows_previous_transition(
    previous: &AiWorkLifecycleRecord,
    current: &AiWorkLifecycleRecord,
) -> bool {
    current.previous_state() == Some(previous.next_state())
        && previous.occurred_at().precedes(current.occurred_at())
}

impl AiWorkLifecycleRecord {
    pub fn work_item_id(&self) -> &AiWorkItemId {
        &self.work_item_id
    }

    pub fn request_id(&self) -> &AiRequestId {
        &self.request_id
    }

    pub fn journal_entry_id(&self) -> &AiJournalEntryId {
        &self.journal_entry_id
    }

    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn previous_state(&self) -> Option<super::AiWorkState> {
        self.previous_state
    }

    pub fn next_state(&self) -> super::AiWorkState {
        self.next_state
    }

    pub fn durability(&self) -> AiDurabilityState {
        self.durability
    }

    pub fn occurred_at(&self) -> &AiTimestamp {
        &self.occurred_at
    }
}

impl AiDurableWorkLifecycle {
    pub fn work_item_id(&self) -> &AiWorkItemId {
        &self.work_item_id
    }

    pub fn request_id(&self) -> &AiRequestId {
        &self.request_id
    }

    pub fn records(&self) -> &[AiWorkLifecycleRecord] {
        &self.records
    }

    pub fn last_sequence(&self) -> u64 {
        self.last_sequence
    }

    pub fn max_attempts(&self) -> u16 {
        self.max_attempts
    }

    pub fn has_contiguous_durable_sequence(&self) -> bool {
        if self.max_attempts == 0
            || self.records.is_empty()
            || !self.records[0].next_state.can_transition_from(None)
            || !matches!(self.durability, AiDurabilityState::Durable)
        {
            return false;
        }
        let mut journal_ids = HashSet::with_capacity(self.records.len());
        for (index, record) in self.records.iter().enumerate() {
            if record.work_item_id() != &self.work_item_id
                || record.request_id() != &self.request_id
                || !matches!(record.durability(), AiDurabilityState::Durable)
                || !journal_ids.insert(record.journal_entry_id().clone())
                || record.sequence() != AI_INITIAL_LIFECYCLE_SEQUENCE + index as u64
                || (index > 0 && !follows_previous_transition(&self.records[index - 1], record))
            {
                return false;
            }
        }
        self.records
            .iter()
            .filter(|record| matches!(record.next_state(), super::AiWorkState::Claimed))
            .count()
            <= usize::from(self.max_attempts)
    }
}
