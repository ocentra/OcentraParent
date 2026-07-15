use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::action_result::*;
use ocentra_network_evidence::bundle::*;
use ocentra_network_evidence::cascade::*;
use ocentra_network_evidence::dns::types::*;
use ocentra_network_evidence::dns_adapter::*;
use ocentra_network_evidence::local_ai_queue::*;
use ocentra_network_evidence::pipeline::*;
use ocentra_network_evidence::policy::*;
use ocentra_network_evidence::risk_budget::*;

#[test]
fn end_to_end_pipeline_carries_refs_from_trigger_to_retention_export() {
    let proof = prove_network_end_to_end_pipeline(pipeline_input())
        .expect_value("pipeline proof should build");

    assert_eq!(proof.trigger_ref, "trigger:network:flow:1");
    assert_eq!(proof.capture_ref, "capture:network:bounded:1");
    assert_eq!(proof.ingest_ref, "ingest:network:metadata:1");
    assert_eq!(proof.typed_event_ref, "event:network:flow-observed:1");
    assert_eq!(proof.capture_ingest.trigger_ref, proof.trigger_ref);
    assert_eq!(proof.capture_ingest.capture_ref, proof.capture_ref);
    assert_eq!(proof.capture_ingest.ingest_ref, proof.ingest_ref);
    assert_eq!(proof.capture_ingest.typed_event_ref, proof.typed_event_ref);
    assert_eq!(
        proof.capture_ingest.summary_refs,
        vec!["summary:network:flow:1".to_owned()]
    );
    assert_eq!(
        proof.evidence_bundle.evidence_refs,
        vec!["evidence:network:tunnel:1".to_owned()]
    );
    assert!(proof
        .evidence_bundle
        .next_checks
        .contains(&NetworkCascadeNextCheck::LocalAiReview));
    assert_eq!(
        proof.local_ai_queue.status,
        NetworkLocalAiQueueStatus::Queued
    );
    assert_eq!(
        proof
            .local_ai_queue
            .job
            .as_ref()
            .expect_value("local AI job should be queued")
            .trigger_ref,
        "trigger:network:flow:1"
    );
    assert_eq!(
        proof.ai_audit.cited_detection_refs,
        vec!["ai-detection:network:1".to_owned()]
    );
    assert_eq!(
        proof.ai_audit.cited_evidence_refs,
        vec!["evidence:network:tunnel:1".to_owned()]
    );
    assert_eq!(
        proof.risk_budget.cited_prior_event_refs,
        vec!["event:network:flow-observed:1".to_owned()]
    );
    assert_eq!(
        proof.action_result.result_state,
        NetworkActionResultState::ManualRequired
    );
    assert_eq!(
        proof.action_result.action_result_ref,
        "action-result:network:dns:1"
    );
    assert_eq!(
        proof.action_result.evidence_refs,
        proof.evidence_bundle.evidence_refs
    );
    assert_eq!(
        proof.retention_delete_export.evidence_refs,
        proof.evidence_bundle.evidence_refs
    );
    assert_eq!(
        proof.retention_delete_export.audit_event_ref,
        "audit:network:adapter:1"
    );
    assert!(proof.retention_delete_export.same_product_path);
}

#[test]
fn end_to_end_pipeline_keeps_weak_evidence_non_enforcing() {
    let proof = prove_network_end_to_end_pipeline(pipeline_input())
        .expect_value("pipeline proof should build");

    assert_eq!(
        proof.policy_mapping.mode,
        NetworkEvidencePolicyMode::ParentReview
    );
    assert_eq!(
        proof.adapter_proof.proof_state,
        NetworkDnsAdapterProofState::ManualRequired
    );
    assert!(proof
        .adapter_proof
        .boundary_reasons
        .contains(&NetworkDnsAdapterBoundaryReason::EvidenceGradeBelowApplyThreshold));
    assert!(proof
        .adapter_proof
        .boundary_reasons
        .contains(&NetworkDnsAdapterBoundaryReason::PolicyNotAdapterApproved));
    assert!(!proof.adapter_proof.adapter_apply_authorized);
    assert_eq!(
        proof.action_result.result_state,
        NetworkActionResultState::ManualRequired
    );
    assert!(!proof.action_result.adapter_result_accepted);
    assert!(!proof.action_result.enforcement_command_published);
    assert!(!proof.adapter_proof.enforcement_command_published);
    assert!(!proof.adapter_action_executed);
    assert_eq!(proof.enforcement_commands_published, 0);
    assert!(proof.weak_or_unavailable_evidence_enforcement_blocked);
}

#[test]
fn end_to_end_pipeline_carries_dry_run_action_result_state() {
    let mut input = pipeline_input();
    input.adapter_dry_run = true;

    let proof =
        prove_network_end_to_end_pipeline(input).expect_value("pipeline proof should build");

    assert_eq!(
        proof.adapter_proof.proof_state,
        NetworkDnsAdapterProofState::DryRun
    );
    assert_eq!(
        proof.action_result.result_state,
        NetworkActionResultState::DryRun
    );
    assert!(!proof.action_result.adapter_result_accepted);
    assert_eq!(proof.enforcement_commands_published, 0);
}

#[test]
fn end_to_end_pipeline_carries_unavailable_action_result_state() {
    let mut input = pipeline_input();
    input.adapter_capability_state = NetworkDnsAdapterCapabilityState::Unavailable;

    let proof =
        prove_network_end_to_end_pipeline(input).expect_value("pipeline proof should build");

    assert_eq!(
        proof.adapter_proof.proof_state,
        NetworkDnsAdapterProofState::Unavailable
    );
    assert_eq!(
        proof.action_result.result_state,
        NetworkActionResultState::Unavailable
    );
    assert!(!proof.action_result.adapter_result_accepted);
    assert_eq!(proof.enforcement_commands_published, 0);
}

#[test]
fn end_to_end_pipeline_keeps_unavailable_evidence_non_enforcing() {
    let mut input = pipeline_input();
    input.sources[0].signal_strength = NetworkCascadeSignalStrength::Unavailable;
    input.sources[0].evidence_grade = NetworkEvidenceGrade::D;

    let proof =
        prove_network_end_to_end_pipeline(input).expect_value("pipeline proof should build");

    assert_eq!(
        proof.local_ai_queue.status,
        NetworkLocalAiQueueStatus::NotRecommended
    );
    assert_eq!(
        proof.policy_mapping.mode,
        NetworkEvidencePolicyMode::ObserveOnly
    );
    assert_eq!(
        proof.risk_budget.risk_budget_state,
        NetworkRiskBudgetState::BlockThreshold
    );
    assert_eq!(
        proof.risk_budget.intervention_state,
        NetworkInterventionState::ManualRequired
    );
    assert!(!proof.adapter_proof.adapter_apply_authorized);
    assert_eq!(proof.enforcement_commands_published, 0);
    assert!(proof.weak_or_unavailable_evidence_enforcement_blocked);
}

#[test]
fn end_to_end_pipeline_rejects_ai_ui_and_network_bypass_claims() {
    let mut ai_bypass = pipeline_input();
    ai_bypass.unsupported_claims.ai_policy_authority_claimed = true;
    assert_eq!(
        prove_network_end_to_end_pipeline(ai_bypass),
        Err(NetworkEndToEndPipelineError::AiPolicyAuthorityRejected)
    );

    let mut ui_bypass = pipeline_input();
    ui_bypass.unsupported_claims.ui_policy_authority_claimed = true;
    assert_eq!(
        prove_network_end_to_end_pipeline(ui_bypass),
        Err(NetworkEndToEndPipelineError::UiPolicyAuthorityRejected)
    );

    let mut network_bypass = pipeline_input();
    network_bypass
        .unsupported_claims
        .network_adapter_authority_claimed = true;
    assert_eq!(
        prove_network_end_to_end_pipeline(network_bypass),
        Err(NetworkEndToEndPipelineError::NetworkAdapterAuthorityRejected)
    );
}

fn pipeline_input() -> NetworkEndToEndPipelineInput {
    NetworkEndToEndPipelineInput {
        refs: pipeline_refs(),
        sources: vec![NetworkCrossSliceEvidenceSource {
            source_kind: NetworkCascadeSourceKind::TunnelIndicator,
            signal_strength: NetworkCascadeSignalStrength::WeakHint,
            evidence_grade: NetworkEvidenceGrade::B,
            evidence_ref: "evidence:network:tunnel:1".to_owned(),
            exact_url_available: false,
            decrypted_payload_available: false,
            policy_action_authority: false,
            adapter_action_authority: false,
        }],
        requested_policy_action: NetworkEvidencePolicyAction::Block,
        adapter_capability_state: NetworkDnsAdapterCapabilityState::Supported,
        adapter_dry_run: false,
        local_ai_enabled: true,
        model_runtime_available: true,
        queue_available: true,
        unsupported_claims: NetworkEndToEndUnsupportedClaims {
            raw_network_payload_claimed: false,
            decrypted_payload_claimed: false,
            page_content_claimed: false,
            exact_url_claimed: false,
            ai_policy_authority_claimed: false,
            ui_policy_authority_claimed: false,
            network_adapter_authority_claimed: false,
            enforcement_command_claimed: false,
        },
    }
}

fn pipeline_refs() -> NetworkEndToEndPipelineRefs {
    NetworkEndToEndPipelineRefs {
        trigger_ref: "trigger:network:flow:1".to_owned(),
        capture_ref: "capture:network:bounded:1".to_owned(),
        ingest_ref: "ingest:network:metadata:1".to_owned(),
        typed_event_ref: "event:network:flow-observed:1".to_owned(),
        summary_refs: vec!["summary:network:flow:1".to_owned()],
        analyzer_alert_refs: vec!["alert:network:signature:1".to_owned()],
        queue_job_ref: "queue-job:local-ai:network:1".to_owned(),
        queue_ref: "queue:local-ai:network".to_owned(),
        model_runtime_ref: "model-runtime:local:network:1".to_owned(),
        ai_detection_ref: "ai-detection:network:1".to_owned(),
        ai_fixture_ref: "fixture:network:ai:1".to_owned(),
        ai_evaluation_run_ref: "ai-evaluation:network:1".to_owned(),
        ai_fixture_set_ref: "fixture-set:network:ai".to_owned(),
        ai_model_card_ref: "model-card:network:ai".to_owned(),
        ai_model_version_ref: "model-version:network:ai:v1".to_owned(),
        ai_baseline_ref: "baseline:network:ai:v1".to_owned(),
        ai_audit_report_ref: "ai-audit:network:1".to_owned(),
        ai_narrative_template_ref: "narrative-template:network:parent:1".to_owned(),
        policy_context_ref: "policy-context:network:1".to_owned(),
        policy_decision_ref: "policy-decision:network:1".to_owned(),
        parent_rule_ref: "parent-rule:network:block-vpn".to_owned(),
        risk_evaluation_ref: "risk-evaluation:network:1".to_owned(),
        child_profile_ref: "child-profile:teen:1".to_owned(),
        risk_budget_ref: "risk-budget:household:network".to_owned(),
        cascade_ref: "cascade:network:1".to_owned(),
        household_policy_ref: "household-policy:network:1".to_owned(),
        dns_adapter_plan_ref: "dns-adapter-plan:network:1".to_owned(),
        action_result_ref: "action-result:network:dns:1".to_owned(),
        target_domain: "vpn.example.test".to_owned(),
        adapter_authorization_ref: Some("adapter-auth:network:dns:1".to_owned()),
        adapter_capability_proof_ref: Some("adapter-capability:network:dns:1".to_owned()),
        apply_artifact_ref: Some("adapter-apply:network:dns:1".to_owned()),
        result_artifact_ref: Some("adapter-result:network:dns:1".to_owned()),
        rollback_artifact_ref: Some("adapter-rollback:network:dns:1".to_owned()),
        audit_event_ref: "audit:network:adapter:1".to_owned(),
        portal_read_model_ref: "portal-read-model:network:1".to_owned(),
        retention_ref: "retention:network:window:1".to_owned(),
        deletion_ref: "delete:network:tombstone:1".to_owned(),
        export_ref: "export:network:bundle:1".to_owned(),
        tombstone_ref: "tombstone:network:1".to_owned(),
    }
}
