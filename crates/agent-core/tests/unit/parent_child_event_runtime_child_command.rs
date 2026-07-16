use ocentra_parent_agent_protocol::child_agent::child_agent_events::{
    ChildCommandAcceptedEvent, ChildCommandReceivedEvent,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::transport::ParentChildRuntimeEventPayload;

use crate::test_text::TestText;

pub(crate) fn child_received(
    payloads: &[ParentChildRuntimeEventPayload],
) -> Result<ChildCommandReceivedEvent, TestText> {
    some(
        payloads.iter().find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ChildCommandReceived(event) => Some(event.clone()),
            _ => None,
        }),
        constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES,
    )
}

pub(crate) fn child_accepted(
    payloads: &[ParentChildRuntimeEventPayload],
) -> Result<ChildCommandAcceptedEvent, TestText> {
    some(
        payloads.iter().find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ChildCommandAccepted(event) => Some(event.clone()),
            _ => None,
        }),
        constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES,
    )
}

fn some<T>(value: Option<T>, context: impl std::fmt::Display) -> Result<T, TestText> {
    value.ok_or_else(|| TestText::from_display(context))
}
