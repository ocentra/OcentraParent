use ocentra_parent_agent_protocol::constants;

pub(crate) type HouseholdMeshBridgeDirection =
    ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgeDirection;
pub(crate) type HouseholdMeshBridgeValidationState =
    ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgeValidationState;
pub(crate) type HouseholdMeshBridgeRejectionReason =
    ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgeRejectionReason;
pub(crate) type HouseholdMeshBridgeCustody =
    ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgeCustody;

pub(crate) fn bridge_custody_label() -> &'static str {
    constants::value::LAN_PROVIDER_CUSTODY_LOCAL_NETWORK_AI_PROVIDER
}
