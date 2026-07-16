use super::proofs::retention_delete_export;
use super::{
    NetworkEndToEndPipelineError, NetworkEndToEndPipelineRefs, NetworkEndToEndUnsupportedClaims,
};

pub(super) fn validate_refs(
    refs: &NetworkEndToEndPipelineRefs,
) -> Result<(), NetworkEndToEndPipelineError> {
    required_ref(
        &refs.trigger_ref,
        NetworkEndToEndPipelineError::EmptyTriggerRef,
    )?;
    required_ref(
        &refs.capture_ref,
        NetworkEndToEndPipelineError::EmptyCaptureRef,
    )?;
    required_ref(
        &refs.ingest_ref,
        NetworkEndToEndPipelineError::EmptyIngestRef,
    )?;
    required_ref(
        &refs.typed_event_ref,
        NetworkEndToEndPipelineError::EmptyTypedEventRef,
    )?;
    required_ref(
        &refs.action_result_ref,
        NetworkEndToEndPipelineError::EmptyActionResultRef,
    )?;
    normalized_refs(
        &refs.summary_refs,
        NetworkEndToEndPipelineError::EmptySummaryRef,
    )?;
    normalized_refs(
        &refs.analyzer_alert_refs,
        NetworkEndToEndPipelineError::EmptyAnalyzerAlertRef,
    )?;
    retention_delete_export(refs, &[])?;
    Ok(())
}

pub(super) fn reject_unsupported_claims(
    claims: NetworkEndToEndUnsupportedClaims,
) -> Result<(), NetworkEndToEndPipelineError> {
    if claims.raw_network_payload_claimed {
        return Err(NetworkEndToEndPipelineError::RawNetworkPayloadRejected);
    }
    if claims.decrypted_payload_claimed {
        return Err(NetworkEndToEndPipelineError::DecryptedPayloadRejected);
    }
    if claims.page_content_claimed {
        return Err(NetworkEndToEndPipelineError::PageContentRejected);
    }
    if claims.exact_url_claimed {
        return Err(NetworkEndToEndPipelineError::ExactUrlRejected);
    }
    if claims.ai_policy_authority_claimed {
        return Err(NetworkEndToEndPipelineError::AiPolicyAuthorityRejected);
    }
    if claims.ui_policy_authority_claimed {
        return Err(NetworkEndToEndPipelineError::UiPolicyAuthorityRejected);
    }
    if claims.network_adapter_authority_claimed {
        return Err(NetworkEndToEndPipelineError::NetworkAdapterAuthorityRejected);
    }
    if claims.enforcement_command_claimed {
        return Err(NetworkEndToEndPipelineError::EnforcementCommandRejected);
    }
    Ok(())
}

pub(super) fn normalized_refs(
    refs: &[String],
    empty_error: NetworkEndToEndPipelineError,
) -> Result<Vec<String>, NetworkEndToEndPipelineError> {
    let mut normalized = Vec::new();
    for value in refs {
        let ref_value = required_ref(value, empty_error.clone())?;
        if !normalized.contains(&ref_value) {
            normalized.push(ref_value);
        }
    }
    if normalized.is_empty() {
        return Err(empty_error);
    }
    Ok(normalized)
}

pub(super) fn required_ref(
    value: &str,
    error: NetworkEndToEndPipelineError,
) -> Result<String, NetworkEndToEndPipelineError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(error)
    } else {
        Ok(trimmed.to_owned())
    }
}
