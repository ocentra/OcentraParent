use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::dns::types::*;
use ocentra_network_evidence::linux_adapter_gate::*;
use ocentra_network_evidence::policy::*;

#[test]
fn linux_adapter_gate_allows_distro_proof_ready_with_rollback_and_audit_refs() {
    let proof =
        plan_network_linux_adapter_gate(distro_ready_input(NetworkLinuxAdapterKind::Nftables))
            .expect_value("complete Linux adapter gate should become distro-proof ready");

    assert_eq!(
        proof.gate_state,
        NetworkLinuxAdapterGateState::DistroProofReady
    );
    assert_eq!(proof.adapter_kind, NetworkLinuxAdapterKind::Nftables);
    assert_eq!(proof.distro_ref, "linux-distro-ref-42");
    assert_eq!(proof.kernel_ref, "linux-kernel-ref-42");
    assert_eq!(proof.policy_decision_ref, "policy-decision-network-42");
    assert_eq!(proof.parent_rule_ref, "parent-rule-network-42");
    assert_eq!(proof.evidence_refs, vec!["network-evidence-42"]);
    assert_eq!(proof.boundary_reasons, Vec::new());
    assert_eq!(proof.missing_required_artifacts, Vec::new());
    assert!(proof.distro_proof_ready);
    assert!(!proof.adapter_apply_authorized);
    assert!(!proof.enforcement_command_published);
    assert!(!proof.generic_linux_support_claimed);
    assert!(!proof.live_adapter_install_claimed);
    assert!(!proof.packet_filtering_claimed);
    assert!(!proof.kernel_hook_loaded_claimed);
    assert!(!proof.tun_interface_mutation_claimed);
    assert!(!proof.service_manager_install_claimed);
    assert!(!proof.exact_url_available);
    assert!(!proof.decrypted_payload_available);
    assert!(!proof.page_content_available);
}

#[test]
fn linux_adapter_gate_preserves_selected_ebpf_and_tun_adapter_kinds() {
    let ebpf = plan_network_linux_adapter_gate(distro_ready_input(NetworkLinuxAdapterKind::Ebpf))
        .expect_value("eBPF proof gate should preserve adapter kind");
    assert_eq!(ebpf.adapter_kind, NetworkLinuxAdapterKind::Ebpf);
    assert_eq!(
        ebpf.gate_state,
        NetworkLinuxAdapterGateState::DistroProofReady
    );

    let tun = plan_network_linux_adapter_gate(distro_ready_input(NetworkLinuxAdapterKind::Tun))
        .expect_value("TUN proof gate should preserve adapter kind");
    assert_eq!(tun.adapter_kind, NetworkLinuxAdapterKind::Tun);
    assert_eq!(
        tun.gate_state,
        NetworkLinuxAdapterGateState::DistroProofReady
    );
}

#[test]
fn linux_adapter_gate_research_only_is_non_executable_without_artifacts() {
    let proof = plan_network_linux_adapter_gate(NetworkLinuxAdapterGateInput {
        research_only: true,
        distro_kernel_proof_ref: None,
        permission_proof_ref: None,
        adapter_api_capability_proof_ref: None,
        adapter_plan_proof_ref: None,
        service_manager_scope_proof_ref: None,
        rollback_plan_ref: None,
        lab_result_artifact_ref: None,
        audit_event_ref: None,
        ..distro_ready_input(NetworkLinuxAdapterKind::Nftables)
    })
    .expect_value("research-only Linux adapter gate should be allowed without artifacts");

    assert_eq!(proof.gate_state, NetworkLinuxAdapterGateState::ResearchOnly);
    assert_eq!(
        proof.boundary_reasons,
        vec![
            NetworkLinuxAdapterGateBoundaryReason::ResearchOnlyRequested,
            NetworkLinuxAdapterGateBoundaryReason::MissingRequiredArtifact
        ]
    );
    assert_eq!(
        proof.missing_required_artifacts,
        vec![
            NetworkLinuxAdapterRequiredArtifact::DistroKernelProof,
            NetworkLinuxAdapterRequiredArtifact::PermissionProof,
            NetworkLinuxAdapterRequiredArtifact::AdapterApiCapabilityProof,
            NetworkLinuxAdapterRequiredArtifact::AdapterPlanProof,
            NetworkLinuxAdapterRequiredArtifact::ServiceManagerScopeProof,
            NetworkLinuxAdapterRequiredArtifact::RollbackPlan,
            NetworkLinuxAdapterRequiredArtifact::LabResultArtifact,
            NetworkLinuxAdapterRequiredArtifact::AuditEvent
        ]
    );
    assert!(!proof.distro_proof_ready);
    assert!(!proof.adapter_apply_authorized);
}

#[test]
fn linux_adapter_gate_routes_weak_or_non_block_policy_to_manual_required() {
    let weak = plan_network_linux_adapter_gate(NetworkLinuxAdapterGateInput {
        policy_mapping: policy_mapping(NetworkEvidenceGrade::B, NetworkEvidencePolicyAction::Block),
        ..distro_ready_input(NetworkLinuxAdapterKind::Nftables)
    })
    .expect_value("grade B block policy handoff should not become Linux proof-ready");

    assert_eq!(
        weak.gate_state,
        NetworkLinuxAdapterGateState::ManualRequired
    );
    assert_eq!(
        weak.boundary_reasons,
        vec![
            NetworkLinuxAdapterGateBoundaryReason::EvidenceGradeBelowProofThreshold,
            NetworkLinuxAdapterGateBoundaryReason::PolicyNotLinuxAdapterApproved
        ]
    );

    let limit = plan_network_linux_adapter_gate(NetworkLinuxAdapterGateInput {
        policy_mapping: policy_mapping(NetworkEvidenceGrade::A, NetworkEvidencePolicyAction::Limit),
        ..distro_ready_input(NetworkLinuxAdapterKind::Nftables)
    })
    .expect_value("non-block mapped actions should stay outside the Linux proof boundary");
    assert_eq!(
        limit.gate_state,
        NetworkLinuxAdapterGateState::ManualRequired
    );
    assert_eq!(
        limit.boundary_reasons,
        vec![NetworkLinuxAdapterGateBoundaryReason::PolicyNotLinuxAdapterApproved]
    );
}

#[test]
fn linux_adapter_gate_marks_manual_required_or_unavailable_capability_without_commands() {
    let manual = plan_network_linux_adapter_gate(NetworkLinuxAdapterGateInput {
        capability_state: NetworkLinuxAdapterCapabilityState::ManualRequired,
        ..distro_ready_input(NetworkLinuxAdapterKind::Nftables)
    })
    .expect_value("manual-required Linux capability should stay reportable");
    assert_eq!(
        manual.gate_state,
        NetworkLinuxAdapterGateState::ManualRequired
    );
    assert_eq!(
        manual.boundary_reasons,
        vec![NetworkLinuxAdapterGateBoundaryReason::CapabilityManualRequired]
    );

    let unavailable = plan_network_linux_adapter_gate(NetworkLinuxAdapterGateInput {
        capability_state: NetworkLinuxAdapterCapabilityState::Unavailable,
        ..distro_ready_input(NetworkLinuxAdapterKind::Nftables)
    })
    .expect_value("unavailable Linux capability should stay reportable");
    assert_eq!(
        unavailable.gate_state,
        NetworkLinuxAdapterGateState::Unavailable
    );
    assert_eq!(
        unavailable.boundary_reasons,
        vec![NetworkLinuxAdapterGateBoundaryReason::CapabilityUnavailable]
    );
}

#[test]
fn linux_adapter_gate_rejects_network_only_content_and_generic_linux_claims() {
    assert_eq!(
        plan_network_linux_adapter_gate(NetworkLinuxAdapterGateInput {
            exact_url_claimed: true,
            ..distro_ready_input(NetworkLinuxAdapterKind::Nftables)
        }),
        Err(NetworkLinuxAdapterGateError::ExactUrlClaimRejected)
    );
    assert_eq!(
        plan_network_linux_adapter_gate(NetworkLinuxAdapterGateInput {
            decrypted_payload_claimed: true,
            ..distro_ready_input(NetworkLinuxAdapterKind::Nftables)
        }),
        Err(NetworkLinuxAdapterGateError::DecryptedPayloadClaimRejected)
    );
    assert_eq!(
        plan_network_linux_adapter_gate(NetworkLinuxAdapterGateInput {
            page_content_claimed: true,
            ..distro_ready_input(NetworkLinuxAdapterKind::Nftables)
        }),
        Err(NetworkLinuxAdapterGateError::PageContentClaimRejected)
    );
    assert_eq!(
        plan_network_linux_adapter_gate(NetworkLinuxAdapterGateInput {
            generic_linux_support_claimed: true,
            ..distro_ready_input(NetworkLinuxAdapterKind::Nftables)
        }),
        Err(NetworkLinuxAdapterGateError::GenericLinuxSupportClaimRejected)
    );
}

#[test]
fn linux_adapter_gate_rejects_live_adapter_claims_and_authority_bypass() {
    assert_eq!(
        plan_network_linux_adapter_gate(NetworkLinuxAdapterGateInput {
            live_adapter_install_claimed: true,
            ..distro_ready_input(NetworkLinuxAdapterKind::Nftables)
        }),
        Err(NetworkLinuxAdapterGateError::LiveAdapterInstallClaimRejected)
    );
    assert_eq!(
        plan_network_linux_adapter_gate(NetworkLinuxAdapterGateInput {
            packet_filtering_claimed: true,
            ..distro_ready_input(NetworkLinuxAdapterKind::Nftables)
        }),
        Err(NetworkLinuxAdapterGateError::PacketFilteringClaimRejected)
    );
    assert_eq!(
        plan_network_linux_adapter_gate(NetworkLinuxAdapterGateInput {
            kernel_hook_loaded_claimed: true,
            ..distro_ready_input(NetworkLinuxAdapterKind::Ebpf)
        }),
        Err(NetworkLinuxAdapterGateError::KernelHookLoadedClaimRejected)
    );
    assert_eq!(
        plan_network_linux_adapter_gate(NetworkLinuxAdapterGateInput {
            tun_interface_mutation_claimed: true,
            ..distro_ready_input(NetworkLinuxAdapterKind::Tun)
        }),
        Err(NetworkLinuxAdapterGateError::TunInterfaceMutationClaimRejected)
    );
    assert_eq!(
        plan_network_linux_adapter_gate(NetworkLinuxAdapterGateInput {
            service_manager_install_claimed: true,
            ..distro_ready_input(NetworkLinuxAdapterKind::Nftables)
        }),
        Err(NetworkLinuxAdapterGateError::ServiceManagerInstallClaimRejected)
    );

    let mut mapping = policy_mapping(NetworkEvidenceGrade::A, NetworkEvidencePolicyAction::Block);
    mapping.enforcement_command_authorized = true;
    assert_eq!(
        plan_network_linux_adapter_gate(NetworkLinuxAdapterGateInput {
            policy_mapping: mapping,
            ..distro_ready_input(NetworkLinuxAdapterKind::Nftables)
        }),
        Err(NetworkLinuxAdapterGateError::PolicyMappingAuthorityRejected)
    );
}

#[test]
fn linux_adapter_gate_rejects_empty_distro_kernel_or_artifact_refs() {
    assert_eq!(
        plan_network_linux_adapter_gate(NetworkLinuxAdapterGateInput {
            distro_ref: " ".to_owned(),
            ..distro_ready_input(NetworkLinuxAdapterKind::Nftables)
        }),
        Err(NetworkLinuxAdapterGateError::EmptyDistroRef)
    );
    assert_eq!(
        plan_network_linux_adapter_gate(NetworkLinuxAdapterGateInput {
            kernel_ref: " ".to_owned(),
            ..distro_ready_input(NetworkLinuxAdapterKind::Nftables)
        }),
        Err(NetworkLinuxAdapterGateError::EmptyKernelRef)
    );
    assert_eq!(
        plan_network_linux_adapter_gate(NetworkLinuxAdapterGateInput {
            adapter_api_capability_proof_ref: Some(" ".to_owned()),
            ..distro_ready_input(NetworkLinuxAdapterKind::Nftables)
        }),
        Err(NetworkLinuxAdapterGateError::EmptyRequiredArtifactRef(
            NetworkLinuxAdapterRequiredArtifact::AdapterApiCapabilityProof
        ))
    );
}

fn distro_ready_input(adapter_kind: NetworkLinuxAdapterKind) -> NetworkLinuxAdapterGateInput {
    NetworkLinuxAdapterGateInput {
        linux_adapter_gate_ref: " linux-adapter-gate-ref-42 ".to_owned(),
        policy_mapping: policy_mapping(NetworkEvidenceGrade::A, NetworkEvidencePolicyAction::Block),
        adapter_kind,
        distro_ref: " linux-distro-ref-42 ".to_owned(),
        kernel_ref: " linux-kernel-ref-42 ".to_owned(),
        capability_state: NetworkLinuxAdapterCapabilityState::DistroReady,
        distro_kernel_proof_ref: Some(" distro-kernel-proof-ref-42 ".to_owned()),
        permission_proof_ref: Some(" permission-proof-ref-42 ".to_owned()),
        adapter_api_capability_proof_ref: Some(" adapter-api-capability-proof-ref-42 ".to_owned()),
        adapter_plan_proof_ref: Some(" adapter-plan-proof-ref-42 ".to_owned()),
        service_manager_scope_proof_ref: Some(" service-manager-scope-proof-ref-42 ".to_owned()),
        rollback_plan_ref: Some(" rollback-plan-ref-42 ".to_owned()),
        lab_result_artifact_ref: Some(" lab-result-artifact-ref-42 ".to_owned()),
        audit_event_ref: Some(" linux-adapter-audit-event-ref-42 ".to_owned()),
        research_only: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        generic_linux_support_claimed: false,
        live_adapter_install_claimed: false,
        packet_filtering_claimed: false,
        kernel_hook_loaded_claimed: false,
        tun_interface_mutation_claimed: false,
        service_manager_install_claimed: false,
    }
}

fn policy_mapping(
    evidence_grade: NetworkEvidenceGrade,
    requested_action: NetworkEvidencePolicyAction,
) -> NetworkEvidencePolicyMapping {
    map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
        policy_decision_ref: " policy-decision-network-42 ".to_owned(),
        parent_rule_ref: " parent-rule-network-42 ".to_owned(),
        evidence_refs: vec![
            " network-evidence-42 ".to_owned(),
            "network-evidence-42".to_owned(),
        ],
        local_ai_result_ref: Some(" local-ai-result-ref-42 ".to_owned()),
        evidence_grade,
        requested_action,
        adapter_capability_proof_ref: None,
    })
    .expect_value("policy mapping input should be valid")
}
