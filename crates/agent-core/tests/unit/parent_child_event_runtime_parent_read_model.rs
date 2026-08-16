use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::parent_controller_events::ParentReadModelProjectedEvent;
use ocentra_parent_agent_protocol::transport::ParentChildRuntimeEventPayload;

use crate::test_text::TestText;

pub(crate) fn parent_read_model(
    payloads: &[ParentChildRuntimeEventPayload],
) -> Result<ParentReadModelProjectedEvent, TestText> {
    some(
        payloads.iter().find_map(|payload| match payload {
            ParentChildRuntimeEventPayload::ParentReadModelProjected(event) => Some(event.clone()),
            _ => None,
        }),
        constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES,
    )
}

fn some<T>(value: Option<T>, context: impl std::fmt::Display) -> Result<T, TestText> {
    value.ok_or_else(|| TestText::from_display(context))
}
