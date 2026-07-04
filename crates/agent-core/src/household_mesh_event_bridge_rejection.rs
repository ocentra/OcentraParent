use ocentra_parent_agent_protocol::constants;

use crate::household_mesh_event_bridge::HouseholdMeshBridgeRejection;

pub(crate) fn rejection_as_str(rejection: HouseholdMeshBridgeRejection) -> &'static str {
    match rejection {
        HouseholdMeshBridgeRejection::UnselectedLocalEvent => {
            constants::household_mesh::REJECTION_UNSELECTED_LOCAL_EVENT
        }
        HouseholdMeshBridgeRejection::UnauthenticatedMessage => {
            constants::household_mesh::REJECTION_UNAUTHENTICATED_MESSAGE
        }
        HouseholdMeshBridgeRejection::DirectRemotePublish => {
            constants::household_mesh::REJECTION_DIRECT_REMOTE_PUBLISH
        }
        HouseholdMeshBridgeRejection::PolicyAuthorityEscalation => {
            constants::household_mesh::REJECTION_POLICY_AUTHORITY_ESCALATION
        }
        HouseholdMeshBridgeRejection::RawPayload => {
            constants::household_mesh::REJECTION_RAW_PAYLOAD
        }
        HouseholdMeshBridgeRejection::MismatchedMessageRef => {
            constants::household_mesh::REJECTION_MISMATCHED_MESSAGE_REF
        }
        HouseholdMeshBridgeRejection::ReplayedMessage => {
            constants::household_mesh::REJECTION_REPLAYED_MESSAGE
        }
        HouseholdMeshBridgeRejection::StaleMessage => {
            constants::household_mesh::REJECTION_STALE_MESSAGE
        }
        HouseholdMeshBridgeRejection::FamilyMismatch => {
            constants::household_mesh::REJECTION_FAMILY_MISMATCH
        }
        HouseholdMeshBridgeRejection::WrongTargetDevice => {
            constants::household_mesh::REJECTION_WRONG_TARGET_DEVICE
        }
    }
}
