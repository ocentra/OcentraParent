use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::parent_controller_events::{
    ParentActionReceivedEvent, ParentChildCommandForwardRequestedEvent,
    ParentChildCommandForwardedEvent, ParentCommandValidatedEvent,
};
use ocentra_parent_agent_protocol::transport::ParentChildRuntimeEventPayload;

use crate::test_text::TestText;

pub(crate) fn parent_action(
    payloads: &[ParentChildRuntimeEventPayload],
) -> Result<ParentActionReceivedEvent, TestText> {
    some(
        payloads.iter().find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ParentActionReceived(event) => Some(event.clone()),
            _ => None,
        }),
        constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES,
    )
}

pub(crate) fn parent_validated(
    payloads: &[ParentChildRuntimeEventPayload],
) -> Result<ParentCommandValidatedEvent, TestText> {
    some(
        payloads.iter().find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ParentCommandValidated(event) => Some(event.clone()),
            _ => None,
        }),
        constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES,
    )
}

pub(crate) fn parent_forward_requested(
    payloads: &[ParentChildRuntimeEventPayload],
) -> Result<ParentChildCommandForwardRequestedEvent, TestText> {
    some(
        payloads.iter().find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ParentChildCommandForwardRequested(event) => {
                Some(event.clone())
            }
            _ => None,
        }),
        constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES,
    )
}

pub(crate) fn parent_forwarded(
    payloads: &[ParentChildRuntimeEventPayload],
) -> Result<ParentChildCommandForwardedEvent, TestText> {
    some(
        payloads.iter().find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ParentChildCommandForwarded(event) => {
                Some(event.clone())
            }
            _ => None,
        }),
        constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES,
    )
}

fn some<T>(value: Option<T>, context: impl std::fmt::Display) -> Result<T, TestText> {
    value.ok_or_else(|| TestText::from_display(context))
}
