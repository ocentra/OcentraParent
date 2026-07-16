use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::apple_network_extension_gate::*;
use ocentra_network_evidence::dns::types::*;
use ocentra_network_evidence::policy::*;

#[test]
fn apple_network_extension_gate_allows_entitlement_ready_without_supervision_claim() {
    let proof = plan_network_apple_network_extension_gate(apple_device_ready_input(false))
        .expect_value(
            "complete Apple Network Extension gate should become entitlement-proof ready",
        );

    assert_eq!(
        proof.gate_state,
        NetworkAppleNetworkExtensionGateState::AppleEntitlementProofReady
    );
    assert_eq!(proof.platform, NetworkAppleNetworkExtensionPlatform::Ios);
    assert_eq!(proof.bundle_ref, "apple-bundle-ref-41");
    assert_eq!(proof.network_extension_ref, "network-extension-ref-41");
    assert_eq!(proof.policy_decision_ref, "policy-decision-network-41");
    assert_eq!(proof.parent_rule_ref, "parent-rule-network-41");
    assert_eq!(proof.evidence_refs, vec!["network-evidence-41"]);
    assert_eq!(proof.boundary_reasons, Vec::new());
    assert_eq!(proof.missing_required_artifacts, Vec::new());
    assert!(!proof.supervision_required);
    assert_eq!(proof.supervision_or_mdm_proof_ref, None);
    assert!(proof.apple_entitlement_proof_ready);
    assert!(!proof.supervision_authority_proved);
    assert!(!proof.adapter_apply_authorized);
    assert!(!proof.enforcement_command_published);
    assert!(!proof.simulator_only_product_support_claimed);
    assert!(!proof.live_network_extension_claimed);
    assert!(!proof.packet_block_claimed);
    assert!(!proof.app_level_control_claimed);
    assert!(!proof.exact_url_available);
    assert!(!proof.decrypted_payload_available);
    assert!(!proof.page_content_available);
}

#[test]
fn apple_network_extension_gate_requires_supervision_or_mdm_proof_when_claimed() {
    let proof = plan_network_apple_network_extension_gate(apple_device_ready_input(true))
        .expect_value("supervision-required gate should accept explicit proof ref");
    assert_eq!(
        proof.gate_state,
        NetworkAppleNetworkExtensionGateState::AppleEntitlementProofReady
    );
    assert!(proof.supervision_required);
    assert!(proof.supervision_authority_proved);
    assert_eq!(
        proof.supervision_or_mdm_proof_ref,
        Some("supervision-or-mdm-proof-ref-41".to_owned())
    );

    let missing =
        plan_network_apple_network_extension_gate(NetworkAppleNetworkExtensionGateInput {
            supervision_required: true,
            supervision_or_mdm_proof_ref: None,
            ..apple_device_ready_input(false)
        })
        .expect_value("missing supervision proof should stay reportable");
    assert_eq!(
        missing.gate_state,
        NetworkAppleNetworkExtensionGateState::ManualRequired
    );
    assert_eq!(
        missing.missing_required_artifacts,
        vec![NetworkAppleNetworkExtensionRequiredArtifact::SupervisionOrMdmProof]
    );
    assert!(!missing.supervision_authority_proved);
}

#[test]
fn apple_network_extension_gate_research_only_is_non_executable_without_artifacts() {
    let proof = plan_network_apple_network_extension_gate(NetworkAppleNetworkExtensionGateInput {
        research_only: true,
        developer_team_proof_ref: None,
        entitlement_approval_proof_ref: None,
        provisioning_profile_proof_ref: None,
        signing_proof_ref: None,
        device_or_testflight_proof_ref: None,
        network_extension_declaration_ref: None,
        extension_configuration_proof_ref: None,
        rollback_plan_ref: None,
        audit_event_ref: None,
        ..apple_device_ready_input(false)
    })
    .expect_value("research-only Apple Network Extension gate should be allowed without artifacts");

    assert_eq!(
        proof.gate_state,
        NetworkAppleNetworkExtensionGateState::ResearchOnly
    );
    assert_eq!(
        proof.boundary_reasons,
        vec![
            NetworkAppleNetworkExtensionGateBoundaryReason::ResearchOnlyRequested,
            NetworkAppleNetworkExtensionGateBoundaryReason::MissingRequiredArtifact
        ]
    );
    assert_eq!(
        proof.missing_required_artifacts,
        vec![
            NetworkAppleNetworkExtensionRequiredArtifact::DeveloperTeamProof,
            NetworkAppleNetworkExtensionRequiredArtifact::EntitlementApprovalProof,
            NetworkAppleNetworkExtensionRequiredArtifact::ProvisioningProfileProof,
            NetworkAppleNetworkExtensionRequiredArtifact::SigningProof,
            NetworkAppleNetworkExtensionRequiredArtifact::DeviceOrTestFlightProof,
            NetworkAppleNetworkExtensionRequiredArtifact::NetworkExtensionDeclaration,
            NetworkAppleNetworkExtensionRequiredArtifact::ExtensionConfigurationProof,
            NetworkAppleNetworkExtensionRequiredArtifact::RollbackPlan,
            NetworkAppleNetworkExtensionRequiredArtifact::AuditEvent
        ]
    );
    assert!(!proof.apple_entitlement_proof_ready);
    assert!(!proof.adapter_apply_authorized);
}

#[test]
fn apple_network_extension_gate_routes_weak_or_non_block_policy_to_manual_required() {
    let weak = plan_network_apple_network_extension_gate(NetworkAppleNetworkExtensionGateInput {
        policy_mapping: policy_mapping(NetworkEvidenceGrade::B, NetworkEvidencePolicyAction::Block),
        ..apple_device_ready_input(false)
    })
    .expect_value("grade B block policy handoff should not become Apple proof-ready");

    assert_eq!(
        weak.gate_state,
        NetworkAppleNetworkExtensionGateState::ManualRequired
    );
    assert_eq!(
        weak.boundary_reasons,
        vec![
            NetworkAppleNetworkExtensionGateBoundaryReason::EvidenceGradeBelowProofThreshold,
            NetworkAppleNetworkExtensionGateBoundaryReason::PolicyNotNetworkExtensionApproved
        ]
    );

    let limit = plan_network_apple_network_extension_gate(NetworkAppleNetworkExtensionGateInput {
        policy_mapping: policy_mapping(NetworkEvidenceGrade::A, NetworkEvidencePolicyAction::Limit),
        ..apple_device_ready_input(false)
    })
    .expect_value("non-block mapped actions should stay outside the Apple proof boundary");
    assert_eq!(
        limit.gate_state,
        NetworkAppleNetworkExtensionGateState::ManualRequired
    );
    assert_eq!(
        limit.boundary_reasons,
        vec![NetworkAppleNetworkExtensionGateBoundaryReason::PolicyNotNetworkExtensionApproved]
    );
}

#[test]
fn apple_network_extension_gate_marks_manual_required_or_unavailable_capability_without_commands() {
    let manual = plan_network_apple_network_extension_gate(NetworkAppleNetworkExtensionGateInput {
        capability_state: NetworkAppleNetworkExtensionCapabilityState::ManualRequired,
        ..apple_device_ready_input(false)
    })
    .expect_value("manual-required Apple capability should stay reportable");
    assert_eq!(
        manual.gate_state,
        NetworkAppleNetworkExtensionGateState::ManualRequired
    );
    assert_eq!(
        manual.boundary_reasons,
        vec![NetworkAppleNetworkExtensionGateBoundaryReason::CapabilityManualRequired]
    );
    assert!(!manual.apple_entitlement_proof_ready);

    let unavailable =
        plan_network_apple_network_extension_gate(NetworkAppleNetworkExtensionGateInput {
            capability_state: NetworkAppleNetworkExtensionCapabilityState::Unavailable,
            ..apple_device_ready_input(false)
        })
        .expect_value("unavailable Apple capability should stay reportable");
    assert_eq!(
        unavailable.gate_state,
        NetworkAppleNetworkExtensionGateState::Unavailable
    );
    assert_eq!(
        unavailable.boundary_reasons,
        vec![NetworkAppleNetworkExtensionGateBoundaryReason::CapabilityUnavailable]
    );
    assert!(!unavailable.apple_entitlement_proof_ready);
}

#[test]
fn apple_network_extension_gate_rejects_network_only_content_and_simulator_claims() {
    assert_eq!(
        plan_network_apple_network_extension_gate(NetworkAppleNetworkExtensionGateInput {
            exact_url_claimed: true,
            ..apple_device_ready_input(false)
        }),
        Err(NetworkAppleNetworkExtensionGateError::ExactUrlClaimRejected)
    );
    assert_eq!(
        plan_network_apple_network_extension_gate(NetworkAppleNetworkExtensionGateInput {
            decrypted_payload_claimed: true,
            ..apple_device_ready_input(false)
        }),
        Err(NetworkAppleNetworkExtensionGateError::DecryptedPayloadClaimRejected)
    );
    assert_eq!(
        plan_network_apple_network_extension_gate(NetworkAppleNetworkExtensionGateInput {
            page_content_claimed: true,
            ..apple_device_ready_input(false)
        }),
        Err(NetworkAppleNetworkExtensionGateError::PageContentClaimRejected)
    );
    assert_eq!(
        plan_network_apple_network_extension_gate(NetworkAppleNetworkExtensionGateInput {
            simulator_only_product_support_claimed: true,
            ..apple_device_ready_input(false)
        }),
        Err(NetworkAppleNetworkExtensionGateError::SimulatorOnlyProductSupportClaimRejected)
    );
}

#[test]
fn apple_network_extension_gate_rejects_live_filtering_app_control_and_authority_bypass() {
    assert_eq!(
        plan_network_apple_network_extension_gate(NetworkAppleNetworkExtensionGateInput {
            live_network_extension_claimed: true,
            ..apple_device_ready_input(false)
        }),
        Err(NetworkAppleNetworkExtensionGateError::LiveNetworkExtensionClaimRejected)
    );
    assert_eq!(
        plan_network_apple_network_extension_gate(NetworkAppleNetworkExtensionGateInput {
            packet_block_claimed: true,
            ..apple_device_ready_input(false)
        }),
        Err(NetworkAppleNetworkExtensionGateError::PacketBlockClaimRejected)
    );
    assert_eq!(
        plan_network_apple_network_extension_gate(NetworkAppleNetworkExtensionGateInput {
            app_level_control_claimed: true,
            ..apple_device_ready_input(false)
        }),
        Err(NetworkAppleNetworkExtensionGateError::AppLevelControlClaimRejected)
    );

    let mut mapping = policy_mapping(NetworkEvidenceGrade::A, NetworkEvidencePolicyAction::Block);
    mapping.adapter_action_authorized = true;
    assert_eq!(
        plan_network_apple_network_extension_gate(NetworkAppleNetworkExtensionGateInput {
            policy_mapping: mapping,
            ..apple_device_ready_input(false)
        }),
        Err(NetworkAppleNetworkExtensionGateError::PolicyMappingAuthorityRejected)
    );
}

#[test]
fn apple_network_extension_gate_rejects_empty_bundle_extension_or_artifact_refs() {
    assert_eq!(
        plan_network_apple_network_extension_gate(NetworkAppleNetworkExtensionGateInput {
            bundle_ref: " ".to_owned(),
            ..apple_device_ready_input(false)
        }),
        Err(NetworkAppleNetworkExtensionGateError::EmptyBundleRef)
    );
    assert_eq!(
        plan_network_apple_network_extension_gate(NetworkAppleNetworkExtensionGateInput {
            network_extension_ref: " ".to_owned(),
            ..apple_device_ready_input(false)
        }),
        Err(NetworkAppleNetworkExtensionGateError::EmptyNetworkExtensionRef)
    );
    assert_eq!(
        plan_network_apple_network_extension_gate(NetworkAppleNetworkExtensionGateInput {
            entitlement_approval_proof_ref: Some(" ".to_owned()),
            ..apple_device_ready_input(false)
        }),
        Err(
            NetworkAppleNetworkExtensionGateError::EmptyRequiredArtifactRef(
                NetworkAppleNetworkExtensionRequiredArtifact::EntitlementApprovalProof
            )
        )
    );
}

fn apple_device_ready_input(supervision_required: bool) -> NetworkAppleNetworkExtensionGateInput {
    NetworkAppleNetworkExtensionGateInput {
        apple_network_extension_gate_ref: " apple-network-extension-gate-ref-41 ".to_owned(),
        policy_mapping: policy_mapping(NetworkEvidenceGrade::A, NetworkEvidencePolicyAction::Block),
        platform: NetworkAppleNetworkExtensionPlatform::Ios,
        bundle_ref: " apple-bundle-ref-41 ".to_owned(),
        network_extension_ref: " network-extension-ref-41 ".to_owned(),
        capability_state: NetworkAppleNetworkExtensionCapabilityState::AppleDeviceReady,
        developer_team_proof_ref: Some(" developer-team-proof-ref-41 ".to_owned()),
        entitlement_approval_proof_ref: Some(" entitlement-approval-proof-ref-41 ".to_owned()),
        provisioning_profile_proof_ref: Some(" provisioning-profile-proof-ref-41 ".to_owned()),
        signing_proof_ref: Some(" signing-proof-ref-41 ".to_owned()),
        device_or_testflight_proof_ref: Some(" device-or-testflight-proof-ref-41 ".to_owned()),
        network_extension_declaration_ref: Some(
            " network-extension-declaration-ref-41 ".to_owned(),
        ),
        extension_configuration_proof_ref: Some(
            " extension-configuration-proof-ref-41 ".to_owned(),
        ),
        rollback_plan_ref: Some(" rollback-plan-ref-41 ".to_owned()),
        audit_event_ref: Some(" apple-network-extension-audit-event-ref-41 ".to_owned()),
        supervision_required,
        supervision_or_mdm_proof_ref: if supervision_required {
            Some(" supervision-or-mdm-proof-ref-41 ".to_owned())
        } else {
            None
        },
        research_only: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        simulator_only_product_support_claimed: false,
        live_network_extension_claimed: false,
        packet_block_claimed: false,
        app_level_control_claimed: false,
    }
}

fn policy_mapping(
    evidence_grade: NetworkEvidenceGrade,
    requested_action: NetworkEvidencePolicyAction,
) -> NetworkEvidencePolicyMapping {
    map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
        policy_decision_ref: " policy-decision-network-41 ".to_owned(),
        parent_rule_ref: " parent-rule-network-41 ".to_owned(),
        evidence_refs: vec![
            " network-evidence-41 ".to_owned(),
            "network-evidence-41".to_owned(),
        ],
        local_ai_result_ref: Some(" local-ai-result-ref-41 ".to_owned()),
        evidence_grade,
        requested_action,
        adapter_capability_proof_ref: None,
    })
    .expect_value("policy mapping input should be valid")
}
