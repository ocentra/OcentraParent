use ocentra_parent_agent_core::network_capture::NetworkObservation;

#[derive(Clone, Debug)]
pub(crate) struct NetworkCaptureObservation {
    pub(crate) source_event_id: String,
    pub(crate) observed_at: String,
    pub(crate) observation: NetworkObservation,
}
