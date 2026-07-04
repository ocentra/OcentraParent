use ocentra_parent_agent_protocol::child_agent::child_agent_events::{
    ChildCapabilityStateUpdatedEvent, ChildRuntimeHealthUpdatedEvent,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::transport::ParentChildRuntimeEventPayload;

use crate::test_text::TestText;

pub(crate) fn child_capability(
    payloads: &[ParentChildRuntimeEventPayload],
) -> Result<ChildCapabilityStateUpdatedEvent, TestText> {
    some(
        payloads.iter().find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ChildCapabilityStateUpdated(event) => {
                Some(event.clone())
            }
            _ => None,
        }),
        constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES,
    )
}

pub(crate) fn child_health(
    payloads: &[ParentChildRuntimeEventPayload],
) -> Result<ChildRuntimeHealthUpdatedEvent, TestText> {
    some(
        payloads.iter().find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ChildRuntimeHealthUpdated(event) => Some(event.clone()),
            _ => None,
        }),
        constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES,
    )
}

fn some<T>(value: Option<T>, context: impl std::fmt::Display) -> Result<T, TestText> {
    value.ok_or_else(|| TestText::from_display(context))
}
