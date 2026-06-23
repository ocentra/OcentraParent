use ocentra_eventing::error::EventingError;

use super::action_handoff_durable_types::BrowserRuntimeActionIntentDurableHandoffError;

pub(crate) type BrowserRuntimeActionIntentChildStatusReadModelState =
    ocentra_parent_agent_protocol::browser::action_handoff_child_status::BrowserRuntimeActionIntentChildStatusReadModelState;
pub(crate) type BrowserRuntimeActionIntentChildStatusRecord =
    ocentra_parent_agent_protocol::browser::action_handoff_child_status::BrowserRuntimeActionIntentChildStatusRecord;
pub(crate) type BrowserRuntimeActionIntentChildStatusReport =
    ocentra_parent_agent_protocol::browser::action_handoff_child_status::BrowserRuntimeActionIntentChildStatusReport;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BrowserRuntimeActionIntentChildStatusError {
    Handoff(BrowserRuntimeActionIntentDurableHandoffError),
    ParentChildRuntime(EventingError),
    PayloadDecode(EventingError),
    MissingPayload,
    HandoffMismatch,
    UnsupportedClaim,
}
