use super::super::*;

pub(super) fn reject_case_claims(
    case: &NetworkAiDetectionFixtureCase,
) -> Result<(), NetworkAiDetectionEvaluationError> {
    if case.confidence_basis_points > 10_000 || case.baseline_confidence_basis_points > 10_000 {
        return Err(NetworkAiDetectionEvaluationError::BasisPointsOutOfRange);
    }
    if case.raw_pcap_input_claimed {
        return Err(NetworkAiDetectionEvaluationError::RawPcapInputRejected);
    }
    if case.decrypted_payload_claimed {
        return Err(NetworkAiDetectionEvaluationError::DecryptedPayloadClaimRejected);
    }
    if case.page_content_claimed {
        return Err(NetworkAiDetectionEvaluationError::PageContentClaimRejected);
    }
    if case.exact_url_claimed {
        return Err(NetworkAiDetectionEvaluationError::ExactUrlClaimRejected);
    }
    Ok(())
}
