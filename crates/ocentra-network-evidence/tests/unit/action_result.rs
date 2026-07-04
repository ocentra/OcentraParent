use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::action_result::*;
use ocentra_network_evidence::dns::types::*;
use ocentra_network_evidence::policy::*;

#[test]
fn action_result_accepts_blocked_state_from_policy_and_adapter_result_refs() {
    let proof = plan_network_action_result_state(apply_ready_input(
        NetworkActionResultRequestedAction::Block,
        NetworkActionResultTargetKind::Domain,
    ))
    .expect_value("complete adapter result proof should produce blocked state");

    assert_eq!(proof.result_state, NetworkActionResultState::Blocked);
    assert_eq!(proof.action_result_ref, "network-action-result-53");
    assert_eq!(proof.policy_decision_ref, "policy-decision-network-53");
    assert_eq!(proof.parent_rule_ref, "parent-rule-network-53");
    assert_eq!(proof.evidence_refs, vec!["network-evidence-53"]);
    assert_eq!(
        proof.local_ai_result_ref,
        Some("local-ai-result-ref-53".to_owned())
    );
    assert_eq!(proof.target_kind, NetworkActionResultTargetKind::Domain);
    assert_eq!(proof.target_ref, "domain-target-ref-53");
    assert_eq!(proof.boundary_reasons, Vec::new());
    assert_eq!(proof.missing_required_artifacts, Vec::new());
    assert_eq!(
        proof.adapter_proof_ref,
        Some("adapter-proof-ref-53".to_owned())
    );
    assert_eq!(
        proof.apply_artifact_ref,
        Some("apply-artifact-ref-53".to_owned())
    );
    assert_eq!(
        proof.result_artifact_ref,
        Some("result-artifact-ref-53".to_owned())
    );
    assert_eq!(proof.audit_event_ref, Some("audit-event-ref-53".to_owned()));
    assert!(proof.adapter_result_accepted);
    assert!(!proof.enforcement_command_published);
    assert!(!proof.host_mutation_claimed);
    assert!(!proof.exact_url_available);
    assert!(!proof.decrypted_payload_available);
    assert!(!proof.page_content_available);
}

#[test]
fn action_result_accepts_process_termination_result_without_live_mutation_claims() {
    let proof = plan_network_action_result_state(apply_ready_input(
        NetworkActionResultRequestedAction::TerminateProcess,
        NetworkActionResultTargetKind::Process,
    ))
    .expect_value("process target can record a terminated result state");

    assert_eq!(proof.result_state, NetworkActionResultState::Terminated);
    assert!(proof.adapter_result_accepted);
    assert!(!proof.host_mutation_claimed);
    assert!(!proof.enforcement_command_published);

    let non_process = plan_network_action_result_state(apply_ready_input(
        NetworkActionResultRequestedAction::TerminateProcess,
        NetworkActionResultTargetKind::Domain,
    ))
    .expect_value("non-process terminate target should stay manual-required");
    assert_eq!(
        non_process.result_state,
        NetworkActionResultState::ManualRequired
    );
    assert_eq!(
        non_process.boundary_reasons,
        vec![NetworkActionResultBoundaryReason::TerminateTargetNotProcessOrApp]
    );
    assert!(!non_process.adapter_result_accepted);
}

#[test]
fn action_result_dry_run_is_non_result_without_adapter_artifacts() {
    let proof = plan_network_action_result_state(NetworkActionResultInput {
        dry_run: true,
        adapter_proof_state: NetworkActionResultAdapterProofState::DryRun,
        adapter_proof_ref: None,
        apply_artifact_ref: None,
        result_artifact_ref: None,
        audit_event_ref: None,
        ..apply_ready_input(
            NetworkActionResultRequestedAction::Block,
            NetworkActionResultTargetKind::Domain,
        )
    })
    .expect_value("dry-run result state should be reportable without artifacts");

    assert_eq!(proof.result_state, NetworkActionResultState::DryRun);
    assert_eq!(
        proof.boundary_reasons,
        vec![
            NetworkActionResultBoundaryReason::DryRunRequested,
            NetworkActionResultBoundaryReason::AdapterProofDryRun,
            NetworkActionResultBoundaryReason::MissingRequiredArtifact
        ]
    );
    assert_eq!(
        proof.missing_required_artifacts,
        vec![
            NetworkActionResultRequiredArtifact::AdapterProof,
            NetworkActionResultRequiredArtifact::ApplyArtifact,
            NetworkActionResultRequiredArtifact::ResultArtifact,
            NetworkActionResultRequiredArtifact::AuditEvent
        ]
    );
    assert!(!proof.adapter_result_accepted);
    assert!(!proof.enforcement_command_published);
}

#[test]
fn action_result_routes_weak_policy_or_manual_adapter_state_to_manual_required() {
    let weak = plan_network_action_result_state(NetworkActionResultInput {
        policy_mapping: policy_mapping(NetworkEvidenceGrade::B, NetworkEvidencePolicyAction::Block),
        ..apply_ready_input(
            NetworkActionResultRequestedAction::Block,
            NetworkActionResultTargetKind::Domain,
        )
    })
    .expect_value("weak evidence should not produce blocked state");
    assert_eq!(weak.result_state, NetworkActionResultState::ManualRequired);
    assert_eq!(
        weak.boundary_reasons,
        vec![
            NetworkActionResultBoundaryReason::EvidenceGradeBelowApplyThreshold,
            NetworkActionResultBoundaryReason::PolicyNotAdapterApproved
        ]
    );
    assert!(!weak.adapter_result_accepted);

    let manual = plan_network_action_result_state(NetworkActionResultInput {
        capability_state: NetworkActionResultCapabilityState::ManualRequired,
        adapter_proof_state: NetworkActionResultAdapterProofState::ManualRequired,
        ..apply_ready_input(
            NetworkActionResultRequestedAction::Block,
            NetworkActionResultTargetKind::Domain,
        )
    })
    .expect_value("manual adapter proof state should be reportable");
    assert_eq!(
        manual.result_state,
        NetworkActionResultState::ManualRequired
    );
    assert_eq!(
        manual.boundary_reasons,
        vec![
            NetworkActionResultBoundaryReason::CapabilityManualRequired,
            NetworkActionResultBoundaryReason::AdapterProofManualRequired
        ]
    );
}

#[test]
fn action_result_reports_unavailable_without_accepting_adapter_result() {
    let proof = plan_network_action_result_state(NetworkActionResultInput {
        capability_state: NetworkActionResultCapabilityState::Unavailable,
        adapter_proof_state: NetworkActionResultAdapterProofState::Unavailable,
        ..apply_ready_input(
            NetworkActionResultRequestedAction::Block,
            NetworkActionResultTargetKind::Domain,
        )
    })
    .expect_value("unavailable adapter proof state should be reportable");

    assert_eq!(proof.result_state, NetworkActionResultState::Unavailable);
    assert_eq!(
        proof.boundary_reasons,
        vec![
            NetworkActionResultBoundaryReason::CapabilityUnavailable,
            NetworkActionResultBoundaryReason::AdapterProofUnavailable
        ]
    );
    assert!(!proof.adapter_result_accepted);
}

#[test]
fn action_result_requires_adapter_proof_apply_result_and_audit_refs() {
    let proof = plan_network_action_result_state(NetworkActionResultInput {
        adapter_proof_ref: None,
        apply_artifact_ref: None,
        result_artifact_ref: None,
        audit_event_ref: None,
        ..apply_ready_input(
            NetworkActionResultRequestedAction::Block,
            NetworkActionResultTargetKind::Domain,
        )
    })
    .expect_value("missing action-result artifacts should produce manual-required state");

    assert_eq!(proof.result_state, NetworkActionResultState::ManualRequired);
    assert_eq!(
        proof.boundary_reasons,
        vec![NetworkActionResultBoundaryReason::MissingRequiredArtifact]
    );
    assert_eq!(
        proof.missing_required_artifacts,
        vec![
            NetworkActionResultRequiredArtifact::AdapterProof,
            NetworkActionResultRequiredArtifact::ApplyArtifact,
            NetworkActionResultRequiredArtifact::ResultArtifact,
            NetworkActionResultRequiredArtifact::AuditEvent
        ]
    );
    assert!(!proof.adapter_result_accepted);
}

#[test]
fn action_result_rejects_content_host_mutation_and_command_claims() {
    assert_eq!(
        plan_network_action_result_state(NetworkActionResultInput {
            exact_url_claimed: true,
            ..apply_ready_input(
                NetworkActionResultRequestedAction::Block,
                NetworkActionResultTargetKind::Domain,
            )
        }),
        Err(NetworkActionResultError::ExactUrlClaimRejected)
    );
    assert_eq!(
        plan_network_action_result_state(NetworkActionResultInput {
            decrypted_payload_claimed: true,
            ..apply_ready_input(
                NetworkActionResultRequestedAction::Block,
                NetworkActionResultTargetKind::Domain,
            )
        }),
        Err(NetworkActionResultError::DecryptedPayloadClaimRejected)
    );
    assert_eq!(
        plan_network_action_result_state(NetworkActionResultInput {
            page_content_claimed: true,
            ..apply_ready_input(
                NetworkActionResultRequestedAction::Block,
                NetworkActionResultTargetKind::Domain,
            )
        }),
        Err(NetworkActionResultError::PageContentClaimRejected)
    );
    assert_eq!(
        plan_network_action_result_state(NetworkActionResultInput {
            host_mutation_claimed: true,
            ..apply_ready_input(
                NetworkActionResultRequestedAction::Block,
                NetworkActionResultTargetKind::Domain,
            )
        }),
        Err(NetworkActionResultError::HostMutationClaimRejected)
    );
    assert_eq!(
        plan_network_action_result_state(NetworkActionResultInput {
            enforcement_command_published: true,
            ..apply_ready_input(
                NetworkActionResultRequestedAction::Block,
                NetworkActionResultTargetKind::Domain,
            )
        }),
        Err(NetworkActionResultError::EnforcementCommandPublishedRejected)
    );
}

#[test]
fn action_result_rejects_empty_refs_and_policy_authority_bypass() {
    assert_eq!(
        plan_network_action_result_state(NetworkActionResultInput {
            action_result_ref: " ".to_owned(),
            ..apply_ready_input(
                NetworkActionResultRequestedAction::Block,
                NetworkActionResultTargetKind::Domain,
            )
        }),
        Err(NetworkActionResultError::EmptyActionResultRef)
    );
    assert_eq!(
        plan_network_action_result_state(NetworkActionResultInput {
            target_ref: " ".to_owned(),
            ..apply_ready_input(
                NetworkActionResultRequestedAction::Block,
                NetworkActionResultTargetKind::Domain,
            )
        }),
        Err(NetworkActionResultError::EmptyTargetRef)
    );

    let mut mapping = policy_mapping(NetworkEvidenceGrade::A, NetworkEvidencePolicyAction::Block);
    mapping.enforcement_command_authorized = true;
    assert_eq!(
        plan_network_action_result_state(NetworkActionResultInput {
            policy_mapping: mapping,
            ..apply_ready_input(
                NetworkActionResultRequestedAction::Block,
                NetworkActionResultTargetKind::Domain,
            )
        }),
        Err(NetworkActionResultError::PolicyMappingAuthorityRejected)
    );
}

fn apply_ready_input(
    requested_action: NetworkActionResultRequestedAction,
    target_kind: NetworkActionResultTargetKind,
) -> NetworkActionResultInput {
    let target_ref = match target_kind {
        NetworkActionResultTargetKind::Domain => " domain-target-ref-53 ",
        NetworkActionResultTargetKind::IpEndpoint => " ip-target-ref-53 ",
        NetworkActionResultTargetKind::Process => " process-target-ref-53 ",
        NetworkActionResultTargetKind::App => " app-target-ref-53 ",
    };

    NetworkActionResultInput {
        action_result_ref: " network-action-result-53 ".to_owned(),
        policy_mapping: policy_mapping(NetworkEvidenceGrade::A, NetworkEvidencePolicyAction::Block),
        requested_action,
        target_kind,
        target_ref: target_ref.to_owned(),
        capability_state: NetworkActionResultCapabilityState::Supported,
        adapter_proof_state: NetworkActionResultAdapterProofState::ApplyReady,
        adapter_proof_ref: Some(" adapter-proof-ref-53 ".to_owned()),
        apply_artifact_ref: Some(" apply-artifact-ref-53 ".to_owned()),
        result_artifact_ref: Some(" result-artifact-ref-53 ".to_owned()),
        audit_event_ref: Some(" audit-event-ref-53 ".to_owned()),
        dry_run: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        host_mutation_claimed: false,
        enforcement_command_published: false,
    }
}

fn policy_mapping(
    evidence_grade: NetworkEvidenceGrade,
    requested_action: NetworkEvidencePolicyAction,
) -> NetworkEvidencePolicyMapping {
    map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
        policy_decision_ref: " policy-decision-network-53 ".to_owned(),
        parent_rule_ref: " parent-rule-network-53 ".to_owned(),
        evidence_refs: vec![
            " network-evidence-53 ".to_owned(),
            "network-evidence-53".to_owned(),
        ],
        local_ai_result_ref: Some(" local-ai-result-ref-53 ".to_owned()),
        evidence_grade,
        requested_action,
        adapter_capability_proof_ref: Some(" adapter-capability-proof-ref-53 ".to_owned()),
    })
    .expect_value("policy mapping input should be valid")
}
