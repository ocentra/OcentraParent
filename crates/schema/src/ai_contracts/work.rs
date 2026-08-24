use serde::{Deserialize, Serialize};

use super::{
    context::{AiPromptReference, AiRuntimeReference},
    identity::{
        AiActorIdentity, AiJournalEntryId, AiRequestId, AiSchemaIdentity, AiTimestamp, AiWorkItemId,
    },
    AiDegradedState, AiDurabilityState, AiText, AiValidationState,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiWorkKind {
    ContextBuild,
    Classification,
    Explanation,
    MemoryDerivation,
    GraphDerivation,
    ParentAssistant,
    RemoteAssistant,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AiWorkState {
    Queued,
    Claimed,
    Running,
    Succeeded,
    Failed,
    Cancelled,
    TimedOut,
    ManualRequired,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiRetryPolicy {
    pub max_attempts: u16,
    pub retry_after_ms: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiWorkRequest {
    pub identity: AiSchemaIdentity,
    pub work_item_id: AiWorkItemId,
    pub work_kind: AiWorkKind,
    pub requested_at: AiTimestamp,
    pub deadline_at: Option<AiTimestamp>,
    pub retry_policy: AiRetryPolicy,
    pub prompt: Option<AiPromptReference>,
    pub runtime: Option<AiRuntimeReference>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiWorkItem {
    pub request: AiWorkRequest,
    pub state: AiWorkState,
    pub attempt: u16,
    pub durability: AiDurabilityState,
    pub validation: AiValidationState,
    pub degraded_state: AiDegradedState,
    pub last_transition_sequence: u64,
    pub last_transition_at: AiTimestamp,
    pub terminal_reason: Option<AiText>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiWorkLifecycleRecord {
    pub work_item_id: AiWorkItemId,
    pub journal_entry_id: AiJournalEntryId,
    pub sequence: u64,
    pub previous_state: Option<AiWorkState>,
    pub next_state: AiWorkState,
    pub actor: AiActorIdentity,
    pub occurred_at: AiTimestamp,
    pub durability: AiDurabilityState,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiDurableWorkLifecycle {
    pub work_item_id: AiWorkItemId,
    pub request_id: AiRequestId,
    pub records: Vec<AiWorkLifecycleRecord>,
    pub last_sequence: u64,
    pub durability: AiDurabilityState,
}

impl AiDurableWorkLifecycle {
    pub fn has_contiguous_durable_sequence(&self) -> bool {
        let contiguous = self
            .records
            .windows(2)
            .all(|pair| pair[0].sequence.checked_add(1) == Some(pair[1].sequence));
        let last_matches = match self.records.last() {
            Some(last) => last.sequence == self.last_sequence,
            None => self.last_sequence == 0,
        };
        contiguous
            && last_matches
            && matches!(
                self.durability,
                AiDurabilityState::Durable | AiDurabilityState::ReplayOnly
            )
    }
}
