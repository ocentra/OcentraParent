use super::*;

pub(super) fn normalized_detection_refs(
    detections: &[NetworkAiDetectionResult],
) -> Result<Vec<String>, NetworkAiAuditReportError> {
    let mut refs = Vec::new();
    for detection in detections {
        reject_detection_claims(detection)?;
        let detection_ref = super::refs::normalize_ref(&detection.detection_ref)
            .ok_or(NetworkAiAuditReportError::EmptyDetectionRef)?;
        if refs.contains(&detection_ref) {
            return Err(NetworkAiAuditReportError::DuplicateDetectionRef);
        }
        refs.push(detection_ref);
    }
    Ok(refs)
}

fn reject_detection_claims(
    detection: &NetworkAiDetectionResult,
) -> Result<(), NetworkAiAuditReportError> {
    if detection.raw_pcap_available {
        return Err(NetworkAiAuditReportError::RawPcapInputRejected);
    }
    if detection.decrypted_payload_available {
        return Err(NetworkAiAuditReportError::DecryptedPayloadClaimRejected);
    }
    if detection.page_content_available {
        return Err(NetworkAiAuditReportError::PageContentClaimRejected);
    }
    if detection.exact_url_available {
        return Err(NetworkAiAuditReportError::ExactUrlClaimRejected);
    }
    if detection.policy_authority {
        return Err(NetworkAiAuditReportError::PolicyAuthorityClaimRejected);
    }
    if detection.adapter_authority {
        return Err(NetworkAiAuditReportError::AdapterAuthorityClaimRejected);
    }
    if detection.enforcement_command_published {
        return Err(NetworkAiAuditReportError::EnforcementCommandClaimRejected);
    }
    Ok(())
}
