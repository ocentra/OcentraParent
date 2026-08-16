use ocentra_eventing::error::EventingError;

use super::ChildAgentServiceError;

impl std::error::Error for ChildAgentServiceError {}

impl From<EventingError> for ChildAgentServiceError {
    fn from(error: EventingError) -> Self {
        Self::Runtime(error)
    }
}
