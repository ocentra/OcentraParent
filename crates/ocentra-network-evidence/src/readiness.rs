use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkReadinessState {
    InternalProofReady,
    ProductionBlockedPendingExternalSignoff,
    ProductionReadyWithExternalSignoff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkReadinessGate {
    SecurityThreatModel,
    PrivacyAndCompliance,
    RetentionDeleteExport,
    KeyAndSecretHandling,
    ProvenanceAndRollback,
    DeploymentRollback,
    SupportAndTraining,
    StagedRollout,
    KnownGapSignoff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkReadinessFindingCode {
    ExternalAuditOrPenTestMissing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRetentionReadinessProof {
    pub encryption_at_rest_ref: String,
    pub quota_rotation_ref: String,
    pub retention_policy_ref: String,
    pub delete_export_ref: String,
    pub custody_ref: String,
    pub private_family_traffic_exclusion_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkHardeningReadinessProof {
    pub key_rotation_ref: String,
    pub secret_handling_ref: String,
    pub rule_set_provenance_ref: String,
    pub rule_set_rollback_ref: String,
    pub ai_model_version_promotion_ref: String,
    pub ai_model_rollback_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkSupportReadinessProof {
    pub parent_guide_ref: String,
    pub user_guide_ref: String,
    pub faq_ref: String,
    pub support_playbook_ref: String,
    pub staff_training_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRolloutReadinessProof {
    pub deployment_runbook_ref: String,
    pub rollback_runbook_ref: String,
    pub staged_rollout_plan_ref: String,
    pub monitoring_ref: String,
    pub incident_response_ref: String,
    pub known_gap_signoff_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkExternalSignoffReadinessProof {
    pub signoff_ref: String,
    pub artifact_ref: String,
    pub artifact_digest_ref: String,
    pub scope_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkReadinessProofInput {
    pub readiness_report_ref: String,
    pub threat_model_ref: String,
    pub privacy_review_ref: String,
    pub compliance_review_ref: String,
    pub retention: NetworkRetentionReadinessProof,
    pub hardening: NetworkHardeningReadinessProof,
    pub support: NetworkSupportReadinessProof,
    pub rollout: NetworkRolloutReadinessProof,
    pub external_audit_or_pen_test: Option<NetworkExternalSignoffReadinessProof>,
    pub production_rollout_claimed: bool,
    pub default_remote_upload_claimed: bool,
    pub raw_pcap_without_custody_claimed: bool,
    pub exact_url_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub private_message_claimed: bool,
    pub search_query_claimed: bool,
    pub policy_authority_claimed: bool,
    pub adapter_authority_claimed: bool,
    pub enforcement_command_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkReadinessProof {
    pub readiness_report_ref: String,
    pub readiness_state: NetworkReadinessState,
    pub gates: Vec<NetworkReadinessGate>,
    pub finding_codes: Vec<NetworkReadinessFindingCode>,
    pub threat_model_ref: String,
    pub privacy_review_ref: String,
    pub compliance_review_ref: String,
    pub retention_refs: Vec<String>,
    pub hardening_refs: Vec<String>,
    pub support_refs: Vec<String>,
    pub rollout_refs: Vec<String>,
    pub external_audit_or_pen_test_refs: Vec<String>,
    pub production_rollout_ready: bool,
    pub default_remote_upload_enabled: bool,
    pub raw_pcap_without_custody_available: bool,
    pub exact_url_available: bool,
    pub decrypted_payload_available: bool,
    pub page_content_available: bool,
    pub private_message_available: bool,
    pub search_query_available: bool,
    pub policy_authority: bool,
    pub adapter_authority: bool,
    pub enforcement_commands_published: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkReadinessProofError {
    EmptyReadinessReportRef,
    EmptyThreatModelRef,
    EmptyPrivacyReviewRef,
    EmptyComplianceReviewRef,
    EmptyRetentionRef,
    EmptyHardeningRef,
    EmptySupportRef,
    EmptyRolloutRef,
    EmptyExternalAuditOrPenTestProofRef,
    DefaultRemoteUploadClaimRejected,
    RawPcapWithoutCustodyClaimRejected,
    ExactUrlClaimRejected,
    DecryptedPayloadClaimRejected,
    PageContentClaimRejected,
    PrivateMessageClaimRejected,
    SearchQueryClaimRejected,
    PolicyAuthorityClaimRejected,
    AdapterAuthorityClaimRejected,
    EnforcementCommandClaimRejected,
}

pub fn evaluate_network_readiness_proof(
    input: NetworkReadinessProofInput,
) -> Result<NetworkReadinessProof, NetworkReadinessProofError> {
    validate_input(&input)?;

    let external_signoff_refs: Vec<String> = input
        .external_audit_or_pen_test
        .as_ref()
        .map(|proof| {
            external_signoff_ref_slice(proof)
                .into_iter()
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default();
    let production_rollout_ready =
        input.production_rollout_claimed && !external_signoff_refs.is_empty();
    let readiness_state = if production_rollout_ready {
        NetworkReadinessState::ProductionReadyWithExternalSignoff
    } else if input.production_rollout_claimed {
        NetworkReadinessState::ProductionBlockedPendingExternalSignoff
    } else {
        NetworkReadinessState::InternalProofReady
    };
    let finding_codes = if input.production_rollout_claimed && external_signoff_refs.is_empty() {
        vec![NetworkReadinessFindingCode::ExternalAuditOrPenTestMissing]
    } else {
        Vec::new()
    };

    Ok(NetworkReadinessProof {
        readiness_report_ref: input.readiness_report_ref,
        readiness_state,
        gates: readiness_gates(),
        finding_codes,
        threat_model_ref: input.threat_model_ref,
        privacy_review_ref: input.privacy_review_ref,
        compliance_review_ref: input.compliance_review_ref,
        retention_refs: retention_refs(input.retention),
        hardening_refs: hardening_refs(input.hardening),
        support_refs: support_refs(input.support),
        rollout_refs: rollout_refs(input.rollout),
        external_audit_or_pen_test_refs: external_signoff_refs,
        production_rollout_ready,
        default_remote_upload_enabled: false,
        raw_pcap_without_custody_available: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
        private_message_available: false,
        search_query_available: false,
        policy_authority: false,
        adapter_authority: false,
        enforcement_commands_published: 0,
    })
}

fn validate_input(input: &NetworkReadinessProofInput) -> Result<(), NetworkReadinessProofError> {
    if input.readiness_report_ref.trim().is_empty() {
        return Err(NetworkReadinessProofError::EmptyReadinessReportRef);
    }
    if input.threat_model_ref.trim().is_empty() {
        return Err(NetworkReadinessProofError::EmptyThreatModelRef);
    }
    if input.privacy_review_ref.trim().is_empty() {
        return Err(NetworkReadinessProofError::EmptyPrivacyReviewRef);
    }
    if input.compliance_review_ref.trim().is_empty() {
        return Err(NetworkReadinessProofError::EmptyComplianceReviewRef);
    }
    validate_retention(&input.retention)?;
    validate_hardening(&input.hardening)?;
    validate_support(&input.support)?;
    validate_rollout(&input.rollout)?;
    validate_external_signoff(input.external_audit_or_pen_test.as_ref())?;
    validate_claims(input)
}

fn validate_claims(input: &NetworkReadinessProofInput) -> Result<(), NetworkReadinessProofError> {
    if input.default_remote_upload_claimed {
        return Err(NetworkReadinessProofError::DefaultRemoteUploadClaimRejected);
    }
    if input.raw_pcap_without_custody_claimed {
        return Err(NetworkReadinessProofError::RawPcapWithoutCustodyClaimRejected);
    }
    if input.exact_url_claimed {
        return Err(NetworkReadinessProofError::ExactUrlClaimRejected);
    }
    if input.decrypted_payload_claimed {
        return Err(NetworkReadinessProofError::DecryptedPayloadClaimRejected);
    }
    if input.page_content_claimed {
        return Err(NetworkReadinessProofError::PageContentClaimRejected);
    }
    if input.private_message_claimed {
        return Err(NetworkReadinessProofError::PrivateMessageClaimRejected);
    }
    if input.search_query_claimed {
        return Err(NetworkReadinessProofError::SearchQueryClaimRejected);
    }
    if input.policy_authority_claimed {
        return Err(NetworkReadinessProofError::PolicyAuthorityClaimRejected);
    }
    if input.adapter_authority_claimed {
        return Err(NetworkReadinessProofError::AdapterAuthorityClaimRejected);
    }
    if input.enforcement_command_claimed {
        return Err(NetworkReadinessProofError::EnforcementCommandClaimRejected);
    }
    Ok(())
}

fn validate_retention(
    proof: &NetworkRetentionReadinessProof,
) -> Result<(), NetworkReadinessProofError> {
    for value in retention_ref_slice(proof) {
        if value.trim().is_empty() {
            return Err(NetworkReadinessProofError::EmptyRetentionRef);
        }
    }
    Ok(())
}

fn validate_hardening(
    proof: &NetworkHardeningReadinessProof,
) -> Result<(), NetworkReadinessProofError> {
    for value in hardening_ref_slice(proof) {
        if value.trim().is_empty() {
            return Err(NetworkReadinessProofError::EmptyHardeningRef);
        }
    }
    Ok(())
}

fn validate_support(
    proof: &NetworkSupportReadinessProof,
) -> Result<(), NetworkReadinessProofError> {
    for value in support_ref_slice(proof) {
        if value.trim().is_empty() {
            return Err(NetworkReadinessProofError::EmptySupportRef);
        }
    }
    Ok(())
}

fn validate_rollout(
    proof: &NetworkRolloutReadinessProof,
) -> Result<(), NetworkReadinessProofError> {
    for value in rollout_ref_slice(proof) {
        if value.trim().is_empty() {
            return Err(NetworkReadinessProofError::EmptyRolloutRef);
        }
    }
    Ok(())
}

fn validate_external_signoff(
    proof: Option<&NetworkExternalSignoffReadinessProof>,
) -> Result<(), NetworkReadinessProofError> {
    if let Some(proof) = proof {
        for value in external_signoff_ref_slice(proof) {
            if value.trim().is_empty() {
                return Err(NetworkReadinessProofError::EmptyExternalAuditOrPenTestProofRef);
            }
        }
    }
    Ok(())
}

fn readiness_gates() -> Vec<NetworkReadinessGate> {
    vec![
        NetworkReadinessGate::SecurityThreatModel,
        NetworkReadinessGate::PrivacyAndCompliance,
        NetworkReadinessGate::RetentionDeleteExport,
        NetworkReadinessGate::KeyAndSecretHandling,
        NetworkReadinessGate::ProvenanceAndRollback,
        NetworkReadinessGate::DeploymentRollback,
        NetworkReadinessGate::SupportAndTraining,
        NetworkReadinessGate::StagedRollout,
        NetworkReadinessGate::KnownGapSignoff,
    ]
}

fn retention_refs(proof: NetworkRetentionReadinessProof) -> Vec<String> {
    vec![
        proof.encryption_at_rest_ref,
        proof.quota_rotation_ref,
        proof.retention_policy_ref,
        proof.delete_export_ref,
        proof.custody_ref,
        proof.private_family_traffic_exclusion_ref,
    ]
}

fn hardening_refs(proof: NetworkHardeningReadinessProof) -> Vec<String> {
    vec![
        proof.key_rotation_ref,
        proof.secret_handling_ref,
        proof.rule_set_provenance_ref,
        proof.rule_set_rollback_ref,
        proof.ai_model_version_promotion_ref,
        proof.ai_model_rollback_ref,
    ]
}

fn support_refs(proof: NetworkSupportReadinessProof) -> Vec<String> {
    vec![
        proof.parent_guide_ref,
        proof.user_guide_ref,
        proof.faq_ref,
        proof.support_playbook_ref,
        proof.staff_training_ref,
    ]
}

fn rollout_refs(proof: NetworkRolloutReadinessProof) -> Vec<String> {
    vec![
        proof.deployment_runbook_ref,
        proof.rollback_runbook_ref,
        proof.staged_rollout_plan_ref,
        proof.monitoring_ref,
        proof.incident_response_ref,
        proof.known_gap_signoff_ref,
    ]
}

fn retention_ref_slice(proof: &NetworkRetentionReadinessProof) -> [&str; 6] {
    [
        proof.encryption_at_rest_ref.as_str(),
        proof.quota_rotation_ref.as_str(),
        proof.retention_policy_ref.as_str(),
        proof.delete_export_ref.as_str(),
        proof.custody_ref.as_str(),
        proof.private_family_traffic_exclusion_ref.as_str(),
    ]
}

fn hardening_ref_slice(proof: &NetworkHardeningReadinessProof) -> [&str; 6] {
    [
        proof.key_rotation_ref.as_str(),
        proof.secret_handling_ref.as_str(),
        proof.rule_set_provenance_ref.as_str(),
        proof.rule_set_rollback_ref.as_str(),
        proof.ai_model_version_promotion_ref.as_str(),
        proof.ai_model_rollback_ref.as_str(),
    ]
}

fn support_ref_slice(proof: &NetworkSupportReadinessProof) -> [&str; 5] {
    [
        proof.parent_guide_ref.as_str(),
        proof.user_guide_ref.as_str(),
        proof.faq_ref.as_str(),
        proof.support_playbook_ref.as_str(),
        proof.staff_training_ref.as_str(),
    ]
}

fn rollout_ref_slice(proof: &NetworkRolloutReadinessProof) -> [&str; 6] {
    [
        proof.deployment_runbook_ref.as_str(),
        proof.rollback_runbook_ref.as_str(),
        proof.staged_rollout_plan_ref.as_str(),
        proof.monitoring_ref.as_str(),
        proof.incident_response_ref.as_str(),
        proof.known_gap_signoff_ref.as_str(),
    ]
}

fn external_signoff_ref_slice(proof: &NetworkExternalSignoffReadinessProof) -> [&str; 4] {
    [
        proof.signoff_ref.as_str(),
        proof.artifact_ref.as_str(),
        proof.artifact_digest_ref.as_str(),
        proof.scope_ref.as_str(),
    ]
}
