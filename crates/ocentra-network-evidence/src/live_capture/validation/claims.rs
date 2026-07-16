use super::{NetworkLiveCaptureProofError, NetworkLiveCaptureProofInput};

pub(super) fn validate_input(
    input: &NetworkLiveCaptureProofInput,
) -> Result<(), NetworkLiveCaptureProofError> {
    if input.live_capture_execution_claimed {
        return Err(NetworkLiveCaptureProofError::LiveCaptureExecutionClaimRejected);
    }
    if input.unbounded_capture_claimed {
        return Err(NetworkLiveCaptureProofError::UnboundedCaptureClaimRejected);
    }
    if input.raw_pcap_without_custody_claimed {
        return Err(NetworkLiveCaptureProofError::RawPcapWithoutCustodyClaimRejected);
    }
    if input.exact_url_claimed {
        return Err(NetworkLiveCaptureProofError::ExactUrlClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkLiveCaptureProofError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkLiveCaptureProofError::PageContentClaimRejected);
    }
    if input.private_message_claimed {
        return Err(NetworkLiveCaptureProofError::PrivateMessageClaimRejected);
    }
    if input.search_query_claimed {
        return Err(NetworkLiveCaptureProofError::SearchQueryClaimRejected);
    }
    if input.policy_authority_claimed {
        return Err(NetworkLiveCaptureProofError::PolicyAuthorityClaimRejected);
    }
    if input.adapter_authority_claimed {
        return Err(NetworkLiveCaptureProofError::AdapterAuthorityClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkLiveCaptureProofError::EnforcementCommandClaimRejected);
    }
    Ok(())
}
