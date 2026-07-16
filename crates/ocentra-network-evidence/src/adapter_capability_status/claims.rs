use super::*;

pub(super) fn reject_status_claims(
    input: &NetworkAdapterCapabilityStatusInput,
) -> Result<(), NetworkAdapterCapabilityStatusError> {
    if input.generic_platform_support_claimed {
        return Err(NetworkAdapterCapabilityStatusError::GenericPlatformSupportClaimRejected);
    }
    if input.live_adapter_execution_claimed {
        return Err(NetworkAdapterCapabilityStatusError::LiveAdapterExecutionClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkAdapterCapabilityStatusError::EnforcementCommandClaimRejected);
    }
    if input.ui_policy_authority_claimed {
        return Err(NetworkAdapterCapabilityStatusError::UiPolicyAuthorityClaimRejected);
    }
    if input.broader_platform_capability_ux_claimed {
        return Err(NetworkAdapterCapabilityStatusError::BroaderPlatformCapabilityUxClaimRejected);
    }
    Ok(())
}
