use std::fmt::Display;

use crate::test_text::TestText;
use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants;

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
