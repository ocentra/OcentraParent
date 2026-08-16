use ocentra_eventing::error::EventingError;

pub type BrowserRuntimeSocialProviderReceiptDurableReadModelState =
    ocentra_parent_agent_protocol::browser::social_provider_receipt_durable::BrowserRuntimeSocialProviderReceiptDurableReadModelState;
pub type BrowserRuntimeSocialProviderReceiptDurableRecord =
    ocentra_parent_agent_protocol::browser::social_provider_receipt_durable::BrowserRuntimeSocialProviderReceiptDurableRecord;
pub type BrowserRuntimeSocialProviderReceiptDurableReport =
    ocentra_parent_agent_protocol::browser::social_provider_receipt_durable::BrowserRuntimeSocialProviderReceiptDurableReport;

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
