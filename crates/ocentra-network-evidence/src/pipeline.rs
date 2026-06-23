use serde::{Deserialize, Serialize};

use crate::{
    build_network_ai_audit_report, build_network_cross_slice_evidence_bundle,
    evaluate_network_ai_detection_fixtures, evaluate_network_risk_budget_threshold,
    map_network_evidence_grade_to_policy, plan_network_action_result_state,
    plan_network_dns_adapter_proof, plan_network_local_ai_queue,
    NetworkActionResultAdapterProofState, NetworkActionResultCapabilityState,
    NetworkActionResultError, NetworkActionResultInput, NetworkActionResultProof,
    NetworkActionResultRequestedAction, NetworkActionResultTargetKind, NetworkAiAuditReport,
    NetworkAiAuditReportError, NetworkAiAuditReportInput, NetworkAiDetectionEvaluationError,
    NetworkAiDetectionEvaluationInput, NetworkAiDetectionEvaluationProof,
    NetworkAiDetectionFixtureCase, NetworkAiDetectionInputKind, NetworkAiDetectionLabel,
    NetworkAiDetectionRiskLevel, NetworkCrossSliceEvidenceBundle,
    NetworkCrossSliceEvidenceBundleError, NetworkCrossSliceEvidenceBundleInput,
    NetworkCrossSliceEvidenceSource, NetworkDnsAdapterAction, NetworkDnsAdapterCapabilityState,
    NetworkDnsAdapterProof, NetworkDnsAdapterProofError, NetworkDnsAdapterProofInput,
    NetworkDnsAdapterProofState, NetworkEvidencePolicyAction, NetworkEvidencePolicyMapping,
    NetworkEvidencePolicyMappingError, NetworkEvidencePolicyMappingInput, NetworkLocalAiQueueError,
    NetworkLocalAiQueueInput, NetworkLocalAiQueuePlan, NetworkRiskBudgetAdapterProofState,
    NetworkRiskBudgetAgeBand, NetworkRiskBudgetEvaluation, NetworkRiskBudgetEvidenceTier,
    NetworkRiskBudgetHouseholdPolicy, NetworkRiskBudgetPriorEvent, NetworkRiskBudgetSignal,
    NetworkRiskBudgetThresholdError, NetworkRiskBudgetThresholdInput, NetworkRiskBudgetThresholds,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkEndToEndPipelineRefs {
    pub trigger_ref: String,
    pub capture_ref: String,
    pub ingest_ref: String,
    pub typed_event_ref: String,
    pub summary_refs: Vec<String>,
    pub analyzer_alert_refs: Vec<String>,
    pub queue_job_ref: String,
    pub queue_ref: String,
    pub model_runtime_ref: String,
    pub ai_detection_ref: String,
    pub ai_fixture_ref: String,
    pub ai_evaluation_run_ref: String,
    pub ai_fixture_set_ref: String,
    pub ai_model_card_ref: String,
    pub ai_model_version_ref: String,
    pub ai_baseline_ref: String,
    pub ai_audit_report_ref: String,
    pub ai_narrative_template_ref: String,
    pub policy_context_ref: String,
    pub policy_decision_ref: String,
    pub parent_rule_ref: String,
    pub risk_evaluation_ref: String,
    pub child_profile_ref: String,
    pub risk_budget_ref: String,
    pub cascade_ref: String,
    pub household_policy_ref: String,
    pub dns_adapter_plan_ref: String,
    pub action_result_ref: String,
    pub target_domain: String,
    pub adapter_authorization_ref: Option<String>,
    pub adapter_capability_proof_ref: Option<String>,
    pub apply_artifact_ref: Option<String>,
    pub result_artifact_ref: Option<String>,
    pub rollback_artifact_ref: Option<String>,
    pub audit_event_ref: String,
    pub portal_read_model_ref: String,
    pub retention_ref: String,
    pub deletion_ref: String,
    pub export_ref: String,
    pub tombstone_ref: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkEndToEndUnsupportedClaims {
    pub raw_network_payload_claimed: bool,
    pub decrypted_payload_claimed: bool,
    pub page_content_claimed: bool,
    pub exact_url_claimed: bool,
    pub ai_policy_authority_claimed: bool,
    pub ui_policy_authority_claimed: bool,
    pub network_adapter_authority_claimed: bool,
    pub enforcement_command_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkEndToEndPipelineInput {
    pub refs: NetworkEndToEndPipelineRefs,
    pub sources: Vec<NetworkCrossSliceEvidenceSource>,
    pub requested_policy_action: NetworkEvidencePolicyAction,
    pub adapter_capability_state: NetworkDnsAdapterCapabilityState,
    pub adapter_dry_run: bool,
    pub local_ai_enabled: bool,
    pub model_runtime_available: bool,
    pub queue_available: bool,
    pub unsupported_claims: NetworkEndToEndUnsupportedClaims,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRetentionDeleteExportProof {
    pub retention_ref: String,
    pub deletion_ref: String,
    pub export_ref: String,
    pub tombstone_ref: String,
    pub audit_event_ref: String,
    pub portal_read_model_ref: String,
    pub evidence_refs: Vec<String>,
    pub same_product_path: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkCaptureIngestProof {
    pub trigger_ref: String,
    pub capture_ref: String,
    pub ingest_ref: String,
    pub typed_event_ref: String,
    pub summary_refs: Vec<String>,
    pub evidence_refs: Vec<String>,
    pub audit_event_ref: String,
    pub same_product_path: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkEndToEndPipelineProof {
    pub trigger_ref: String,
    pub capture_ref: String,
    pub ingest_ref: String,
    pub typed_event_ref: String,
    pub capture_ingest: NetworkCaptureIngestProof,
    pub evidence_bundle: NetworkCrossSliceEvidenceBundle,
    pub local_ai_queue: NetworkLocalAiQueuePlan,
    pub ai_detection: NetworkAiDetectionEvaluationProof,
    pub ai_audit: NetworkAiAuditReport,
    pub risk_budget: NetworkRiskBudgetEvaluation,
    pub policy_mapping: NetworkEvidencePolicyMapping,
    pub adapter_proof: NetworkDnsAdapterProof,
    pub action_result: NetworkActionResultProof,
    pub retention_delete_export: NetworkRetentionDeleteExportProof,
    pub ai_advisory_only: bool,
    pub policy_is_action_authority: bool,
    pub ui_policy_authority: bool,
    pub network_adapter_authority: bool,
    pub adapter_action_executed: bool,
    pub enforcement_commands_published: usize,
    pub weak_or_unavailable_evidence_enforcement_blocked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkEndToEndPipelineError {
    EmptyTriggerRef,
    EmptyCaptureRef,
    EmptyIngestRef,
    EmptyTypedEventRef,
    EmptyActionResultRef,
    EmptySummaryRef,
    EmptyAnalyzerAlertRef,
    EmptyAuditEventRef,
    EmptyPortalReadModelRef,
    EmptyRetentionRef,
    EmptyDeletionRef,
    EmptyExportRef,
    EmptyTombstoneRef,
    RawNetworkPayloadRejected,
    DecryptedPayloadRejected,
    PageContentRejected,
    ExactUrlRejected,
    AiPolicyAuthorityRejected,
    UiPolicyAuthorityRejected,
    NetworkAdapterAuthorityRejected,
    EnforcementCommandRejected,
    Bundle(NetworkCrossSliceEvidenceBundleError),
    LocalAi(NetworkLocalAiQueueError),
    AiDetection(NetworkAiDetectionEvaluationError),
    AiAudit(NetworkAiAuditReportError),
    RiskBudget(NetworkRiskBudgetThresholdError),
    Policy(NetworkEvidencePolicyMappingError),
    DnsAdapter(NetworkDnsAdapterProofError),
    ActionResult(NetworkActionResultError),
}

struct NetworkEndToEndPipelineParts {
    capture_ingest: NetworkCaptureIngestProof,
    evidence_bundle: NetworkCrossSliceEvidenceBundle,
    local_ai_queue: NetworkLocalAiQueuePlan,
    ai_detection: NetworkAiDetectionEvaluationProof,
    ai_audit: NetworkAiAuditReport,
    risk_budget: NetworkRiskBudgetEvaluation,
    policy_mapping: NetworkEvidencePolicyMapping,
    adapter_proof: NetworkDnsAdapterProof,
    action_result: NetworkActionResultProof,
    retention_delete_export: NetworkRetentionDeleteExportProof,
}

pub fn prove_network_end_to_end_pipeline(
    input: NetworkEndToEndPipelineInput,
) -> Result<NetworkEndToEndPipelineProof, NetworkEndToEndPipelineError> {
    validate_refs(&input.refs)?;
    reject_unsupported_claims(input.unsupported_claims)?;
    let parts = build_pipeline_parts(&input)?;
    let NetworkEndToEndPipelineInput { refs, .. } = input;
    let enforcement_commands_published = parts.adapter_proof.enforcement_command_published as usize;
    let weak_or_unavailable_evidence_enforcement_blocked =
        !parts.adapter_proof.adapter_apply_authorized;
    let adapter_proof = parts.adapter_proof;

    Ok(NetworkEndToEndPipelineProof {
        trigger_ref: refs.trigger_ref,
        capture_ref: refs.capture_ref,
        ingest_ref: refs.ingest_ref,
        typed_event_ref: refs.typed_event_ref,
        capture_ingest: parts.capture_ingest,
        evidence_bundle: parts.evidence_bundle,
        local_ai_queue: parts.local_ai_queue,
        ai_detection: parts.ai_detection,
        ai_audit: parts.ai_audit,
        risk_budget: parts.risk_budget,
        policy_mapping: parts.policy_mapping,
        adapter_proof,
        action_result: parts.action_result,
        retention_delete_export: parts.retention_delete_export,
        ai_advisory_only: true,
        policy_is_action_authority: true,
        ui_policy_authority: false,
        network_adapter_authority: false,
        adapter_action_executed: false,
        enforcement_commands_published,
        weak_or_unavailable_evidence_enforcement_blocked,
    })
}

fn build_pipeline_parts(
    input: &NetworkEndToEndPipelineInput,
) -> Result<NetworkEndToEndPipelineParts, NetworkEndToEndPipelineError> {
    let summary_refs = normalized_refs(
        &input.refs.summary_refs,
        NetworkEndToEndPipelineError::EmptySummaryRef,
    )?;
    let bundle = build_network_cross_slice_evidence_bundle(NetworkCrossSliceEvidenceBundleInput {
        trigger_ref: input.refs.trigger_ref.clone(),
        sources: input.sources.clone(),
    })
    .map_err(NetworkEndToEndPipelineError::Bundle)?;
    let capture_ingest = capture_ingest_proof(&input.refs, &summary_refs, &bundle.evidence_refs)?;
    let ai_detection =
        evaluate_network_ai_detection_fixtures(&ai_detection_input(&input.refs, &bundle)?)
            .map_err(NetworkEndToEndPipelineError::AiDetection)?;
    let ai_audit = build_network_ai_audit_report(&ai_audit_input(&input.refs, &ai_detection))
        .map_err(NetworkEndToEndPipelineError::AiAudit)?;
    let risk_budget =
        evaluate_network_risk_budget_threshold(risk_budget_input(&input.refs, &ai_audit))
            .map_err(NetworkEndToEndPipelineError::RiskBudget)?;
    let policy_mapping = map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
        policy_decision_ref: input.refs.policy_decision_ref.clone(),
        parent_rule_ref: input.refs.parent_rule_ref.clone(),
        evidence_refs: bundle.evidence_refs.clone(),
        local_ai_result_ref: Some(input.refs.ai_detection_ref.clone()),
        evidence_grade: bundle.evidence_grade,
        requested_action: input.requested_policy_action,
        adapter_capability_proof_ref: input.refs.adapter_capability_proof_ref.clone(),
    })
    .map_err(NetworkEndToEndPipelineError::Policy)?;
    let adapter_proof =
        plan_network_dns_adapter_proof(dns_adapter_input(input, policy_mapping.clone()))
            .map_err(NetworkEndToEndPipelineError::DnsAdapter)?;
    let action_result = plan_network_action_result_state(action_result_input(
        input,
        policy_mapping.clone(),
        &adapter_proof,
    ))
    .map_err(NetworkEndToEndPipelineError::ActionResult)?;
    let retention_delete_export = retention_delete_export(&input.refs, &bundle.evidence_refs)?;
    let local_ai_queue = plan_network_local_ai_queue(NetworkLocalAiQueueInput {
        queue_job_ref: input.refs.queue_job_ref.clone(),
        queue_ref: input.refs.queue_ref.clone(),
        model_runtime_ref: input.refs.model_runtime_ref.clone(),
        bundle: bundle.clone(),
        summary_refs,
        local_ai_enabled: input.local_ai_enabled,
        model_runtime_available: input.model_runtime_available,
        queue_available: input.queue_available,
        raw_network_payload_available: false,
        page_content_available: false,
        policy_action_authority: false,
        adapter_action_authority: false,
    })
    .map_err(NetworkEndToEndPipelineError::LocalAi)?;

    Ok(NetworkEndToEndPipelineParts {
        capture_ingest,
        evidence_bundle: bundle,
        local_ai_queue,
        ai_detection,
        ai_audit,
        risk_budget,
        policy_mapping,
        adapter_proof,
        action_result,
        retention_delete_export,
    })
}

fn capture_ingest_proof(
    refs: &NetworkEndToEndPipelineRefs,
    summary_refs: &[String],
    evidence_refs: &[String],
) -> Result<NetworkCaptureIngestProof, NetworkEndToEndPipelineError> {
    Ok(NetworkCaptureIngestProof {
        trigger_ref: required_ref(
            &refs.trigger_ref,
            NetworkEndToEndPipelineError::EmptyTriggerRef,
        )?,
        capture_ref: required_ref(
            &refs.capture_ref,
            NetworkEndToEndPipelineError::EmptyCaptureRef,
        )?,
        ingest_ref: required_ref(
            &refs.ingest_ref,
            NetworkEndToEndPipelineError::EmptyIngestRef,
        )?,
        typed_event_ref: required_ref(
            &refs.typed_event_ref,
            NetworkEndToEndPipelineError::EmptyTypedEventRef,
        )?,
        summary_refs: summary_refs.to_vec(),
        evidence_refs: evidence_refs.to_vec(),
        audit_event_ref: required_ref(
            &refs.audit_event_ref,
            NetworkEndToEndPipelineError::EmptyAuditEventRef,
        )?,
        same_product_path: true,
    })
}

fn ai_detection_input(
    refs: &NetworkEndToEndPipelineRefs,
    bundle: &NetworkCrossSliceEvidenceBundle,
) -> Result<NetworkAiDetectionEvaluationInput, NetworkEndToEndPipelineError> {
    let summary_refs = normalized_refs(
        &refs.summary_refs,
        NetworkEndToEndPipelineError::EmptySummaryRef,
    )?;
    let analyzer_alert_refs = normalized_refs(
        &refs.analyzer_alert_refs,
        NetworkEndToEndPipelineError::EmptyAnalyzerAlertRef,
    )?;
    Ok(NetworkAiDetectionEvaluationInput {
        evaluation_run_ref: refs.ai_evaluation_run_ref.clone(),
        fixture_set_ref: refs.ai_fixture_set_ref.clone(),
        model_card_ref: refs.ai_model_card_ref.clone(),
        model_version_ref: refs.ai_model_version_ref.clone(),
        baseline_ref: refs.ai_baseline_ref.clone(),
        cases: vec![NetworkAiDetectionFixtureCase {
            detection_ref: refs.ai_detection_ref.clone(),
            fixture_ref: refs.ai_fixture_ref.clone(),
            summary_ref: summary_refs[0].clone(),
            evidence_refs: bundle.evidence_refs.clone(),
            analyzer_alert_refs,
            expected_label: NetworkAiDetectionLabel::VpnProxyTunnel,
            predicted_label: NetworkAiDetectionLabel::VpnProxyTunnel,
            confidence_basis_points: 9_100,
            baseline_confidence_basis_points: 9_000,
            risk_level: NetworkAiDetectionRiskLevel::High,
            input_kinds: vec![
                NetworkAiDetectionInputKind::SummaryRefs,
                NetworkAiDetectionInputKind::EvidenceRefs,
                NetworkAiDetectionInputKind::AnalyzerAlertRefs,
                NetworkAiDetectionInputKind::FixtureLabel,
            ],
            raw_pcap_input_claimed: false,
            decrypted_payload_claimed: false,
            page_content_claimed: false,
            exact_url_claimed: false,
        }],
        minimum_precision_basis_points: 8_000,
        minimum_recall_basis_points: 8_000,
        maximum_average_drift_basis_points: 500,
        model_execution_claimed: false,
        remote_ai_claimed: false,
        raw_pcap_input_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        exact_url_claimed: false,
        policy_authority_claimed: false,
        adapter_authority_claimed: false,
        enforcement_command_claimed: false,
    })
}

fn ai_audit_input(
    refs: &NetworkEndToEndPipelineRefs,
    detection: &NetworkAiDetectionEvaluationProof,
) -> NetworkAiAuditReportInput {
    NetworkAiAuditReportInput {
        audit_report_ref: refs.ai_audit_report_ref.clone(),
        narrative_template_ref: refs.ai_narrative_template_ref.clone(),
        model_version_ref: refs.ai_model_version_ref.clone(),
        policy_context_ref: refs.policy_context_ref.clone(),
        detection_results: detection.results.clone(),
        parent_rule_refs: vec![refs.parent_rule_ref.clone()],
        remote_ai_claimed: false,
        raw_pcap_input_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        exact_url_claimed: false,
        private_message_claimed: false,
        search_query_claimed: false,
        policy_authority_claimed: false,
        adapter_authority_claimed: false,
        enforcement_command_claimed: false,
    }
}

fn risk_budget_input(
    refs: &NetworkEndToEndPipelineRefs,
    ai_audit: &NetworkAiAuditReport,
) -> NetworkRiskBudgetThresholdInput {
    NetworkRiskBudgetThresholdInput {
        evaluation_ref: refs.risk_evaluation_ref.clone(),
        child_profile_ref: refs.child_profile_ref.clone(),
        risk_budget_ref: refs.risk_budget_ref.clone(),
        cascade_ref: refs.cascade_ref.clone(),
        age_band: NetworkRiskBudgetAgeBand::ThirteenToFifteen,
        profile_risk_weight_points: 15,
        thresholds: NetworkRiskBudgetThresholds {
            monitor_points: 10,
            ask_parent_points: 25,
            warn_child_points: 45,
            limit_points: 65,
            block_points: 80,
            max_points: 100,
        },
        household_policy: NetworkRiskBudgetHouseholdPolicy {
            household_policy_ref: refs.household_policy_ref.clone(),
            parent_rule_refs: vec![refs.parent_rule_ref.clone()],
            child_warning_allowed: true,
            limit_policy_allowed: true,
            block_policy_allowed: true,
            strict_block_policy_enabled: true,
            safe_behavior_credit_cap_points: 0,
            safe_behavior_credit_expiry_ref: None,
            safe_behavior_audit_reason_ref: None,
            safe_behavior_ui_explanation_ref: None,
        },
        signals: vec![NetworkRiskBudgetSignal {
            signal_ref: refs.ai_audit_report_ref.clone(),
            audit_report: ai_audit.clone(),
            evidence_tier: NetworkRiskBudgetEvidenceTier::AiAuditWithCitations,
            base_risk_points: 70,
            safe_behavior_credit_points: 0,
            known_safe: false,
            expected_activity: false,
            signature_only: false,
        }],
        prior_events: vec![NetworkRiskBudgetPriorEvent {
            event_ref: refs.typed_event_ref.clone(),
            risk_points: 10,
            within_window: true,
            same_household_rule: true,
        }],
        adapter_proof_state: NetworkRiskBudgetAdapterProofState::Missing,
        raw_pcap_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        exact_url_claimed: false,
        private_message_claimed: false,
        search_query_claimed: false,
        policy_authority_claimed: false,
        adapter_authority_claimed: false,
        enforcement_command_claimed: false,
        extra_privilege_grant_claimed: false,
        allowance_grant_claimed: false,
        time_grant_claimed: false,
    }
}

fn dns_adapter_input(
    input: &NetworkEndToEndPipelineInput,
    policy_mapping: NetworkEvidencePolicyMapping,
) -> NetworkDnsAdapterProofInput {
    NetworkDnsAdapterProofInput {
        dns_adapter_plan_ref: input.refs.dns_adapter_plan_ref.clone(),
        policy_mapping,
        requested_action: NetworkDnsAdapterAction::Block,
        target_domain: input.refs.target_domain.clone(),
        redirect_target_domain: None,
        capability_state: input.adapter_capability_state,
        adapter_authorization_ref: input.refs.adapter_authorization_ref.clone(),
        adapter_capability_proof_ref: input.refs.adapter_capability_proof_ref.clone(),
        apply_artifact_ref: input.refs.apply_artifact_ref.clone(),
        result_artifact_ref: input.refs.result_artifact_ref.clone(),
        rollback_artifact_ref: input.refs.rollback_artifact_ref.clone(),
        audit_event_ref: Some(input.refs.audit_event_ref.clone()),
        dry_run: input.adapter_dry_run,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
    }
}

fn action_result_input(
    input: &NetworkEndToEndPipelineInput,
    policy_mapping: NetworkEvidencePolicyMapping,
    adapter_proof: &NetworkDnsAdapterProof,
) -> NetworkActionResultInput {
    NetworkActionResultInput {
        action_result_ref: input.refs.action_result_ref.clone(),
        policy_mapping,
        requested_action: NetworkActionResultRequestedAction::Block,
        target_kind: NetworkActionResultTargetKind::Domain,
        target_ref: input.refs.target_domain.clone(),
        capability_state: action_capability_state(input.adapter_capability_state),
        adapter_proof_state: action_adapter_proof_state(adapter_proof.proof_state),
        adapter_proof_ref: input.refs.adapter_capability_proof_ref.clone(),
        apply_artifact_ref: input.refs.apply_artifact_ref.clone(),
        result_artifact_ref: input.refs.result_artifact_ref.clone(),
        audit_event_ref: Some(input.refs.audit_event_ref.clone()),
        dry_run: input.adapter_dry_run,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        host_mutation_claimed: false,
        enforcement_command_published: false,
    }
}

fn action_capability_state(
    state: NetworkDnsAdapterCapabilityState,
) -> NetworkActionResultCapabilityState {
    match state {
        NetworkDnsAdapterCapabilityState::Supported => {
            NetworkActionResultCapabilityState::Supported
        }
        NetworkDnsAdapterCapabilityState::ManualRequired => {
            NetworkActionResultCapabilityState::ManualRequired
        }
        NetworkDnsAdapterCapabilityState::Unavailable => {
            NetworkActionResultCapabilityState::Unavailable
        }
    }
}

fn action_adapter_proof_state(
    state: NetworkDnsAdapterProofState,
) -> NetworkActionResultAdapterProofState {
    match state {
        NetworkDnsAdapterProofState::ApplyReady => NetworkActionResultAdapterProofState::ApplyReady,
        NetworkDnsAdapterProofState::DryRun => NetworkActionResultAdapterProofState::DryRun,
        NetworkDnsAdapterProofState::ManualRequired => {
            NetworkActionResultAdapterProofState::ManualRequired
        }
        NetworkDnsAdapterProofState::Unavailable => {
            NetworkActionResultAdapterProofState::Unavailable
        }
    }
}

fn retention_delete_export(
    refs: &NetworkEndToEndPipelineRefs,
    evidence_refs: &[String],
) -> Result<NetworkRetentionDeleteExportProof, NetworkEndToEndPipelineError> {
    Ok(NetworkRetentionDeleteExportProof {
        retention_ref: required_ref(
            &refs.retention_ref,
            NetworkEndToEndPipelineError::EmptyRetentionRef,
        )?,
        deletion_ref: required_ref(
            &refs.deletion_ref,
            NetworkEndToEndPipelineError::EmptyDeletionRef,
        )?,
        export_ref: required_ref(
            &refs.export_ref,
            NetworkEndToEndPipelineError::EmptyExportRef,
        )?,
        tombstone_ref: required_ref(
            &refs.tombstone_ref,
            NetworkEndToEndPipelineError::EmptyTombstoneRef,
        )?,
        audit_event_ref: required_ref(
            &refs.audit_event_ref,
            NetworkEndToEndPipelineError::EmptyAuditEventRef,
        )?,
        portal_read_model_ref: required_ref(
            &refs.portal_read_model_ref,
            NetworkEndToEndPipelineError::EmptyPortalReadModelRef,
        )?,
        evidence_refs: evidence_refs.to_vec(),
        same_product_path: true,
    })
}

fn validate_refs(refs: &NetworkEndToEndPipelineRefs) -> Result<(), NetworkEndToEndPipelineError> {
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

fn reject_unsupported_claims(
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

fn normalized_refs(
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

fn required_ref(
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
