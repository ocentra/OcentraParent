use ocentra_eventing::error::EventingError;

pub(crate) type BrowserRuntimeActionIntentDurableHandoffReadModelState =
    ocentra_parent_agent_protocol::browser::action_handoff_durable::BrowserRuntimeActionIntentDurableHandoffReadModelState;
pub(crate) type BrowserRuntimeActionIntentDurableHandoffRecord =
    ocentra_parent_agent_protocol::browser::action_handoff_durable::BrowserRuntimeActionIntentDurableHandoffRecord;
pub(crate) type BrowserRuntimeActionIntentDurableHandoffReport =
    ocentra_parent_agent_protocol::browser::action_handoff_durable::BrowserRuntimeActionIntentDurableHandoffReport;

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
