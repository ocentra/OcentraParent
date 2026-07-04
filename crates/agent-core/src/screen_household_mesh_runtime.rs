use ocentra_eventing::bus::reports::dead_letter::DeadLetter;
use ocentra_eventing::bus::reports::handler::PublishReport;
use ocentra_eventing::{
    bus::subscriber::EventSubscriber, bus::EventBus, envelope::EventMetadata,
    envelope::EventSource, error::EventingError, ids::CorrelationId, ids::EventCustody,
    ids::EventId, ids::EventType, ids::RecordedAt, ids::RuntimeInstanceId, ids::SourceComponent,
    ids::SourceService, ids::SubscriberId, ids::TargetHandler,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::{
    ScreenHouseholdMeshEventPayload as ProtocolScreenHouseholdMeshEventPayload,
    ScreenHouseholdMeshPhase, ScreenHouseholdMeshResultValidation, ScreenMeshCustodyBoundary,
    ScreenMeshPayloadMode, ScreenMeshResultRejectionReason,
};

use crate::{
    screen_household_mesh_runtime_refs::{
        child_validation_state, claim_state, lease_state, mesh_aggregate_key, policy_state,
        previous_mesh_phase_ref, provider_result_state,
    },
    screen_household_mesh_runtime_state::custody_label,
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

pub(crate) fn screen_household_mesh_event_payload_from_input(
    phase: ScreenHouseholdMeshPhase,
    input: &ScreenHouseholdMeshInput,
) -> ScreenHouseholdMeshEventPayload {
    screen_household_mesh_runtime_impl::screen_household_mesh_event_payload_from_input(phase, input)
}

pub(crate) fn screen_mesh_rejection_reason(
    input: &ScreenHouseholdMeshInput,
    submission: &ScreenHouseholdMeshResultSubmission,
) -> Option<ScreenMeshResultRejectionReason> {
    screen_household_mesh_runtime_impl::screen_mesh_rejection_reason(input, submission)
}

pub(crate) fn provider_result_ref(
    phase: ScreenHouseholdMeshPhase,
    input: &ScreenHouseholdMeshInput,
) -> Option<String> {
    screen_household_mesh_runtime_impl::provider_result_ref(phase, input)
}

pub(crate) fn policy_decision_ref(
    phase: ScreenHouseholdMeshPhase,
    input: &ScreenHouseholdMeshInput,
) -> Option<String> {
    screen_household_mesh_runtime_impl::policy_decision_ref(phase, input)
}

pub(crate) fn screen_mesh_event_metadata(
    phase: ScreenHouseholdMeshPhase,
    input: &ScreenHouseholdMeshInput,
) -> Result<EventMetadata, EventingError> {
    screen_household_mesh_runtime_impl::screen_mesh_event_metadata(phase, input)
}

pub(crate) fn screen_mesh_event_source(
    phase: ScreenHouseholdMeshPhase,
) -> Result<EventSource, EventingError> {
    screen_household_mesh_runtime_impl::screen_mesh_event_source(phase)
}
