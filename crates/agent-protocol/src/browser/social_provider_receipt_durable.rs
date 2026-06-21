use ocentra_eventing::{
    error::EventingError, ids::CorrelationId, ids::EventId, ids::EventType, ids::SourceComponent,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrowserRuntimeSocialProviderReceiptDurableReadModelState {
    ProviderDispatchRequiredManualReceipt,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserRuntimeSocialProviderReceiptDurableRecord {
    pub sequence: u64,
    pub request_event_id: EventId,
    pub request_event_type: EventType,
    pub correlation_id: CorrelationId,
    pub state: BrowserRuntimeSocialProviderReceiptDurableReadModelState,
    pub action_intent_id: String,
    pub provider_attempt_ref: SourceComponent,
    pub provider_receipt_proof_ref: SourceComponent,
    pub durable_result_ref: SourceComponent,
    pub durable_store_ref: SourceComponent,
    pub read_model_ref: SourceComponent,
    pub support_status_ref: SourceComponent,
    pub source_ref: SourceComponent,
    pub evidence_ref: SourceComponent,
}

#[derive(Clone, Debug)]
pub struct BrowserRuntimeSocialProviderReceiptDurableReport {
    pub request_event_count: usize,
    pub durable_record_count: usize,
    pub read_model_row_count: usize,
    pub provider_dispatch_required_count: usize,
    pub manual_receipt_required_count: usize,
    pub provider_receipt_count: usize,
    pub provider_dispatch_count: usize,
    pub connector_native_runtime_count: usize,
    pub parent_notification_ui_delivery_count: usize,
    pub report_delivery_execution_count: usize,
    pub final_policy_execution_count: usize,
    pub enforcement_execution_count: usize,
    pub duplicate_request_event_rejected: bool,
    pub row_matches_receipt_response: bool,
    pub row_matches_request_event: bool,
    pub provider_receipt_claimed: bool,
    pub provider_dispatch_claimed: bool,
    pub connector_native_runtime_claimed: bool,
    pub parent_notification_ui_delivery_claimed: bool,
    pub report_delivery_execution_claimed: bool,
    pub final_policy_execution_claimed: bool,
    pub enforcement_claimed: bool,
    pub rows: Vec<BrowserRuntimeSocialProviderReceiptDurableRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrowserRuntimeSocialProviderReceiptDurableError {
    Eventing(EventingError),
    EmptyReceipt,
    DuplicateRequestEvent,
    MissingReceiptRef,
    RowMismatch,
    UnsupportedClaim,
}

impl From<EventingError> for BrowserRuntimeSocialProviderReceiptDurableError {
    fn from(error: EventingError) -> Self {
        Self::Eventing(error)
    }
}
