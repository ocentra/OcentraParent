use super::super::*;

pub(super) fn validate_claims(
    input: &NetworkRiskBudgetThresholdInput,
) -> Result<(), NetworkRiskBudgetThresholdError> {
    if input.raw_pcap_claimed {
        return Err(NetworkRiskBudgetThresholdError::RawPcapClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkRiskBudgetThresholdError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkRiskBudgetThresholdError::PageContentClaimRejected);
    }
    if input.exact_url_claimed {
        return Err(NetworkRiskBudgetThresholdError::ExactUrlClaimRejected);
    }
    if input.private_message_claimed {
        return Err(NetworkRiskBudgetThresholdError::PrivateMessageClaimRejected);
    }
    if input.search_query_claimed {
        return Err(NetworkRiskBudgetThresholdError::SearchQueryClaimRejected);
    }
    if input.policy_authority_claimed {
        return Err(NetworkRiskBudgetThresholdError::PolicyAuthorityClaimRejected);
    }
    if input.adapter_authority_claimed {
        return Err(NetworkRiskBudgetThresholdError::AdapterAuthorityClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkRiskBudgetThresholdError::EnforcementCommandClaimRejected);
    }
    if input.extra_privilege_grant_claimed {
        return Err(NetworkRiskBudgetThresholdError::ExtraPrivilegeGrantRejected);
    }
    if input.allowance_grant_claimed {
        return Err(NetworkRiskBudgetThresholdError::AllowanceGrantRejected);
    }
    if input.time_grant_claimed {
        return Err(NetworkRiskBudgetThresholdError::TimeGrantRejected);
    }
    Ok(())
}
