use crate::household_mesh::{
    HouseholdMeshAuthenticationState, HouseholdMeshBridgeRejectionReason,
    HouseholdMeshPolicyAuthority, HouseholdMeshTransportEnvelope,
};

pub(super) fn rejection_reason(
    message: &HouseholdMeshTransportEnvelope,
) -> Option<HouseholdMeshBridgeRejectionReason> {
    if message.authentication_state != HouseholdMeshAuthenticationState::PairedTrustedDevice {
        return Some(HouseholdMeshBridgeRejectionReason::UnauthenticatedPeer);
    }
    if message.direct_remote_publish_requested {
        return Some(HouseholdMeshBridgeRejectionReason::DirectRemotePublish);
    }
    if message.raw_payload_included {
        return Some(HouseholdMeshBridgeRejectionReason::RawScreenPayload);
    }
    if message.policy_authority != HouseholdMeshPolicyAuthority::ChildAgentOnly {
        return Some(HouseholdMeshBridgeRejectionReason::PolicyAuthorityEscalation);
    }
    None
}
