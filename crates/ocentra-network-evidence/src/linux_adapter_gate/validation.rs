use super::{NetworkLinuxAdapterGateError, NetworkLinuxAdapterGateInput};

pub(super) fn reject_unsupported_claims(
    input: &NetworkLinuxAdapterGateInput,
) -> Result<(), NetworkLinuxAdapterGateError> {
    if input.exact_url_claimed {
        return Err(NetworkLinuxAdapterGateError::ExactUrlClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkLinuxAdapterGateError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkLinuxAdapterGateError::PageContentClaimRejected);
    }
    if input.generic_linux_support_claimed {
        return Err(NetworkLinuxAdapterGateError::GenericLinuxSupportClaimRejected);
    }
    if input.live_adapter_install_claimed {
        return Err(NetworkLinuxAdapterGateError::LiveAdapterInstallClaimRejected);
    }
    if input.packet_filtering_claimed {
        return Err(NetworkLinuxAdapterGateError::PacketFilteringClaimRejected);
    }
    if input.kernel_hook_loaded_claimed {
        return Err(NetworkLinuxAdapterGateError::KernelHookLoadedClaimRejected);
    }
    if input.tun_interface_mutation_claimed {
        return Err(NetworkLinuxAdapterGateError::TunInterfaceMutationClaimRejected);
    }
    if input.service_manager_install_claimed {
        return Err(NetworkLinuxAdapterGateError::ServiceManagerInstallClaimRejected);
    }
    Ok(())
}

pub(super) fn reject_policy_mapping_authority(
    input: &NetworkLinuxAdapterGateInput,
) -> Result<(), NetworkLinuxAdapterGateError> {
    if input.policy_mapping.adapter_action_authorized
        || input.policy_mapping.enforcement_command_authorized
    {
        return Err(NetworkLinuxAdapterGateError::PolicyMappingAuthorityRejected);
    }
    Ok(())
}
