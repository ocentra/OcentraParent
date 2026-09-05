use crate::{
    CausationId, CorrelationId, EventId, EventMetadata, EventingError, StoredEventEnvelope,
};

#[derive(Clone)]
pub(super) struct CausalParent {
    event_id: EventId,
    correlation_id: CorrelationId,
}

impl CausalParent {
    pub(super) fn from_stored(parent: &StoredEventEnvelope) -> Self {
        Self {
            event_id: parent.event_id.clone(),
            correlation_id: parent.correlation_id.clone(),
        }
    }

    pub(super) fn derive_metadata(
        &self,
        mut child: EventMetadata,
    ) -> Result<EventMetadata, EventingError> {
        if let Some(causation_id) = child.causation_id.take() {
            return Err(EventingError::CallerSuppliedCausation { causation_id });
        }
        child.correlation_id = self.correlation_id.clone();
        child.causation_id = Some(CausationId::parse(self.event_id.as_str().to_owned())?);
        Ok(child)
    }
}
