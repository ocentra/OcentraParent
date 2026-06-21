use ocentra_parent_agent_protocol::constants;

pub(crate) type ScreenMeshPayloadMode = ocentra_parent_agent_protocol::ScreenMeshPayloadMode;
pub(crate) type ScreenMeshClaimState = ocentra_parent_agent_protocol::ScreenMeshClaimState;
pub(crate) type ScreenMeshLeaseState = ocentra_parent_agent_protocol::ScreenMeshLeaseState;
pub(crate) type ScreenMeshProviderResultState =
    ocentra_parent_agent_protocol::ScreenMeshProviderResultState;
pub(crate) type ScreenMeshChildValidationState =
    ocentra_parent_agent_protocol::ScreenMeshChildValidationState;
pub(crate) type ScreenMeshPolicyState = ocentra_parent_agent_protocol::ScreenMeshPolicyState;
pub(crate) type ScreenMeshResultRejectionReason =
    ocentra_parent_agent_protocol::ScreenMeshResultRejectionReason;
pub(crate) type ScreenMeshCustodyBoundary =
    ocentra_parent_agent_protocol::ScreenMeshCustodyBoundary;

pub(crate) fn custody_label() -> &'static str {
    constants::value::LAN_PROVIDER_CUSTODY_LOCAL_NETWORK_AI_PROVIDER
}
