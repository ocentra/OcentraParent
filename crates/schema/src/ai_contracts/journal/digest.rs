use super::{AiJournalEntryKind, AiJournalPayloadReference};
use crate::ai_contracts::identity::{AiDigest, AiJournalEntryId, AiJournalStreamId, AiTimestamp};
use crate::ai_contracts::{AiCustodyState, AiDurabilityState, AiRedactionState, AiRetentionState};

const AI_JOURNAL_ENTRY_DIGEST_DOMAIN: &[u8] = b"ocentra.ai.journal-entry.v1";

pub(super) fn digest_for(
    journal_entry_id: &AiJournalEntryId,
    stream_id: &AiJournalStreamId,
    sequence: u64,
    kind: AiJournalEntryKind,
    payload: &AiJournalPayloadReference,
    custody: AiCustodyState,
    retention: AiRetentionState,
    redaction: AiRedactionState,
    durability: AiDurabilityState,
    occurred_at: &AiTimestamp,
) -> Result<AiDigest, &'static str> {
    let binding = (
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
    );
    let content = serde_json::to_vec(&binding)
        .map_err(|_| "AI journal entry content cannot be canonically encoded")?;
    Ok(AiDigest::from_canonical_binding(
        AI_JOURNAL_ENTRY_DIGEST_DOMAIN,
        &[content.as_slice()],
    ))
}
