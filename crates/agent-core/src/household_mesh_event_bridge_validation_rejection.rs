use crate::household_mesh_event_bridge::HouseholdMeshBridgeRejection;

pub(super) fn rejection_from_protocol_validation(
    validation: ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgeValidation,
) -> HouseholdMeshBridgeRejection {
    use ocentra_parent_agent_protocol::household_mesh::HouseholdMeshBridgeRejectionReason as Reason;

    match validation.rejection_reason {
        Some(Reason::UnauthenticatedPeer | Reason::UnauthorizedPeer) => {
            HouseholdMeshBridgeRejection::UnauthenticatedMessage
        }
        Some(Reason::DirectRemotePublish) => HouseholdMeshBridgeRejection::DirectRemotePublish,
        Some(Reason::PolicyAuthorityEscalation) => {
            HouseholdMeshBridgeRejection::PolicyAuthorityEscalation
        }
        Some(Reason::RawScreenPayload) => HouseholdMeshBridgeRejection::RawPayload,
        Some(Reason::ReplayedMessage) => HouseholdMeshBridgeRejection::ReplayedMessage,
        Some(Reason::StaleMessage) => HouseholdMeshBridgeRejection::StaleMessage,
        Some(Reason::FamilyMismatch) => HouseholdMeshBridgeRejection::FamilyMismatch,
        Some(Reason::WrongTargetDevice) => HouseholdMeshBridgeRejection::WrongTargetDevice,
        Some(
            Reason::PrivateLocalEvent | Reason::UnselectedEvent | Reason::UnsupportedLanMessage,
        ) => HouseholdMeshBridgeRejection::UnselectedLocalEvent,
        Some(Reason::MismatchedMessageRef) | None => {
            HouseholdMeshBridgeRejection::MismatchedMessageRef
        }
    }
}
