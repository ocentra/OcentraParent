use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::android_vpn_service_gate::*;
use ocentra_network_evidence::dns::types::*;
use ocentra_network_evidence::policy::*;

#[test]
fn android_vpn_service_gate_allows_physical_device_ready_without_device_owner_claim() {
    let proof = plan_network_android_vpn_service_gate(physical_device_ready_input(false))
        .expect_value("complete Android VpnService gate should become physical-device proof-ready");

    assert_eq!(
        proof.gate_state,
        NetworkAndroidVpnServiceGateState::PhysicalDeviceProofReady
    );
    assert_eq!(proof.package_ref, "android-package-ref-40");
    assert_eq!(proof.vpn_service_ref, "vpn-service-ref-40");
    assert_eq!(proof.policy_decision_ref, "policy-decision-network-40");
    assert_eq!(proof.parent_rule_ref, "parent-rule-network-40");
    assert_eq!(proof.evidence_refs, vec!["network-evidence-40"]);
    assert_eq!(proof.boundary_reasons, Vec::new());
    assert_eq!(proof.missing_required_artifacts, Vec::new());
    assert!(!proof.device_owner_required);
    assert_eq!(proof.device_owner_proof_ref, None);
    assert!(proof.physical_device_proof_ready);
    assert!(!proof.device_owner_authority_proved);
    assert!(!proof.adapter_apply_authorized);
    assert!(!proof.enforcement_command_published);
    assert!(!proof.emulator_only_product_support_claimed);
    assert!(!proof.live_vpn_tunnel_claimed);
    assert!(!proof.packet_block_claimed);
    assert!(!proof.app_package_correlation_claimed);
    assert!(!proof.exact_url_available);
    assert!(!proof.decrypted_payload_available);
    assert!(!proof.page_content_available);
}

#[test]
fn android_vpn_service_gate_requires_device_owner_proof_when_claimed() {
    let proof = plan_network_android_vpn_service_gate(physical_device_ready_input(true))
        .expect_value("Device Owner-required gate should accept explicit proof ref");
    assert_eq!(
        proof.gate_state,
        NetworkAndroidVpnServiceGateState::PhysicalDeviceProofReady
    );
    assert!(proof.device_owner_required);
    assert!(proof.device_owner_authority_proved);
    assert_eq!(
        proof.device_owner_proof_ref,
        Some("device-owner-proof-ref-40".to_owned())
    );

    let missing = plan_network_android_vpn_service_gate(NetworkAndroidVpnServiceGateInput {
        device_owner_required: true,
        device_owner_proof_ref: None,
        ..physical_device_ready_input(false)
    })
    .expect_value("missing Device Owner proof should stay reportable");
    assert_eq!(
        missing.gate_state,
        NetworkAndroidVpnServiceGateState::ManualRequired
    );
    assert_eq!(
        missing.missing_required_artifacts,
        vec![NetworkAndroidVpnServiceRequiredArtifact::DeviceOwnerProof]
    );
    assert!(!missing.device_owner_authority_proved);
}

#[test]
fn android_vpn_service_gate_research_only_is_non_executable_without_artifacts() {
    let proof = plan_network_android_vpn_service_gate(NetworkAndroidVpnServiceGateInput {
        research_only: true,
        vpn_service_declaration_ref: None,
        user_consent_proof_ref: None,
        physical_device_proof_ref: None,
        package_identity_proof_ref: None,
        virtual_interface_proof_ref: None,
        traffic_observation_proof_ref: None,
        rollback_plan_ref: None,
        audit_event_ref: None,
        ..physical_device_ready_input(false)
    })
    .expect_value("research-only Android VpnService gate should be allowed without artifacts");

    assert_eq!(
        proof.gate_state,
        NetworkAndroidVpnServiceGateState::ResearchOnly
    );
    assert_eq!(
        proof.boundary_reasons,
        vec![
            NetworkAndroidVpnServiceGateBoundaryReason::ResearchOnlyRequested,
            NetworkAndroidVpnServiceGateBoundaryReason::MissingRequiredArtifact
        ]
    );
    assert_eq!(
        proof.missing_required_artifacts,
        vec![
            NetworkAndroidVpnServiceRequiredArtifact::VpnServiceDeclaration,
            NetworkAndroidVpnServiceRequiredArtifact::UserConsentProof,
            NetworkAndroidVpnServiceRequiredArtifact::PhysicalDeviceProof,
            NetworkAndroidVpnServiceRequiredArtifact::PackageIdentityProof,
            NetworkAndroidVpnServiceRequiredArtifact::VirtualInterfaceProof,
            NetworkAndroidVpnServiceRequiredArtifact::TrafficObservationProof,
            NetworkAndroidVpnServiceRequiredArtifact::RollbackPlan,
            NetworkAndroidVpnServiceRequiredArtifact::AuditEvent
        ]
    );
    assert!(!proof.physical_device_proof_ready);
    assert!(!proof.adapter_apply_authorized);
}

#[test]
fn android_vpn_service_gate_routes_weak_or_non_block_policy_to_manual_required() {
    let weak = plan_network_android_vpn_service_gate(NetworkAndroidVpnServiceGateInput {
        policy_mapping: policy_mapping(NetworkEvidenceGrade::B, NetworkEvidencePolicyAction::Block),
        ..physical_device_ready_input(false)
    })
    .expect_value("grade B block policy handoff should not become Android VpnService proof-ready");

    assert_eq!(
        weak.gate_state,
        NetworkAndroidVpnServiceGateState::ManualRequired
    );
    assert_eq!(
        weak.boundary_reasons,
        vec![
            NetworkAndroidVpnServiceGateBoundaryReason::EvidenceGradeBelowProofThreshold,
            NetworkAndroidVpnServiceGateBoundaryReason::PolicyNotVpnServiceApproved
        ]
    );

    let limit = plan_network_android_vpn_service_gate(NetworkAndroidVpnServiceGateInput {
        policy_mapping: policy_mapping(NetworkEvidenceGrade::A, NetworkEvidencePolicyAction::Limit),
        ..physical_device_ready_input(false)
    })
    .expect_value(
        "non-block mapped actions should stay outside the Android VpnService proof boundary",
    );
    assert_eq!(
        limit.gate_state,
        NetworkAndroidVpnServiceGateState::ManualRequired
    );
    assert_eq!(
        limit.boundary_reasons,
        vec![NetworkAndroidVpnServiceGateBoundaryReason::PolicyNotVpnServiceApproved]
    );
}

#[test]
fn android_vpn_service_gate_marks_manual_required_or_unavailable_capability_without_commands() {
    let manual = plan_network_android_vpn_service_gate(NetworkAndroidVpnServiceGateInput {
        capability_state: NetworkAndroidVpnServiceCapabilityState::ManualRequired,
        ..physical_device_ready_input(false)
    })
    .expect_value("manual-required Android VpnService capability should stay reportable");
    assert_eq!(
        manual.gate_state,
        NetworkAndroidVpnServiceGateState::ManualRequired
    );
    assert_eq!(
        manual.boundary_reasons,
        vec![NetworkAndroidVpnServiceGateBoundaryReason::CapabilityManualRequired]
    );
    assert!(!manual.physical_device_proof_ready);

    let unavailable = plan_network_android_vpn_service_gate(NetworkAndroidVpnServiceGateInput {
        capability_state: NetworkAndroidVpnServiceCapabilityState::Unavailable,
        ..physical_device_ready_input(false)
    })
    .expect_value("unavailable Android VpnService capability should stay reportable");
    assert_eq!(
        unavailable.gate_state,
        NetworkAndroidVpnServiceGateState::Unavailable
    );
    assert_eq!(
        unavailable.boundary_reasons,
        vec![NetworkAndroidVpnServiceGateBoundaryReason::CapabilityUnavailable]
    );
    assert!(!unavailable.physical_device_proof_ready);
}

#[test]
fn android_vpn_service_gate_rejects_network_only_content_and_live_android_claims() {
    assert_eq!(
        plan_network_android_vpn_service_gate(NetworkAndroidVpnServiceGateInput {
            exact_url_claimed: true,
            ..physical_device_ready_input(false)
        }),
        Err(NetworkAndroidVpnServiceGateError::ExactUrlClaimRejected)
    );
    assert_eq!(
        plan_network_android_vpn_service_gate(NetworkAndroidVpnServiceGateInput {
            decrypted_payload_claimed: true,
            ..physical_device_ready_input(false)
        }),
        Err(NetworkAndroidVpnServiceGateError::DecryptedPayloadClaimRejected)
    );
    assert_eq!(
        plan_network_android_vpn_service_gate(NetworkAndroidVpnServiceGateInput {
            page_content_claimed: true,
            ..physical_device_ready_input(false)
        }),
        Err(NetworkAndroidVpnServiceGateError::PageContentClaimRejected)
    );
    assert_eq!(
        plan_network_android_vpn_service_gate(NetworkAndroidVpnServiceGateInput {
            emulator_only_product_support_claimed: true,
            ..physical_device_ready_input(false)
        }),
        Err(NetworkAndroidVpnServiceGateError::EmulatorOnlyProductSupportClaimRejected)
    );
}

#[test]
fn android_vpn_service_gate_rejects_live_filtering_package_correlation_and_authority_bypass() {
    assert_eq!(
        plan_network_android_vpn_service_gate(NetworkAndroidVpnServiceGateInput {
            live_vpn_tunnel_claimed: true,
            ..physical_device_ready_input(false)
        }),
        Err(NetworkAndroidVpnServiceGateError::LiveVpnTunnelClaimRejected)
    );
    assert_eq!(
        plan_network_android_vpn_service_gate(NetworkAndroidVpnServiceGateInput {
            packet_block_claimed: true,
            ..physical_device_ready_input(false)
        }),
        Err(NetworkAndroidVpnServiceGateError::PacketBlockClaimRejected)
    );
    assert_eq!(
        plan_network_android_vpn_service_gate(NetworkAndroidVpnServiceGateInput {
            app_package_correlation_claimed: true,
            ..physical_device_ready_input(false)
        }),
        Err(NetworkAndroidVpnServiceGateError::AppPackageCorrelationClaimRejected)
    );

    let mut mapping = policy_mapping(NetworkEvidenceGrade::A, NetworkEvidencePolicyAction::Block);
    mapping.enforcement_command_authorized = true;
    assert_eq!(
        plan_network_android_vpn_service_gate(NetworkAndroidVpnServiceGateInput {
            policy_mapping: mapping,
            ..physical_device_ready_input(false)
        }),
        Err(NetworkAndroidVpnServiceGateError::PolicyMappingAuthorityRejected)
    );
}

#[test]
fn android_vpn_service_gate_rejects_empty_package_service_or_artifact_refs() {
    assert_eq!(
        plan_network_android_vpn_service_gate(NetworkAndroidVpnServiceGateInput {
            package_ref: " ".to_owned(),
            ..physical_device_ready_input(false)
        }),
        Err(NetworkAndroidVpnServiceGateError::EmptyPackageRef)
    );
    assert_eq!(
        plan_network_android_vpn_service_gate(NetworkAndroidVpnServiceGateInput {
            vpn_service_ref: " ".to_owned(),
            ..physical_device_ready_input(false)
        }),
        Err(NetworkAndroidVpnServiceGateError::EmptyVpnServiceRef)
    );
    assert_eq!(
        plan_network_android_vpn_service_gate(NetworkAndroidVpnServiceGateInput {
            physical_device_proof_ref: Some(" ".to_owned()),
            ..physical_device_ready_input(false)
        }),
        Err(NetworkAndroidVpnServiceGateError::EmptyRequiredArtifactRef(
            NetworkAndroidVpnServiceRequiredArtifact::PhysicalDeviceProof
        ))
    );
}

fn physical_device_ready_input(device_owner_required: bool) -> NetworkAndroidVpnServiceGateInput {
    NetworkAndroidVpnServiceGateInput {
        android_vpn_service_gate_ref: " android-vpn-service-gate-ref-40 ".to_owned(),
        policy_mapping: policy_mapping(NetworkEvidenceGrade::A, NetworkEvidencePolicyAction::Block),
        package_ref: " android-package-ref-40 ".to_owned(),
        vpn_service_ref: " vpn-service-ref-40 ".to_owned(),
        capability_state: NetworkAndroidVpnServiceCapabilityState::PhysicalDeviceReady,
        vpn_service_declaration_ref: Some(" vpn-service-declaration-ref-40 ".to_owned()),
        user_consent_proof_ref: Some(" user-consent-proof-ref-40 ".to_owned()),
        physical_device_proof_ref: Some(" physical-device-proof-ref-40 ".to_owned()),
        package_identity_proof_ref: Some(" package-identity-proof-ref-40 ".to_owned()),
        virtual_interface_proof_ref: Some(" virtual-interface-proof-ref-40 ".to_owned()),
        traffic_observation_proof_ref: Some(" traffic-observation-proof-ref-40 ".to_owned()),
        rollback_plan_ref: Some(" rollback-plan-ref-40 ".to_owned()),
        audit_event_ref: Some(" android-vpn-audit-event-ref-40 ".to_owned()),
        device_owner_required,
        device_owner_proof_ref: if device_owner_required {
            Some(" device-owner-proof-ref-40 ".to_owned())
        } else {
            None
        },
        research_only: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        emulator_only_product_support_claimed: false,
        live_vpn_tunnel_claimed: false,
        packet_block_claimed: false,
        app_package_correlation_claimed: false,
    }
}

fn policy_mapping(
    evidence_grade: NetworkEvidenceGrade,
    requested_action: NetworkEvidencePolicyAction,
) -> NetworkEvidencePolicyMapping {
    map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
        policy_decision_ref: " policy-decision-network-40 ".to_owned(),
        parent_rule_ref: " parent-rule-network-40 ".to_owned(),
        evidence_refs: vec![
            " network-evidence-40 ".to_owned(),
            "network-evidence-40".to_owned(),
        ],
        local_ai_result_ref: Some(" local-ai-result-ref-40 ".to_owned()),
        evidence_grade,
        requested_action,
        adapter_capability_proof_ref: None,
    })
    .expect_value("policy mapping input should be valid")
}
