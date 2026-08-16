use crate::household_mesh_event_bridge::HouseholdMeshBridgeRejection;

pub(crate) fn bridge_rejection_reason(
    rejection: HouseholdMeshBridgeRejection,
) -> crate::household_mesh_bridge_runtime_state::HouseholdMeshBridgeRejectionReason {
    use crate::household_mesh_bridge_runtime_state::HouseholdMeshBridgeRejectionReason as Reason;
    use crate::household_mesh_event_bridge::HouseholdMeshBridgeRejection as Rejection;

    match rejection {
        Rejection::UnselectedLocalEvent => Reason::UnselectedEvent,
        Rejection::UnauthenticatedMessage => Reason::UnauthenticatedPeer,
        Rejection::DirectRemotePublish => Reason::DirectRemotePublish,
        Rejection::PolicyAuthorityEscalation => Reason::PolicyAuthorityEscalation,
        Rejection::RawPayload => Reason::RawScreenPayload,
        Rejection::MismatchedMessageRef => Reason::MismatchedMessageRef,
        Rejection::ReplayedMessage => Reason::ReplayedMessage,
        Rejection::StaleMessage => Reason::StaleMessage,
        Rejection::FamilyMismatch => Reason::FamilyMismatch,
        Rejection::WrongTargetDevice => Reason::WrongTargetDevice,
    }
}
