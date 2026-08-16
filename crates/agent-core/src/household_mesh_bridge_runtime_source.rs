use ocentra_eventing::{
    envelope::EventMetadata, envelope::EventSource, error::EventingError, ids::EventCustody,
    ids::EventId, ids::RecordedAt, ids::RuntimeInstanceId, ids::RuntimeRole, ids::SourceComponent,
    ids::SourceService, ids::TargetHandler,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::household_mesh::{
    household_mesh_bridge_input::HouseholdMeshBridgeInput, HouseholdMeshBridgePhase,
};

use crate::{
    household_mesh_bridge_runtime_refs::bridge_aggregate_key,
    household_mesh_bridge_runtime_state::bridge_custody_label,
};

pub(crate) fn bridge_event_metadata(
    phase: HouseholdMeshBridgePhase,
    input: &HouseholdMeshBridgeInput,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        ocentra_eventing::ids::CorrelationId::parse(bridge_aggregate_key(&input.correlation_id))?,
        bridge_event_source(phase)?,
        RecordedAt::parse(&input.observed_at)?,
        Some(TargetHandler::parse(phase.target_handler())?),
    ))
}

fn bridge_event_source(phase: HouseholdMeshBridgePhase) -> Result<EventSource, EventingError> {
    Ok(EventSource::new(
        EventCustody::parse(bridge_custody_label())?,
        bridge_runtime_role(phase)?,
        SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
        SourceComponent::parse(constants::household_mesh::RUNTIME_COMPONENT_HOUSEHOLD_MESH_BRIDGE)?,
        RuntimeInstanceId::parse(bridge_runtime_instance(phase))?,
    ))
}

fn bridge_runtime_role(phase: HouseholdMeshBridgePhase) -> Result<RuntimeRole, EventingError> {
    let value = match phase {
        HouseholdMeshBridgePhase::LocalEventSelected
        | HouseholdMeshBridgePhase::LanMessageExported => constants::eventing_source::ROLE_AGENT,
        HouseholdMeshBridgePhase::LanMessageReceived
        | HouseholdMeshBridgePhase::LocalEventRepublished => {
            constants::eventing_source::ROLE_ANALYZER
        }
    };
    RuntimeRole::parse(value)
}

fn bridge_runtime_instance(phase: HouseholdMeshBridgePhase) -> &'static str {
    match phase {
        HouseholdMeshBridgePhase::LocalEventSelected
        | HouseholdMeshBridgePhase::LanMessageExported => {
            constants::household_mesh::RUNTIME_INSTANCE_CHILD_MESH_BRIDGE
        }
        HouseholdMeshBridgePhase::LanMessageReceived
        | HouseholdMeshBridgePhase::LocalEventRepublished => {
            constants::household_mesh::RUNTIME_INSTANCE_PROVIDER_MESH_BRIDGE
        }
    }
}
