mod refs;
mod validation;

use serde::{Deserialize, Serialize};

use self::{
    refs::{hardening_refs, readiness_gates, retention_refs, rollout_refs, support_refs},
    validation::validate_input,
};

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
pub struct NetworkReadinessProofInput {
    pub readiness_report_ref: String,
    pub threat_model_ref: String,
    pub privacy_review_ref: String,
    pub compliance_review_ref: String,
    pub retention: NetworkRetentionReadinessProof,
    pub hardening: NetworkHardeningReadinessProof,
    pub support: NetworkSupportReadinessProof,
    pub rollout: NetworkRolloutReadinessProof,
    pub external_audit_or_pen_test_ref: Option<String>,
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
    pub external_audit_or_pen_test_ref: Option<String>,
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
    EmptyExternalAuditOrPenTestRef,
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

    let external_signoff = input.external_audit_or_pen_test_ref.clone();
    let production_rollout_ready = input.production_rollout_claimed && external_signoff.is_some();
    let readiness_state = if production_rollout_ready {
        NetworkReadinessState::ProductionReadyWithExternalSignoff
    } else if input.production_rollout_claimed {
        NetworkReadinessState::ProductionBlockedPendingExternalSignoff
    } else {
        NetworkReadinessState::InternalProofReady
    };
    let finding_codes = if input.production_rollout_claimed && external_signoff.is_none() {
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
        external_audit_or_pen_test_ref: external_signoff,
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
