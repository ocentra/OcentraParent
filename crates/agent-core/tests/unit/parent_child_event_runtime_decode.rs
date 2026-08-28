use std::fmt::Display;

use ocentra_eventing::error::EventingError;
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
            Ok(envelope.into_payload())
        })
        .collect()
}

pub(crate) fn ok<T, E: core::fmt::Debug>(
    result: Result<T, E>,
    context: impl Display,
) -> Result<T, TestText> {
    result.map_err(|error| TestText::from_display(format!("{context}: {error:?}")))
}

pub(crate) fn expect_no_subscriber<T>(
    result: Result<T, EventingError>,
    context: impl Display,
) -> Result<(), TestText> {
    match result {
        Err(EventingError::NoSubscriber { event_type })
            if event_type.as_str() == constants::child_agent::EVENT_COMMAND_RECEIVED =>
        {
            Ok(())
        }
        Err(error) => Err(TestText::from_display(format!(
            "{context}: expected NoSubscriber({}), got {error:?}",
            constants::child_agent::EVENT_COMMAND_RECEIVED
        ))),
        Ok(_) => Err(TestText::from_display(format!(
            "{context}: expected NoSubscriber({}), got success",
            constants::child_agent::EVENT_COMMAND_RECEIVED
        ))),
    }
}
