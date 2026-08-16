use ocentra_eventing::bus::reports::dead_letter::DeadLetter;
use ocentra_eventing::bus::reports::handler::PublishReport;
use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::screen_evidence::{
    ScreenHouseholdMeshEventPayload as ProtocolScreenHouseholdMeshEventPayload,
    ScreenHouseholdMeshResultValidation,
};

#[path = "screen_household_mesh_runtime_impl.rs"]
mod screen_household_mesh_runtime_impl;

pub type ScreenHouseholdMeshEventPayload = ProtocolScreenHouseholdMeshEventPayload;
pub type ScreenHouseholdMeshInput =
    ocentra_parent_agent_protocol::screen_evidence::screen_household_mesh_input::ScreenHouseholdMeshInput;
pub type ScreenHouseholdMeshResultSubmission =
    ocentra_parent_agent_protocol::screen_evidence::screen_household_mesh_input::ScreenHouseholdMeshResultSubmission;

#[derive(Clone, Debug)]
pub struct ScreenHouseholdMeshReport {
    pub publish_reports: Vec<PublishReport>,
    pub stored_events: Vec<ocentra_eventing::envelope::StoredEventEnvelope>,
    pub dead_letters: Vec<DeadLetter>,
}

impl ScreenHouseholdMeshReport {
    pub fn raw_screenshot_escaped(&self) -> bool {
        self.stored_events.iter().any(|event| {
            event
                .decode::<ScreenHouseholdMeshEventPayload>()
                .map(|envelope| {
                    envelope.payload.custody_boundary.raw_screenshot_transferred
                        || envelope
                            .payload
                            .custody_boundary
                            .raw_screenshot_retained_by_provider
                })
                .unwrap_or(true)
        })
    }
}

pub async fn publish_screen_household_mesh_chain_for_input(
    input: ScreenHouseholdMeshInput,
) -> Result<ScreenHouseholdMeshReport, EventingError> {
    screen_household_mesh_runtime_impl::publish_screen_household_mesh_chain_for_input(input).await
}

pub fn validate_screen_household_mesh_result(
    input: &ScreenHouseholdMeshInput,
    submission: &ScreenHouseholdMeshResultSubmission,
) -> ScreenHouseholdMeshResultValidation {
    screen_household_mesh_runtime_impl::validate_screen_household_mesh_result(input, submission)
}
