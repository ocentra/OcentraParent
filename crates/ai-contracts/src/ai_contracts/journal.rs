use serde::{Deserialize, Serialize};

use super::identity::{
    AiDigest, AiExplanationId, AiJournalEntryId, AiJournalStreamId, AiRequestId, AiResultId,
    AiTimestamp, AiWorkItemId,
};
use super::{AiCustodyState, AiDurabilityState, AiRedactionState, AiRetentionState};

mod cursor;
mod entry;
mod payload;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiJournalEntryKind {
    WorkLifecycle,
    ContextBuilt,
    ResultValidated,
    ExplanationPublished,
    RemoteAssistant,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiJournalPayloadKind {
    WorkItem,
    EvidenceContext,
    Result,
    Explanation,
    RemoteAssistant,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiJournalPayloadReference {
    payload_kind: AiJournalPayloadKind,
    request_id: AiRequestId,
    work_item_id: Option<AiWorkItemId>,
    result_id: Option<AiResultId>,
    explanation_id: Option<AiExplanationId>,
    content_digest: AiDigest,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiJournalEntry {
    journal_entry_id: AiJournalEntryId,
    stream_id: AiJournalStreamId,
    sequence: u64,
    kind: AiJournalEntryKind,
    payload: AiJournalPayloadReference,
    custody: AiCustodyState,
    retention: AiRetentionState,
    redaction: AiRedactionState,
    durability: AiDurabilityState,
    occurred_at: AiTimestamp,
    digest: AiDigest,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiJournalCursor {
    stream_id: AiJournalStreamId,
    after_sequence: u64,
    after_entry_id: Option<AiJournalEntryId>,
    durable: AiDurabilityState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiJournalAppendResult {
    entry: AiJournalEntry,
    accepted: bool,
    next_sequence: u64,
    durability: AiDurabilityState,
}
