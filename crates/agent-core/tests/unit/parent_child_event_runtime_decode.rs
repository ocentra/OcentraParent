use std::fmt::Display;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::transport::{
    ParentChildRuntimeEventPayload, ParentChildRuntimeReport,
};

use crate::test_text::TestText;

pub(crate) fn decode_payloads(
    report: &ParentChildRuntimeReport,
) -> Result<Vec<ParentChildRuntimeEventPayload>, TestText> {
    report
        .stored_events
        .iter()
        .map(|event| {
            let envelope: ocentra_eventing::envelope::EventEnvelope<
                ParentChildRuntimeEventPayload,
            > = ok(
                event.decode(),
                constants::parent_controller::ERROR_PARENT_CHILD_RUNTIME_PAYLOAD_DECODES,
            )?;
            Ok(envelope.payload)
        })
        .collect()
}

pub(crate) fn ok<T, E: core::fmt::Debug>(
    result: Result<T, E>,
    context: impl Display,
) -> Result<T, TestText> {
    result.map_err(|error| TestText::from_display(format!("{context}: {error:?}")))
}
