use ocentra_eventing::{error::EventingError, ids::EventType};
use ocentra_parent_agent_protocol::transport::{ParentChildRuntimePhase, ParentChildRuntimeReport};

pub(crate) type ParentChildRuntimeInput =
    ocentra_parent_agent_protocol::transport::parent_child_runtime_input::ParentChildRuntimeInput;

pub async fn publish_parent_child_runtime_for_validated_intent(
    _input: ParentChildRuntimeInput,
) -> Result<ParentChildRuntimeReport, EventingError> {
    Err(EventingError::NoSubscriber {
        event_type: EventType::parse(ParentChildRuntimePhase::ChildCommandReceived.event_type())?,
    })
}
