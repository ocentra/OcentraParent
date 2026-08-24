use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::{
    context::{AiPromptReference, AiRuntimeReference},
    identity::{
        AiActorIdentity, AiJournalEntryId, AiRequestId, AiSchemaIdentity, AiTimestamp, AiWorkItemId,
    },
    AiDegradedState, AiDurabilityState, AiSafeText, AiValidationState,
    AI_INITIAL_LIFECYCLE_SEQUENCE,
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

impl AiWorkState {
    pub fn can_transition_from(self, previous: Option<Self>) -> bool {
        match (previous, self) {
            (None, Self::Queued) => true,
            (Some(Self::Queued), Self::Claimed | Self::Cancelled | Self::ManualRequired) => true,
            (
                Some(Self::Claimed),
                Self::Running | Self::Cancelled | Self::Failed | Self::ManualRequired,
            ) => true,
            (
                Some(Self::Running),
                Self::Succeeded
                | Self::Failed
                | Self::Cancelled
                | Self::TimedOut
                | Self::ManualRequired,
            ) => true,
            _ => false,
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Succeeded
                | Self::Failed
                | Self::Cancelled
                | Self::TimedOut
                | Self::ManualRequired
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiRetryPolicy {
    max_attempts: u16,
    retry_after_ms: Option<u64>,
}

impl AiRetryPolicy {
    pub fn new(max_attempts: u16, retry_after_ms: Option<u64>) -> Result<Self, &'static str> {
        if max_attempts == 0 {
            return Err("AI retry policy requires at least one attempt");
        }
        Ok(Self {
            max_attempts,
            retry_after_ms,
        })
    }

    pub fn max_attempts(&self) -> u16 {
        self.max_attempts
    }

    pub fn retry_after_ms(&self) -> Option<u64> {
        self.retry_after_ms
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiRetryPolicyFields {
    max_attempts: u16,
    retry_after_ms: Option<u64>,
}

impl<'de> Deserialize<'de> for AiRetryPolicy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let fields = AiRetryPolicyFields::deserialize(deserializer)?;
        Self::new(fields.max_attempts, fields.retry_after_ms).map_err(serde::de::Error::custom)
    }
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

impl AiWorkRequest {
    pub fn new(
        identity: AiSchemaIdentity,
        work_item_id: AiWorkItemId,
        work_kind: AiWorkKind,
        requested_at: AiTimestamp,
        deadline_at: Option<AiTimestamp>,
        retry_policy: AiRetryPolicy,
        prompt: Option<AiPromptReference>,
        runtime: Option<AiRuntimeReference>,
    ) -> Result<Self, &'static str> {
        if !requested_at.is_well_formed()
            || deadline_at
                .as_ref()
                .is_some_and(|deadline| !requested_at.precedes(deadline))
        {
            return Err("AI work request has an invalid requested/deadline timestamp");
        }
        Ok(Self {
            identity,
            work_item_id,
            work_kind,
            requested_at,
            deadline_at,
            retry_policy,
            prompt,
            runtime,
        })
    }

    pub fn identity(&self) -> &AiSchemaIdentity {
        &self.identity
    }

    pub fn work_item_id(&self) -> &AiWorkItemId {
        &self.work_item_id
    }

    pub fn work_kind(&self) -> AiWorkKind {
        self.work_kind
    }

    pub fn request_id(&self) -> &AiRequestId {
        self.identity.request_id()
    }
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

impl AiWorkItem {
    pub(crate) fn queued(request: AiWorkRequest) -> Result<Self, &'static str> {
        let requested_at = request.requested_at.clone();
        Ok(Self {
            last_transition_at: requested_at,
            request,
            state: AiWorkState::Queued,
            attempt: 0,
            durability: AiDurabilityState::AppendPending,
            validation: AiValidationState::ManualRequired,
            degraded_state: AiDegradedState::None,
            last_transition_sequence: AI_INITIAL_LIFECYCLE_SEQUENCE,
            terminal_reason: None,
        })
    }

    pub(crate) fn transition(
        &self,
        next_state: AiWorkState,
        sequence: u64,
        occurred_at: AiTimestamp,
        durability: AiDurabilityState,
        validation: AiValidationState,
        degraded_state: AiDegradedState,
        terminal_reason: Option<AiSafeText>,
    ) -> Result<Self, &'static str> {
        if !next_state.can_transition_from(Some(self.state))
            || self
                .last_transition_sequence
                .checked_add(1)
                .is_none_or(|expected| sequence != expected)
            || !matches!(durability, AiDurabilityState::Durable)
            || !occurred_at.is_well_formed()
            || !self.last_transition_at.precedes(&occurred_at)
            || (next_state.is_terminal() && terminal_reason.is_none())
            || (!next_state.is_terminal() && terminal_reason.is_some())
        {
            return Err("AI work item transition is illegal or not durable");
        }
        Ok(Self {
            request: self.request.clone(),
            state: next_state,
            attempt: self.attempt.saturating_add(u16::from(matches!(
                next_state,
                AiWorkState::Claimed | AiWorkState::Running
            ))),
            durability,
            validation,
            degraded_state,
            last_transition_sequence: sequence,
            last_transition_at: occurred_at,
            terminal_reason,
        })
    }

    pub fn request(&self) -> &AiWorkRequest {
        &self.request
    }

    pub fn state(&self) -> AiWorkState {
        self.state
    }

    pub fn attempt(&self) -> u16 {
        self.attempt
    }

    pub fn durability(&self) -> AiDurabilityState {
        self.durability
    }

    pub fn last_transition_sequence(&self) -> u64 {
        self.last_transition_sequence
    }
}

/// Owner-issued lifecycle transition. It is serialized for journal/read-model
/// output but cannot be minted from wire state.
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

impl AiWorkLifecycleRecord {
    pub(crate) fn new(
        work_item_id: AiWorkItemId,
        request_id: AiRequestId,
        journal_entry_id: AiJournalEntryId,
        sequence: u64,
        previous_state: Option<AiWorkState>,
        next_state: AiWorkState,
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

    pub fn previous_state(&self) -> Option<AiWorkState> {
        self.previous_state
    }

    pub fn next_state(&self) -> AiWorkState {
        self.next_state
    }

    pub fn durability(&self) -> AiDurabilityState {
        self.durability
    }

    pub fn occurred_at(&self) -> &AiTimestamp {
        &self.occurred_at
    }
}

/// Owner-issued durable lifecycle aggregate.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AiDurableWorkLifecycle {
    work_item_id: AiWorkItemId,
    request_id: AiRequestId,
    records: Vec<AiWorkLifecycleRecord>,
    last_sequence: u64,
    durability: AiDurabilityState,
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
            if index > 0 {
                let previous = &records[index - 1];
                if record.previous_state() != Some(previous.next_state())
                    || !previous.occurred_at().precedes(record.occurred_at())
                {
                    return Err("AI durable lifecycle contains an ambiguous state transition");
                }
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
