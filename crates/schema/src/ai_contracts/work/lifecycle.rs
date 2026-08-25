use std::collections::HashSet;

use super::{AiDurableWorkLifecycle, AiWorkLifecycleRecord};
use crate::ai_contracts::identity::{
    AiActorIdentity, AiJournalEntryId, AiRequestId, AiTimestamp, AiWorkItemId,
};
use crate::ai_contracts::{AiDurabilityState, AI_INITIAL_LIFECYCLE_SEQUENCE};

fn follows_previous_transition(
    previous: &AiWorkLifecycleRecord,
    current: &AiWorkLifecycleRecord,
) -> bool {
    current.previous_state() == Some(previous.next_state())
        && previous.occurred_at().precedes(current.occurred_at())
}

impl AiWorkLifecycleRecord {
    pub(crate) fn new(
        work_item_id: AiWorkItemId,
        request_id: AiRequestId,
        journal_entry_id: AiJournalEntryId,
        sequence: u64,
        previous_state: Option<super::AiWorkState>,
        next_state: super::AiWorkState,
        actor: AiActorIdentity,
        occurred_at: AiTimestamp,
        durability: AiDurabilityState,
    ) -> Result<Self, &'static str> {
        if !next_state.can_transition_from(previous_state)
            || (sequence == AI_INITIAL_LIFECYCLE_SEQUENCE && previous_state.is_some())
            || (sequence != AI_INITIAL_LIFECYCLE_SEQUENCE && previous_state.is_none())
            || !matches!(durability, AiDurabilityState::Durable)
            || !occurred_at.is_well_formed()
        {
            return Err("AI lifecycle record is not a legal durable transition");
        }
        Ok(Self {
            work_item_id,
            request_id,
            journal_entry_id,
            sequence,
            previous_state,
            next_state,
            actor,
            occurred_at,
            durability,
        })
    }

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
    pub(crate) fn new(
        work_item_id: AiWorkItemId,
        request_id: AiRequestId,
        records: Vec<AiWorkLifecycleRecord>,
    ) -> Result<Self, &'static str> {
        if records.is_empty() || !records[0].next_state.can_transition_from(None) {
            return Err("AI durable lifecycle must start with the exact initial transition");
        }
        let mut journal_ids = HashSet::with_capacity(records.len());
        for (index, record) in records.iter().enumerate() {
            if record.work_item_id() != &work_item_id
                || record.request_id() != &request_id
                || !matches!(record.durability(), AiDurabilityState::Durable)
                || !journal_ids.insert(record.journal_entry_id().clone())
                || record.sequence() != AI_INITIAL_LIFECYCLE_SEQUENCE + index as u64
            {
                return Err("AI durable lifecycle has mismatched identity, durability, or duplicate journal identity");
            }
            if index > 0 && !follows_previous_transition(&records[index - 1], record) {
                return Err("AI durable lifecycle contains an ambiguous state transition");
            }
        }
        let last_sequence = records
            .last()
            .map(AiWorkLifecycleRecord::sequence)
            .ok_or("AI durable lifecycle is empty")?;
        Ok(Self {
            work_item_id,
            request_id,
            records,
            last_sequence,
            durability: AiDurabilityState::Durable,
        })
    }

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

    pub fn has_contiguous_durable_sequence(&self) -> bool {
        Self::new(
            self.work_item_id.clone(),
            self.request_id.clone(),
            self.records.clone(),
        )
        .is_ok()
            && matches!(self.durability, AiDurabilityState::Durable)
    }
}
