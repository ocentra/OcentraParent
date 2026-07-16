use super::{NetworkLiveCaptureExecutionError, NetworkLiveCaptureExecutionInput};

pub(super) fn validate_input(
    input: &NetworkLiveCaptureExecutionInput,
) -> Result<(), NetworkLiveCaptureExecutionError> {
    if input.decrypted_payload_claimed {
        return Err(NetworkLiveCaptureExecutionError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkLiveCaptureExecutionError::PageContentClaimRejected);
    }
    if input.private_message_claimed {
        return Err(NetworkLiveCaptureExecutionError::PrivateMessageClaimRejected);
    }
    if input.search_query_claimed {
        return Err(NetworkLiveCaptureExecutionError::SearchQueryClaimRejected);
    }
    if input.policy_authority_claimed {
        return Err(NetworkLiveCaptureExecutionError::PolicyAuthorityClaimRejected);
    }
    if input.adapter_authority_claimed {
        return Err(NetworkLiveCaptureExecutionError::AdapterAuthorityClaimRejected);
    }
    if input.host_filtering_claimed {
        return Err(NetworkLiveCaptureExecutionError::HostFilteringClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkLiveCaptureExecutionError::EnforcementCommandClaimRejected);
    }
    Ok(())
}
