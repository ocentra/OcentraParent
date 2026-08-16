use super::*;

pub(super) fn reject_global_claims(
    input: &NetworkAiAuditReportInput,
) -> Result<(), NetworkAiAuditReportError> {
    if input.remote_ai_claimed {
        return Err(NetworkAiAuditReportError::RemoteAiClaimRejected);
    }
    if input.raw_pcap_input_claimed {
        return Err(NetworkAiAuditReportError::RawPcapInputRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkAiAuditReportError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkAiAuditReportError::PageContentClaimRejected);
    }
    if input.exact_url_claimed {
        return Err(NetworkAiAuditReportError::ExactUrlClaimRejected);
    }
    if input.private_message_claimed {
        return Err(NetworkAiAuditReportError::PrivateMessageClaimRejected);
    }
    if input.search_query_claimed {
        return Err(NetworkAiAuditReportError::SearchQueryClaimRejected);
    }
    if input.policy_authority_claimed {
        return Err(NetworkAiAuditReportError::PolicyAuthorityClaimRejected);
    }
    if input.adapter_authority_claimed {
        return Err(NetworkAiAuditReportError::AdapterAuthorityClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkAiAuditReportError::EnforcementCommandClaimRejected);
    }
    Ok(())
}
