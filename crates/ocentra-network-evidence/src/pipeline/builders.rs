use super::{
    NetworkActionResultAdapterProofState, NetworkActionResultCapabilityState,
    NetworkActionResultInput, NetworkActionResultRequestedAction, NetworkActionResultTargetKind,
    NetworkAiAuditReport, NetworkAiAuditReportInput, NetworkAiDetectionEvaluationInput,
    NetworkAiDetectionEvaluationProof, NetworkAiDetectionFixtureCase, NetworkAiDetectionInputKind,
    NetworkAiDetectionLabel, NetworkAiDetectionRiskLevel, NetworkCrossSliceEvidenceBundle,
    NetworkDnsAdapterAction, NetworkDnsAdapterCapabilityState, NetworkDnsAdapterProof,
    NetworkDnsAdapterProofInput, NetworkDnsAdapterProofState, NetworkEndToEndPipelineError,
    NetworkEndToEndPipelineInput, NetworkEndToEndPipelineParts, NetworkEndToEndPipelineRefs,
    NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingInput, NetworkLocalAiQueueInput,
    NetworkRiskBudgetAdapterProofState, NetworkRiskBudgetAgeBand, NetworkRiskBudgetEvaluation,
    NetworkRiskBudgetEvidenceTier, NetworkRiskBudgetHouseholdPolicy, NetworkRiskBudgetPriorEvent,
    NetworkRiskBudgetSignal, NetworkRiskBudgetThresholdInput, NetworkRiskBudgetThresholds,
};
use crate::{
    build_network_ai_audit_report, build_network_cross_slice_evidence_bundle,
    evaluate_network_ai_detection_fixtures, evaluate_network_risk_budget_threshold,
    map_network_evidence_grade_to_policy, plan_network_action_result_state,
    plan_network_dns_adapter_proof, plan_network_local_ai_queue,
    NetworkCrossSliceEvidenceBundleInput,
};

use super::proofs::{capture_ingest_proof, retention_delete_export};
use super::validation::normalized_refs;

pub(super) fn build_pipeline_parts(
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
