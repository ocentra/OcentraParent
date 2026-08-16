use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::dns::types::*;
use ocentra_network_evidence::policy::*;
use ocentra_network_evidence::windows_wfp_gate::*;

#[test]
fn windows_wfp_gate_allows_lab_proof_ready_with_signed_permissioned_artifacts() {
    let proof = plan_network_windows_wfp_gate(lab_ready_input())
        .expect_value("complete WFP gate proof should become lab-proof ready");

    assert_eq!(proof.gate_state, NetworkWindowsWfpGateState::LabProofReady);
    assert_eq!(proof.target_ref, "network-target-ref-39");
    assert_eq!(proof.wfp_provider_ref, "wfp-provider-ref-39");
    assert_eq!(proof.wfp_layer_ref, "wfp-layer-ref-39");
    assert_eq!(proof.policy_decision_ref, "policy-decision-network-39");
    assert_eq!(proof.parent_rule_ref, "parent-rule-network-39");
    assert_eq!(proof.evidence_refs, vec!["network-evidence-39"]);
    assert_eq!(proof.boundary_reasons, Vec::new());
    assert_eq!(proof.missing_required_artifacts, Vec::new());
    assert_eq!(
        proof.administrator_permission_proof_ref,
        Some("administrator-permission-proof-ref-39".to_owned())
    );
    assert_eq!(
        proof.driver_signing_proof_ref,
        Some("driver-signing-proof-ref-39".to_owned())
    );
    assert_eq!(
        proof.driver_package_proof_ref,
        Some("driver-package-proof-ref-39".to_owned())
    );
    assert_eq!(
        proof.provider_registration_plan_ref,
        Some("provider-registration-plan-ref-39".to_owned())
    );
    assert_eq!(
        proof.layer_capability_matrix_ref,
        Some("layer-capability-matrix-ref-39".to_owned())
    );
    assert_eq!(
        proof.rollback_plan_ref,
        Some("rollback-plan-ref-39".to_owned())
    );
    assert_eq!(
        proof.lab_result_artifact_ref,
        Some("lab-result-artifact-ref-39".to_owned())
    );
    assert_eq!(
        proof.audit_event_ref,
        Some("wfp-audit-event-ref-39".to_owned())
    );
    assert!(proof.wfp_lab_proof_ready);
    assert!(!proof.adapter_apply_authorized);
    assert!(!proof.enforcement_command_published);
    assert!(!proof.live_driver_install_claimed);
    assert!(!proof.callout_registration_claimed);
    assert!(!proof.packet_block_claimed);
    assert!(!proof.kernel_payload_inspection_claimed);
    assert!(!proof.exact_url_available);
    assert!(!proof.decrypted_payload_available);
    assert!(!proof.page_content_available);
}

#[test]
fn windows_wfp_gate_research_only_is_non_executable_without_artifacts() {
    let proof = plan_network_windows_wfp_gate(NetworkWindowsWfpGateInput {
        research_only: true,
        administrator_permission_proof_ref: None,
        driver_signing_proof_ref: None,
        driver_package_proof_ref: None,
        provider_registration_plan_ref: None,
        layer_capability_matrix_ref: None,
        rollback_plan_ref: None,
        lab_result_artifact_ref: None,
        audit_event_ref: None,
        ..lab_ready_input()
    })
    .expect_value("research-only WFP gate should be allowed without authority artifacts");

    assert_eq!(proof.gate_state, NetworkWindowsWfpGateState::ResearchOnly);
    assert_eq!(
        proof.boundary_reasons,
        vec![
            NetworkWindowsWfpGateBoundaryReason::ResearchOnlyRequested,
            NetworkWindowsWfpGateBoundaryReason::MissingRequiredArtifact
        ]
    );
    assert_eq!(
        proof.missing_required_artifacts,
        vec![
            NetworkWindowsWfpRequiredArtifact::AdministratorPermissionProof,
            NetworkWindowsWfpRequiredArtifact::DriverSigningProof,
            NetworkWindowsWfpRequiredArtifact::DriverPackageProof,
            NetworkWindowsWfpRequiredArtifact::ProviderRegistrationPlan,
            NetworkWindowsWfpRequiredArtifact::LayerCapabilityMatrix,
            NetworkWindowsWfpRequiredArtifact::RollbackPlan,
            NetworkWindowsWfpRequiredArtifact::LabResultArtifact,
            NetworkWindowsWfpRequiredArtifact::AuditEvent
        ]
    );
    assert!(!proof.wfp_lab_proof_ready);
    assert!(!proof.adapter_apply_authorized);
}

#[test]
fn windows_wfp_gate_requires_signed_permissioned_artifacts_before_lab_readiness() {
    let proof = plan_network_windows_wfp_gate(NetworkWindowsWfpGateInput {
        administrator_permission_proof_ref: None,
        driver_signing_proof_ref: None,
        driver_package_proof_ref: None,
        provider_registration_plan_ref: None,
        layer_capability_matrix_ref: None,
        rollback_plan_ref: None,
        lab_result_artifact_ref: None,
        audit_event_ref: None,
        ..lab_ready_input()
    })
    .expect_value("missing WFP authority artifacts should stay manual-required");

    assert_eq!(proof.gate_state, NetworkWindowsWfpGateState::ManualRequired);
    assert_eq!(
        proof.boundary_reasons,
        vec![NetworkWindowsWfpGateBoundaryReason::MissingRequiredArtifact]
    );
    assert_eq!(proof.missing_required_artifacts.len(), 8);
    assert!(!proof.wfp_lab_proof_ready);
    assert!(!proof.adapter_apply_authorized);
}

#[test]
fn windows_wfp_gate_routes_weak_or_non_block_policy_to_manual_required() {
    let weak = plan_network_windows_wfp_gate(NetworkWindowsWfpGateInput {
        policy_mapping: policy_mapping(NetworkEvidenceGrade::B, NetworkEvidencePolicyAction::Block),
        ..lab_ready_input()
    })
    .expect_value("grade B block policy handoff should not become WFP lab-ready");

    assert_eq!(weak.gate_state, NetworkWindowsWfpGateState::ManualRequired);
    assert_eq!(
        weak.boundary_reasons,
        vec![
            NetworkWindowsWfpGateBoundaryReason::EvidenceGradeBelowProofThreshold,
            NetworkWindowsWfpGateBoundaryReason::PolicyNotWfpApproved
        ]
    );

    let limit = plan_network_windows_wfp_gate(NetworkWindowsWfpGateInput {
        policy_mapping: policy_mapping(NetworkEvidenceGrade::A, NetworkEvidencePolicyAction::Limit),
        ..lab_ready_input()
    })
    .expect_value("non-block mapped actions should stay outside the WFP proof-ready boundary");
    assert_eq!(limit.gate_state, NetworkWindowsWfpGateState::ManualRequired);
    assert_eq!(
        limit.boundary_reasons,
        vec![NetworkWindowsWfpGateBoundaryReason::PolicyNotWfpApproved]
    );
}

#[test]
fn windows_wfp_gate_marks_manual_required_or_unavailable_capability_without_commands() {
    let manual = plan_network_windows_wfp_gate(NetworkWindowsWfpGateInput {
        capability_state: NetworkWindowsWfpGateCapabilityState::ManualRequired,
        ..lab_ready_input()
    })
    .expect_value("manual-required WFP capability should stay reportable");
    assert_eq!(
        manual.gate_state,
        NetworkWindowsWfpGateState::ManualRequired
    );
    assert_eq!(
        manual.boundary_reasons,
        vec![NetworkWindowsWfpGateBoundaryReason::CapabilityManualRequired]
    );
    assert!(!manual.wfp_lab_proof_ready);

    let unavailable = plan_network_windows_wfp_gate(NetworkWindowsWfpGateInput {
        capability_state: NetworkWindowsWfpGateCapabilityState::Unavailable,
        ..lab_ready_input()
    })
    .expect_value("unavailable WFP capability should stay reportable");
    assert_eq!(
        unavailable.gate_state,
        NetworkWindowsWfpGateState::Unavailable
    );
    assert_eq!(
        unavailable.boundary_reasons,
        vec![NetworkWindowsWfpGateBoundaryReason::CapabilityUnavailable]
    );
    assert!(!unavailable.wfp_lab_proof_ready);
}

#[test]
fn windows_wfp_gate_rejects_network_only_content_and_live_wfp_claims() {
    assert_eq!(
        plan_network_windows_wfp_gate(NetworkWindowsWfpGateInput {
            exact_url_claimed: true,
            ..lab_ready_input()
        }),
        Err(NetworkWindowsWfpGateError::ExactUrlClaimRejected)
    );
    assert_eq!(
        plan_network_windows_wfp_gate(NetworkWindowsWfpGateInput {
            decrypted_payload_claimed: true,
            ..lab_ready_input()
        }),
        Err(NetworkWindowsWfpGateError::DecryptedPayloadClaimRejected)
    );
    assert_eq!(
        plan_network_windows_wfp_gate(NetworkWindowsWfpGateInput {
            page_content_claimed: true,
            ..lab_ready_input()
        }),
        Err(NetworkWindowsWfpGateError::PageContentClaimRejected)
    );
    assert_eq!(
        plan_network_windows_wfp_gate(NetworkWindowsWfpGateInput {
            live_driver_install_claimed: true,
            ..lab_ready_input()
        }),
        Err(NetworkWindowsWfpGateError::LiveDriverInstallClaimRejected)
    );
}

#[test]
fn windows_wfp_gate_rejects_callout_packet_kernel_command_and_authority_bypass() {
    assert_eq!(
        plan_network_windows_wfp_gate(NetworkWindowsWfpGateInput {
            callout_registration_claimed: true,
            ..lab_ready_input()
        }),
        Err(NetworkWindowsWfpGateError::CalloutRegistrationClaimRejected)
    );
    assert_eq!(
        plan_network_windows_wfp_gate(NetworkWindowsWfpGateInput {
            packet_block_claimed: true,
            ..lab_ready_input()
        }),
        Err(NetworkWindowsWfpGateError::PacketBlockClaimRejected)
    );
    assert_eq!(
        plan_network_windows_wfp_gate(NetworkWindowsWfpGateInput {
            kernel_payload_inspection_claimed: true,
            ..lab_ready_input()
        }),
        Err(NetworkWindowsWfpGateError::KernelPayloadInspectionClaimRejected)
    );
    assert_eq!(
        plan_network_windows_wfp_gate(NetworkWindowsWfpGateInput {
            command_invocation_claimed: true,
            ..lab_ready_input()
        }),
        Err(NetworkWindowsWfpGateError::CommandInvocationRejected)
    );

    let mut mapping = policy_mapping(NetworkEvidenceGrade::A, NetworkEvidencePolicyAction::Block);
    mapping.adapter_action_authorized = true;
    assert_eq!(
        plan_network_windows_wfp_gate(NetworkWindowsWfpGateInput {
            policy_mapping: mapping,
            ..lab_ready_input()
        }),
        Err(NetworkWindowsWfpGateError::PolicyMappingAuthorityRejected)
    );
}

#[test]
fn windows_wfp_gate_rejects_empty_target_provider_layer_or_artifact_refs() {
    assert_eq!(
        plan_network_windows_wfp_gate(NetworkWindowsWfpGateInput {
            target_ref: " ".to_owned(),
            ..lab_ready_input()
        }),
        Err(NetworkWindowsWfpGateError::EmptyTargetRef)
    );
    assert_eq!(
        plan_network_windows_wfp_gate(NetworkWindowsWfpGateInput {
            wfp_provider_ref: " ".to_owned(),
            ..lab_ready_input()
        }),
        Err(NetworkWindowsWfpGateError::EmptyWfpProviderRef)
    );
    assert_eq!(
        plan_network_windows_wfp_gate(NetworkWindowsWfpGateInput {
            wfp_layer_ref: " ".to_owned(),
            ..lab_ready_input()
        }),
        Err(NetworkWindowsWfpGateError::EmptyWfpLayerRef)
    );
    assert_eq!(
        plan_network_windows_wfp_gate(NetworkWindowsWfpGateInput {
            driver_signing_proof_ref: Some(" ".to_owned()),
            ..lab_ready_input()
        }),
        Err(NetworkWindowsWfpGateError::EmptyRequiredArtifactRef(
            NetworkWindowsWfpRequiredArtifact::DriverSigningProof
        ))
    );
}

fn lab_ready_input() -> NetworkWindowsWfpGateInput {
    NetworkWindowsWfpGateInput {
        wfp_gate_ref: " windows-wfp-gate-ref-39 ".to_owned(),
        policy_mapping: policy_mapping(NetworkEvidenceGrade::A, NetworkEvidencePolicyAction::Block),
        target_ref: " network-target-ref-39 ".to_owned(),
        wfp_provider_ref: " wfp-provider-ref-39 ".to_owned(),
        wfp_layer_ref: " wfp-layer-ref-39 ".to_owned(),
        capability_state: NetworkWindowsWfpGateCapabilityState::LabReady,
        administrator_permission_proof_ref: Some(
            " administrator-permission-proof-ref-39 ".to_owned(),
        ),
        driver_signing_proof_ref: Some(" driver-signing-proof-ref-39 ".to_owned()),
        driver_package_proof_ref: Some(" driver-package-proof-ref-39 ".to_owned()),
        provider_registration_plan_ref: Some(" provider-registration-plan-ref-39 ".to_owned()),
        layer_capability_matrix_ref: Some(" layer-capability-matrix-ref-39 ".to_owned()),
        rollback_plan_ref: Some(" rollback-plan-ref-39 ".to_owned()),
        lab_result_artifact_ref: Some(" lab-result-artifact-ref-39 ".to_owned()),
        audit_event_ref: Some(" wfp-audit-event-ref-39 ".to_owned()),
        research_only: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        live_driver_install_claimed: false,
        callout_registration_claimed: false,
        packet_block_claimed: false,
        kernel_payload_inspection_claimed: false,
        command_invocation_claimed: false,
    }
}

fn policy_mapping(
    evidence_grade: NetworkEvidenceGrade,
    requested_action: NetworkEvidencePolicyAction,
) -> NetworkEvidencePolicyMapping {
    map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
        policy_decision_ref: " policy-decision-network-39 ".to_owned(),
        parent_rule_ref: " parent-rule-network-39 ".to_owned(),
        evidence_refs: vec![
            " network-evidence-39 ".to_owned(),
            "network-evidence-39".to_owned(),
        ],
        local_ai_result_ref: Some(" local-ai-result-ref-39 ".to_owned()),
        evidence_grade,
        requested_action,
        adapter_capability_proof_ref: None,
    })
    .expect_value("policy mapping input should be valid")
}
