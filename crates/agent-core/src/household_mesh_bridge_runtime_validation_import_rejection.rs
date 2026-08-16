use crate::household_mesh_bridge_runtime_state::HouseholdMeshBridgeRejectionReason;
use crate::household_mesh_event_bridge::HouseholdMeshBridgeRejection;

pub(crate) fn import_rejection_reason(
    rejection: HouseholdMeshBridgeRejection,
) -> HouseholdMeshBridgeRejectionReason {
    match rejection {
        HouseholdMeshBridgeRejection::UnselectedLocalEvent => {
            HouseholdMeshBridgeRejectionReason::UnselectedEvent
        }
        HouseholdMeshBridgeRejection::UnauthenticatedMessage => {
            HouseholdMeshBridgeRejectionReason::UnauthenticatedPeer
        }
        HouseholdMeshBridgeRejection::DirectRemotePublish => {
            HouseholdMeshBridgeRejectionReason::DirectRemotePublish
        }
        HouseholdMeshBridgeRejection::PolicyAuthorityEscalation => {
            HouseholdMeshBridgeRejectionReason::PolicyAuthorityEscalation
        }
        HouseholdMeshBridgeRejection::RawPayload => {
            HouseholdMeshBridgeRejectionReason::RawScreenPayload
        }
        HouseholdMeshBridgeRejection::MismatchedMessageRef => {
            HouseholdMeshBridgeRejectionReason::MismatchedMessageRef
        }
        HouseholdMeshBridgeRejection::ReplayedMessage => {
            HouseholdMeshBridgeRejectionReason::ReplayedMessage
        }
        HouseholdMeshBridgeRejection::StaleMessage => {
            HouseholdMeshBridgeRejectionReason::StaleMessage
        }
        HouseholdMeshBridgeRejection::FamilyMismatch => {
            HouseholdMeshBridgeRejectionReason::FamilyMismatch
        }
        HouseholdMeshBridgeRejection::WrongTargetDevice => {
            HouseholdMeshBridgeRejectionReason::WrongTargetDevice
        }
    }
}
