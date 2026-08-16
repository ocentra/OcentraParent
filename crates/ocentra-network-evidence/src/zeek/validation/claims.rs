use super::{NetworkZeekAnalyzerError, NetworkZeekAnalyzerInput};

pub(super) fn reject_unsupported_claims(
    input: &NetworkZeekAnalyzerInput,
) -> Result<(), NetworkZeekAnalyzerError> {
    if input.exact_url_claimed {
        return Err(NetworkZeekAnalyzerError::ExactUrlClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkZeekAnalyzerError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkZeekAnalyzerError::PageContentClaimRejected);
    }
    if input.signature_alert_claimed {
        return Err(NetworkZeekAnalyzerError::SignatureAlertClaimRejected);
    }
    if input.live_analyzer_invocation_claimed {
        return Err(NetworkZeekAnalyzerError::LiveAnalyzerInvocationClaimRejected);
    }
    if input.policy_authority_claimed {
        return Err(NetworkZeekAnalyzerError::PolicyAuthorityClaimRejected);
    }
    if input.adapter_authority_claimed {
        return Err(NetworkZeekAnalyzerError::AdapterAuthorityClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkZeekAnalyzerError::EnforcementCommandClaimRejected);
    }
    Ok(())
}
