use serde::{Deserialize, Serialize};

use super::{
    identity::{
        AiDigest, AiExplanationId, AiJournalEntryId, AiJournalStreamId, AiRequestId, AiResultId,
        AiTimestamp, AiWorkItemId,
    },
    AiCustodyState, AiDurabilityState, AiRedactionState, AiRetentionState,
};

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiJournalPayloadReference {
    pub payload_kind: AiJournalPayloadKind,
    pub request_id: AiRequestId,
    pub work_item_id: Option<AiWorkItemId>,
    pub result_id: Option<AiResultId>,
    pub explanation_id: Option<AiExplanationId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiJournalEntry {
    pub journal_entry_id: AiJournalEntryId,
    pub stream_id: AiJournalStreamId,
    pub sequence: u64,
    pub kind: AiJournalEntryKind,
    pub payload: AiJournalPayloadReference,
    pub custody: AiCustodyState,
    pub retention: AiRetentionState,
    pub redaction: AiRedactionState,
    pub durability: AiDurabilityState,
    pub occurred_at: AiTimestamp,
    pub digest: AiDigest,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiJournalCursor {
    pub stream_id: AiJournalStreamId,
    pub after_sequence: u64,
    pub after_entry_id: Option<AiJournalEntryId>,
    pub durable: AiDurabilityState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiJournalAppendResult {
    pub entry: AiJournalEntry,
    pub accepted: bool,
    pub next_sequence: u64,
    pub durability: AiDurabilityState,
}
