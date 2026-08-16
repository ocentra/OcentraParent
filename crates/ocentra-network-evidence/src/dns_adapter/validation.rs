use super::{NetworkDnsAdapterProofError, NetworkDnsAdapterProofInput};

pub(super) fn reject_unsupported_claims(
    input: &NetworkDnsAdapterProofInput,
) -> Result<(), NetworkDnsAdapterProofError> {
    if input.exact_url_claimed {
        return Err(NetworkDnsAdapterProofError::ExactUrlClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkDnsAdapterProofError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkDnsAdapterProofError::PageContentClaimRejected);
    }
    Ok(())
}

pub(super) fn reject_policy_mapping_authority(
    input: &NetworkDnsAdapterProofInput,
) -> Result<(), NetworkDnsAdapterProofError> {
    if input.policy_mapping.adapter_action_authorized
        || input.policy_mapping.enforcement_command_authorized
    {
        return Err(NetworkDnsAdapterProofError::PolicyMappingAuthorityRejected);
    }

    Ok(())
}
