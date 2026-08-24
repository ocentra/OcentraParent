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

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiJournalPayloadReference {
    payload_kind: AiJournalPayloadKind,
    request_id: AiRequestId,
    work_item_id: Option<AiWorkItemId>,
    result_id: Option<AiResultId>,
    explanation_id: Option<AiExplanationId>,
}

impl AiJournalPayloadReference {
    pub fn new(
        payload_kind: AiJournalPayloadKind,
        request_id: AiRequestId,
        work_item_id: Option<AiWorkItemId>,
        result_id: Option<AiResultId>,
        explanation_id: Option<AiExplanationId>,
    ) -> Result<Self, &'static str> {
        let valid = match payload_kind {
            AiJournalPayloadKind::WorkItem => {
                work_item_id.is_some() && result_id.is_none() && explanation_id.is_none()
            }
            AiJournalPayloadKind::EvidenceContext => {
                work_item_id.is_none() && result_id.is_none() && explanation_id.is_none()
            }
            AiJournalPayloadKind::Result => {
                work_item_id.is_some() && result_id.is_some() && explanation_id.is_none()
            }
            AiJournalPayloadKind::Explanation => {
                work_item_id.is_none() && result_id.is_some() && explanation_id.is_some()
            }
            AiJournalPayloadKind::RemoteAssistant => {
                work_item_id.is_none() && result_id.is_none() && explanation_id.is_none()
            }
        };
        valid
            .then_some(Self {
                payload_kind,
                request_id,
                work_item_id,
                result_id,
                explanation_id,
            })
            .ok_or("AI journal payload identities do not match the payload kind")
    }

    pub fn payload_kind(&self) -> AiJournalPayloadKind {
        self.payload_kind
    }

    pub fn request_id(&self) -> &AiRequestId {
        &self.request_id
    }

    pub fn work_item_id(&self) -> Option<&AiWorkItemId> {
        self.work_item_id.as_ref()
    }

    pub fn result_id(&self) -> Option<&AiResultId> {
        self.result_id.as_ref()
    }

    pub fn explanation_id(&self) -> Option<&AiExplanationId> {
        self.explanation_id.as_ref()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiJournalPayloadReferenceFields {
    payload_kind: AiJournalPayloadKind,
    request_id: AiRequestId,
    work_item_id: Option<AiWorkItemId>,
    result_id: Option<AiResultId>,
    explanation_id: Option<AiExplanationId>,
}

impl<'de> Deserialize<'de> for AiJournalPayloadReference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fields = AiJournalPayloadReferenceFields::deserialize(deserializer)?;
        Self::new(
            fields.payload_kind,
            fields.request_id,
            fields.work_item_id,
            fields.result_id,
            fields.explanation_id,
        )
        .map_err(serde::de::Error::custom)
    }
}

/// Owner-issued durable journal entry. Wire data cannot mint durable custody
/// or redaction state by deserializing this type.
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

impl AiJournalEntry {
    pub(crate) fn new(
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

    pub fn payload(&self) -> &AiJournalPayloadReference {
        &self.payload
    }

    pub fn durability(&self) -> AiDurabilityState {
        self.durability
    }
}

/// Owner-issued durable replay cursor.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiJournalCursor {
    stream_id: AiJournalStreamId,
    after_sequence: u64,
    after_entry_id: Option<AiJournalEntryId>,
    durable: AiDurabilityState,
}

impl AiJournalCursor {
    pub(crate) fn new(
        stream_id: AiJournalStreamId,
        after_sequence: u64,
        after_entry_id: Option<AiJournalEntryId>,
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

    pub fn stream_id(&self) -> &AiJournalStreamId {
        &self.stream_id
    }

    pub fn after_sequence(&self) -> u64 {
        self.after_sequence
    }
}

/// Owner-issued durable append result.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiJournalAppendResult {
    entry: AiJournalEntry,
    accepted: bool,
    next_sequence: u64,
    durability: AiDurabilityState,
}

impl AiJournalAppendResult {
    pub(crate) fn new(
        entry: AiJournalEntry,
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

    pub fn entry(&self) -> &AiJournalEntry {
        &self.entry
    }

    pub fn accepted(&self) -> bool {
        self.accepted
    }

    pub fn next_sequence(&self) -> u64 {
        self.next_sequence
    }
}
