use super::{
    NetworkHardeningReadinessProof, NetworkReadinessProofError, NetworkReadinessProofInput,
    NetworkRetentionReadinessProof, NetworkRolloutReadinessProof, NetworkSupportReadinessProof,
};

pub(super) fn validate_input(
    input: &NetworkReadinessProofInput,
) -> Result<(), NetworkReadinessProofError> {
    ensure_non_empty(
        &input.readiness_report_ref,
        NetworkReadinessProofError::EmptyReadinessReportRef,
    )?;
    ensure_non_empty(
        &input.threat_model_ref,
        NetworkReadinessProofError::EmptyThreatModelRef,
    )?;
    ensure_non_empty(
        &input.privacy_review_ref,
        NetworkReadinessProofError::EmptyPrivacyReviewRef,
    )?;
    ensure_non_empty(
        &input.compliance_review_ref,
        NetworkReadinessProofError::EmptyComplianceReviewRef,
    )?;
    validate_retention(&input.retention)?;
    validate_hardening(&input.hardening)?;
    validate_support(&input.support)?;
    validate_rollout(&input.rollout)?;
    if input
        .external_audit_or_pen_test_ref
        .as_ref()
        .is_some_and(|value| value.trim().is_empty())
    {
        return Err(NetworkReadinessProofError::EmptyExternalAuditOrPenTestRef);
    }
    validate_claims(input)
}

fn validate_claims(input: &NetworkReadinessProofInput) -> Result<(), NetworkReadinessProofError> {
    [
        (
            input.default_remote_upload_claimed,
            NetworkReadinessProofError::DefaultRemoteUploadClaimRejected,
        ),
        (
            input.raw_pcap_without_custody_claimed,
            NetworkReadinessProofError::RawPcapWithoutCustodyClaimRejected,
        ),
        (
            input.exact_url_claimed,
            NetworkReadinessProofError::ExactUrlClaimRejected,
        ),
        (
            input.decrypted_payload_claimed,
            NetworkReadinessProofError::DecryptedPayloadClaimRejected,
        ),
        (
            input.page_content_claimed,
            NetworkReadinessProofError::PageContentClaimRejected,
        ),
        (
            input.private_message_claimed,
            NetworkReadinessProofError::PrivateMessageClaimRejected,
        ),
        (
            input.search_query_claimed,
            NetworkReadinessProofError::SearchQueryClaimRejected,
        ),
        (
            input.policy_authority_claimed,
            NetworkReadinessProofError::PolicyAuthorityClaimRejected,
        ),
        (
            input.adapter_authority_claimed,
            NetworkReadinessProofError::AdapterAuthorityClaimRejected,
        ),
        (
            input.enforcement_command_claimed,
            NetworkReadinessProofError::EnforcementCommandClaimRejected,
        ),
    ]
    .into_iter()
    .find_map(|(claimed, error)| claimed.then_some(error))
    .map_or(Ok(()), Err)
}

fn validate_retention(
    proof: &NetworkRetentionReadinessProof,
) -> Result<(), NetworkReadinessProofError> {
    validate_ref_group(
        &[
            &proof.encryption_at_rest_ref,
            &proof.quota_rotation_ref,
            &proof.retention_policy_ref,
            &proof.delete_export_ref,
            &proof.custody_ref,
            &proof.private_family_traffic_exclusion_ref,
        ],
        NetworkReadinessProofError::EmptyRetentionRef,
    )
}

fn validate_hardening(
    proof: &NetworkHardeningReadinessProof,
) -> Result<(), NetworkReadinessProofError> {
    validate_ref_group(
        &[
            &proof.key_rotation_ref,
            &proof.secret_handling_ref,
            &proof.rule_set_provenance_ref,
            &proof.rule_set_rollback_ref,
            &proof.ai_model_version_promotion_ref,
            &proof.ai_model_rollback_ref,
        ],
        NetworkReadinessProofError::EmptyHardeningRef,
    )
}

fn validate_support(
    proof: &NetworkSupportReadinessProof,
) -> Result<(), NetworkReadinessProofError> {
    validate_ref_group(
        &[
            &proof.parent_guide_ref,
            &proof.user_guide_ref,
            &proof.faq_ref,
            &proof.support_playbook_ref,
            &proof.staff_training_ref,
        ],
        NetworkReadinessProofError::EmptySupportRef,
    )
}

fn validate_rollout(
    proof: &NetworkRolloutReadinessProof,
) -> Result<(), NetworkReadinessProofError> {
    validate_ref_group(
        &[
            &proof.deployment_runbook_ref,
            &proof.rollback_runbook_ref,
            &proof.staged_rollout_plan_ref,
            &proof.monitoring_ref,
            &proof.incident_response_ref,
            &proof.known_gap_signoff_ref,
        ],
        NetworkReadinessProofError::EmptyRolloutRef,
    )
}

fn validate_ref_group(
    values: &[&String],
    error: NetworkReadinessProofError,
) -> Result<(), NetworkReadinessProofError> {
    values
        .iter()
        .find(|value| value.trim().is_empty())
        .map_or(Ok(()), |_| Err(error))
}

fn ensure_non_empty(
    value: &str,
    error: NetworkReadinessProofError,
) -> Result<(), NetworkReadinessProofError> {
    if value.trim().is_empty() {
        Err(error)
    } else {
        Ok(())
    }
}
