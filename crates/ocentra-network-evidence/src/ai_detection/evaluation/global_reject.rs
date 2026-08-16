use super::super::*;

pub(super) fn reject_global_claims(
    input: &NetworkAiDetectionEvaluationInput,
) -> Result<(), NetworkAiDetectionEvaluationError> {
    if input.model_execution_claimed {
        return Err(NetworkAiDetectionEvaluationError::ModelExecutionClaimRejected);
    }
    if input.remote_ai_claimed {
        return Err(NetworkAiDetectionEvaluationError::RemoteAiClaimRejected);
    }
    if input.raw_pcap_input_claimed {
        return Err(NetworkAiDetectionEvaluationError::RawPcapInputRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkAiDetectionEvaluationError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkAiDetectionEvaluationError::PageContentClaimRejected);
    }
    if input.exact_url_claimed {
        return Err(NetworkAiDetectionEvaluationError::ExactUrlClaimRejected);
    }
    if input.policy_authority_claimed {
        return Err(NetworkAiDetectionEvaluationError::PolicyAuthorityClaimRejected);
    }
    if input.adapter_authority_claimed {
        return Err(NetworkAiDetectionEvaluationError::AdapterAuthorityClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkAiDetectionEvaluationError::EnforcementCommandClaimRejected);
    }
    Ok(())
}
