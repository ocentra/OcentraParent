use ocentra_eventing::bus::reports::dead_letter::DeadLetter;
use ocentra_eventing::bus::reports::handler::PublishReport;
use ocentra_eventing::error::EventingError;

use crate::{
    household_mesh_bridge_runtime_spine::HouseholdMeshBridgeSpine,
    household_mesh_bridge_runtime_validation::validate_household_mesh_bridge_export as validate_household_mesh_bridge_export_impl,
    household_mesh_bridge_runtime_validation::validate_household_mesh_bridge_import as validate_household_mesh_bridge_import_impl,
};

pub type HouseholdMeshBridgeValidation =
    ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgeValidation;
pub type HouseholdMeshBridgeEventPayload =
    ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgeEventPayload;
pub type HouseholdMeshBridgeInput =
    ocentra_parent_agent_protocol::household_mesh::household_mesh_bridge_input::HouseholdMeshBridgeInput;
pub type HouseholdMeshBridgeExportCandidate =
    ocentra_parent_agent_protocol::household_mesh::household_mesh_bridge_input::HouseholdMeshBridgeExportCandidate;
pub type HouseholdMeshBridgeInboundEnvelope =
    ocentra_parent_agent_protocol::household_mesh::household_mesh_bridge_input::HouseholdMeshBridgeInboundEnvelope;

#[derive(Clone, Debug)]
pub struct HouseholdMeshBridgeReport {
    pub publish_reports: Vec<PublishReport>,
    pub stored_events: Vec<ocentra_eventing::envelope::StoredEventEnvelope>,
    pub dead_letters: Vec<DeadLetter>,
}

impl HouseholdMeshBridgeReport {
    pub fn violates_bridge_custody(&self) -> bool {
        self.stored_events.iter().any(|event| {
            event
                .decode::<HouseholdMeshBridgeEventPayload>()
                .map(|envelope| {
                    envelope.payload.custody.remote_direct_publish_allowed
                        || envelope.payload.custody.raw_screenshot_transferred
                        || envelope.payload.custody.private_local_event_exported
                })
                .unwrap_or(true)
        })
    }
}

pub async fn publish_household_mesh_bridge_chain_for_input(
    input: HouseholdMeshBridgeInput,
) -> Result<HouseholdMeshBridgeReport, EventingError> {
    let spine = HouseholdMeshBridgeSpine::with_default_handlers().await?;
    spine.publish_input_chain(input).await
}

pub fn validate_household_mesh_bridge_export(
    candidate: &HouseholdMeshBridgeExportCandidate,
) -> HouseholdMeshBridgeValidation {
    validate_household_mesh_bridge_export_impl(candidate)
}

pub fn validate_household_mesh_bridge_import(
    envelope: &HouseholdMeshBridgeInboundEnvelope,
) -> HouseholdMeshBridgeValidation {
    validate_household_mesh_bridge_import_impl(envelope)
}
