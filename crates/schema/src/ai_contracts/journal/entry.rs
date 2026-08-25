use super::{AiJournalEntry, AiJournalEntryKind, AiJournalPayloadKind};
use crate::ai_contracts::identity::{AiDigest, AiJournalEntryId, AiJournalStreamId, AiTimestamp};
use crate::ai_contracts::{AiCustodyState, AiDurabilityState, AiRedactionState, AiRetentionState};

impl AiJournalEntry {
    pub(crate) fn new(
        journal_entry_id: AiJournalEntryId,
        stream_id: AiJournalStreamId,
        sequence: u64,
        kind: AiJournalEntryKind,
        payload: super::AiJournalPayloadReference,
        custody: AiCustodyState,
        retention: AiRetentionState,
        redaction: AiRedactionState,
        durability: AiDurabilityState,
        occurred_at: AiTimestamp,
        digest: AiDigest,
    ) -> Result<Self, &'static str> {
        if !matches!(durability, AiDurabilityState::Durable)
            || matches!(
                custody,
                AiCustodyState::Deleted | AiCustodyState::Unavailable
            )
            || matches!(
                retention,
                AiRetentionState::Deleted | AiRetentionState::Tombstoned
            )
            || matches!(redaction, AiRedactionState::RejectedPrivatePayload)
            || !occurred_at.is_well_formed()
        {
            return Err("AI journal entry is not durable or has unsafe custody metadata");
        }
        let kind_matches_payload = matches!(
            (kind, payload.payload_kind()),
            (
                AiJournalEntryKind::WorkLifecycle,
                AiJournalPayloadKind::WorkItem
            ) | (
                AiJournalEntryKind::ContextBuilt,
                AiJournalPayloadKind::EvidenceContext
            ) | (
                AiJournalEntryKind::ResultValidated,
                AiJournalPayloadKind::Result
            ) | (
                AiJournalEntryKind::ExplanationPublished,
                AiJournalPayloadKind::Explanation
            ) | (
                AiJournalEntryKind::RemoteAssistant,
                AiJournalPayloadKind::RemoteAssistant
            )
        );
        if !kind_matches_payload {
            return Err("AI journal entry kind does not match payload kind");
        }
        Ok(Self {
            journal_entry_id,
            stream_id,
            sequence,
            kind,
            payload,
            custody,
            retention,
            redaction,
            durability,
            occurred_at,
            digest,
        })
    }

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
}
