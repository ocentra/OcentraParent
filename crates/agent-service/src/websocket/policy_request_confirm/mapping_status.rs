use ocentra_parent_agent_protocol::activity::policy_preview::PolicyAssistantConfirmationState as CorePolicyAssistantConfirmationState;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyAssistantConfirmationState as ProtocolPolicyAssistantConfirmationState;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestOrigin as CorePolicyRequestOrigin;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestOrigin as ProtocolPolicyRequestOrigin;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus as CorePolicyRequestStatus;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicyRequestStatus as ProtocolPolicyRequestStatus;

pub(super) fn map_request_origin(origin: ProtocolPolicyRequestOrigin) -> CorePolicyRequestOrigin {
    origin
}

pub(super) fn map_confirmation_state(
    state: ProtocolPolicyAssistantConfirmationState,
) -> CorePolicyAssistantConfirmationState {
    state
}

pub(super) fn map_request_status(status: ProtocolPolicyRequestStatus) -> CorePolicyRequestStatus {
    if matches!(status, ProtocolPolicyRequestStatus::ReplayRejected) {
        CorePolicyRequestStatus::PreviewOnly
    } else {
        status
    }
}

pub(super) fn map_protocol_request_status(
    status: CorePolicyRequestStatus,
) -> ProtocolPolicyRequestStatus {
    status
}

pub(super) fn map_protocol_confirmation_state(
    state: CorePolicyAssistantConfirmationState,
) -> ProtocolPolicyAssistantConfirmationState {
    state
}
