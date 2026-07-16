use ocentra_eventing::{
    error::EventingError, ids::CorrelationId, ids::EventId, ids::EventType, ids::SourceComponent,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowserRuntimeActionIntentDurableHandoffReadModelState {
    PreparedNotDispatched,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserRuntimeActionIntentDurableHandoffRecord {
    pub sequence: u64,
    pub request_event_id: EventId,
    pub request_event_type: EventType,
    pub correlation_id: CorrelationId,
    pub state: BrowserRuntimeActionIntentDurableHandoffReadModelState,
    pub policy_preview_id: String,
    pub action_intent_id: String,
    pub source_event_ref: SourceComponent,
    pub durable_result_ref: SourceComponent,
    pub durable_store_ref: SourceComponent,
    pub outbox_ref: SourceComponent,
    pub handoff_ref: SourceComponent,
    pub read_model_ref: SourceComponent,
    pub support_status_ref: SourceComponent,
    pub source_ref: SourceComponent,
    pub evidence_ref: SourceComponent,
}

#[derive(Clone, Debug)]
pub struct BrowserRuntimeActionIntentDurableHandoffReport {
    pub request_event_count: usize,
    pub durable_record_count: usize,
    pub read_model_row_count: usize,
    pub prepared_not_dispatched_count: usize,
    pub dispatch_attempt_count: usize,
    pub adapter_execution_count: usize,
    pub browser_mutation_count: usize,
    pub child_intervention_execution_count: usize,
    pub final_policy_execution_count: usize,
    pub enforcement_execution_count: usize,
    pub duplicate_request_event_rejected: bool,
    pub row_matches_handoff_response: bool,
    pub row_matches_request_event: bool,
    pub external_transport_implemented: bool,
    pub adapter_dispatch_claimed: bool,
    pub browser_mutation_claimed: bool,
    pub child_intervention_execution_claimed: bool,
    pub final_policy_execution_claimed: bool,
    pub enforcement_claimed: bool,
    pub rows: Vec<BrowserRuntimeActionIntentDurableHandoffRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrowserRuntimeActionIntentDurableHandoffError {
    Eventing(EventingError),
    EmptyHandoff,
    DuplicateRequestEvent,
    MissingHandoffRef,
    RowMismatch,
    UnsupportedClaim,
}

impl From<EventingError> for BrowserRuntimeActionIntentDurableHandoffError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
