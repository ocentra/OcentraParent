use serde::{Deserialize, Serialize};

use super::context::{AiPromptReference, AiRuntimeReference};
use super::identity::{
    AiActorIdentity, AiJournalEntryId, AiRequestId, AiSchemaIdentity, AiTimestamp, AiWorkItemId,
};
use super::{AiDegradedState, AiDurabilityState, AiSafeText, AiValidationState};

mod item;
mod lifecycle;
mod request;
mod state;

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

/// Opaque, single-transition authority issued by the owning runtime.  The
/// schema crate deliberately exposes no constructor for this token: a caller
/// must receive a real owner-issued capability before it can append a work
/// transition or lifecycle record.
#[derive(Debug)]
pub(crate) struct AiWorkTransitionAuthority {
    work_item_id: AiWorkItemId,
    request_id: AiRequestId,
    actor: AiActorIdentity,
    sequence: u64,
    next_state: AiWorkState,
    max_attempts: u16,
}

impl AiWorkTransitionAuthority {
    fn permits(
        &self,
        work_item_id: &AiWorkItemId,
        request_id: &AiRequestId,
        sequence: u64,
        next_state: AiWorkState,
        max_attempts: u16,
    ) -> bool {
        &self.work_item_id == work_item_id
            && &self.request_id == request_id
            && self.sequence == sequence
            && self.next_state == next_state
            && self.max_attempts == max_attempts
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiRetryPolicy {
    max_attempts: u16,
    retry_after_ms: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiWorkRequest {
    identity: AiSchemaIdentity,
    work_item_id: AiWorkItemId,
    work_kind: AiWorkKind,
    requested_at: AiTimestamp,
    deadline_at: Option<AiTimestamp>,
    retry_policy: AiRetryPolicy,
    prompt: Option<AiPromptReference>,
    runtime: Option<AiRuntimeReference>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiWorkItem {
    request: AiWorkRequest,
    state: AiWorkState,
    attempt: u16,
    durability: AiDurabilityState,
    validation: AiValidationState,
    degraded_state: AiDegradedState,
    last_transition_sequence: u64,
    last_transition_at: AiTimestamp,
    terminal_reason: Option<AiSafeText>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiWorkLifecycleRecord {
    work_item_id: AiWorkItemId,
    request_id: AiRequestId,
    journal_entry_id: AiJournalEntryId,
    sequence: u64,
    previous_state: Option<AiWorkState>,
    next_state: AiWorkState,
    actor: AiActorIdentity,
    occurred_at: AiTimestamp,
    durability: AiDurabilityState,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiDurableWorkLifecycle {
    work_item_id: AiWorkItemId,
    request_id: AiRequestId,
    records: Vec<AiWorkLifecycleRecord>,
    max_attempts: u16,
    last_sequence: u64,
    durability: AiDurabilityState,
}
