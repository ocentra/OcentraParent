use ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgePhase;

use crate::household_mesh_bridge_runtime_state::HouseholdMeshBridgeDirection;

pub(crate) fn bridge_direction_for_phase(
    phase: HouseholdMeshBridgePhase,
) -> HouseholdMeshBridgeDirection {
    match phase {
        HouseholdMeshBridgePhase::LocalEventSelected
        | HouseholdMeshBridgePhase::LanMessageExported => HouseholdMeshBridgeDirection::Export,
        HouseholdMeshBridgePhase::LanMessageReceived
        | HouseholdMeshBridgePhase::LocalEventRepublished => HouseholdMeshBridgeDirection::Import,
    }
}
