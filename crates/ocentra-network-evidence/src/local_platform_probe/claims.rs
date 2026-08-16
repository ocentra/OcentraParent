use crate::NetworkAdapterCapabilityStatusProof;

use super::{NetworkLocalPlatformProbeError, NetworkLocalPlatformProbeUnsupportedClaims};

pub(super) fn reject_input_claims(
    claims: &NetworkLocalPlatformProbeUnsupportedClaims,
) -> Result<(), NetworkLocalPlatformProbeError> {
    [
        (
            claims.exact_url_claimed,
            NetworkLocalPlatformProbeError::ExactUrlClaimRejected,
        ),
        (
            claims.decrypted_payload_claimed,
            NetworkLocalPlatformProbeError::DecryptedPayloadClaimRejected,
        ),
        (
            claims.page_content_claimed,
            NetworkLocalPlatformProbeError::PageContentClaimRejected,
        ),
        (
            claims.live_adapter_execution_claimed,
            NetworkLocalPlatformProbeError::LiveAdapterExecutionClaimRejected,
        ),
        (
            claims.enforcement_command_claimed,
            NetworkLocalPlatformProbeError::EnforcementCommandClaimRejected,
        ),
        (
            claims.ui_policy_authority_claimed,
            NetworkLocalPlatformProbeError::UiPolicyAuthorityClaimRejected,
        ),
        (
            claims.production_platform_support_claimed,
            NetworkLocalPlatformProbeError::ProductionPlatformSupportClaimRejected,
        ),
    ]
    .into_iter()
    .find_map(|(claimed, error)| claimed.then_some(error))
    .map_or(Ok(()), Err)
}

pub(super) fn reject_status_claims(
    status: &NetworkAdapterCapabilityStatusProof,
) -> Result<(), NetworkLocalPlatformProbeError> {
    [
        (
            !status.no_live_adapter_execution_claimed,
            NetworkLocalPlatformProbeError::AdapterStatusClaimsLiveExecution,
        ),
        (
            !status.no_enforcement_commands_published,
            NetworkLocalPlatformProbeError::AdapterStatusPublishesEnforcementCommand,
        ),
        (
            !status.ui_has_no_policy_authority,
            NetworkLocalPlatformProbeError::AdapterStatusAllowsUiPolicyAuthority,
        ),
    ]
    .into_iter()
    .find_map(|(claimed, error)| claimed.then_some(error))
    .map_or(Ok(()), Err)
}
